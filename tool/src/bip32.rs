use bip32::{DerivationPath, Prefix, XPrv, XPub};
use bip39::Mnemonic;
use std::str::FromStr;

pub fn mnemonic_to_x_prv(words: String, passphrase: String) -> XPrv {
    let mnemonic = Mnemonic::from_str(words.as_str()).unwrap();
    let seed = mnemonic.to_seed(&passphrase);
    XPrv::new(&seed).unwrap()
}

pub fn derive_private_by_path(path: String, x_prv: String) -> XPrv {
    let mut x_prv = XPrv::from_str(x_prv.as_str()).unwrap();
    let path = DerivationPath::from_str(path.as_str()).unwrap();
    for child_number in path.iter() {
        x_prv = x_prv.derive_child(child_number).unwrap();
    }
    x_prv
}

pub fn derive_public_by_path(path: String, x_pub: String) -> XPub {
    let mut x_pub = XPub::from_str(x_pub.as_str()).unwrap();
    // 限定为非harden的path
    let path = DerivationPath::from_str(path.as_str()).unwrap();
    for child_number in path.iter() {
        x_pub = x_pub.derive_child(child_number).unwrap();
    }
    x_pub
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_mnemonic_to_x_prv() {
        let words = "panda eyebrow bullet gorilla call smoke muffin taste mesh discover soft ostrich alcohol speed nation flash devote level hobby quick inner drive ghost inside".to_string();
        let passphrase =
            "9f6a2878b2520799a44ef18bc7df394e7061a224d2c33cd015b157d746869863".to_string();
        let xprv = mnemonic_to_x_prv(words, passphrase);
        assert_eq!(xprv, XPrv::from_str("xprv9s21ZrQH143K2UJrCYj9JXWw3PD4G1EoP33gz5UXmoTEgqjutX9Sx1ZKc34iSCZc8HD88BuuYfFiKqAXthsBB8W1ctWUGUZD2EPs65fjHaF").unwrap());
        println!("xprv: {:?}", xprv.to_string(Prefix::XPRV));
    }

    #[test]
    fn test_derive_x_key() {
        let x_prv = "xprv9s21ZrQH143K2UJrCYj9JXWw3PD4G1EoP33gz5UXmoTEgqjutX9Sx1ZKc34iSCZc8HD88BuuYfFiKqAXthsBB8W1ctWUGUZD2EPs65fjHaF".to_string();
        let x_pub = "xpub6CLvyPtShnSUYUoGZjdWFFMsYfSH9nJWs6fesbY5jihJKQMfSrNhapj2n6Bq22iAP5tMAbqffpjV6ozkxQnjFrjceJJsxBeEha9k2335J5e".to_string();
        println!("x_pub: {:?}", x_pub);
        let derive_key = derive_private_by_path("m/44'/0'/0'".to_string(), x_prv);
        println!("derive_key: {:?}", derive_key.to_string(Prefix::XPRV));
        assert_eq!(derive_key, XPrv::from_str("xprv9yMaZtMYsQtBKzioTi6Vt7R8zdbnkKafVsk45D8UBPAKSc2WuK4T32QYvoiSrtNzmX1MQ7yPRnb5KQp9jSkhsArAQ5oP6A9XaMRCyFjBwBM").unwrap());
        let derive_pub_key = derive_public_by_path("m/0/1".to_string(), x_pub);
        println!(
            "derive_pub_key: {:?}",
            derive_pub_key.to_string(Prefix::XPUB)
        );
        assert_eq!(derive_pub_key, XPub::from_str("xpub6GA7ErDewmHKd39n4FmHiaCe8fPeHRKW95P1bqy5kisuG67jS4HkBrnUg8ccxVbou2WKHJgXRojm4p2gMHcRoLvjsu9Ty7Q9HGa6ruJfJ5N").unwrap());
    }
}
