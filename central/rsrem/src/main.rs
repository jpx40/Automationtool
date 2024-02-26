#![allow(clippy::useless_attribute)]
#![allow(unused_imports)]
#![allow(dead_code)]
#![allow(unused_variables)]
pub mod ssh;
pub mod user;
use dns_lookup::{getaddrinfo, AddrInfoHints, SockType};
use ssh::connection::*;
use ssh::*;
use ssh2::{Channel, Session, Sftp, Stream};
use std::borrow;
use std::clone;
use std::fs::File;
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
use std::{path::PathBuf, time::Duration};
use user::User;

fn main() {
    let host = "127.0.0.1";
    let port = 22;

    let connection = Connection::new(host.to_string(), port);
    let user = User::new("jonas".to_string(), "Artemis34!!".to_string());
    let mut local_session = ssh_connect(user, connection);
    let notbook_user = User::new("jonas".to_string(), "artemisJP!!".to_string());
    let notbook_connection = Connection::new("192.168.178.46".to_string(), port);

    let mut session = ssh_connect(notbook_user, notbook_connection);
    let path = Path::new("test.txt");
    let size = path.metadata().unwrap().len();
    if size == 0 {
        panic!("File size is zero");
    };
    let mut buffer: Vec<u8> = Vec::new();

    let _ = File::open(path).unwrap().read_to_end(&mut buffer);
    //   let mut reader = BufReader::new(file);

    //let buf = reader.fill_buf().unwrap();
    let times: Option<(u64, u64)> = None;

    let os = check_os_type(&mut local_session);
    println!("{}", os);
    let remote_file = RemoteFile::new(path.to_str().unwrap().to_string());
    //   file_upload(&mut session, path, size, &buffer, times);
    // remote_file.send(&mut session);

    let test = String::from_utf8(buffer).unwrap();
    println!("String: {}\nSize: {}", test, size);

    let _ = check_if_nu_exist(&mut local_session);

    let _ = check_if_nu_exist(&mut session);
}

//fn read_config() {}
