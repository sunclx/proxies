use serde::{Deserialize, Serialize};
use serde_yaml::Value;

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all(serialize = "kebab-case", deserialize = "kebab-case"))]
pub struct Config {
    pub port: u16,
    pub socks_port: u16,
    pub allow_lan: bool,
    pub mode: String,
    pub log_level: String,
    pub external_controller: String,
    pub proxies: Vec<Value>,
    pub proxy_groups: Vec<Value>,
    pub rules: Value,
    #[serde(flatten)]
    pub value: Value,
}

// #[derive(Debug, Serialize, Deserialize)]
// #[serde(rename_all(serialize = "kebab-case", deserialize = "kebab-case"))]
// pub struct Group {
//     pub name: String,
//     #[serde(rename(serialize = "type", deserialize = "type"))]
//     pub type_: String,
//     pub proxies: Vec<String>,
// }
