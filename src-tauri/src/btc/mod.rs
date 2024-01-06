use bdk::keys::bip39::{Language, Mnemonic, WordCount};
use bdk::keys::{GeneratableKey, GeneratedKey};
use bdk::miniscript::Tap;

pub mod commands;
pub mod wallet;
use crate::error::Error;

pub fn create_mnemonic(words_amount: WordCount) -> Result<Vec<String>, Error> {
    let mnemonic: GeneratedKey<_, Tap> =
        Mnemonic::generate((words_amount, Language::English)).unwrap();
    let mnemonic_key = mnemonic.into_key();
    let words_array = mnemonic_key
        .to_string()
        .split_whitespace()
        .into_iter()
        .map(|s| s.to_string())
        .collect::<Vec<String>>();

    return Ok(words_array);
}
