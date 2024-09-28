use std::fs;
use std::io::prelude::*;
use std::net::{TcpListener, TcpStream};
use web_server::ThreadPool;

fn main() {
  let listener: TcpListener = TcpListener::bind("127.0.0.1:8080").unwrap();

  let pool = ThreadPool::new(4);

  for stream in listener.incoming() {
    let stream: TcpStream = stream.unwrap();

    pool.execute(|| {
      handle_connection(stream);
    });
  }
}

fn handle_connection(mut stream: TcpStream) {
  let mut buffer: [u8; 1024] = [0; 1024];

  stream.read(&mut buffer).unwrap();

  let get: &[u8; 16] = b"GET / HTTP/1.1\r\n";
  let sleep = b"GET /sleep HTTP/1.1\r\n";
  let (statusline, filename) = if buffer.starts_with(get) {
    ("HTTP/1.1 200 OK", "index.html")
  } else if buffer.starts_with(sleep) {
    std::thread::sleep(std::time::Duration::from_secs(5));
    ("HTTP/1.1 200 OK", "index.html")
  } else {
    ("HTTP/1.1 404 NOT FOUND", "404.html")
  };

  let contents: String = fs::read_to_string(filename).unwrap();
  let response: String = format!(
    "{}\r\nContent-Length: {}\r\nServer: Rust bhai ka server hai\r\n\r\n{}",
    statusline,
    contents.len(),
    contents
  );

  stream.write(response.as_bytes()).unwrap();
}
