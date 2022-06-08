// proof_of_work.rs
//

use core::slice::SlicePattern;
use std::borrow::Borrow;
use data_encoding::HEXLOWER;

use crate::Block;
use num_bigint::{BigInt, Sign};

const TARGET_BITS: i32 = 8;             //
const MAX_NONCE:   i64 = i64::MAX;        //

/// 工作量证明
pub struct ProofOfWork {
    block: Block,       // 该证明所在的区块
    target: BigInt,     // 证明的目标值
}

impl ProofOfWork {
    /// 新建一个pow
    pub fn new(block: Block) -> Self {
        let mut target = BigInt::from(1);
        target <<= 256 - TARGET_BITS;

        ProofOfWork{block, target}
    }

    /// 准备运行数据
    fn prepare_data(&self, nonce: i64) -> Vec<u8> {
        let mut datas = vec![];
        datas.extend(self.block.get_pre_block_hash().as_bytes());
        datas.extend(self.block.hash_transactions());
        datas.extend(self.block.get_timestamp().to_be_bytes());
        datas.extend(nonce.to_be_bytes());

        datas
    }

    /// 运行pow, 返回有效nonce和hash
    pub fn run(&self) -> (i64, String) {
        let mut nonce: i64 = 0;
        let mut hash = Vec::new();

        println!("Mining the block");
        while nonce < MAX_NONCE {
            let data = self.prepare_data(nonce);
            hash = crate::sha256_digest(data.as_slice());
            let hash_int = BigInt::from_bytes_be(Sign::Plus, hash.as_slice());
            if hash_int.lt(self.target.borrow()) {
                println!("{}", HEXLOWER.encode(hash.as_slice()));
                break;
            } else {
                nonce += 1;
            }
        }

        println!();
        return (nonce, HEXLOWER.encode(hash.as_slice()));
    }
}

#[cfg(test)]
mod tests {
    use num_bigint::BigInt;

    #[test]
    fn test_bigint_from_bytes() {
        let a = BigInt::from(256);
        let (s, vec) = a.to_bytes_be();

        println!("{:?}, {:?}", s, vec);

        let b = BigInt::from_signed_bytes_be(vec.as_slice());
        println!("{}", b)
    }

}
