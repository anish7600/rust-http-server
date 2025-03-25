use std::io::{Read, Write};
use std::net::{TcpListener, TcpStream};
use std::thread;
use std::sync::Arc;
use std::fs;

use super::http::{HttpRequest, HttpResponse};
use super::router::Router;

pub fn parse_request(stream: &mut TcpStream) -> Result<HttpRequest, std::io::Error> {
    let mut buffer = [0; 1024];
    stream.read(&mut buffer)?;

    let request_str = String::from_utf8_lossy(&buffer[..]);
    let mut lines = request_str.lines();

    // Parse request line
    let request_line = lines.next().unwrap_or("");
    let parts: Vec<&str> = request_line.split_whitespace().collect();
    
    if parts.len() < 3 {
        return Err(std::io::Error::new(
            std::io::ErrorKind::InvalidData, 
            "Invalid request line"
        ));
    }

    let method = parts[0].to_string();
    let path = parts[1].to_string();

    // Parse headers
    let mut headers = std::collections::HashMap::new();
    let mut body = String::new();

    let mut is_body = false;
    for line in lines {
        if line.is_empty() {
            is_body = true;
            continue;
        }

        if is_body {
            body.push_str(line);
        } else {
            let header_parts: Vec<&str> = line.splitn(2, ": ").collect();
            if header_parts.len() == 2 {
                headers.insert(
                    header_parts[0].to_string(), 
                    header_parts[1].to_string()
                );
            }
        }
    }

    Ok(HttpRequest {
        method,
        path,
        headers,
        body,
    })
}

pub fn write_response(stream: &mut TcpStream, response: &HttpResponse) -> std::io::Result<()> {
    let status_line = format!(
        "HTTP/1.1 {} {}\r\n", 
        response.status_code, 
        response.status_message
    );

    let mut response_str = status_line;
    for (key, value) in &response.headers {
        response_str.push_str(&format!("{}: {}\r\n", key, value));
    }
    response_str.push_str("\r\n");

    stream.write_all(response_str.as_bytes())?;
    stream.write_all(&response.body)?;
    stream.flush()?;

    Ok(())
}

pub fn handle_connection(mut stream: TcpStream, router: Arc<Router>) {
    match parse_request(&mut stream) {
        Ok(request) => {
            let response = router.handle_request(&request);
            if let Err(e) = write_response(&mut stream, &response) {
                eprintln!("Error writing response: {}", e);
            }
        }
        Err(e) => {
            eprintln!("Error parsing request: {}", e);
            
            let error_response = HttpResponse::bad_request();

            if let Err(write_err) = write_response(&mut stream, &error_response) {
                eprintln!("Error writing error response: {}", write_err);
            }
        }
    }
}
