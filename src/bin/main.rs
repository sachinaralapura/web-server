use std::net::TcpListener;
use webserver::{handle_connection, read_config, ThreadPool};
fn main() {
    let (server_address, threads): (String, usize) = read_config();

    let listener: TcpListener = TcpListener::bind(server_address).unwrap();
    let pool: ThreadPool = ThreadPool::new(threads);

    for stream in listener.incoming() {
        let stream = stream.unwrap();
        pool.execute(|| {
            handle_connection(stream);
        })
    }
}
