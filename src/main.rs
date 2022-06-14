// main.rs

use blockchain::{Blockchain, UTXOSet, Wallets, validate_address, utils, Transaction, ADDRESS_CHECKSUM_LEN, send_tx, CENTERAL_NODE, convert_address, hash_pub_key, GLOBAL_CONFIG, Server};
use data_encoding::HEXLOWER;
use log::LevelFilter;
use structopt::StructOpt;

const MINE_TRUE: usize = 1; //
                            //
#[derive(Debug, StructOpt)]
#[structopt(name="blockchain")]
struct Opt {
    #[structopt(subcommand)]
    command: Command,
}

#[derive(StructOpt, Debug)]
enum Command {
    #[structopt(name="create-blockchain", about="Create a new blockchain")]
    CreateBlockchain {
        #[structopt(name="address", help="The address to send genesis block reward to")]
        address: String,
    },
    #[structopt(name="create-wallet", about="Create a new wallet")]
    CreateWallet,
    #[structopt(name="get-balance", about="Create a new wallet")]
    GetBalance {
        #[structopt(name="address", help="The wallet adddress")]
        address: String,
    },
    #[structopt(name="list-addresses", about="Print local wallet address")]
    ListAddresses,
    #[structopt(name="send", about="Create a new wallet")]
    Send {
        #[structopt(name="from", help="Source wallet address")]
        from: String,
        #[structopt(name="to", help="Destination wallet address")]
        to: String,
        #[structopt(name="amount", help="Amount to send")]
        amount: i32,
        #[structopt(name="mine", help="Mine immediately on the same node")]
        mine: usize,
    },
    #[structopt(name="print-chain", about="Print local wallet address")]
    PrintChain,
    #[structopt(name="reindex-utxo", about="Reindex utxo set")]
    ReindexUTXO,
    #[structopt(name="start-node", about="Start a node")]
    StartNode {
        #[structopt(name="miner", help="Enable mining mode and send reward to ADDRESS")]
        miner: Option<String>,
    },
}

fn main() {
    env_logger::builder().filter_level(LevelFilter::Info).init();
    let opt = Opt::from_args();
    match opt.command {
        Command::CreateBlockchain { address } => {
            let blockchain = Blockchain::create_blockchain(address.as_str());
            let utxo_set = UTXOSet::new(blockchain);
            utxo_set.reindex();
            println!("Create blockchain addr: {} Done!", address);
        },
        Command::CreateWallet => {
            let mut wallets = Wallets::new();
            let address = wallets.create_wallet();
            println!("Your new address: {}", address);
        },
        Command::GetBalance { address } => {
            let address_valid = validate_address(address.as_str());
            if !address_valid {
                panic!("ERROR: address {} is not valid", address)
            }

            let payload = utils::base58_decode(address.as_str());
            let pub_key_hash = &payload[1..payload.len() - ADDRESS_CHECKSUM_LEN];

            let blockchain = Blockchain::open_blockchain();
            let utxo_set = UTXOSet::new(blockchain);
            let utxos = utxo_set.find_utxo(pub_key_hash);
            let mut balance = 0;
            for utxo in utxos {
                balance += utxo.get_cost();
            }
            println!("Balance of {}: {}", address, balance);
        },
        Command::ListAddresses => {
            let wallets = Wallets::new();
            for address in wallets.get_addresses() {
                println!("{}", address)
            }
        },
        Command::Send { from, to, amount, mine } => {
            if !validate_address(from.as_str()) {
                panic!("ERROR: Sender address is not valid")
            }
            if !validate_address(to.as_str()) {
                panic!("ERROR: Recipient address is not valid")
            }
            let blockchain = Blockchain::open_blockchain();
            let utxo_set = UTXOSet::new(blockchain.clone());
            // 创建 UTXO 交易
            let transaction =
                Transaction::new_utxo_transaction(from.as_str(), to.as_str(), amount, &utxo_set);

            if mine == MINE_TRUE {
                // 挖矿奖励
                let coinbase_tx = Transaction::new_coinbase_tx(from.as_str());
                // 挖新区块
                let block = blockchain.mine_block(&vec![transaction, coinbase_tx]);
                // 更新 UTXO 集
                utxo_set.update(block);
            } else {
                send_tx(CENTERAL_NODE, &transaction);
            }
            println!("Success!")
        },
        Command::PrintChain => {
            let mut block_iterator = Blockchain::open_blockchain().iterator();
            loop {
                let option = block_iterator.next();
                if option.is_none() {
                    break;
                }
                let block = option.unwrap();
                println!("Pre block hash: {}", block.get_pre_block_hash());
                println!("Cur block hash: {}", block.get_hash());
                println!("Cur block Timestamp: {}", block.get_timestamp());
                for tx in block.get_transactions() {
                    let cur_txid_hex = HEXLOWER.encode(tx.get_id());
                    println!("- Transaction txid_hex: {}", cur_txid_hex);

                    if tx.is_coinbase() == false {
                        for input in tx.get_vin() {
                            let txid_hex = HEXLOWER.encode(input.get_txid());
                            let pub_key_hash = hash_pub_key(input.get_pub_key());
                            let address = convert_address(pub_key_hash.as_slice());
                            println!(
                                "-- Input txid = {}, vout = {}, from = {}",
                                txid_hex,
                                input.get_outid(),
                                address,
                            )
                        }
                    }
                    for output in tx.get_vout() {
                        let pub_key_hash = output.get_pub_key_hash();
                        let address = convert_address(pub_key_hash);
                        println!("-- Output value = {}, to = {}", output.get_cost(), address,)
                    }
                }
                println!()
            }
        },
        Command::ReindexUTXO => {
            let blockchain = Blockchain::open_blockchain();
            let utxo_set = UTXOSet::new(blockchain);
            utxo_set.reindex();
            let count = utxo_set.count_transactions();
            println!("Done! There are {} transactions in the UTXO set.", count);
        },
        Command::StartNode { miner } => {
            if let Some(addr) = miner {
                if validate_address(addr.as_str()) == false {
                    panic!("Wrong miner address!")
                }
                println!("Mining is on. Address to receive rewards: {}", addr);
                GLOBAL_CONFIG.set_mining_addr(addr);
            }
            let blockchain = Blockchain::open_blockchain();
            let sockert_addr = GLOBAL_CONFIG.get_node_addr();
            Server::new(blockchain).run(sockert_addr.as_str());
        },
    }
}

