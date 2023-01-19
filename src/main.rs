use std::io::{Read, Write};
use std::net::{TcpListener, TcpStream};

fn main() {
    // You can use print statements as follows for debugging, they'll be visible when running tests.

    // Uncomment this block to pass the first stage

    let listener = TcpListener::bind("127.0.0.1:6379").unwrap();
    println!("Logs from your program will appear here!");
    println!("Listening on port 6379");

    for stream in listener.incoming() {
        match stream {
            Ok(mut _stream) => {
                let mut buffer = [0; 512];
                loop {
                    let response = _stream.read(&mut buffer).unwrap();
                    if response == 0 {
                        println!("client closed the connection");
                        break;
                    }
                    // send pong response
                    send_pong_response(&mut _stream);
                }
            }
            Err(e) => {
                println!("error: {}", e);
                // break;
            }
        }
    }
}

fn send_pong_response(stream: &mut TcpStream) {
    let response = "+PONG\r\n";
    match stream.write(response.as_bytes()) {
        Ok(_) => println!("PONG response sent"),
        Err(e) => println!("Error while sending PONG response: {}", e),
    }
    stream.flush().unwrap();
}

// handle multiple requests
