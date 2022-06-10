// wallets.rs
//
//

use std::{
    env::current_dir, 
    fs::{OpenOptions, File}, 
    io::{BufWriter, Write, Read}, 
    collections::HashMap
};

use crate::wallet::Wallet;

pub const WALLET_FILE: &str = "wallet.dat";

/// 钱包管理
pub struct Wallets {
    wallets: HashMap<String, Wallet>,
}

impl Wallets {

    /// 新建wallets
    pub fn new() -> Self {
        let mut wallets = Wallets {
            wallets: HashMap::new(),
        };
        wallets.load_from_file();

        wallets
    }

    /// 创建一个新钱包
    pub fn create_wallet(&mut self) -> String {
        let wallet = Wallet::new();
        let address = wallet.get_address();
        self.wallets.insert(address.clone(), wallet);
        self.save_to_file();

        return address;
    }

    pub fn get_addresses(&self) -> Vec<String> {
        self.wallets.keys().cloned().collect()
    }

    pub fn get_wallet(&self, address: &str) -> Option<&Wallet> {
        self.wallets.get(address)
    }

    /// 保存wallets到文件
    pub fn save_to_file(&self) {
        let path = current_dir().unwrap().join(WALLET_FILE);
        let file = OpenOptions::new()
            .create(true)
            .write(true)
            .open(&path)
            .expect("unable to open wallet.dat");
        let mut writer = BufWriter::new(file);
        let wallets_bytes = bincode::serialize(&self.wallets).expect("unable to serialize wallets");
        writer.write(wallets_bytes.as_slice()).unwrap();
        let _ = writer.flush();
    }

    /// 从文件加载wallets
    pub fn load_from_file(&mut self) {
        let path = current_dir().unwrap().join(WALLET_FILE);
        let mut file = File::open(path).unwrap();
        let metadata = file.metadata().expect("unable to read metadata");
        let mut buf = vec![0; metadata.len() as usize];
        let _ = file.read(&mut buf).expect("buffer overflow");
        let wallets = bincode::deserialize(&buf[..]).expect("unable to deserialize file data");
        self.wallets = wallets;
    }
}

#[cfg(test)]
mod tests {
    use crate::Wallets;

    #[test]
    fn test_new_wallets() {
        let mut wallets = Wallets::new();
        let address = wallets.create_wallet();

        println!("The new wallet address is {}", address);
    }

}
