use std::{net::TcpListener, thread};
use webserver::{handle_connection, read_config};

fn main() {
    let (server_address, threads): (String, String) = read_config();

    let listener: TcpListener = TcpListener::bind(server_address).unwrap();
    // let pool = ThreadPool::new(threads);
    for stream in listener.incoming() {
        let stream = stream.unwrap();
        handle_connection(stream);
        // pool.execute(|| {
        // });
    }
}
