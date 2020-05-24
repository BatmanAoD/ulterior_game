use std::net::SocketAddr;
use std::thread;

use mles_utils::*;

fn main() {
    // XXX this is just demo code
    let addr = "127.0.0.1:8077".parse::<SocketAddr>().unwrap();
    let uid = "Sender".to_string();
    //set matching channel
    let channel = "Channel".to_string();
    //set message
    let message = "Hello World!".to_string();

    //send hello world to awaiting client
    let mut conn = MsgConn::new(uid, channel);
    conn = conn.connect_with_message(addr, message.into_bytes());
    conn.close();
}
