#![allow(clippy::useless_attribute)]
#![allow(unused_imports)]
#![allow(dead_code)]
#![allow(unused_variables)]

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

pub struct PingResult(bool, String, String);

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
