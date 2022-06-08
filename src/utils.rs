// utils.rs
//

use crypto::digest::Digest;
use ring::{
    digest::{Context, SHA256},
    rand::SystemRandom,
    signature::{
        EcdsaKeyPair,
        ECDSA_P256_SHA256_FIXED_SIGNING,
        ECDSA_P256_SHA256_FIXED
    }
};
use std::{time::{SystemTime, UNIX_EPOCH}, iter::repeat};

/// 当前时间戳
pub fn current_timestamp() -> u64 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .expect("Time went backwards")
        .as_millis() as u64
}

/// sha256 摘要算法
pub fn sha256_digest(data: &[u8]) -> Vec<u8> {
    let mut context = Context::new(&SHA256);
    context.update(data);

    context.finish().as_ref().to_vec()
}

/// ripedmd160 消息摘要
pub fn ripemd160_digest(data: &[u8]) -> Vec<u8> {
    let mut ripemd160 = crypto::ripemd160::Ripemd160::new();
    ripemd160.input(data);

    let mut buf: Vec<u8> = repeat(0).take(ripemd160.output_bytes()).collect();
    ripemd160.result(&mut buf);

    buf
}

/// base58 编码
pub fn base58_encode(data: &[u8]) -> String {
    bs58::encode(data).into_string()
}

/// base58 解码
pub fn base58_decode(data: &str) -> Vec<u8> {
    bs58::decode(data).into_vec().unwrap()
}

/// 新建pkcs8密钥对
pub fn new_key_pair() -> Vec<u8> {
    let rng = SystemRandom::new();
    let pkcs8 = EcdsaKeyPair::generate_pkcs8(&ECDSA_P256_SHA256_FIXED_SIGNING, &rng).unwrap();
    pkcs8.as_ref().to_vec()
}

/// 使用私钥,计算消息签名摘要
pub fn ecdsa_p256_sha256_sign_digest(pkcs8: &[u8], message: &[u8]) -> Vec<u8>  {
    let key_pair = EcdsaKeyPair::from_pkcs8(&ECDSA_P256_SHA256_FIXED_SIGNING, pkcs8).unwrap();
    let rng = ring::rand::SystemRandom::new();
    key_pair.sign(&rng, message).unwrap().as_ref().to_vec()
}

/// 使用公钥, 验证消息签名是否合法
pub fn ecdsa_p256_sha256_sign_verify(pub_key: &[u8], signature: &[u8], message: &[u8]) -> bool {
    let peer_pub_key = ring::signature::UnparsedPublicKey::new(&ECDSA_P256_SHA256_FIXED, pub_key);
    peer_pub_key.verify(message, signature.as_ref()).is_ok()
}
