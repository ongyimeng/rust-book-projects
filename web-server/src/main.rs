use std::net::{TcpListener, TcpStream};
use std::io::prelude::*;
use std::fs::File;

extern crate web_server;
use web_server::ThreadPool;

fn handle_connection(mut stream: TcpStream) {
   let mut buffer = [0; 512];
   stream.read(&mut buffer).unwrap();
   let request = String::from_utf8_lossy(&buffer[..]);
   // println!("Request: {}", request);

   if request.starts_with("GET") {
      let mut file = File::open("response.html").unwrap();
      let mut contents = String::new();
      file.read_to_string(&mut contents).unwrap();
   
      let response = format!(
          "HTTP/1.1 200 OK\r\nContent-Length: {}\r\nContent-Type: text/html\r\n\r\n{}",
          contents.len(),
          contents
      );
   
      stream.write(response.as_bytes()).unwrap();
      stream.flush().unwrap();
   } else {
      let response = "HTTP/1.1 404 NOT FOUND\r\nContent-Length: 13\r\nContent-Type: text/plain\r\n\r\n404 Not Found";

      stream.write(response.as_bytes()).unwrap();
      stream.flush().unwrap();
   }
}


fn main() {
   let listener = TcpListener::bind("127.0.0.1:7878").unwrap();
   let thread_pool = ThreadPool::new(4);

   for stream in listener.incoming() {
         let stream = stream.unwrap();

         thread_pool.execute(|| {
            handle_connection(stream);
         });
   }
}
