use regex::Regex;
use std::{
    fs,
    io::{prelude::*, BufReader},
    net::{TcpListener, TcpStream},
};

const WWWW_PATH: &str = "./www";
const HTTP_STATUS_200_OK: &str = "HTTP/1.1 200 OK";
const HTTP_STATUS_404_NOT_FOUND: &str = "HTTP/1.1 404 NOT FOUND";
const HTTP_STATUS_405_METHOD_NOT_ALLOWED: &str = "HTTP/1.1 405 METHOD NOT ALLOWED";

fn main() {
    const HTTP_PORT: u32 = 7878;

    let listener = TcpListener::bind(format!("127.0.0.1:{HTTP_PORT}")).unwrap();

    println!("Server listening at localhost:{HTTP_PORT}");

    for stream in listener.incoming() {
        let stream = stream.unwrap();

        handle_connection(stream);
    }
}

fn handle_connection(mut stream: TcpStream) {
    let buf_reader = BufReader::new(&mut stream);

    // let http_request: Vec<_> = buf_reader
    //     .lines()
    //     .map(|result| result.unwrap())
    //     .take_while(|line| !line.is_empty())
    //     .collect();

    let request_line = buf_reader.lines().next().unwrap().unwrap();

    println!("Request: {:#?}", request_line);

    let request_line_regexp = Regex::new(
        r#"(?P<method>GET|POST|PUT|PATCH|OPTIONS) /(?P<filename>.*) HTTP/(\d(?:\.\d)?)"#,
    )
    .unwrap();

    let (http_method, url_path) = match request_line_regexp.captures(&request_line) {
        Some(x) => (
            x.name("method").unwrap().as_str(),
            x.name("filename").unwrap().as_str(),
        ),
        None => unreachable!(),
    };

    let filename = if url_path == "" {
        "index.html"
    } else {
        url_path
    };

    let (status_line, contents) = if http_method == "GET" {
        match fs::read_to_string(format!("{WWWW_PATH}/{filename}")) {
            Ok(file_content) => (HTTP_STATUS_200_OK, file_content),
            Err(error) => (
                HTTP_STATUS_404_NOT_FOUND,
                format!("Cannot display file {filename}: {error}"),
            ),
        }
    } else {
        (
            HTTP_STATUS_405_METHOD_NOT_ALLOWED,
            format!("Method {http_method} not allowed!"),
        )
    };

    let length = contents.len();

    let response = format!("{status_line}\r\nContent-Lenght: {length}\r\n\r\n{contents}");

    stream.write_all(response.as_bytes()).unwrap();
}
