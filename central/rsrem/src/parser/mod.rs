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
    pub config: Config,
    pub task: HashMap<String, Task>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Config {
    pub host: Option<String>,
    pub user: Option<String>,
    pub port: Option<i32>,
    pub password: Option<String>,
    pub database: Option<String>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Task {
    pub command: Option<String>,
}

impl TomlConfig {
    pub fn new() -> TomlConfig {
        TomlConfig {
            config: Config {
                host: None,
                user: None,
                port: None,
                password: None,
                database: None,
            },
            task: HashMap::new(),
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
