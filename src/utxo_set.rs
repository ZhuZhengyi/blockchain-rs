// utxo_set.rs
//

use crate::{
    Blockchain, 
    block::Block, 
    transaction::TxOutput
};
use data_encoding::HEXLOWER;
use std::collections::HashMap;

const UTXO_TREE: &str = "chainstate";

/// UTXO(Unspent Transaction Output)集合
pub struct UTXOSet {
    blockchain: Blockchain,
}

impl UTXOSet {

    /// 新建一个utxo_set
    pub fn new(blockchain: Blockchain) -> Self {
        UTXOSet{blockchain}
    }

    /// 获取utxo set对应的blockchain
    pub fn get_blockchain(&self) -> &Blockchain {
        &self.blockchain
    }

    /// 通过block上的交易,更新utxo set
    pub fn update(&self, block: Block) {
        let utxo_tree = self.get_utxo_tree();
        for tx in block.get_transactions() {
            // 处理非coinbase tx 交易输入
            if !tx.is_coinbase() { 
                for txin in tx.get_vin() {
                    let mut updated_outs = vec![];
                    let out_bytes = utxo_tree.get(txin.get_txid()).unwrap().unwrap();
                    let outs: Vec<TxOutput> = bincode::deserialize(out_bytes.as_ref()).expect("");
                    for (idx, out) in outs.iter().enumerate() {
                        if idx != txin.get_outid() {
                            updated_outs.push(out.clone());
                        }
                    }

                    if updated_outs.len() == 0 { //如果输入对应的输出个数为0, 则从db中删除
                        let _ = utxo_tree.remove(txin.get_txid());
                    } else { //否则更新交易对应的output
                        let out_bytes = bincode::serialize(&updated_outs).expect("serialize updated outs failed");
                        utxo_tree.insert(txin.get_txid(), out_bytes);
                    }
                }
            }
            // 处理output
            let mut new_outputs = vec![];
            for out in tx.get_vout() {
                new_outputs.push(out.clone())
            }
            let outs_bytes =
                bincode::serialize(&new_outputs).expect("unable to serialize TXOutput");
            let _ = utxo_tree.insert(tx.get_id(), outs_bytes).unwrap();
        }

    }

    /// 统计UTXO集中tx数量
    pub fn count_transactions(&self) -> i32 {
        self.get_utxo_tree().iter().count() as u32 as i32
    }

    fn get_utxo_tree(&self) -> sled::Tree {
        self.blockchain
            .get_db()
            .open_tree(UTXO_TREE)
            .unwrap()
    }

    /// 查找pub_key_hash对应的所有utxo
    pub fn find_utxo(&self, pub_key_hash: &[u8]) -> Vec<TxOutput> {
        let mut utxos = vec![];
        let utxo_tree = self.get_utxo_tree();
        for item in utxo_tree.iter() {
            let (_, v) = item.unwrap();
            let outs: Vec<TxOutput> = bincode::deserialize(v.to_vec().as_slice()).expect("unable to deserialize TxOuput");
            for out in outs.iter() {
                if out.is_locked_with_key(pub_key_hash) {
                    utxos.push(out.clone());
                }
            }
        }

        utxos
    }

    /// 重建utxo
    pub fn reindex(&self) {
        let utxo_tree = self.get_utxo_tree();
        utxo_tree.clear();

        let utxo_map = self.blockchain.find_utxo();
        for (txid_hex, outs) in &utxo_map {
            let txid = HEXLOWER.decode(txid_hex.as_bytes()).unwrap();
            let value = bincode::serialize(outs).unwrap();
            utxo_tree.insert(txid.as_slice(), value);
        }
    }

    /// 查找所有可消费的output
    pub fn find_spendable_outputs(&self, pub_key_hash: &[u8], amount: i32) -> (i32, HashMap<String, Vec<usize>>) {
        let mut accmulated = 0;
        let mut unspent_outputs: HashMap<String, Vec<usize>> = HashMap::new();
        let utxo_tree = self.get_utxo_tree();
        for item in utxo_tree.iter() {
            let (k, v) = item.unwrap();
            let txid_hex = HEXLOWER.encode(k.to_vec().as_slice());
            let outs: Vec<TxOutput> = bincode::deserialize(v.to_vec().as_slice()).expect("unable to deserialize TxOutput");
            for (idx, out) in outs.iter().enumerate() {
                if out.is_locked_with_key(pub_key_hash) && accmulated < amount {
                    accmulated += out.get_value();
                    if unspent_outputs.contains_key(txid_hex.as_str()) {
                        unspent_outputs.get_mut(txid_hex.as_str())
                            .unwrap()
                            .push(idx)
                    } else {
                        unspent_outputs.insert(txid_hex.clone(), vec![idx]);
                    }
                }
            }
        }

        (accmulated, unspent_outputs)
    }
}
