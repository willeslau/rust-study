use std::{io::{Read, Write}, net::TcpListener, process::exit};
use std::net::TcpStream;

fn main() {
    // Create the server
    let listener = TcpListener::bind("0.0.0.0:8080");

    // Check if there are errors bring up the server
    let listener = match listener {
        Ok(f) => f,
        Err(e) => {
            print!("cannot create server due to: {}", e);
            exit(1);
        }
    };

    println!("Server is listening on port 8080 of 0.0.0.0");

    for stream in listener.incoming() {
        // Read the stream
        let stream = stream.unwrap();
        println!("connection established, {:?}\n", stream);
        handle_connection(stream);
    }
}

fn handle_connection(mut stream: TcpStream) {
    let mut buffer = [0; 1024];
    
    // Read value into buffer
    stream.read(&mut buffer).unwrap();

    // Write value back into stream
    stream.write(&mut buffer).unwrap();

    // Flush the value
    stream.flush().unwrap();
}