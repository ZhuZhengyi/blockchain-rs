# Blockchain with rust

## howto

```shell

$ cargo build 

$ mkdir -p data/node{1,2,3}

[node1]$ cd data/node1
## 创建wallet
[node1]$ ../../target/debug/blockchain_rust createwallet
Your new address: 13teoDGrDNhMrgGHHRESv7bhcnJoXnicqE

## 创建block0
[node1]$ ../../target/debug/blockchain_rust createblockchain 13teoDGrDNhMrgGHHRESv7bhcnJoXnicqE
Mining the block
00518dae1ee13a19da96d24f865654eed74dfb3188f4ae0fe56616c03535acb9

Done!
## 手动同步创世区块数据
[node2] cd data/node2; cp -rf ../node1/data .
[node3] cd data/node3; cp -rf ../node1/data .

## node2创建钱包
[node2]$ ../../target/debug/blockchain_rust createwallet
Your new address: 1JvjDQGrmzVLLAo9dPsXYNNednuNZrGrAm
[node2]$ ../../target/debug/blockchain_rust createwallet
Your new address: 1JvjDQGrmzVLLAo9dPsXYNNednuNZrGrAm
[node2]$ ../../target/debug/blockchain_rust createwallet
Your new address: 1FztXwduMcABm6x2yxRWgozqT8g1dgeiFS

```
