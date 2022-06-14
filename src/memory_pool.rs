// memory_pool.rs
//

use std::{
    sync::RwLock, 
    collections::HashMap
};
use data_encoding::HEXLOWER;
use crate::Transaction;

/// 交易内存池 ( K -> txid_hex, V => Transaction )
pub struct MemoryPool {
    inner: RwLock<HashMap<String, Transaction>>,
}

impl MemoryPool {
    pub fn new() -> Self {
        MemoryPool {
            inner: RwLock::new(HashMap::new()),
        }
    }

    /// 是否包含txid_hex
    pub fn contains(&self, txid_hex: &str) -> bool {
        self.inner.read()
            .unwrap()
            .contains_key(txid_hex)
    }

    pub fn add(&self, tx: Transaction) {
        let txid_hex = HEXLOWER.encode(tx.get_id());
        self.inner.write().unwrap().insert(txid_hex, tx);
    }

    pub fn get(&self, txid_hex: &str) -> Option<Transaction> {
        self.inner.read().unwrap().get(txid_hex).cloned()
    }

    pub fn remove(&self, txid_hex: &str) {
        self.inner.write()
            .unwrap()
            .remove(txid_hex);
    }

    pub fn get_all(&self) -> Vec<Transaction> {
        self.inner.read()
            .unwrap()
            .iter()
            .map(|(_, v)|v.clone())
            .collect()
    }

    pub fn len(&self) -> usize {
        self.inner.read().unwrap().len()
    }
}

/// 传输中的块, 用于来跟踪已下载的块, 这能够实现从不同的节点下载块.
pub struct BlockInTransit {
    inner: RwLock<Vec<Vec<u8>>>,
}

impl BlockInTransit {
    pub fn new() -> BlockInTransit {
        BlockInTransit {
            inner: RwLock::new(vec![]),
        }
    }

    pub fn add_blocks(&self, blocks: &[Vec<u8>]) {
        let mut inner = self.inner.write().unwrap();
        for hash in blocks {
            inner.push(hash.to_vec());
        }
    }

    pub fn first(&self) -> Option<Vec<u8>> {
        let inner = self.inner.read().unwrap();
        if let Some(block_hash) = inner.first() {
            return Some(block_hash.to_vec());
        }
        None
    }

    pub fn remove(&self, block_hash: &[u8]) {
        let mut inner = self.inner.write().unwrap();
        if let Some(idx) = inner.iter().position(|x| x.eq(block_hash)) {
            inner.remove(idx);
        }
    }

    pub fn clear(&self) {
        let mut inner = self.inner.write().unwrap();
        inner.clear();
    }

    pub fn len(&self) -> usize {
        self.inner.read().unwrap().len()
    }
}

#[cfg(test)]
mod tests {

}
