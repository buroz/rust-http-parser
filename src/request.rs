use crate::response;
use std::io::{BufRead, BufReader};
use std::net::{SocketAddr, TcpStream};
use std::str;

fn request_url(buffer: &[u8]) -> Option<&str> {
  let mut headers = [httparse::EMPTY_HEADER; 16];
  let mut req = httparse::Request::new(&mut headers);
  match req.parse(&buffer) {
    Ok(_) => match req.path {
      Some(ref path) => {
        return Some(path);
      }
      None => {
        return None;
      }
    },
    Err(_) => {
      return None;
    }
  }
}

fn read_request_head(stream: &TcpStream) -> Vec<u8> {
  let mut reader = BufReader::new(stream);
  let mut buff = Vec::new();
  let mut read_bytes = reader.read_until(b'\n', &mut buff).unwrap();
  while read_bytes > 0 {
    read_bytes = reader.read_until(b'\n', &mut buff).unwrap();
    if read_bytes == 2 && &buff[(buff.len() - 2)..] == b"\r\n" {
      break;
    }
  }
  return buff;
}

pub fn handle_request(stream: TcpStream, _client_addr: SocketAddr) {
  let request_bytes = read_request_head(&stream);
  let mut headers = [httparse::EMPTY_HEADER; 16];
  let mut req = httparse::Request::new(&mut headers);

  let path = request_url(&request_bytes);
  println!("{:?}", path); // bi çare bul

  req.parse(&request_bytes).expect("cannot read the req"); // DON'T DELETE!

  match req.path {
    Some(path) => {
      if path.starts_with("/files") {
        response::serve_static_file(stream, &path[7..]);
      } else if path == "/hello" {
        response::respond_hello_world(stream);
      } else {
        response::respond_file_not_found(stream);
      }
    }
    None => {
      response::respond_error(stream);
    }
  };
}
