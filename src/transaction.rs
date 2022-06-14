// transaction.rs

use crate::{
    wallet::{self, hash_pub_key},
    Blockchain, 
    Wallets,
    utxo_set::UTXOSet
};
use data_encoding::HEXLOWER;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

const SUBSIDY: i32 = 10;         //挖矿奖励

/// 交易输入
#[derive(Clone, Default, Serialize, Deserialize)]
pub struct TxInput {
    txid: Vec<u8>,          // 交易的id
    outid: usize,           // 该交易输入对应交易输出的索引
    signature: Vec<u8>,     // 交易签名,
    pub_key: Vec<u8>,       // 公钥,
}

impl TxInput {
    /// 创建一个交易输入
    pub fn new(txid: &[u8], vout: usize) -> Self {
        TxInput {
            txid: txid.to_vec(),
            outid: vout,
            signature: vec![],
            pub_key: vec![],
        }
    }

    /// 获取交易id
    pub fn get_txid(&self) -> &[u8] {
        self.txid.as_slice()
    }

    /// 获取交易输出索引
    pub fn get_outid(&self) -> usize {
        self.outid
    }

    /// 获取交易公钥
    pub fn get_pub_key(&self) -> &[u8] {
        return self.pub_key.as_slice();
    }

    ///
    pub fn uses_key(&self, pub_key_hash: &[u8]) -> bool {
        let locking_hash = wallet::hash_pub_key(self.pub_key.as_slice());
        return locking_hash.eq(pub_key_hash);
    }
}

/// 交易输出
#[derive(Clone, Serialize, Deserialize)]
pub struct TxOutput {
    cost: i32,                  //交易花费币的数量
    pub_key_hash: Vec<u8>,      //公钥hash
}

impl TxOutput {
    /// 新建一个交易输出
    pub fn new(value: i32, address: &str) -> Self {
        let mut output = TxOutput {
            cost: value,
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

    // 检查是否lock
    pub fn is_locked_with_key(&self, key_hash: &[u8]) -> bool {
        self.pub_key_hash.eq(key_hash)
    }

    /// 获取公钥hash
    pub fn get_pub_key_hash(&self) -> &[u8] {
        self.pub_key_hash.as_slice()
    }

    /// 获取花费
    pub fn get_cost(&self) -> i32 {
        self.cost
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

    /// 新建coinbase交易
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

        return tx;
    }

    /// 新建一笔utxo交易
    pub fn new_utxo_transaction(from: &str, to: &str, amount: i32, utxo_set: &UTXOSet) -> Self {
        let wallets = Wallets::new();
        let wallet = wallets.get_wallet(from).unwrap();
        let pub_key_hash = hash_pub_key(wallet.get_public_key());

        let (accumulated, valid_outputs) = utxo_set.find_spendable_outputs(pub_key_hash.as_slice(), amount);
        if accumulated < amount {
            panic!("Error! not enough funds");
        }

        let mut inputs = vec![];
        for (txid_hex, outs) in valid_outputs {
            let txid = HEXLOWER.decode(txid_hex.as_bytes()).unwrap();
            for out in outs {
                let input = TxInput {
                    txid: txid.clone(),
                    outid: out,
                    pub_key: wallet.get_public_key().to_vec(),
                    signature: vec![],
                };
                inputs.push(input);
            }
        }


        let mut outputs = vec![TxOutput::new(amount, to)];
        // 如果 UTXO 总数超过所需，则产生找零
        if accumulated > amount {
            outputs.push(TxOutput::new(accumulated - amount, from)) // to: 币收入
        }
        // 4.生成交易
        let mut tx = Transaction {
            id: vec![],
            vin: inputs,
            vout: outputs,
        };
        // 生成交易ID
        tx.id = tx.hash();
        // 5.交易中的 TXInput 签名
        tx.sign(utxo_set.get_blockchain(), wallet.get_pkcs8());

        return tx;
    }

    /// 序列化该交易为一个字节数组
    pub fn serialize(&self) -> Vec<u8> {
        bincode::serialize(self).unwrap().to_vec()
    }

    /// 反序列化, byte数组->交易
    pub fn deserialize(bytes: &[u8]) -> Self {
        bincode::deserialize(bytes).unwrap()
    }

    /// 对交易每个输入进行签名
    pub fn sign(&mut self, blockchain: &Blockchain, pkcs8: &[u8]) {
        let mut tx_copy = self.trimmed_copy();

        for (idx, vin) in self.vin.iter_mut().enumerate() {
            // 查找输入引用的交易
            let prev_tx_option = blockchain.find_transaction(vin.get_txid());
            if prev_tx_option.is_none() {
                panic!("ERROR: Previous transaction is not correct")
            }
            let prev_tx = prev_tx_option.unwrap();
            tx_copy.vin[idx].signature = vec![];
            tx_copy.vin[idx].pub_key = prev_tx.vout[vin.outid].pub_key_hash.clone();
            tx_copy.id = tx_copy.hash();
            tx_copy.vin[idx].pub_key = vec![];

            // 使用私钥对数据签名
            vin.signature = crate::ecdsa_p256_sha256_sign_digest(pkcs8, tx_copy.get_id());
        }
    }

    /// 计算交易hash
    fn hash(&mut self) -> Vec<u8> {
        let tx_copy = Transaction {
            id: vec![],
            vin: self.vin.clone(),
            vout: self.vout.clone(),
        };
        crate::sha256_digest(tx_copy.serialize().as_slice())
    }

    /// 获取交易id
    pub fn get_id(&self) -> &[u8] {
        self.id.as_slice()
    }

    /// 获取交易id副本
    pub fn get_id_bytes(&self) -> Vec<u8> {
        self.id.clone()
    }

    /// 获取交易输入
    pub fn get_vin(&self) -> &[TxInput] {
        self.vin.as_slice()
    }

    /// 获取交易输出
    pub fn get_vout(&self) -> &[TxOutput] {
        self.vout.as_slice()
    }

    /// 验证交易的签名是否合法
    pub fn verify(&self, blockchain: &Blockchain) -> bool {
        if self.is_coinbase() {
            return true;
        }
        let mut tx_copy = self.trimmed_copy();
        for (idx, vin) in self.vin.iter().enumerate() {
            let prev_tx = blockchain.find_transaction(vin.get_txid()).unwrap();

            tx_copy.vin[idx].signature = vec![];
            tx_copy.vin[idx].pub_key = prev_tx.vout[vin.outid].pub_key_hash.clone();
            tx_copy.id = tx_copy.hash();
            tx_copy.vin[idx].pub_key = vec![];

            let verify = crate::ecdsa_p256_sha256_sign_verify(
                vin.pub_key.as_slice(),
                vin.signature.as_slice(),
                tx_copy.get_id(),
            );
            if !verify {
                return false;
            }
        }

        return true;
    }

    /// 是否为coinbase交易.
    /// coinbase交易的输入为
    pub fn is_coinbase(&self) -> bool {
        return self.vin.len() == 1 && self.vin[0].pub_key.len() == 0;
    }

    /// 修剪交易后的副本
    fn trimmed_copy(&self) -> Transaction {
        Transaction {
            id: self.id.clone(),
            vin: self.vin.iter().cloned().map(|input|TxInput::new(input.get_txid(), input.get_outid())).collect(),
            vout: self.vout.iter().cloned().collect(),
        }
    }
}
