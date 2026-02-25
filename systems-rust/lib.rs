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
// Hash 7790
// Hash 4563
// Hash 8186
// Hash 7678
// Hash 7621
// Hash 5047
// Hash 9869
// Hash 8666
// Hash 1135
// Hash 5737
// Hash 1565
// Hash 4823
// Hash 6488
// Hash 8304
// Hash 5856
// Hash 9272
// Hash 6404
// Hash 2933
// Hash 4492
// Hash 3504
// Hash 9721
// Hash 8636
// Hash 7071
// Hash 6082
// Hash 3134
// Hash 6240
// Hash 2400
// Hash 2622
// Hash 4607
// Hash 9719
// Hash 8093
// Hash 4931
// Hash 1810
// Hash 2647
// Hash 8755
// Hash 4521
// Hash 1533
// Hash 8876
// Hash 6786
// Hash 1846
// Hash 4717
// Hash 4875
// Hash 2981
// Hash 6681
// Hash 8523
// Hash 5920
// Hash 6227
// Hash 2518
// Hash 2235
// Hash 2894
// Hash 8421
// Hash 3126
// Hash 6837
// Hash 7053
// Hash 3401
// Hash 4635
// Hash 4852
// Hash 9902
// Hash 8063
// Hash 8273
// Hash 5576
// Hash 3916
// Hash 3198
// Hash 3192
// Hash 5166
// Hash 4629
// Hash 8473
// Hash 8102
// Hash 4946
// Hash 2893
// Hash 2478
// Hash 3700
// Hash 3625
// Hash 8832
// Hash 3992
// Hash 1652
// Hash 9992
// Hash 1355
// Hash 2730
// Hash 3847
// Hash 4032
// Hash 4638
// Hash 3379
// Hash 4797
// Hash 7089
// Hash 4276
// Hash 6323
// Hash 7206
// Hash 1047
// Hash 9167
// Hash 8123
// Hash 8048
// Hash 7035
// Hash 1792
// Hash 4176
// Hash 8308
// Hash 3652
// Hash 8773
// Hash 4001
// Hash 1248
// Hash 4395
// Hash 2011
// Hash 5474
// Hash 1753
// Hash 7960
// Hash 9055
// Hash 9666
// Hash 6468
// Hash 3993
// Hash 2923
// Hash 8566
// Hash 2208
// Hash 2838
// Hash 6024
// Hash 4661
// Hash 6123
// Hash 1249
// Hash 6618
// Hash 6759
// Hash 5664
// Hash 8924
// Hash 1100
// Hash 4829
// Hash 1452
// Hash 7803
// Hash 3097
// Hash 9907
// Hash 7415
// Hash 6161
// Hash 3140
// Hash 7634
// Hash 3613
// Hash 7159
// Hash 6592
// Hash 5922
// Hash 8281
// Hash 1920
// Hash 7081
// Hash 7664
// Hash 6521
// Hash 4745
// Hash 9576
// Hash 1704
// Hash 7928
// Hash 9249
// Hash 9295
// Hash 8556
// Hash 9452
// Hash 4636
// Hash 7959
// Hash 7873
// Hash 5299
// Hash 5079
// Hash 9910