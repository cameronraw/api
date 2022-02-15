use std::{net::{TcpListener, TcpStream}, panic, io::{Read, Write}};
use library::logger as logger;
fn main() {

    const LOCALHOST: &str = "127.0.0.1";
    const PORT: &str = "7878";

    logger::write("Testing the logger");

    let listener = TcpListener::bind(format!("{}:{}", LOCALHOST, PORT));

    let listener = match listener {
        Ok(listener) => listener,
        Err(error) => panic!("Problem connecting to {}:{} : {:?}", LOCALHOST, PORT, error)
    };

    for stream in listener.incoming() {
        let stream = stream;

        let stream = match stream {
            Ok(stream) => stream,
            Err(error) => panic!("Error reading from incoming stream {:?}", error)
        };

        handle_connection(stream);
    }
}

fn handle_connection(mut stream: TcpStream){

    let mut buffer = [0; 1024];

    let stream_read_result = stream.read(&mut buffer);

    match stream_read_result {
        Ok(_) => (),
        Err(error) => logger::write(format!("Error reading from stream: {:?}", error).as_str()),
    }

    let response = "HTTP/1.1 200 OK\r\n\r\n";

    let stream_write_result = stream.write(response.as_bytes());

    match stream_write_result {
        Ok(_) => (),
        Err(error) => panic!("Problem writing to stream: {:?}", error)
    }

    let stream_flush_result = stream.flush();

    match stream_flush_result {
        Ok(_) => (),
        Err(error) => panic!("Problem flushing stream: {:?}", error)
    }
}
