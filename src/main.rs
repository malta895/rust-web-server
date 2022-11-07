use std::{
    fs,
    io::{prelude::*, BufReader},
    net::{TcpListener, TcpStream},
};

const WWWW_PATH: &str = "./www";
const HTTP_STATUS_200_OK: &str = "HTTP/1.1 200 OK";
const HTTP_STATUS_404_NOT_FOUND: &str = "HTTP/1.1 404 NOT FOUND";

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

    let file_name = "hello.html";

    let (status_line, contents) = match fs::read_to_string(format!("{WWWW_PATH}/{file_name}")) {
        Ok(file_content) => (HTTP_STATUS_200_OK, file_content),
        Err(error) => (
            HTTP_STATUS_404_NOT_FOUND,
            format!("Cannot display file {file_name}: {error}"),
        ),
    };
    let length = contents.len();

    let response = format!("{status_line}\r\nContent-Lenght: {length}\r\n\r\n{contents}");

    stream.write_all(response.as_bytes()).unwrap();
}
