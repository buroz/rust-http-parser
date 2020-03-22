use std::fs::File;
use std::io::{Read, Write};
use std::net::TcpStream;

pub fn respond_error(mut stream: TcpStream) {
  let response = b"HTTP/1.1 500 Internal Server Error\r\nContent-Type: text/html; charset=UTF-8\r\n\r\n<html><body>500 - Server Error</body></html>\r\n";
  stream.write(response).expect("Write failed");
}

pub fn respond_file_not_found(mut stream: TcpStream) {
  let response = b"HTTP/1.1 404 File Not Found\r\nContent-Type: text/html; charset=UTF-8\r\n\r\n<html><body>404 - File Not Found</body></html>\r\n";
  stream.write(response).expect("Write failed");
}

pub fn respond_hello_world(mut stream: TcpStream) {
  let response = b"HTTP/1.1 200 OK\r\nContent-Type: text/html; charset=UTF-8\r\n\r\n<html><body>Hello world</body></html>\r\n";
  stream.write(response).expect("Write failed");
}

pub fn serve_static_file(mut stream: TcpStream, path: &str) {
  let mut file = match File::open(format!("www/{}", path)) {
    Ok(file) => file,
    Err(_) => File::open("www/404.html").expect("404.html file missing!"),
  };
  let mut contents = String::new();
  file.read_to_string(&mut contents).unwrap();
  let response = format!("HTTP/1.1 200 OK\r\n\r\n{}", contents);
  stream
    .write(response.as_bytes())
    .expect("Failed to write to stream!");
}
