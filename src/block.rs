
/// 区块
pub struct Block {
    timestamp: i64,         // 生成区块时间戳
    block_hash: String,     // 区块hash
    pre_block_hash: String, // 上一个区块hash
    nonce: i64,             //
    height: usize,          // 区块高度(该区块相对于创世区块的个数)
}

