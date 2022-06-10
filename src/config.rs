// config.rs

use std::{sync::RwLock, collections::HashMap, env};

use once_cell::sync::Lazy;

pub static GLOBAL_CONFIG: Lazy<Config> = Lazy::new(||Config::new());

static DEFAULT_NODE_ADDR: &str = "127.0.0.1:2001";
const NODE_ADDRESS_KEY: &str = "NODE_ADDRESS";
const MINING_ADDRESS_KEY: &str = "MINING_ADDRESS";

/// 配置
pub struct Config {
    inner: RwLock<HashMap<String, String>>,
}

impl Config {
    /// 新建一个配置
    pub fn new() -> Self {
        let mut node_addr = String::from(DEFAULT_NODE_ADDR);
        if let Ok(addr) = env::var(NODE_ADDRESS_KEY) {
            node_addr = addr
        }
        let mut map = HashMap::new();
        map.insert(String::from(NODE_ADDRESS_KEY), node_addr);

        Config {
            inner: RwLock::new(map)
        }
    }

    /// 获取节点地址
    pub fn get_node_addr(&self) -> String {
        self.inner.read()
            .unwrap()
            .get(NODE_ADDRESS_KEY)
            .unwrap()
            .clone()
    }

    /// 设置旷工地址
    pub fn set_mining_addr(&self, addr: String) {
        self.inner.write()
            .unwrap()
            .insert(String::from(MINING_ADDRESS_KEY), addr);
    }

    pub fn get_mining_addr(&self) -> Option<String> {
        self.inner.read()
            .unwrap()
            .get(MINING_ADDRESS_KEY)
            .cloned()
    }

    /// 当前节点是否为矿工
    pub fn is_miner(&self) -> bool {
        self.inner.read()
            .unwrap()
            .contains_key(MINING_ADDRESS_KEY)
    }
}
