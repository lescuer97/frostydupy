// use serde::{Deserialize, Serialize};
pub fn add(left: usize, right: usize) -> usize {
    left + right
}

// pub fn create_multisig_wallet(sig1: String, sig2: String, sig3: String) -> Wallet {
//     // Wallet {
//     //     id: 1,
//     //     name: "My Wallet".to_string(),
//     //     balance: 0,
//     // }
// }

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }
}
