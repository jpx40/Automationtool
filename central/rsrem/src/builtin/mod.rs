use std::{
    collections::HashMap,
    fmt::Debug,
    path::{self, PathBuf},
    str::{from_utf8, FromStr},
};

use crate::parser;
use arraystring::typenum::False;
use clap::builder::OsStr;
use nix::libc::printf;
use openssl::aes::unwrap_key;
use std::path::Path;
//use syn::{parse::Nothing, Path};

pub fn shell(cmd: parser::Cmd) -> String {
    let command: String = cmd.name.unwrap() + " " + &cmd.args.unwrap().join(" ");
    command.to_string()
}

pub struct CopyParam {
    location: String,
    file: Option<String>,
    dir: Option<String>,
    path: Option<String>,
}

impl CopyParam {
    fn new(location: String, file: String) -> CopyParam {
        CopyParam {
            location: location,
            path: Some(file),
            dir: None,
            file: None,
        }
    }
}
pub fn copy(cmd: parser::Cmd) -> CopyParam {
    let mut s: HashMap<String, String> = HashMap::new();
    let f = cmd.file.unwrap();
    let mut st = String::new();
    let mut file = String::new();

    let tmp = cmd.file.as_ref().unwrap().to_owned().clone();
    let mut path = PathBuf::from(&tmp).as_path();
    let dir = String::new();

    let mut pb = PathBuf::new();
    match cmd.file {
        Some(f) => {
            pb = PathBuf::from(f);
            match pb.is_file() {
                true => {
                    file = path.file_name().unwrap().to_string_lossy().to_string();
                }
                false => println!("no file"),
            }
        }

        None => st = "Nothing".to_string(),
    }

    let file = PathBuf::from(f)
        .canonicalize()
        .unwrap()
        .to_string_lossy()
        .to_string();

    CopyParam::new(cmd.location.unwrap(), file)
}
