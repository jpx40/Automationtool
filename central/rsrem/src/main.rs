use ssh2::Session;
use std::fs::File;
use std::io::prelude::*;
use std::net::TcpStream;
use std::path::Path;
use toml;

fn main() {}

fn read_config() {}

fn ssh_connect() {
    // https://docs.rs/ssh2/latest/ssh2/
    let tcp = TcpStream::connect("127.0.0.1:22").unwrap();
    // Try to authenticate with the first identity in the agent.
    let mut sess = Session::new().unwrap();
    sess.set_tcp_stream(tcp);
    sess.handshake().unwrap();
    sess.userauth_password("username", "password").unwrap();
    assert!(sess.authenticated());
}
