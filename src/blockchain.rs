// blockchain.rs
//

use data_encoding::HEXLOWER;
use sled::{
    Db,
    Tree,
    transaction::TransactionResult
};
use std::{
    collections::HashMap,
    env::current_dir,
    sync::{Arc, RwLock},
};
use crate::{
    Transaction,
    block::Block,
    transaction::TxOutput
};

const BLOCKS_TREE: &str = "blocks";
const TIP_BLOCK_HASH_KEY: &str = "tip_block_hash";

/// 区块链
#[derive(Clone)]
pub struct Blockchain {
    tip_hash: Arc<RwLock<String>>,      //最后一个block的hash
    db: Db,                 // 保存blockchain的db
}

impl Blockchain {

    /// 打开Blockchain实例
    pub fn open_blockchain() -> Self {
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
            let block = Block::generate_genesis_block(&coinbase_tx);
            Self::update_blocks_tree(&blocks_tree, &block);
            tip_hash = String::from(block.get_hash());
        } else {
            tip_hash = String::from_utf8(data.unwrap().to_vec()).unwrap();
        }

        Blockchain { tip_hash: Arc::new(RwLock::new(tip_hash)), db }
    }

    /// 更新最后一个block
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

    ///
    pub fn get_tip_hash(&self) -> String {
        self.tip_hash.read().unwrap().clone()
    }

    ///
    pub fn set_tip_hash(&self, new_tip_hash: &str) {
        let mut tip_hash = self.tip_hash.write().unwrap();
        *tip_hash = String::from(new_tip_hash);
    }

    ///
    fn get_block_tree(&self) -> Tree {
        self.db.open_tree(BLOCKS_TREE).unwrap()
    }

    ///
    pub fn get_best_height(&self) -> usize {
        let tip_block_bytes = self.get_block_tree()
            .get(self.get_tip_hash())
            .unwrap()
            .expect("the tip hash is valid");
        let tip_block = Block::deserialize(tip_block_bytes.as_ref());

        tip_block.get_height()
    }

    /// 增加一个区块到链上
    pub fn add_block(&self, block: &Block) {
        let block_tree = self.get_block_tree();
        if let Some(_) = block_tree.get(block.get_hash()).unwrap() {
            return;
        }

        let _: TransactionResult<(), ()> = block_tree.transaction(|tx_db|{
            tx_db.insert(block.get_hash(), block.serialize());
            let tip_block_bytes = tx_db.get(self.get_tip_hash())
                .unwrap()
                .expect("the tip hash is not valid");
            let tip_block = Block::deserialize(tip_block_bytes.as_ref());
            if block.get_height() > tip_block.get_height() {
                let _ = tx_db.insert(TIP_BLOCK_HASH_KEY, block.get_hash()).unwrap();
                self.set_tip_hash(block.get_hash());
            }
            Ok(())
        });
    }

    /// 获取block
    pub fn get_block(&self, block_hash: &[u8]) -> Option<Block> {
        if let Some(block_bytes) = self.get_block_tree().get(block_hash).unwrap() {
            let block = Block::deserialize(block_bytes.as_ref());
            return Some(block);
        }
        return None;
    }

    /// 获取所有区块hash
    pub fn get_block_hashes(&self) -> Vec<Vec<u8>> {
        let mut iter = self.iterator();
        let mut hashes = vec![];
        loop {
            let option = iter.next();
            if option.is_none() {
                break;
            }
            let block = option.unwrap();
            hashes.push(block.get_hash_bytes());
        }

        return hashes;
    }

    /// 挖出一个新块
    pub fn mine_block(&self, transactions: &[Transaction]) -> Block {
        // 首先检查交易是否合法
        for tx in transactions {
            if !tx.verify(self) {
                panic!("ERROR: Invalid tx");
            }
        }

        //
        let best_height = self.get_best_height();
        let block = Block::new(self.get_tip_hash(), transactions, best_height+1);
        let block_tree = self.get_block_tree();
        Self::update_blocks_tree(&block_tree, &block);
        self.set_tip_hash(block.get_hash());

        block
    }

    pub fn iterator(&self) -> BlockchainIterator {
        BlockchainIterator::new(self.get_tip_hash(), self.db.clone())
    }

    /// 根据id从链中查找交易
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

    /// 查找该链上所有未花费的交易输出
    pub fn find_utxo(&self) -> HashMap<String, Vec<TxOutput>> {
        let mut utxo: HashMap<String, Vec<TxOutput>> = HashMap::new();
        let mut spent_txos: HashMap<String, Vec<usize>> = HashMap::new();

        let mut iterator = self.iterator();
        loop {
            let option = iterator.next();
            if option.is_none() {
                break;
            }
            let block = option.unwrap();
            'outer: for tx in block.get_transactions() {
                let txid_hex = HEXLOWER.encode(tx.get_id());
                for (idx, out) in tx.get_vout().iter().enumerate() {
                    // 过滤已花费的输出
                    if let Some(outs) = spent_txos.get(txid_hex.as_str()) {
                        for spend_out_idx in outs {
                            if idx.eq(spend_out_idx) {
                                continue 'outer;
                            }
                        }
                    }
                    if utxo.contains_key(txid_hex.as_str()) {
                        utxo.get_mut(txid_hex.as_str()).unwrap().push(out.clone());
                    } else {
                        utxo.insert(txid_hex.clone(), vec![out.clone()]);
                    }
                }
                if tx.is_coinbase() {
                    continue;
                }
                // 在输入中查找已花费输出
                for txin in tx.get_vin() {
                    let txid_hex = HEXLOWER.encode(txin.get_txid());
                    if spent_txos.contains_key(txid_hex.as_str()) {
                        spent_txos
                            .get_mut(txid_hex.as_str())
                            .unwrap()
                            .push(txin.get_outid());
                    } else {
                        spent_txos.insert(txid_hex, vec![txin.get_outid()]);
                    }
                }
            }
        }

        utxo
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

    /// blochchain 中的下一个block
    pub fn next(&mut self) -> Option<Block> {
        let block_tree = self.db.open_tree(BLOCKS_TREE).unwrap();
        let data = block_tree.get(self.current_hash.clone()).unwrap();
        if data.is_none() {
            return None;
        }
        let block = Block::deserialize(data.unwrap().to_vec().as_slice());
        self.current_hash = block.get_pre_block_hash().clone();

        return Some(block);
    }
}


#[cfg(test)]
mod tests{

}

