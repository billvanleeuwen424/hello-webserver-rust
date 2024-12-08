use std::{
    fs,
    io::{prelude::*, BufReader},
    net::{TcpListener, TcpStream},
};

use hello_webserver::ThreadPool;

fn main() {
    // create socket, bind to it
    let listener = TcpListener::bind("127.0.0.1:7878").unwrap();

    // handle up to 4 requests at once
    let pool = ThreadPool::new(4);

    // listen on the socket
    for stream in listener.incoming() {
        let stream = stream.unwrap();
        // handle stream coming from the connection
        pool.execute(|| handle_connection(stream));
    }
}

fn handle_connection(mut stream: TcpStream) {
    // turn TCP Stream into a buffered reader
    let buf_reader = BufReader::new(&stream);

    // Get the first line of the bufreader
    let request_line = buf_reader.lines().next().unwrap().unwrap();
    let (status_line, filename) = if request_line == "GET / HTTP/1.1" {
        ("HTTP/1.1 200 OK", "hello.html")
    } else {
        ("HTTP/1.1 404 NOT FOUND", "404.html")
    };

    // read in the file, format response and send it up socket
    let contents = fs::read_to_string(filename).unwrap();
    let length = contents.len();
    let response = format!("{status_line}\r\nContent-Length: {length}\r\n\r\n{contents}");
    stream.write_all(response.as_bytes()).unwrap();
}
