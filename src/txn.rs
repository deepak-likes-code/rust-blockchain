use serde::{Deserialize, Serialize};

// TXInput represents a transaction input
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct TXInput {
    pub txid: String,
    pub vout: i32,
    pub script_sig: String,
}

// TXOutput represents a transaction output
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct TXOutput {
    pub value: i32,
    pub script_pub_key: String,
}

impl TXInput {
    // Checks whether address inititated the transaction
    pub fn can_unlock_output_with(&self, unlocking_data: &str) -> bool {
        self.script_sig == unlocking_data
    }
}

impl TXOutput {
    // checks if the output can be unlocked with the given data
    pub fn can_be_unlock_with(&self, unlocking_data: &str) -> bool {
        self.script_pub_key == unlocking_data
    }
}
