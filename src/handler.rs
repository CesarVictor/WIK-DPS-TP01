use crate::header::Header;
use std::io::{Read, Result, Write};
use std::net::TcpStream;

pub(crate) fn handle_client(mut stream: TcpStream) -> Result<()> {
    let mut buffer = [0; 1024];
    let size = stream.read(&mut buffer)?;

    let request = String::from_utf8_lossy(&buffer[..size]);

    if request.starts_with("GET /ping ") {
        let header: Vec<String> = request
            .lines()
            .skip(1)
            .filter(|line| !line.trim().is_empty())
            .map(|line| line.to_string())
            .collect();

        let header = Header::new(&header);
        let response_body = header.serialize();

        let response = format!(
            "HTTP/1.1 200 OK\r\nContent-Type: application/json\r\nContent-Length: {}\r\n\r\n{}",
            response_body.len(),
            response_body
        );

        stream.write_all(response.as_bytes())?;
        stream.flush()?;
    } else {
        let response = "HTTP/1.1 404 Not Found\r\nContent-Length: 0\r\n\r\n";
        stream.write_all(response.as_bytes())?;
        stream.flush()?;
    }

    Ok(())
}
