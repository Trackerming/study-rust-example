use crate::wallet::{SignInfo, Wallet};
use bitcoin::absolute::LockTime;
use bitcoin::hex::DisplayHex;
use bitcoin::psbt::PsbtSighashType;
use bitcoin::secp256k1::{Message, Secp256k1};
use bitcoin::sighash::SighashCache;
use bitcoin::transaction::Version;
use bitcoin::{
    consensus::serialize, Address, Amount, EcdsaSighashType, OutPoint, PrivateKey, Psbt, ScriptBuf,
    Sequence, Transaction, TxIn, TxOut, Txid, Witness,
};
use std::collections::{BTreeMap, HashMap};
use std::str::FromStr;

pub struct Tx {
    inputs: Vec<TxIn>,
    outputs: Vec<TxOut>,
    input_sign_map: HashMap<usize, SignInfo>,
    lock_time: LockTime,
    version: Version,
    input_sum_satoshi: u64,
    output_sum_satoshi: u64,
}

impl Tx {
    pub fn new(version: Version, lock_time: LockTime) -> Self {
        Tx {
            inputs: vec![],
            outputs: vec![],
            input_sign_map: HashMap::new(),
            lock_time,
            version,
            input_sum_satoshi: 0,
            output_sum_satoshi: 0,
        }
    }

    pub fn add_input(&mut self, prev_out: OutPoint, satoshi: u64) -> usize {
        let tx_in = TxIn {
            previous_output: prev_out,
            sequence: Sequence::ENABLE_LOCKTIME_NO_RBF,
            // 签名之后填充完整
            script_sig: ScriptBuf::default(),
            witness: Witness::default(),
        };
        self.inputs.push(tx_in);
        self.input_sum_satoshi = self.input_sum_satoshi + satoshi;
        return self.inputs.len() - 1;
    }

    pub fn add_inputs(&mut self, utxos: Vec<Utxo>) {
        for utxo in utxos {
            let txid =
                Txid::from_str(utxo.txid).expect(&format!("Txid from str failed {:?}", utxo.txid));
            let index = self.add_input(
                OutPoint {
                    txid,
                    vout: utxo.vout,
                },
                utxo.base_elem.satoshi,
            );
            self.input_sign_map.insert(
                index,
                SignInfo::new(utxo.base_elem.address, utxo.path.to_string()),
            );
        }
    }

    pub fn add_output(&mut self, satoshi: u64, address: Address) {
        self.outputs.push(TxOut {
            value: Amount::from_sat(satoshi),
            script_pubkey: address.script_pubkey(),
        });
        self.output_sum_satoshi = self.output_sum_satoshi + satoshi;
    }

    pub fn add_outputs(&mut self, base_elems: Vec<BaseElem>) {
        for base_elem in base_elems {
            self.add_output(base_elem.satoshi, base_elem.address);
        }
    }

    pub fn to_unsigned_tx(&self) -> Transaction {
        Transaction {
            version: self.version,
            lock_time: self.lock_time,
            input: self.inputs.to_vec(),
            output: self.outputs.to_vec(),
        }
    }

    pub fn signing_keys(&self, wallet: &Wallet) -> BTreeMap<bitcoin::PublicKey, PrivateKey> {
        let mut key_map = BTreeMap::new();
        for (_index, info) in self.input_sign_map.iter() {
            let key_pair = wallet.get_key_pair(String::from(&info.path));
            key_map.insert(key_pair.0, key_pair.1);
        }
        key_map
    }

    pub fn psbt_workflow(&self, wallet: &Wallet) {
        // step1: create psbt
        let mut psbt = Psbt::from_unsigned_tx(self.to_unsigned_tx()).expect("create psbt faild.");
        // step2: sign
        let sig_type = PsbtSighashType::from_str("SIGHASH_ALL").unwrap();
        for input in psbt.inputs.iter_mut() {
            input.sighash_type = Some(sig_type);
        }
        let secp = Secp256k1::new();
        let keys = self.signing_keys(wallet);
        let sign_result = psbt.sign(&keys, &secp).expect("sign failed");
        println!("psbt: {:?}, \nsigningKeys: {:?}", psbt, sign_result);
        // finalize
        todo!();
    }

    pub fn sign_with_key(&self, wallet: &Wallet) -> (String, String) {
        let mut tx = self.to_unsigned_tx();
        let secp = Secp256k1::new();
        //let mut tx;
        let sighash_type = EcdsaSighashType::All;
        let mut input_ptr: *mut TxIn = &mut tx.input[0];
        for (index, input) in self.inputs.iter().enumerate() {
            let mut sighasher = SighashCache::new(&mut tx);
            let sign_info = self
                .input_sign_map
                .get(&index)
                .expect("sign_with_key get signing info");
            let script = sign_info.address.script_pubkey();
            let sighash = sighasher
                .legacy_signature_hash(index, &script, sighash_type as u32)
                .expect("get sig hash");
            let (_, _, private_key) = wallet.get_key_pair(String::from(&sign_info.path));
            let msg = Message::from(sighash);
            let sig = secp.sign_ecdsa(&msg, &private_key);
            let signature = bitcoin::ecdsa::Signature {
                sig,
                hash_ty: EcdsaSighashType::All,
            };
            println!("{:?}", signature);
            let pk = private_key.public_key(&secp);
            unsafe {
                input_ptr = input_ptr.offset(index as isize);
                (*input_ptr).script_sig = ScriptBuf::builder()
                    .push_slice(&sig.serialize_compact())
                    .push_key(&pk.into())
                    .into_script();
            }
            tx = (*sighasher.into_transaction()).clone().into();
        }
        let tx_hash = tx.txid().to_string();
        let raw_tx = serialize(&tx).to_lower_hex_string();
        println!("{:#?}\ntxid: {:?}\nrawTx: {:?}", tx, tx_hash, raw_tx);
        (tx_hash, raw_tx)
    }
}

pub struct BaseElem {
    satoshi: u64,
    address: Address,
}

pub struct Utxo<'a> {
    txid: &'a str,
    vout: u32,
    base_elem: BaseElem,
    path: &'a str,
}

impl<'a> Utxo<'a> {
    pub fn new(txid: &'a str, vout: u32, satoshi: u64, address: &str, path: &'a str) -> Self {
        Utxo {
            txid,
            vout,
            base_elem: BaseElem {
                address: Address::from_str(address)
                    .expect("address from str failed")
                    .assume_checked(),
                satoshi,
            },
            path,
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;
    /*
     *   coin: 'btc',
     *   network: 'testnet',
     *   passpharse: 'abc123',
     *   mnemonic: 'can equal feed rather divide uncle color bright city segment paddle zone',
     *   hdRoot: 'tprv8ZgxMBicQKsPdDAwStDtyTJhrCUjkXk4gUyRkRc5b8ksR8B7gW9wtqd3oUdncXKpi2Zcdavs1rxTUtbwYQdtfBGaRQBSF1x5fXUZvT1wRjS',
     *   derivePrivateKey: 'tprv8ffZXFj3XabUqnYmtEbZfpdHkujkUhqtBeEcxpLkFW6yDU5LbWdoyL681QcTMmwVxv7UMosJQ92wDZBEpuUyTRw5ytrG5adgmMLSfhEZHyB',
     *   derivePublicKey: 'tpubDCMbffmHfxH9jFaZmtGA5EHQKwFge32nkwqQFLP3fmuN3xL7DuTQ9phzBZFHBmbW6VJTLiuVZhL5Mj6yCSbu8f7YghYzCAd6tgJAMHvBN9R',
     */
    #[test]
    fn test() {
        let mut utxos: Vec<Utxo> = vec![];
        // m/1/0
        utxos.push(Utxo::new(
            "03862a49e9d3abcad879a8d78361a88d82358fc495a4409274579154c199e259",
            1,
            1795137,
            "mnbh2vywAhGsbKUMHTAMT2CBbgtMF27Ch5",
            "m/1/0",
        ));
        let mut base_elems = vec![];
        // m/0/2
        base_elems.push(BaseElem {
            satoshi: 1000000,
            address: Address::from_str("mp1Rd4hSjKN4BQVKtY4SRg4DKBKN4QPDbw")
                .expect("address from str")
                .assume_checked(),
        });
        // m/0/1
        base_elems.push(BaseElem {
            satoshi: 90000,
            address: Address::from_str("mnApsMhv1LhaYs17xrshXCtFATfnroFWYp")
                .expect("address from str")
                .assume_checked(),
        });
        // m/0/1
        base_elems.push(BaseElem {
            satoshi: 700000,
            address: Address::from_str("mnApsMhv1LhaYs17xrshXCtFATfnroFWYp")
                .expect("address from str")
                .assume_checked(),
        });
        let wallet = Wallet::new("tprv8ffZXFj3XabUqnYmtEbZfpdHkujkUhqtBeEcxpLkFW6yDU5LbWdoyL681QcTMmwVxv7UMosJQ92wDZBEpuUyTRw5ytrG5adgmMLSfhEZHyB", "tpubDCMbffmHfxH9jFaZmtGA5EHQKwFge32nkwqQFLP3fmuN3xL7DuTQ9phzBZFHBmbW6VJTLiuVZhL5Mj6yCSbu8f7YghYzCAd6tgJAMHvBN9R", "testnet");
        let mut tx = Tx::new(Version::ONE, LockTime::ZERO);
        tx.add_inputs(utxos);
        tx.add_outputs(base_elems);
        let raw_tx = tx.sign_with_key(&wallet);
        assert_eq!(
            raw_tx.0,
            "03690837e561a5a7b848b1336cad5dafacb475db3df34c10137c636ab1cbd3c1".to_string()
        );
        assert_eq!(raw_tx.1, "010000000159e299c1549157749240a495c48f35828da86183d7a879d8caabd3e9492a860301000000634036f514d6912bc7977f079457ca82d13d9b4323c1486a363864f09b0ee6743f5534b6094543d839ba302635d52c624563724b0115b2c593251b92606916860b522102c4e492b2b03b72af609c8b2d6083493de198a771c3e00c45d5ad067b92724c3ffeffffff0340420f00000000001976a9145d23771c146de5af0739da2cf6accc5c812a655488ac905f0100000000001976a91448fa5ccd4d37a65f9a2600304fb9eb470e50696e88ac60ae0a00000000001976a91448fa5ccd4d37a65f9a2600304fb9eb470e50696e88ac00000000".to_string());
    }
}
