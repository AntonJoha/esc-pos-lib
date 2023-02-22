use std::net::TcpStream;
use std::io::prelude::*;

///TODO remove this
///assert_eq!(example(), 0);
fn example() -> i32 {
    0
}

///Establishes a tcpsocket with a printer.
///Sends the message in the Vec<u8>
pub fn send_message(msg: &Vec<u8>, mut address: String, port: u32) -> Result<(), String> {
    
    address.push_str(":");
    address.push_str(&port.to_string());
    let mut stream = match TcpStream::connect(address) {
        Ok(stream) => stream,
        Err(e) => return Err(e.to_string()),
    };
    match stream.write(&msg[0..]) {
        Ok(_) => Ok(()),
        Err(e) => Err(e.to_string()),
    }
}
