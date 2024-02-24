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
    println!("Request : {} ", String::from_utf8_lossy(&buffer[..]));

    // ------------------------------- base route ---------------------------------------------

    let get_base: &[u8; 16] = b"GET / HTTP/1.1\r\n";
    let get_base_css: &[u8; 25] = b"GET /index.css HTTP/1.1\r\n";

    // html
    let (status_line, filename) = if buffer.starts_with(get_base) {
        ("HTTP/1.1 200 OK\r\n\r\n", "index.html")
    }
    //css
    else if buffer.starts_with(get_base_css) {
        ("", "index.css")
    }
    //error
    else {
        ("HTTP/1.1 404 NOT FOUND\r\n\r\n", "error.html")
    };

    let send_contents = format!("{}{}", status_line, read_file(filename.to_string()));
    write_flush(stream, send_contents);
}

// --------------------------------------------------------------------------------------------

fn write_flush(mut stream: TcpStream, send_contents: String) {
    println!("------------------- attempt to send response ------------------------");

    match stream.write(send_contents.as_bytes()) {
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

//----------------------------------------------------------------------------------------

fn read_file(file_name: String) -> String {
    let contents: Result<String, std::io::Error> = fs::read_to_string(&file_name);
    match contents {
        Ok(contents) => contents,
        Err(err) => {
            println!("[{} file unable to open ]\nerr : {}", file_name, err);
            handle_error()
        }
    }
}

//------------------------------------------------------------------------------------------------

fn handle_error() -> String {
    let error_html = fs::read_to_string("error.html").unwrap();
    error_html
}
