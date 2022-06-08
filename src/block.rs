// block.rs
//
use crate::{ProofOfWork, Transaction};
use serde::{Deserialize, Serialize};
use sled::IVec;

/// 区块
#[derive(Clone, Serialize, Deserialize)]
pub struct Block {
    timestamp: u64,         // 生成区块时间戳
    nonce: i64,             // 计数
    height: usize,          // 区块高度(该区块相对于创世区块的个数)
    hash: String,     // 区块hash
    pre_block_hash: String, // 上一个区块hash
    transactions: Vec<Transaction>, //交易
}

impl Block {
    /// 新建一个区块
    pub fn new_block(pre_block_hash: String, transactions: &[Transaction], height: usize) -> Self {
        let mut block = Block {
            timestamp: crate::current_timestamp(),
            nonce: 0,
            height,
            pre_block_hash,
            hash: String::new(),
            transactions: transactions.to_vec(),
        };

        let pow = ProofOfWork::new(block.clone());
        let (nonce, hash) = pow.run();
        block.nonce = nonce;
        block.hash = hash;

        return block;
    }

    /// 生成创世区块
    pub fn generate_genesis_bloc(transaction: &Transaction) -> Self {
        let transactions = vec![transaction.clone()];
        return Block::new_block(String::from("None"), &transactions, 0);
    }

    /// 计算区块所有交易hash
    pub fn hash_transactions(&self) -> Vec<u8> {
        let mut tx_hashes = vec![];
        for transaction in &self.transactions {
            tx_hashes.extend(transaction.get_id());
        }

        crate::sha256_digest(tx_hashes.as_slice())
    }


    /// 反序列化, 字节数组 -> Block
    pub fn deserialize(bytes: &[u8]) -> Self {
        bincode::deserialize(bytes).unwrap()
    }

    /// 序列化, Block -> 字节数组
    pub fn serialize(&self) -> Vec<u8> {
        bincode::serialize(self).unwrap().to_vec()
    }

    pub fn get_pre_block_hash(&self) -> String {
        self.pre_block_hash.clone()
    }

    pub fn get_hash(&self) -> &str {
        self.hash.as_str()
    }

    pub fn get_hash_bytes(&self) -> Vec<u8> {
        self.hash.as_bytes().to_vec()
    }

    pub fn get_timestamp(&self) -> u64 {
        self.timestamp
    }
}

impl From<Block> for IVec {
    fn from(b: Block) -> Self {
        let bytes = bincode::serialize(&b).unwrap();
        Self::from(bytes)
    }
}
