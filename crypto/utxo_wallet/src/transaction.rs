use crate::wallet::SignInfo;
use bitcoin::absolute::LockTime;
use bitcoin::transaction::Version;
use bitcoin::{
    Address, Amount, OutPoint, ScriptBuf, Sequence, Transaction, TxIn, TxOut, Txid, Witness,
};
use std::collections::HashMap;
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
        self.inputs.push(TxIn {
            previous_output: prev_out,
            sequence: Sequence::ENABLE_LOCKTIME_NO_RBF,
            // 签名之后填充完整
            script_sig: ScriptBuf::default(),
            witness: Witness::default(),
        });
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
