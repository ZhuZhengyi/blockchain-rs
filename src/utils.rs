// utils.rs
//

use ring::digest::{Context, SHA256};
use std::time::{SystemTime, UNIX_EPOCH};

/// 当前时间戳
pub fn current_timestamp() -> i64 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .expect("Time went backwards")
        .as_millis() as i64
}

/// sha256 摘要算法
pub fn sha256_digest(data: &[u8]) -> Vec<u8> {
    let mut context = Context::new(&SHA256);
    context.update(data);

    context.finish().as_ref().to_vec()
}

/// base58 编码
pub fn base58_encode(data: &[u8]) -> String {
    bs58::encode(data).into_string()
}
