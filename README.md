# Multi-threaded TCP Web Server in Rust

This project is a basic **multi-threaded raw TCP web server** written in Rust. It handles multiple client connections concurrently using a custom thread pool.

## Features

- **Multi-threading**: Handles multiple requests simultaneously with a thread pool.
- **Raw TCP Handling**: Uses `TcpListener` and `TcpStream` for direct TCP connection management.
- **Basic Routing**: Supports simple routing, including a `/sleep` endpoint that delays the response.
- **Minimal and Fast**: No external web frameworks – pure Rust for performance.


### Endpoints

- `GET /` – Returns the contents of `index.html`.
- `GET /sleep` – Responds after a 5-second delay.
- Invalid routes return a `404` page.

 
