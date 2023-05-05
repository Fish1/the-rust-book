use std::{
    net::{TcpListener, TcpStream},
    io::{prelude::*, BufReader, BufRead}, fs, thread, time::Duration
};

use webserver::ThreadPool;

fn main() {
    let listener = TcpListener::bind("127.0.0.1:7878").unwrap();
    let thread_pool = ThreadPool::new();

    for stream in listener.incoming() {
        let stream = stream.unwrap();
        println!("Connection established!");
        thread::spawn(|| {
            handle_connection(stream);
        });
    }

    fn handle_connection(mut stream: TcpStream) {
        let buf_reader = BufReader::new(&mut stream);
        let http_request: Vec<_> = buf_reader
            .lines()
            .map(|result| result.unwrap())
            .take_while(|line| !line.is_empty())
            .collect(); 
        println!("Request: {:#?}", http_request);

        let requested_file: Vec<&str> = http_request[0].split(" ").collect();
        let requested_file = match requested_file.get(1) {
            Some(x) => x,
            _ => "index.html"
        };

        let file = ["html/", requested_file].join("");
        println!("req: {:?}", file);

        if let Ok(contents) = fs::read_to_string(file) {
            let status = String::from("HTTP/1.1 200 OK");
            respond(stream, &status, &contents);
        } else {
            let status = String::from("HTTP/1.1 404 NOT FOUND");
            let contents = fs::read_to_string("html/errors/404.html").unwrap();
            respond(stream, &status, &contents);
        }
    }

    fn respond(mut stream: TcpStream, status: &str, contents: &str) {
        thread::sleep(Duration::from_secs(5));
        let length = contents.len();
        let response = format!(
            "{status}\r\n\
            Content-Length: {length}\r\n\r\n\
            {contents}"
        );
        stream.write_all(response.as_bytes()).unwrap();
    }
}
