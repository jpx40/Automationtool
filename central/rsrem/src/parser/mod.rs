use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fs::File;
use std::io::Read;
use std::mem;
use std::path::Path;
use toml;
use walkdir::WalkDir;

//https://doc.rust-lang.org/rust-by-example/std/box.html
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct TomlConfig {
    pub config: Option<Config>,
    pub group: Option<HashMap<String, Config>>,
    pub task: Option<HashMap<String, Task>>,
    pub env: Option<HashMap<String, String>>,
    pub include: Option<Vec<String>>,
    pub exclude: Option<Vec<String>>,
    pub path: Option<Vec<String>>,
    pub include_path: Option<Vec<String>>,
    pub exclude_path: Option<Vec<String>>,
    pub plugin: Option<Vec<String>>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Config {
    pub host: Option<String>,
    pub user: Option<String>,
    pub port: Option<u32>,
    pub password: Option<String>,
    pub database: Option<String>,
    pub key: Option<String>,
    pub secret: Option<String>,
    pub token: Option<String>,
    pub access_key: Option<String>,
    pub key_file: Option<String>,
    pub cert_file: Option<String>,
    pub ca_file: Option<String>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Task {
    pub name: String,
    pub cmd: HashMap<String, HashMap<String, Cmd>>,
}

#[derive(Debug)]
pub enum Command {
    Builtin,
    Plugin,
}

pub struct Plugin {}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Cmd {
    pub name: Option<String>,
    pub args: Option<Vec<String>>,
    pub envs: Option<Vec<String>>,
    pub command: Option<String>,
    pub file: Option<String>,
    pub dir: Option<String>,
    pub ip: Option<String>,
    pub option: Option<Vec<String>>,
    pub location: Option<String>,
}

impl TomlConfig {
    pub fn new() -> TomlConfig {
        TomlConfig {
            config: None,
            group: Some(HashMap::new()),
            task: Some(HashMap::new()),
            env: Some(HashMap::new()),
            path: Some(Vec::new()),
            include_path: Some(Vec::new()),
            include: Some(Vec::new()),
            exclude: Some(Vec::new()),
            exclude_path: Some(Vec::new()),
            plugin: Some(Vec::new()),
        }
    }
}

impl Config {
    pub fn new() -> Config {
        Config {
            host: None,
            user: None,
            port: None,
            password: None,
            database: None,
            key: None,
            secret: None,
            token: None,
            access_key: None,
            key_file: None,
            cert_file: None,
            ca_file: None,
        }
    }
}
pub fn parse_toml(s: &str) -> Result<TomlConfig, String> {
    let path = Path::new(&s);
    let mut file = File::open(path).map_err(|e| format!("Failed to open {:?}: {}", path, e))?;

    let mut contents = String::new();
    let _ = &mut file.read_to_string(&mut contents);
    let config: TomlConfig = toml::from_str(&contents).unwrap();
    Ok(config)
}
