// transaction.rs

use crate::{base58_decode, wallet, Blockchain};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

const SUBSIDY: i32 = 10;         //挖矿奖励

/// 交易输入
#[derive(Clone, Default, Serialize, Deserialize)]
pub struct TxInput {
    txid: Vec<u8>,          // 前一个交易的id
    vout: usize,            // 输出的索引
    signature: Vec<u8>,     // 交易签名
    pub_key: Vec<u8>,       // 公钥
}

impl TxInput {
    /// 创建一个交易输入
    pub fn new(txid: &[u8], vout: usize) -> Self {
        TxInput {
            txid: txid.to_vec(),
            vout,
            signature: vec![],
            pub_key: vec![],
        }
    }

    /// 获取交易id
    pub fn get_txid(&self) -> &[u8] {
        self.txid.as_slice()
    }

    /// 获取交易输出索引
    pub fn get_vout(&self) -> usize {
        self.vout
    }

    /// 获取交易公钥
    pub fn get_pub_key(&self) -> &[u8] {
        self.pub_key.as_slice()
    }

    ///
    pub fn uses_key(&self, pub_key_hash: &[u8]) -> bool {
        todo!()
    }
}



/// 交易输出
#[derive(Clone, Serialize, Deserialize)]
pub struct TxOutput {
    value: i32,                 //交易花费币的数量
    pub_key_hash: Vec<u8>,      //公钥hash
}

impl TxOutput {
    /// 新建一个交易输出
    pub fn new(value: i32, address: &str) -> Self {
        let mut output = TxOutput {
            value,
            pub_key_hash: vec![],
        };
        output.lock(address);

        return output;
    }

    /// lock
    fn lock(&mut self, address: &str) {
        let payload = crate::base58_decode(address);
        self.pub_key_hash = payload[1..payload.len()-wallet::ADDRESS_CHECKSUM_LEN].to_vec();
    }
    
    fn is_locked_with_key(&self, key_hash: &[u8]) -> bool {
        self.pub_key_hash.eq(key_hash)
    }
}

/// 一次交易
#[derive(Clone, Default, Serialize, Deserialize)]
pub struct Transaction {
    id: Vec<u8>,            // 本次交易ID
    vin: Vec<TxInput>,      // 本次交易输入
    vout: Vec<TxOutput>,    // 本次交易输出
}

impl Transaction {

    /// 新建coinbase交易, 
    /// coinbase交易的输入是
    pub fn new_coinbase_tx(to: &str) -> Self {
        let tx_out = TxOutput::new(SUBSIDY, to);
        let mut tx_in = TxInput::default();
        tx_in.signature = Uuid::new_v4().as_bytes().to_vec();

        let mut tx = Transaction {
            id: vec![],
            vin: vec![tx_in],
            vout: vec![tx_out],
        };
        tx.id = tx.hash();

        tx
    }

    pub fn serialize(&self) -> Vec<u8> {
        bincode::serialize(self).unwrap().to_vec()
    }

    fn hash(&mut self) -> Vec<u8> {
        let tx_copy = Transaction {
            id: vec![],
            vin: self.vin.clone(),
            vout: self.vout.clone(),
        };
        crate::sha256_digest(tx_copy.serialize().as_slice())
    }

    pub fn get_id(&self) -> &[u8] {
        self.id.as_slice()
    }

    pub fn verify(&self, blockchain: &Blockchain) -> bool {


        todo!()
    }

    /// 是否为coinbase交易. 
    /// coinbase交易的输入为
    pub fn is_coinbase(&self) -> bool {
        return self.vin.len() == 1 && self.vin[0].pub_key.len() == 0;
    }

}
