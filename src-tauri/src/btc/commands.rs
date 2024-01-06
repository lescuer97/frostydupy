use bdk::{
    bitcoin::{
        bip32::{DerivationPath, ExtendedPubKey},
        secp256k1::Secp256k1,
        Network,
    },
    descriptor::ExtendedDescriptor,
    keys::{
        bip39::{Language, Mnemonic, WordCount},
        DerivableKey, ExtendedKey,
    },
    Wallet,
};
use bitcoincore_rpc::{Auth, Client, RpcApi};
use dirs::config_local_dir;
use std::{
    fs::{self, File},
    io::BufWriter,
    str::FromStr,
};

use crate::{config::{NETWORK, APP_NAME}, error::Error};

use super::{
    create_mnemonic,
    wallet::config::{DerivationConfig, WalletConfig, WalletType},
};

#[tauri::command]
pub fn create_wallet(name: String, network: Network) -> Result<Vec<String>, Error> {
    return Ok(create_mnemonic(WordCount::Words12)?);
}

#[tauri::command]
pub fn approve_wallet(words: Vec<String>, name: String) -> Result<(), Error> {
    let secp = Secp256k1::new();
    let words_string = words.join(" ");
    let mnemonic: Mnemonic = Mnemonic::parse_in(Language::English, &words_string).unwrap();
    let mnemonic_key: ExtendedKey = (mnemonic, None).into_extended_key().unwrap();

    let xpriv_option = mnemonic_key.into_xprv(NETWORK);

    if let Some(xpriv) = xpriv_option {
        let xpub = ExtendedPubKey::from_priv(&secp, &xpriv);

        let derivation_path: DerivationPath = DerivationPath::from_str("m/84h/0h/0h/0").unwrap();

        let bip84 = DerivationConfig::new(WalletType::BIP84, xpriv, derivation_path.clone());
        let bip86 = DerivationConfig::new(WalletType::BIP86, xpriv, derivation_path.clone());

        let wallet_config = WalletConfig {
            words: words_string,
            chain: NETWORK,
            name,
            xfp: xpub.fingerprint(),
            account: 0,
            xpub: xpub.to_string(),
            bip84,
            bip86,
        };
        let home_config = config_local_dir();

        if let Some(dir) = home_config {
            let mut wallet_path = dir.clone();
            wallet_path.push(APP_NAME);
            wallet_path.push("wallets");
            wallet_path.push(format!("{}.json", &wallet_config.name));
            // Create file
            let file = File::create(&wallet_path)?;
            let writer = BufWriter::new(file);
            serde_json::to_writer_pretty(writer, &wallet_config)?;

            return Ok(());
        }

        return Ok(());
    }

    return Ok(());
}

#[tauri::command]
pub fn get_wallets_config() -> Result<Vec<WalletConfig>, Error> {
    let home_config = config_local_dir();
    let mut wallets_config: Vec<WalletConfig> = Vec::new();

    if let Some(mut dir) = home_config {
        dir.push(APP_NAME);
        dir.push("wallets");

        fs::create_dir_all(&dir).expect("Could not create config directory");

        if dir.exists() && dir.is_dir() {
            for entry in fs::read_dir(dir)? {
                let entry = entry?;

                let file_path = entry.path();
                let json_content = fs::read_to_string(&file_path)?;
                let wallet_config: WalletConfig = serde_json::from_str(&json_content)?;
                wallets_config.push(wallet_config);
            }
        }
        return Ok(wallets_config);
    } else {
        return Ok(wallets_config);
    }
}

#[tauri::command]
pub fn get_block_height(connection: tauri::State<'_, Client>) -> Result<(), Error> {
    let block_height = connection.get_block_count()?;
    println!("Block height: {:?}", block_height);
    return Ok(());
}

pub fn core_rpc_client() -> Result<Client, Error> {
    let auth = Auth::UserPass("polaruser".to_string(), "polarpass".to_string());
    let client = Client::new("http://127.0.0.1:18443", auth)?;

    return Ok(client);
}

pub fn get_wallet_db(wallet_config: Vec<WalletConfig>) -> Result<Vec<Wallet>, Error> {
    let home_config = config_local_dir();
    let wallets: Vec<Wallet> = Vec::new();

    if let Some(dir) = home_config {
        for name in wallet_config {
            let mut db_path = dir.clone();
            db_path.push(APP_NAME);
            db_path.push("wallets");
            // let store: Store<'static, ChangeSet> = Store::new_from_path(MAGIC_DB, db_path).unwrap();

            let descriptor = ExtendedDescriptor::from_str(&name.bip84.unwrap().desc).unwrap();

            let wallet = Wallet::new_no_persist(descriptor, None, /* store, */ NETWORK);

            // walle
        }

        return Ok(wallets);
    }

    return Ok(wallets);
}
