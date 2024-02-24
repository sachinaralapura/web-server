use std::fs;
use std::io::prelude::*;
use std::net::{TcpListener, TcpStream};
fn main() {
    let listener: TcpListener = TcpListener::bind("127.0.0.1:3001").unwrap();

    for stream in listener.incoming() {
        let stream = stream.unwrap();
        handle_connection(stream);
    }
}

fn handle_connection(mut stream: TcpStream) {
    let mut buffer: [u8; 512] = [0; 512];
    stream.read(&mut buffer).unwrap();
    // println!("Request : {} ", String::from_utf8_lossy(&buffer[..]));
    // ------------------ read index.html file ---------------------

    let html_contents: Result<String, std::io::Error> = fs::read_to_string("index.html");

    let html_contents: String = match html_contents {
        Ok(contents) => contents,
        Err(err) => {
            println!("[index.html file unable to open ]\nerr : {}", err);
            let error_html = fs::read_to_string("error.html").unwrap();
            error_html
        }
    };

    // --------------- writing response ----------------------------
    println!("------------------- attempt to send response ------------------------");
    let response = format!("HTTP/1.1 200 OK\r\n\r\n{}", html_contents);

    match stream.write(response.as_bytes()) {
        Ok(res) => println!(
            "response send successfully , number of bytes send : {} ",
            res
        ),
        Err(err) => println!("{}", err),
    }

    match stream.flush() {
        Ok(_) => println!("flushed"),
        Err(err) => println!("error during flushing : {}", err),
    }
}
