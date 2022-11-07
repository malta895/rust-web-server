use std::{
    fs,
    io::{prelude::*, BufReader},
    net::{TcpListener, TcpStream},
};

const HELLO_HTML_PATH: &str = "./www/hello.html";
fn main() {
    let listener = TcpListener::bind("127.0.0.1:7878").unwrap();

    for stream in listener.incoming() {
        let stream = stream.unwrap();

        handle_connection(stream);
    }
}

fn handle_connection(mut stream: TcpStream) {
    let buf_reader = BufReader::new(&mut stream);

    let http_request: Vec<_> = buf_reader
        .lines()
        .map(|result| result.unwrap())
        .take_while(|line| !line.is_empty())
        .collect();

    println!("Request: {:#?}", http_request);

    let status_line = "HTTP/1.1 200 OK";
    let contents = match fs::read_to_string(HELLO_HTML_PATH) {
        Ok(file_content) => file_content,
        Err(error) => format!("Cannot display file {HELLO_HTML_PATH}: {error}"),
    };
    let length = contents.len();

    let response = format!("{status_line}\r\nContent-Lenght: {length}\r\n\r\n{contents}");

    stream.write_all(response.as_bytes()).unwrap();
}
