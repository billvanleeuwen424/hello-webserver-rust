use std::{
    io::{BufRead, BufReader, Write},
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

    // split each line of the request into a vectory until line is empty aka \n\n
    let http_request: Vec<_> = buf_reader
        .lines()
        .map(|result| result.unwrap())
        .take_while(|line| !line.is_empty())
        .collect();

    let response = "HTTP/1.1 200 OK/r/n/r/n";

    stream.write_all(response.as_bytes()).unwrap();
}
