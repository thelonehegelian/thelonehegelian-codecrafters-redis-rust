use std::{
    io::Read,
    net::{TcpListener, TcpStream},
};

use std::io::Write;

fn main() {
    // You can use print statements as follows for debugging, they'll be visible when running tests.
    println!("Logs from your program will appear here!");

    // Uncomment this block to pass the first stage

    let listener = TcpListener::bind("127.0.0.1:6379").unwrap();

    for stream in listener.incoming() {
        match stream {
            Ok(mut _stream) => {
                // define a buffer 0-5
                let mut buffer = [10; 12];
                _stream.read_exact(&mut buffer).unwrap();
                let command = String::from_utf8_lossy(&buffer);
                // split command using the split method
                let command = command.split("\r\n").collect::<Vec<&str>>();
                let ping_command = command.last().unwrap().to_ascii_lowercase();
                // check if the command is ping
                if ping_command == &*"ping" {
                    // send pong response
                    send_pong_response(&mut _stream);
                }
            }
            Err(e) => {
                println!("error: {}", e);
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
