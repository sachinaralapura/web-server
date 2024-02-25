use std::io::prelude::*;
use std::net::TcpStream;
use std::sync::{mpsc, Arc, Mutex};
use std::{fs, thread};

use toml::Value;

trait FnBox {
    fn call_box(self: Box<Self>);
}

impl<F: FnOnce()> FnBox for F {
    fn call_box(self: Box<Self>) {
        (*self)()
    }
}

type Job = Box<dyn FnBox + Send + 'static>;
/// A data structure between ThreadPool and the threads
struct Worker {
    id: usize,
    thread: thread::JoinHandle<()>,
}

impl Worker {
    fn new(id: usize, receiver: Arc<Mutex<mpsc::Receiver<Job>>>) -> Worker {
        let thread: thread::JoinHandle<()> = thread::spawn(move || loop {
            let job = receiver.lock().unwrap().recv().unwrap();
            println!("Worker {} got a job; executing.", id);
            job.call_box();
        });

        Worker { id, thread }
    }
}

pub struct ThreadPool {
    workers: Vec<Worker>,
    sender: mpsc::Sender<Job>,
}

impl ThreadPool {
    /// Create a new ThreadPool.
    ///
    /// The size is the number of threads in the pool.
    ///
    /// # Panics
    ///
    /// The `new` function will panic if the size is zero or < zero.
    pub fn new(size: usize) -> ThreadPool {
        assert!(size > 0);

        let mut workers: Vec<Worker> = Vec::with_capacity(size);
        let (sender, receiver) = mpsc::channel();

        let receiver: Arc<Mutex<mpsc::Receiver<Job>>> = Arc::new(Mutex::new(receiver));

        for id in 0..size {
            workers.push(Worker::new(id, Arc::clone(&receiver)));
        }

        ThreadPool { workers, sender }
    }

    pub fn execute<F>(&self, f: F)
    where
        F: FnOnce() + Send + 'static,
    {
        let job: Box<F> = Box::new(f);
        self.sender.send(job).unwrap();
    }
}

pub fn handle_connection(mut stream: TcpStream) {
    let mut buffer: [u8; 512] = [0; 512];
    stream.read(&mut buffer).unwrap();
    println!("Request : {} ", String::from_utf8_lossy(&buffer[..]));

    // ------------------------------- base route -----------------------------------------

    let get_base: &[u8; 16] = b"GET / HTTP/1.1\r\n";
    let get_base_css: &[u8; 25] = b"GET /index.css HTTP/1.1\r\n";
    // let sleep = b"GET /sleep HTTP/1.1\r\n";

    // html
    let (status_line, filename) = if buffer.starts_with(get_base) {
        ("HTTP/1.1 200 OK\r\n\r\n", "static/index.html")
    }
    //css
    else if buffer.starts_with(get_base_css) {
        ("", "static/index.css")
    }
    // else if buffer.starts_with(sleep) {
    //     thread::sleep(Duration::from_secs(3));
    //     ("HTTP/1.1 200 OK\r\n\r\n", "static/index.html")
    // }
    //error
    else {
        ("HTTP/1.1 404 NOT FOUND\r\n\r\n", "static/error.html")
    };

    let send_contents = format!("{}{}", status_line, read_file(filename.to_string()));
    write_flush(stream, send_contents);
}

// -----------------------------------------------------------------------------------

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

//----------------------------------------------------------------------------------------

fn handle_error() -> String {
    let error_html = fs::read_to_string("error.html").unwrap();
    error_html
}

//---------------------------------------------------------------------------------------

pub fn read_config() -> (String, usize) {
    // Read the contents of the config.toml file
    let config_content: String =
        fs::read_to_string("config.toml").expect("Failed to read config.toml");

    // Parse the TOML content into a TOML value
    let config: Value = config_content.parse().expect("Failed to parse config.toml");
    // Extract the server address from the config
    let server_address: &str = config["server"]["address"]
        .as_str()
        .expect("Server address not found in config.toml");
    let threads = config["server"]["threads"]
        .as_integer()
        .expect("threads not found");
    (server_address.to_string(), threads as usize)
}
