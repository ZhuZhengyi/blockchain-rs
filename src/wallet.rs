// wallet.rs
//

use core::slice::SlicePattern;

use ring::signature::{EcdsaKeyPair, ECDSA_P256_SHA256_FIXED_SIGNING, KeyPair};
use serde::{Serialize, Deserialize};

const VERSION: u8 = 0x00;
pub const ADDRESS_CHECKSUM_LEN: usize = 4;  //地址checksum长度

/// 钱包
#[derive(Clone, Serialize, Deserialize)]
pub struct Wallet {
    pkcs8: Vec<u8>,             // 公私钥对
    public_key: Vec<u8>,        // 公钥
}

impl Wallet {

    /// 新建一个钱包
    pub fn new() -> Self { 
        let pkcs8 = crate::new_key_pair();
        let key_pair = EcdsaKeyPair::from_pkcs8(&ECDSA_P256_SHA256_FIXED_SIGNING, pkcs8.as_ref()).unwrap();

        Wallet{pkcs8, public_key:key_pair.public_key().as_ref().to_vec()}
    }

    pub fn get_public_key(&self) -> &[u8] {
        self.public_key.as_slice()
    }

    pub fn get_pkcs8(&self) -> &[u8] {
        self.pkcs8.as_slice()
    }

    /// 获取钱包地址
    /// address = base58( version+ hash(pub_key) + checksum)
    pub fn get_address(&self) -> String {
        let pub_key_hash = hash_pub_key(&self.public_key);
        convert_address(pub_key_hash.as_slice())
    }
}

/// 计算公钥hash
pub fn hash_pub_key(pub_key: &[u8]) -> Vec<u8> {
    let pub_key_sha256 = crate::sha256_digest(pub_key);
    crate::ripemd160_digest(pub_key_sha256.as_slice())
}

/// 计算payload校验和
pub fn checksum(payload: &[u8]) -> Vec<u8> {
    let first_sha = crate::sha256_digest(payload);
    let second_sha = crate::sha256_digest(first_sha.as_slice());

    second_sha[0..ADDRESS_CHECKSUM_LEN].to_vec()
}

/// 验证地址是否有效 
pub fn validate_address(address: &str) -> bool {
    let payload = crate::base58_decode(address);
    let actual_checksum = payload[payload.len()-ADDRESS_CHECKSUM_LEN..].to_vec();
    let version = payload[0];
    let pub_key_hash = payload[1..payload.len()-ADDRESS_CHECKSUM_LEN].to_vec();

    let mut target_vec = vec![];
    target_vec.push(version);
    target_vec.extend(pub_key_hash);
    let target_checksum = checksum(target_vec.as_slice());

    actual_checksum.eq(target_checksum.as_slice())
}

/// 通过公钥hash key计算address
pub fn convert_address(pub_hash_key: &[u8]) -> String {
    let mut payload: Vec<u8> = vec![];
    payload.push(VERSION);
    payload.extend(pub_hash_key);
    let checksum = checksum(payload.as_slice());
    payload.extend(checksum.as_slice());

    crate::base58_encode(&payload.as_slice())
}
