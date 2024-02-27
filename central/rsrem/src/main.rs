#![allow(clippy::useless_attribute)]
#![allow(unused_imports)]
#![allow(dead_code)]
#![allow(unused_variables)]
pub mod parser;
pub mod ssh;
pub mod user;
use dns_lookup::{getaddrinfo, AddrInfoHints, SockType};
use parser::TomlConfig;
use ssh::connection::*;
use ssh::*;
use ssh2::CheckResult;
use ssh2::{Channel, Session, Sftp, Stream};
use std::clone;
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
use std::{path::PathBuf, time::Duration};
use user::User;

fn main() {
    let host = "127.0.0.1";
    let port = 22;

    // let connection = Connection::new(host.to_string(), port);
    // let user = User::new("jonas".to_string(), "Artemis34!!".to_string());
    // let mut local_session = ssh_connect(user, connection);
    // let notbook_user = User::new("jonas".to_string(), "artemisJP!!".to_string());
    //let notbook_connection = Connection::new("192.168.178.46".to_string(), port);

    //let mut session = ssh_connect(notbook_user, notbook_connection);
    let path = Path::new("test.txt");
    let size = path.metadata().unwrap().len();
    if size == 0 {
        panic!("File size is zero");
    };
    let mut buffer: Vec<u8> = Vec::new();

    let _ = File::open(path).unwrap().read_to_end(&mut buffer);
    //   let mut reader = BufReader::new(file);

    //let buf = reader.fill_buf().unwrap();
    /*     let times: Option<(u64, u64)> = None; */

    let mut conf: TomlConfig = TomlConfig::new();
    let mut config: TomlConfig = TomlConfig::new();
    let check_result = parser::parse_toml("script/test.toml");

    match check_result {
        Ok(r) => {
            println!("{:?}", r);
            conf = r;
            config = conf.clone();
        }
        Err(r) => println!("{:?}", r),
    }
    // let user: User = User::new(
    //     conf.config.clone().unwrap().user.unwrap(),
    //     conf.config.clone().unwrap().password.unwrap(),
    // );
    // let connection = Connection::new(conf.config.clone().unwrap().host.unwrap(), 22);
    // let mut session = ssh_connect(user, connection);
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
                match task.command {
                    Some(c) => {
                        s = execute_task(&mut session, &c).unwrap();
                        //    println!("{}", s);
                        println!("Task: {}", k);
                        println!("Config: {}", k);
                    }
                    None => {
                        println!("No command");
                    }
                }
            }
            None => match config.config.clone() {
                Some(cfg) => {
                    match &cfg.key {
                        Some(u) => {
                            let user = User::with_key(u.to_string());
                            let connection = Connection::new(cfg.host.clone().unwrap(), cfg.port.unwrap());
                            session = ssh_connect_with_key(user, connection);

                        }
                        None => {
                            match &cfg.user {
                                Some(u) => {}
                                None => {}


                        }
                    }

                    let user = User::new(cfg.user.clone().unwrap(), cfg.password.clone().unwrap());
                    let connection = Connection::new(cfg.host.clone().unwrap(), cfg.port.unwrap());
                    session = ssh_connect(user, connection);
                    match task.command {
                        Some(c) => {
                            s = execute_task(&mut session, &c).unwrap();
                            // println!("{}", s);
                            println!("Task: {}", k);
                            println!("Default Config",);
                        }
                        None => {
                            println!("No command");
                        }
                    }
                }
                None => {
                    println!("No config");
                }
            },
        }
    }
    // let os = check_os_type(&mut local_session);
    // println!("{}", os);
    // let remote_file = RemoteFile::new(path.to_str().unwrap().to_string());
    //   file_upload(&mut session, path, size, &buffer, times);
    // remote_file.send(&mut session);

    // let test = String::from_utf8(buffer).unwrap();
    // println!("String: {}\nSize: {}", test, size);
    //
    // let _ = check_if_nu_exist(&mut local_session);

    // let _ = check_if_nu_exist(&mut session);
}

//fn read_config() {}
