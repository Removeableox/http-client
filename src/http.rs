use std::{
    net::TcpStream,
    io::{Write, Read},
};

pub fn get_http(url: String) -> String {
    let request = format!(
        "GET / HTTP/1.1\r\n\
        Host: {url}\r\n\
        Connection: close\r\n\
        \r\n"
    );

    let mut stream = TcpStream::connect(url).unwrap();
    stream.write_all(request.as_bytes()).unwrap();
    let mut buff = String::new();
    stream.read_to_string(&mut buff).unwrap();

    buff
}
