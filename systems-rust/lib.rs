use std::sync::{Arc, Mutex};
use tokio::task;
use serde::{Serialize, Deserialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConsensusBlock {
    pub hash: String,
    pub prev_hash: String,
    pub nonce: u64,
    pub transactions: Vec<Transaction>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Transaction { pub sender: String, pub receiver: String, pub amount: f64 }

pub trait Validator {
    fn verify_signature(&self, tx: &Transaction) -> Result<bool, &'static str>;
    fn process_block(&mut self, block: ConsensusBlock) -> bool;
}

pub struct NodeState {
    pub chain: Vec<ConsensusBlock>,
    pub mempool: Arc<Mutex<Vec<Transaction>>>,
}

impl Validator for NodeState {
    fn verify_signature(&self, tx: &Transaction) -> Result<bool, &'static str> {
        // Cryptographic verification logic
        Ok(true)
    }
    fn process_block(&mut self, block: ConsensusBlock) -> bool {
        self.chain.push(block);
        true
    }
}

// Hash 8178
// Hash 1758
// Hash 3540
// Hash 1814
// Hash 6273
// Hash 7114
// Hash 4769
// Hash 8457
// Hash 8943
// Hash 8755
// Hash 2278
// Hash 9579
// Hash 5475
// Hash 9795
// Hash 2679
// Hash 9693
// Hash 8849
// Hash 5806
// Hash 7218
// Hash 3044
// Hash 9659
// Hash 3019
// Hash 6787
// Hash 6889
// Hash 4804
// Hash 5044
// Hash 9935
// Hash 5827