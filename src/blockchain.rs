// blockchain.rs
//

use sled::{Db, Tree, transaction::TransactionResult};

use std::{sync::{Arc, RwLock}, env::current_dir};

use crate::{Transaction, block::Block};

const BLOCKS_TREE: &str = "blocks";
const TIP_BLOCK_HASH_KEY: &str = "tip_block_hash";

/// 区块链
#[derive(Clone)]
pub struct Blockchain {
    tip_hash: Arc<RwLock<String>>,      //最后一个block的hash
    db: Db,
}

impl Blockchain {
    /// 新建Blockchain实例
    pub fn new_blockchain() -> Self {
        let db = sled::open(current_dir().unwrap().join("data")).unwrap();
        let blocks_tree = db.open_tree(BLOCKS_TREE).unwrap();
        let tip_bytes = blocks_tree.get(TIP_BLOCK_HASH_KEY).unwrap().expect("No existing blockchain found");
        let tip_hash = String::from_utf8(tip_bytes.to_vec()).unwrap();

        Blockchain {
            tip_hash: Arc::new(RwLock::new(tip_hash)),
            db,
        }
    }

    /// 创建一条新的区块链
    pub fn create_blockchain(genesis_address: &str) -> Self {
        let tip_hash;
        let db = sled::open(current_dir().unwrap().join("data")).unwrap();
        let blocks_tree = db.open_tree(BLOCKS_TREE).unwrap();
        let data = blocks_tree.get(TIP_BLOCK_HASH_KEY).unwrap();
        if data.is_none() {
            let coinbase_tx = Transaction::new_coinbase_tx(genesis_address);
            let block = Block::generate_genesis_bloc(&coinbase_tx);
            Self::update_blocks_tree(&blocks_tree, &block);
            tip_hash = String::from(block.get_hash());
        } else {
            tip_hash = String::from_utf8(data.unwrap().to_vec()).unwrap();
        }

        Blockchain { tip_hash: Arc::new(RwLock::new(tip_hash)), db }
    }

    fn update_blocks_tree(blocks_tree: &Tree, block: &Block) {
        let block_hash = block.get_hash();
        let _: TransactionResult<(), ()> = blocks_tree.transaction(|tx_db| {
            let _ = tx_db.insert(block_hash, block.clone());
            let _ = tx_db.insert(TIP_BLOCK_HASH_KEY, block_hash);
            Ok(())
        });
    }

    pub fn get_db(&self) -> &Db {
        &self.db
    }

    pub fn get_tip_hash(&self) -> String {
        self.tip_hash.read().unwrap.clone()
    }

    pub fn set_tip_hash(&self, new_tip_hash: &str) {
        let mut tip_hash = self.tip_hash.write().unwrap();
        *tip_hash = String::from(new_tip_hash);
    }

    /// 挖出一个新块
    pub fn mine_block(&self, transactions: &[Transaction]) -> Block {

        todo!()
    }

    pub fn iterator(&self) -> BlockchainIterator {
        BlockchainIterator::new(self.get_tip_hash(), self.db.clone())
    }

    /// 从区块链中查找交易
    pub fn find_transaction(&self, txid: &[u8]) -> Option<Transaction> {
        let mut iterator = self.iterator();
        loop {
            let option = iterator.next();
            if option.is_none() {
                break;
            }
            let block = option.unwrap();
            for transaction in block.get_transactions() {
                if txid.eq(transaction.get_id()) {
                    return Some(transaction.clone());
                }
            }
        }
        None
    }

}

/// blockchain 迭代器
pub struct BlockchainIterator {
    db: Db,
    current_hash: String,
}

impl BlockchainIterator {

    /// 新建blockchain iterator
    pub fn new(tip_hash: String, db: Db) -> Self {
        BlockchainIterator { db, current_hash: tip_hash, }
    }
    
    pub fn next(&self) -> Option<Block> {

        todo!()
    } 
}


