#[allow(unused_imports)]
use std::env;
#[allow(unused_imports)]
use std::fs;
use std::io::{Read, Write, Error, ErrorKind};
use std::net::TcpListener;

const PING: &str = "PING";


fn main() -> std::io::Result<()> {

    let listener = TcpListener::bind("127.0.0.1:6379").unwrap();

    
    match listener.accept() {
    
        Ok((mut socket, _addr)) => {

            println!("{}", _addr);
    
            let mut buff           = String::new();
            let _bytes_read        = socket.read_to_string(&mut buff).unwrap();

            match unserialize(&buff) {

                Ok(o) => {

                    let response_bytes =  o.as_bytes();
                    let bytes_written  = socket.write(response_bytes).unwrap();
                    
                    if bytes_written < response_bytes.len() {

                        panic!("{}", Error::new(ErrorKind::Interrupted, 
                            format!("Sent {}/{} bytes", bytes_written, response_bytes.len())))

                    }

                    socket.flush().unwrap()

                }

                _ => panic!("{}", Error::new(
                        ErrorKind::InvalidInput, 
                        format!("Unkown command {:#?}", &buff.as_str()))),
            }
        }

        Err(e) => println!("couldn't accept client: {:?}", e),
    }

    Ok(())
}

// According to Redis Official Documentation
// In RESP, the first byte determines the data type:
// For Arrays:         '*'
// For Errors:         '-'
// For Integers:       ':'
// For Bulk Strings:   '$'
// For Simple Strings: '+'

fn unserialize(command: &str) -> std::io::Result<&str> {

    match command {

         x if x.starts_with('+') => {     
             let unprefixed_str = x.trim_start_matches(|prefix| prefix == '+').trim();
             let unsuffixed_str = unprefixed_str.trim_end_matches(r"\r\n");
             if  unsuffixed_str == PING { return Ok(r"+PONG\r\n") }
            
            Ok(r"+PONG\r\n")

        }
        _ => Ok(r"+PONG\r\n"),
    }

}

// RESP uses prefixed lengths to transfer bulk data,
// so there is never a need to scan the payload for special characters,
// like with JSON, nor to quote the payload that needs to be sent to the server.

