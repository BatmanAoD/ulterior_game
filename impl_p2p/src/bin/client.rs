use std::net::SocketAddr;

use mles_utils::*;

fn main() {
    // XXX this is just demo code
    //set server address to connect
    let addr = "127.0.0.1:8077".parse::<SocketAddr>().unwrap();
    //set channel
    let channel = "Channel".to_string();
    //set user
    let uid = "Receiver".to_string();    
    //connect client to server
    let mut conn = MsgConn::new(uid, channel);
    conn = conn.connect(addr);

    //blocking read for hello world
    let (conn, msg) = conn.read_message();
    let msg = String::from_utf8_lossy(msg.as_slice());
    assert_eq!("Hello World!", msg);
    println!("Just received: {}", msg);
    conn.close();
}
