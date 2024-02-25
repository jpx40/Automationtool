use ssh2::Session;
use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;
use std::net::TcpStream;
use std::net::{Ipv4Addr, Ipv6Addr};
use std::path::Path;
use std::str::Bytes;
use std::string::String;
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

    let mut session = ssh_connect(notbook_user, notbook_connection);
    let path = Path::new("test.txt");
    let size = path.metadata().unwrap().len();
    if size == 0 {
        panic!("File size is zero");
    }
    let mut buffer: Vec<u8> = Vec::new();

    let _ = File::open(path).unwrap().read_to_end(&mut buffer);
    //   let mut reader = BufReader::new(file);

    //let buf = reader.fill_buf().unwrap();
    file_upload(&mut session, path, size, &buffer);

    let test = String::from_utf8(buffer).unwrap();
    println!("String: {}Size: {}", test, size);
}

fn read_config() {}

fn ssh_connect(user: User, connection: Connection) -> Session {
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
    sess
}

fn file_upload(session: &mut Session, file: &Path, size: u64, buf: &[u8]) {
    let mut remote_file = session.scp_send(file, 0o644, size, None).unwrap();

    remote_file.write_all(buf).unwrap();
    // Close the channel and wait for the whole content to be transferred
    remote_file.send_eof().unwrap();
    remote_file.wait_eof().unwrap();
    remote_file.close().unwrap();
    remote_file.wait_close().unwrap();
}
