use std::{
    fs,
    io::{prelude::*, BufReader},
    net::{TcpListener, TcpStream},
};

fn main() {
    // create socket, bind to in
    let listener = TcpListener::bind("127.0.0.1:7878").unwrap();

    // listen on the socket
    for stream in listener.incoming() {
        let stream = stream.unwrap();
        // handle stream coming from the connection
        handle_connection(stream);
    }
}

fn handle_connection(mut stream: TcpStream) {
    // turn TCP Stream into a buffered reader
    let buf_reader = BufReader::new(&stream);

    // do some unwrap magic to get the first line of the request
    let request_line = buf_reader.lines().next().unwrap().unwrap();

    // only want the '/' path nothing else
    if request_line == "GET / HTTP/1.1" {
        let status_line = "HTTP/1.1 200 OK";
        let contents = fs::read_to_string("hello.html").unwrap();
        let length = contents.len();

        let response = format!("{status_line}\r\nContent-Length: {length}\r\n\r\n{contents}");

        stream.write_all(response.as_bytes()).unwrap();
    } else {
        let status_line = "HTTP/1.1 404 NOT FOUND";
        let contents = fs::read_to_string("404.html").unwrap();
        let length = contents.len();

        let response = format!("{status_line}\r\nContent-Length: {length}\r\n\r\n{contents}");

        stream.write_all(response.as_bytes()).unwrap();
    }
}
