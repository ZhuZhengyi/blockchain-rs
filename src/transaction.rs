// transaction.rs


use serde::{Deserialize, Serialize};


const SUBSIDY: i32 = 10;         //挖矿奖励


/// 交易输入
#[derive(Clone, Default, Serialize, Deserialize)]
pub struct TxInput {
    txid: Vec<u8>,          //
    vout: usize,            //
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
}

/// 交易输出
#[derive(Clone, Serialize, Deserialize)]
pub struct TxOutput {
    value: i32,                 //
    pub_key_hash: Vec<u8>,      //公钥hash
}

impl TxOutput {
    /// 新建一个交易输出
    pub fn new(value: i32, address: &str) -> Self {
        let mut output = TxOutput {
            value,
            pub_key_hash: vec![],
        }
        output.lock(address);
        output
    }

    fn lock(&mut self, address: &str) {

    }
}

/// 一次交易
pub struct Transaction {
    id: Vec<u8>,            // 本次交易ID
    vin: Vec<TxInput>,      // 本次交易输入
    vout: Vec<TxOutput>,    // 本次交易输出
}

impl Transaction {
    /// 新建coinbase交易, 
    pub fn new_coinbase_tx(to: &str) -> Self {
        let tx_out = TxOutput::new(SUBSIDY, to);
        let mut tx_in = TxInput::default();
        tx_in.signature = Uuid::new_v4().as_bytes().to_vec();

        let mut tx = Transaction {
            id: vec![],
            vin: vec![tx_in],
            vout: vec![tx_out],
        }
        tx.id = tx.hash();

        tx
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

}
