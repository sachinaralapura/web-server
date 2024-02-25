use std::net::TcpListener;

use webserver::handle_connection;

fn main() {
    let listener: TcpListener = TcpListener::bind("127.0.0.1:3001").unwrap();

    for stream in listener.incoming() {
        let stream = stream.unwrap();
        handle_connection(stream);
    }
}