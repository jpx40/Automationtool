#![allow(clippy::useless_attribute)]
#![allow(unused_imports)]
#![allow(dead_code)]
#![allow(unused_variables)]
pub mod builtin;
pub mod parser;
pub mod ssh;
pub mod user;
use argfile;
use clap_serde_derive::{
    clap::{self, Parser},
    ClapSerde,
};
use clio::*;
use core::str;
use dns_lookup::{getaddrinfo, AddrInfoHints, SockType};
use parser::TomlConfig;
use ssh::connection::*;
use ssh::*;
use ssh2::CheckResult;
use ssh2::{Channel, Session, Sftp, Stream};
use std::fs;
use std::fs::File;
use std::hash::RandomState;
use std::io::prelude::*;
use std::io::BufReader;
use std::io::{Error, ErrorKind};
use std::net::IpAddr;
use std::net::TcpStream;
use std::net::{Ipv4Addr, Ipv6Addr};
use std::path::Path;
use std::result;
use std::str::Bytes;
use std::string::String;
use std::thread::Result;
use std::{borrow, task};
use std::{clone, collections::HashMap};
use std::{path::PathBuf, time::Duration};
use toml;
use user::User;

#[derive(Parser)]
#[command(version, about , long_about = None)]
struct Args {
    input: Vec<std::path::PathBuf>,

    #[clap(short, long = "config")]
    config_path: std::path::PathBuf,

    /// Rest of arguments
    #[clap(flatten)]
    pub config: <Config as ClapSerde>::Opt,
}
#[derive(ClapSerde)]
struct Config {
    /// String argument
    #[clap(short, long)]
    name: String,
}

fn main() {
    let mut args = Args::parse();

    let c = cli(args);

    start(c);
    //https://gitlab.com/DPDmancul/clap-serde-derive
    //  led config = if let Ok(f) = File::open(&args.config_path) {
    // start("script/test.toml");
}
pub fn cli(args: Args) -> TomlConfig {
    let mut out = TomlConfig::new();

    let config = if let Ok(f) = PathBuf::from(args.config_path).canonicalize() {
        let file = fs::read_to_string(f).unwrap();
        let cf = toml::from_str::<TomlConfig>(&file);

        match cf {
            Ok(c) => out = c,

            Err(c) => println!("{:?}", c),
        }
    };
    out
}
pub fn start(c: TomlConfig) {
    let mut conf: TomlConfig = TomlConfig::new();
    // let mut config: TomlConfig = TomlConfig::new();
    // let check_result = parser::parse_toml(file);
    let config = c;
    conf = config.clone();
    // match check_result {
    //     Ok(r) => {
    //         println!("{:?}", r);
    //         conf = r;
    //         config = conf.clone();
    //     }
    //     Err(r) => println!("{:?}", r),
    // }
    for (k, v) in conf.task.unwrap().iter() {
        let mut cf: Option<parser::Config> = None;
        let mut session: Session;
        let task = v.clone();
        let mut s = String::new();

        for (kg, vg) in conf.group.clone().unwrap().iter() {
            if k == kg {
                cf = Some(vg.clone());
            }
        }
        match cf {
            Some(cfg) => {
                let user = User::new(cfg.user.clone().unwrap(), cfg.password.clone().unwrap());
                let connection = Connection::new(cfg.host.clone().unwrap(), cfg.port.unwrap());
                session = ssh_connect(user, connection);
                println!("Task: {:?}", &task.cmd);
                execute_task(&mut session, task);
                // match task.command {
                //     Some(c) => {
                //         s = execute_task(&mut session, &c).unwrap();
                //         //    println!("{}", s);
                //         println!("Task: {}", k);
                //         println!("Config: {}", k);
                //     }
                //     None => {
                //         println!("No command");
                //     }
                // }
            }
            None => match config.config.clone() {
                Some(cfg) => {
                    let user = User::new(cfg.user.clone().unwrap(), cfg.password.clone().unwrap());
                    let connection = Connection::new(cfg.host.clone().unwrap(), cfg.port.unwrap());
                    session = ssh_connect(user, connection);
                    execute_task(&mut session, task);

                    // match task.command {
                    //     Some(c) => {
                    //         s = execute_task(&mut session, &c).unwrap();
                    //         // println!("{}", s);
                    //         println!("Task: {}", k);
                    //         println!("Default Config",);
                    //     }
                    //     None => {
                    //         println!("No command");
                    //     }
                    // }
                }
                None => {
                    println!("No config");
                }
            },
        }
    }
}
pub fn execute_task(session: &mut Session, task: parser::Task) {
    let c = task.cmd;

    for (k, v) in c.iter() {
        if k == "builtin" {
            exec_cmd(session, v);
        } else {
            println!("Not a builtin: {}", k);
        }
    }
}

pub fn exec_cmd(session: &mut Session, cmd: &HashMap<String, parser::Cmd>) {
    for (k, v) in cmd.iter() {
        if k == "shell" {
            let script: String = builtin::shell(v.clone());
            let out: String = ssh::execute(session, script).unwrap();
            println!("{}", out);
        } else if k == "copy" {
            let s = builtin::copy(v.clone());
            let _ = ssh::copy_file(session, s);
        }
    }
}
fn match_user(conf: parser::Config, session: &mut Session) {}
//fn read_config() {}
