use std::{io::Read, net::{TcpListener, TcpStream}, thread};

const LOCAL_ADDR: &str = "127.0.0.1:8080";

fn create_srv() -> TcpListener {
    let srv = TcpListener::bind(LOCAL_ADDR).expect("asd");
    srv
}

fn handle_client(socket: &mut TcpStream) {
    let mut buf = [0;32];
    loop {
        let res = socket.read(&mut buf).expect("unable to read");
        if res == 0 {
            return;
        }
        println!("recv: {}", std::str::from_utf8(&buf).unwrap());
    }
}

fn main() {
    println!("[chat-app]: Hello!");
    let srv = TcpListener::bind(LOCAL_ADDR).expect("[chat-app]: Invalid address");

    loop {
        if let Ok((mut socket, addr)) = srv.accept() {
            println!("[chat-app]: user {} connected", addr);
            thread::spawn( move || { 
                handle_client(&mut socket);
            });
        }
    }


        // if let Ok((mut socket,addr)) = srv.accept() {
        //     let mut buf = vec![0; 32];
            
        //     match socket.read_exact(&mut buf) {
        //         Ok(_) => {
        //             println!("{:?}", buf);//.into_iter().collect::<Vec<_>>());
        //         }
        //         Err(_) => {

        //         }
        //     }
        // }
}
