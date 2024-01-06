use bdk::bitcoin::{bip32::Fingerprint, Network};
use bdk::{
    bitcoin::{
        bip32::{DerivationPath, ExtendedPrivKey, KeySource},
        secp256k1::Secp256k1,
    },
    keys::{
        DerivableKey,
        DescriptorKey::{self, Secret},
    },
    miniscript::{Segwitv0, Tap},
};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct WalletConfig {
    pub words: String,
    pub chain: Network,
    pub xfp: Fingerprint,
    pub account: u32,
    pub name: String,
    pub xpub: String,
    pub bip84: Option<DerivationConfig>,
    pub bip86: Option<DerivationConfig>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DerivationConfig {
    pub name: WalletType,
    pub xfp: Fingerprint,
    pub deriv: String,
    pub xpub: String,
    pub desc: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub enum WalletType {
    BIP84,
    BIP86,
}
impl From<WalletType> for String {
    fn from(a: WalletType) -> Self {
        match a {
            WalletType::BIP84 => "bip84".to_string(),
            WalletType::BIP86 => "bip86".to_string(),
        }
    }
}
impl<'a> From<&'a WalletType> for &'a str {
    fn from(a: &'a WalletType) -> Self {
        match a {
            WalletType::BIP84 => "bip84",
            WalletType::BIP86 => "bip86",
        }
    }
}

impl DerivationConfig {
    pub fn new(
        wallet_type: WalletType,
        xpriv: ExtendedPrivKey,
        deriv: DerivationPath,
    ) -> Option<Self> {
        let secp = Secp256k1::new();

        let fingerprint = xpriv.fingerprint(&secp);

        let origin: KeySource = (fingerprint, deriv.clone());

        match wallet_type {
            WalletType::BIP84 => {
                let derived_xpriv: DescriptorKey<Segwitv0> = xpriv
                    .into_descriptor_key(Some(origin.clone()), deriv.clone())
                    .unwrap();

                if let Secret(key, _, _) = derived_xpriv {
                    // println!("DERIVED XPRIV: {:?}", derived_xprv_desc_key);
                    println!("key : {:?}", key);

                    let pub_key = key.to_public(&secp).unwrap();

                    println!("pub_key : {:?}", pub_key.to_string());

                    let config = DerivationConfig {
                        name: wallet_type,
                        xfp: fingerprint,
                        deriv: deriv.to_string(),
                        xpub: pub_key.to_string(),
                        desc: format!("wsh({0})", pub_key.to_string()),
                    };

                    return Some(config);
                }
            }

            WalletType::BIP86 => {
                let derived_xpriv: DescriptorKey<Tap> = xpriv
                    .into_descriptor_key(Some(origin.clone()), deriv.clone())
                    .unwrap();

                if let Secret(key, _, _) = derived_xpriv {
                    // println!("DERIVED XPRIV: {:?}", derived_xprv_desc_key);
                    println!("key : {:?}", key);

                    let pub_key = key.to_public(&secp).unwrap();

                    println!("pub_key : {:?}", pub_key.to_string());

                    let config = DerivationConfig {
                        name: wallet_type,
                        xfp: fingerprint,
                        deriv: deriv.to_string(),
                        xpub: pub_key.to_string(),
                        desc: format!("wsh({0})", pub_key.to_string()),
                    };

                    return Some(config);
                }
            }
        }
        return None;
    }
}
