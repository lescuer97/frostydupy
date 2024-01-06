// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use std::{
    fs::{self, File},
    io::{BufReader, BufWriter},
};

pub mod btc;
pub mod config;
pub mod error;

// use crate::config::Config;
use crate::{
    btc::commands::{approve_wallet, core_rpc_client, get_block_height, get_wallets_config},
    config::Config,
};
use dirs::config_local_dir;
use std::io::prelude::*;

use crate::btc::commands::create_wallet;

static MAGIC_DB: &[u8] = b"BDKSTORAGE";

#[tauri::command]
/// Checks if config exists if not it creates one
fn check_and_setup_config() -> tauri::Result<bool> {
    let home_config = config_local_dir();
    if let Some(mut dir) = home_config {
        dir.push("arbitrator");
        fs::create_dir_all(&dir).expect("Could not create config directory");
        dir.push("config.json");

        let mut config: Config = Config::default();

        if dir.exists() {
            let file = File::open(&dir)?;
            let mut reader = BufReader::new(file);
            let mut file: String = String::new();

            reader.read_to_string(&mut file)?;

            config = serde_json::from_str(&file)?;
        } else {
            // Create file
            let file = File::create(&dir)?;
            let writer = BufWriter::new(file);
            serde_json::to_writer_pretty(writer, &config)?;
        }
        println!("Config: {:?}", config);

        return Ok(true);
    } else {
        return Ok(false);
    }
}

// pub struct Db {
//     store: Store<'static, ChangeSet>,
// }
// impl Db {
//     pub fn new(path: PathBuf) -> anyhow::Result<Self> {
//         let store: Store<'_, ChangeSet> = Store::new_from_path(MAGIC_DB, &path)?;
//         Ok(Self { store })
//     }
//
//     pub fn load(&mut self) -> anyhow::Result<ChangeSet> {
//         let (changeset, res) = self.store.aggregate_changesets();
//         res?;
//         Ok(changeset)
//     }
//
//     pub fn save<C: Into<ChangeSet>>(&mut self, changeset: C) -> anyhow::Result<()> {
//         Ok(self.store.append_changeset(&changeset.into())?)
//     }
// }

fn main() -> tauri::Result<()> {
    let wallet_config = get_wallets_config().unwrap();

    println!("Wallets: {:?}", wallet_config);

    let client = core_rpc_client().unwrap();

    tauri::Builder::default()
        .setup(|_app| Ok(()))
        .manage(client)
        .invoke_handler(tauri::generate_handler![
            check_and_setup_config,
            get_wallets_config,
            get_block_height,
            create_wallet,
            approve_wallet,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");

    Ok(())
}
