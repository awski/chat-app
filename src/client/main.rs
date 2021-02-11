use std::net::TcpStream;
use std::io::{Read, Write};

const LOCAL_ADDR: &str = "127.0.0.1:8080";

fn main() {
    let mut conn = TcpStream::connect(LOCAL_ADDR).expect("[chat-app]: unable to connect to the host");
    
    loop {
        println!("write msg:");
        let mut buf = String::new();
        std::io::stdin().read_line(&mut buf).expect("[chat-app]: failed to read input");
        conn.write_all(buf.as_bytes()).expect("[chat-app]: failed to send msg");
    }
}
