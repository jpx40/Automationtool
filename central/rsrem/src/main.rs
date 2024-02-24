use ssh2::Session;
use std::fs::File;
use std::io::prelude::*;
use std::net::TcpStream;
use std::net::{Ipv4Addr, Ipv6Addr};
use std::path::Path;
use toml;
#[derive(Debug, Clone)]
struct User {
    username: String,
    password: String,
}

impl User {
    fn new(username: String, password: String) -> User {
        User { username, password }
    }
}

#[derive(Debug, Clone)]
struct Connection {
    host: String,
    port: i32,
    ipv4: Option<Ipv4Addr>,
    ipv6: Option<Ipv6Addr>,
}

impl Connection {
    fn new(host: String, port: i32) -> Connection {
        Connection {
            host,
            port,
            ipv4: None,
            ipv6: None,
        }
    }
}

struct SSHConfig {}
struct Config {
    ssh: SSHConfig,
}

fn main() {
    let host = "127.0.0.1";
    let port = 22;

    let connection = Connection::new(host.to_string(), port);
    let user = User::new("jonas".to_string(), "Artemis34!!".to_string());
    let notbook_user = User::new("jonas".to_string(), "artemisJP!!".to_string());
    let notbook_connection = Connection::new("192.168.178.46".to_string(), port);
    ssh_connect(user, connection);

    ssh_connect(notbook_user, notbook_connection);
}

fn read_config() {}

fn ssh_connect(user: User, connection: Connection) {
    // https://docs.rs/ssh2/latest/ssh2/

    let address: String = connection.host + ":" + &connection.port.to_string();
    let tcp = TcpStream::connect(address).unwrap();
    // Try to authenticate with the first identity in the agent.
    let mut sess = Session::new().unwrap();
    sess.set_tcp_stream(tcp);
    sess.handshake().unwrap();
    sess.userauth_password(&user.username, &user.password)
        .unwrap();
    assert!(sess.authenticated());
}
