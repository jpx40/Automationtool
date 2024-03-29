#![allow(clippy::useless_attribute)]
#![allow(unused_imports)]
#![allow(dead_code)]
#![allow(unused_variables)]

use crate::builtin;
use crate::user::User;
use dns_lookup::{getaddrinfo, AddrInfoHints, SockType};
use ssh2::{Channel, Session, Sftp, Stream};
use std::borrow;
use std::clone;
use std::fs;
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
use std::sync::Arc;
use std::thread::Result;
use std::{path::PathBuf, time::Duration};
pub mod connection;
use connection::Connection;

pub fn exec_script() {}

pub fn load_file(path: &Path) -> String {
    fs::read_to_string(path).unwrap()
}

pub fn load_file_bytes(path: &Path) -> Vec<u8> {
    fs::read(path).unwrap()
}
//https://rust-unofficial.github.io/patterns/idioms/default.html
#[derive(Debug, PartialEq, Clone)]
pub struct RemoteFile {
    pub path: String,
    pub filename: Option<String>,
    pub mode: i32,
    pub size: u64,
    pub times: Option<(u64, u64)>,
    pub buf: Option<Vec<u8>>,
}

impl RemoteFile {
    pub fn new(path: String) -> Self {
        Self {
            path: path.clone(),
            mode: 0o644,
            filename: None,
            size: File::open(Path::new(&path))
                .unwrap()
                .metadata()
                .unwrap()
                .len(),
            times: None,
            buf: None,
        }
    }

    pub fn set_mode(&mut self, mode: i32) {
        self.mode = mode;
    }

    pub fn add_buf(&mut self, buf: Vec<u8>) {
        self.buf = Some(buf);
    }
    pub fn set_times(&mut self, times: Option<(u64, u64)>) {
        self.times = times;
    }
    pub fn set_size(&mut self, size: u64) {
        self.size = size;
    }
    pub fn set_path(&mut self, path: String) {
        self.path = path;
    }

    pub fn send(self, session: &mut Session) {
        let mut buffer: Vec<u8>;
        match self.buf {
            Some(b) => {
                buffer = b;
            }
            None => {
                buffer = Vec::new();

                let _ = File::open(&self.path).unwrap().read_to_end(&mut buffer);
            }
        };
        let mut remote_file = session
            .scp_send(Path::new(&self.path), self.mode, self.size, self.times)
            .unwrap();

        remote_file.write_all(&buffer).unwrap();
        // Close the channel and wait for the whole content to be transferred
        remote_file.send_eof().unwrap();
        remote_file.wait_eof().unwrap();
        remote_file.close().unwrap();
        remote_file.wait_close().unwrap();
    }
    pub fn send_with_buf(&mut self, session: &mut Session, buf: &[u8]) {
        let mut remote_file = session
            .scp_send(Path::new(&self.path), self.mode, self.size, self.times)
            .unwrap();

        remote_file.write_all(buf).unwrap();
        // Close the channel and wait for the whole content to be transferred
        remote_file.send_eof().unwrap();
        remote_file.wait_eof().unwrap();
        remote_file.close().unwrap();
        remote_file.wait_close().unwrap();
    }
}

pub fn copy_file(session: &mut Session, param: builtin::CopyParam) {
    let mut s = String::new();
    let mut remote_file = RemoteFile::new(param.location);
    let mut rem = remote_file.send(session);
}
pub fn execute(session: &mut Session, script: String) -> Result<String> {
    let mut s = String::new();
    let mut channel = session.channel_session().unwrap();
    channel.exec(&script).unwrap();
    channel.read_to_string(&mut s).unwrap();
    channel.wait_close().unwrap();

    Ok(s)
}
pub fn execute_task_with_key(session: &mut Session, script: &str) -> Result<String> {
    let mut s = String::new();
    let mut channel = session.channel_session().unwrap();
    channel.exec(script).unwrap();
    channel.read_to_string(&mut s).unwrap();
    channel.wait_close().unwrap();

    Ok(s)
}
pub fn check_if_nu_exist(session: &mut Session) -> bool {
    let mut nu = String::new();
    let mut nu_exist: bool = false;

    let script = load_file(Path::new("check_nu_exist.sh"));
    let mut channel = session.channel_session().unwrap();
    channel.exec(&script).unwrap();
    channel.read_to_string(&mut nu).unwrap();
    println!("{}", nu);

    if nu.contains("false") {
        println!("nu not exist");
    } else if nu.contains("true") {
        nu_exist = true;
        println!("nu exist");
    } else {
        println!("ERROR: Unknown response from check_nu_exist.sh");
    }
    nu_exist
}
pub fn file_upload(
    session: &mut Session,
    file: &Path,
    size: u64,
    buf: &[u8],
    times: Option<(u64, u64)>,
) {
    let mut remote_file = session.scp_send(file, 0o644, size, times).unwrap();

    remote_file.write_all(buf).unwrap();
    // Close the channel and wait for the whole content to be transferred
    remote_file.send_eof().unwrap();
    remote_file.wait_eof().unwrap();
    remote_file.close().unwrap();
    remote_file.wait_close().unwrap();
}

pub fn check_os_type(session: &mut Session) -> String {
    let mut os = String::new();
    //https://megamorf.gitlab.io/2021/05/08/detect-operating-system-in-shell-script/
    let script = r#"case "$OSTYPE" in
  solaris*) echo "SOLARIS" ;;
  darwin*)  echo "OSX" ;; 
  linux*)   echo "LINUX" ;;
  bsd*)     echo "BSD" ;;
  msys*)    echo "WINDOWS" ;;
  *)        echo "unknown: $OSTYPE" ;;
esac"#;

    let mut channel = session.channel_session().unwrap();
    channel.exec(script).unwrap();
    channel.read_to_string(&mut os).unwrap();
    os.to_string()
}
