#![allow(clippy::useless_attribute)]
#![allow(unused_imports)]
#![allow(dead_code)]
#![allow(unused_variables)]

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

#[derive(Debug, Clone)]
pub struct User {
    pub username: String,
    pub password: String,
}

impl User {
    pub fn new(username: String, password: String) -> User {
        User { username, password }
    }
}

#[derive(Debug, Clone)]
pub struct Connection {
    pub host: Option<String>,
    pub port: Option<u32>,
    pub ipv4: Option<Ipv4Addr>,
    pub ipv6: Option<Ipv6Addr>,
}

impl Connection {
    pub fn new(host: String, port: u32) -> Connection {
        Connection {
            host: Some(host),
            port: Some(port),
            ipv4: None,
            ipv6: None,
        }
    }
}
pub struct PingResult(bool, String, String);
impl Connection {
    pub fn connect(&self) -> Session {
        let tcp = TcpStream::connect(format!("{:?}:{:?}", self.host, self.port)).unwrap();
        let mut session = Session::new().unwrap();
        session.set_tcp_stream(tcp);
        session.handshake().unwrap();
        session
    }
    pub fn ping(&self) -> (bool, String, Option<IpAddr>) {
        let mut ip: String = String::new();
        let mut status: bool;
        let r: (bool, String, Option<IpAddr>);
        let ip_addr: IpAddr;

        match &self.host {
            Some(i) => {
                let check = ipaddress::IPAddress::is_valid(i.to_string());
                if !check {
                    let host = self.host.clone().unwrap();
                    let result = dns_lookup::lookup_host(&host);
                    match result {
                        Ok(i) => ip = i[0].to_string(),
                        Err(_) => {
                            panic!("No IP address found")
                        }
                    };
                } else {
                    ip = i.to_string();
                };

                status = true;
            }
            _ => status = false,
        }
        if !status {
            match &self.ipv4 {
                Some(i) => {
                    ip = i.to_string();

                    status = true;
                }
                _ => status = false,
            };
        }
        if !status {
            match &self.ipv6 {
                Some(i) => {
                    ip = i.to_string();
                    status = true;
                }
                _ => status = false,
            };
        }
        if status {
            let options = ping_rs::PingOptions {
                ttl: 128,
                dont_fragment: true,
            };
            // let mut ip_addr: IpAddr = Ipv4Addr::new(127, 0, 0, 1);
            if ip.contains(":") {
                ip_addr = IpAddr::V6(ip.parse().unwrap());
            } else {
                ip_addr = IpAddr::V4(ip.parse().unwrap());
            };
            let timeout = Duration::from_secs(1);
            //let ip_addr = ipaddress::IPAddress::s
            let _ = ping_rs::send_ping(&ip_addr, timeout, &[1, 2, 3, 4], Some(&options));
            r = (true, "success".to_string(), Some(ip_addr));
        } else {
            let s: String = "No IP address found".to_string();
            r = (false, s, None);
        }
        r
    }
}
//    match self {i.ipv4 => {ip = ip + &self.ipv4.unwrap().to_string()}, i.ipv6 => { ip = ip + &self.ipv6.unwrap().to_string()} _ => {return Err(Error::new(ErrorKind::Other, "No IP address found"))}}

pub struct SSHConfig {}
pub struct Config {
    pub ssh: SSHConfig,
}

pub fn load_file(path: &Path) -> String {
    fs::read_to_string(path).unwrap()
}

pub fn load_file_bytes(path: &Path) -> Vec<u8> {
    fs::read(path).unwrap()
}

pub fn ssh_connect(user: User, connection: Connection) -> Session {
    // https://docs.rs/ssh2/latest/ssh2/
    let (test, host, _) = connection.ping();
    if !test {
        panic!("{}", host);
    };

    let port = connection.port.unwrap();
    let host = connection.host.unwrap();

    let address: String = host + ":" + port.to_string().as_str();
    let tcp = TcpStream::connect(address).unwrap();
    // Try to authenticate with the first identity in the agent.
    let mut sess = Session::new().unwrap();
    sess.set_tcp_stream(tcp);
    sess.handshake().unwrap();
    sess.userauth_password(&user.username, &user.password)
        .unwrap();
    assert!(sess.authenticated());
    sess
}

//https://rust-unofficial.github.io/patterns/idioms/default.html
#[derive(Debug, PartialEq, Clone)]
pub struct RemoteFile {
    pub path: String,
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
