use regex::Regex;
use std::{
    fs,
    io::{prelude::*, BufReader, Error},
    net::{TcpListener, TcpStream},
    thread,
    time::Duration,
};

use rust_web_server::ThreadPool;
const WWWW_PATH: &str = "./www";
const HTTP_STATUS_200_OK: (u32, &str) = (200, "HTTP/1.1 200 OK");
const HTTP_STATUS_404_NOT_FOUND: (u32, &str) = (404, "HTTP/1.1 404 NOT FOUND");
const HTTP_STATUS_405_METHOD_NOT_ALLOWED: (u32, &str) = (405, "HTTP/1.1 405 METHOD NOT ALLOWED");

//const HTTP_STATUS_403_FORBIDDEN: (u32, &str) = (403, "HTTP/1.1 403 FORBIDDEN");

const INDEX_FILENAME: &str = "index.html";

fn main() {
    const HTTP_PORT: u32 = 7878;

    let listener = TcpListener::bind(format!("127.0.0.1:{HTTP_PORT}")).unwrap();

    println!(
        r#"
		-------------------------------------------------
		|	Server listening at localhost:{HTTP_PORT}"	|
		-------------------------------------------------
"#
    );

    let pool = ThreadPool::new(100);

    for stream in listener.incoming() {
        let stream = stream.unwrap();

        pool.execute(|| {
            handle_connection(stream);
        });
    }
}

fn parse_request_to_method_and_path(request_line: String) -> (String, String) {
    let request_line_regexp = Regex::new(
        r#"(?P<method>GET|POST|PUT|PATCH|OPTIONS) /(?P<filename>.*) HTTP/(\d(?:\.\d)?)"#,
    )
    .unwrap();

    match request_line_regexp.captures(&request_line) {
        Some(x) => (
            String::from(x.name("method").unwrap().as_str()),
            String::from(x.name("filename").unwrap().as_str()),
        ),
        None => unreachable!(),
    }
}

fn get_error_status_line_and_code(http_status: (u32, &str), error: Error) -> (&str, String) {
    let file_path_and_name = format!("{WWWW_PATH}/{}.html", http_status.0);
    match fs::read_to_string(&file_path_and_name) {
        Ok(file_contents) => {
            println!("Sending {} page due to error: {error}", http_status.0);
            (http_status.1, file_contents)
        }
        Err(inner_error) =>
            (http_status.1,
             format!(
                 "Cannot display file {file_path_and_name}: {error}; Another error occurred while trying to retrieve {} page: {inner_error}",
                 http_status.0
             )
            )
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

    let (http_method, url_path) = parse_request_to_method_and_path(request_line);

    let filename = if url_path == "" {
        INDEX_FILENAME
    } else if url_path == "sleep" {
        thread::sleep(Duration::from_secs(5));
        INDEX_FILENAME
    } else {
        url_path.as_str()
    };

    let (status_line, contents) = if http_method == "GET" {
        match fs::read_to_string(format!("{WWWW_PATH}/{filename}")) {
            Ok(file_contents) => (HTTP_STATUS_200_OK.1, file_contents),
            Err(error) => get_error_status_line_and_code(HTTP_STATUS_404_NOT_FOUND, error),
        }
    } else {
        (
            HTTP_STATUS_405_METHOD_NOT_ALLOWED.1,
            format!("Method {http_method} not allowed!"),
        )
    };

    let length = contents.len();

    let response = format!("{status_line}\r\nContent-Lenght: {length}\r\n\r\n{contents}");

    stream.write_all(response.as_bytes()).unwrap();
}
