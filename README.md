# Blockchain with rust

## howto

```shell

$ cargo build 

$ mkdir -p data/node{1,2,3}

[node1]$ cd data/node1

## 创建wallet_1
[node1]$ ../../target/debug/blockchai create-wallet
Your new address: 13teoDGrDNhMrgGHHRESv7bhcnJoXnicqE

## 创建blockchain
[node1]$ ../../target/debug/blockchain_rust create-blockchain 13teoDGrDNhMrgGHHRESv7bhcnJoXnicqE
Mining the block
00518dae1ee13a19da96d24f865654eed74dfb3188f4ae0fe56616c03535acb9

Done!

## 查看wallet_1
[node1]$ ../bin/blockchain get-balance  13teoDGrDNhMrgGHHRESv7bhcnJoXnicqE

## 手动同步创世区块数据
[node2]$ cd data/node2; cp -rf ../node1/data .
[node3]$ cd data/node3; cp -rf ../node1/data .

## node2创建钱包
[node2]$ ../../target/debug/blockchain create-wallet
Your new address: 1JvjDQGrmzVLLAo9dPsXYNNednuNZrGrAm

[node2]$ ../../target/debug/blockchain create-wallet
Your new address: 1JvjDQGrmzVLLAo9dPsXYNNednuNZrGrAm

[node2]$ ../../target/debug/blockchain create-wallet
Your new address: 1FztXwduMcABm6x2yxRWgozqT8g1dgeiFS

## 启动node1
[node1]$ export NODE_ADDRESS=127.0.0.1:2001
[node1]$ ../bin/blockchain start-node

## 发送代币
[node1]$ ../bin/blockchain send ${WALLET_0} ${WALLET_1} 3 1
[node1]$ ../bin/blockchain send ${WALLET_0} ${WALLET_2} 5 1

## 查看交易后钱包余额
[node1]$ ../bin/blockchain get-balance ${WALLET_0} 
[node1]$ ../bin/blockchain get-balance ${WALLET_1} 
[node1]$ ../bin/blockchain get-balance ${WALLET_2} 
[node1]$ ../bin/blockchain get-balance ${WALLET_3} 

## 重启node1
[node1]$ ../bin/blockchain start-node

## 启动node2
[node2]$ export NODE_ADDRESS=127.0.0.1:2002
[node2]$ ../bin/blockchain start-node

## 启动node3
[node3]$ export NODE_ADDRESS=127.0.0.1:2003
[node3]$ export MINER_ADDR=
[node3]$ ../bin/blockchain start-node ${MINER_ADDR}

## 查看chain
[node3]$ ../bin/blockchain print-chain

```
