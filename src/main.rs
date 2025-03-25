use std::io::{Read, Write};
use std::net::{TcpListener, TcpStream};
use std::thread;
use std::sync::Arc;
use std::collections::HashMap;
use std::fs;

// HTTP Request Structure
#[derive(Debug)]
struct HttpRequest {
    method: String,
    path: String,
    headers: HashMap<String, String>,
    body: String,
}

// HTTP Response Structure
#[derive(Debug)]
struct HttpResponse {
    status_code: u16,
    status_message: String,
    headers: HashMap<String, String>,
    body: Vec<u8>,
}

// Router for handling different routes
struct Router {
    routes: HashMap<(String, String), Box<dyn Fn(&HttpRequest) -> HttpResponse + Send + Sync>>,
}

impl Router {
    fn new() -> Self {
        Router {
            routes: HashMap::new(),
        }
    }

    fn add_route<F>(&mut self, method: String, path: String, handler: F)
    where
        F: Fn(&HttpRequest) -> HttpResponse + 'static + Send + Sync,
    {
        self.routes.insert((method, path), Box::new(handler));
    }

    fn handle_request(&self, request: &HttpRequest) -> HttpResponse {
        if let Some(handler) = self.routes.get(&(request.method.clone(), request.path.clone())) {
            handler(request)
        } else {
            HttpResponse {
                status_code: 404,
                status_message: "Not Found".to_string(),
                headers: HashMap::new(),
                body: b"404 - Route Not Found".to_vec(),
            }
        }
    }
}

// HTTP Server Implementation
struct HttpServer {
    router: Arc<Router>,
}

impl HttpServer {
    fn new(router: Router) -> Self {
        HttpServer {
            router: Arc::new(router),
        }
    }

    fn start(&self, address: &str) -> std::io::Result<()> {
        let listener = TcpListener::bind(address)?;
        println!("Server listening on {}", address);

        for stream in listener.incoming() {
            match stream {
                Ok(stream) => {
                    let router = Arc::clone(&self.router);
                    thread::spawn(move || {
                        handle_connection(stream, router);
                    });
                }
                Err(e) => {
                    eprintln!("Error accepting connection: {}", e);
                }
            }
        }
        Ok(())
    }
}

// Parse incoming HTTP request
fn parse_request(stream: &mut TcpStream) -> Result<HttpRequest, std::io::Error> {
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
    let mut headers = HashMap::new();
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

// Write HTTP response to stream
fn write_response(stream: &mut TcpStream, response: &HttpResponse) -> std::io::Result<()> {
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

// Handle individual TCP connection
fn handle_connection(mut stream: TcpStream, router: Arc<Router>) {
    match parse_request(&mut stream) {
        Ok(request) => {
            let response = router.handle_request(&request);
            if let Err(e) = write_response(&mut stream, &response) {
                eprintln!("Error writing response: {}", e);
            }
        }
        Err(e) => {
            eprintln!("Error parsing request: {}", e);
            
            let error_response = HttpResponse {
                status_code: 400,
                status_message: "Bad Request".to_string(),
                headers: HashMap::new(),
                body: b"400 - Bad Request".to_vec(),
            };

            if let Err(write_err) = write_response(&mut stream, &error_response) {
                eprintln!("Error writing error response: {}", write_err);
            }
        }
    }
}

fn main() -> std::io::Result<()> {
    // Create a new router
    let mut router = Router::new();

    // Home route
    router.add_route(
        "GET".to_string(), 
        "/".to_string(), 
        |_req| HttpResponse {
            status_code: 200,
            status_message: "OK".to_string(),
            headers: [("Content-Type".to_string(), "text/html".to_string())]
                .iter().cloned().collect(),
            body: b"<html><body><h1>Welcome to Rust HTTP Server!</h1></body></html>".to_vec(),
        }
    );

    // Users route
    router.add_route(
        "GET".to_string(), 
        "/users".to_string(), 
        |_req| HttpResponse {
            status_code: 200,
            status_message: "OK".to_string(),
            headers: [("Content-Type".to_string(), "application/json".to_string())]
                .iter().cloned().collect(),
            body: b"[{\"id\":1,\"name\":\"Alice\"},{\"id\":2,\"name\":\"Bob\"}]".to_vec(),
        }
    );

    // Static file serving route
    router.add_route(
        "GET".to_string(), 
        "/static/".to_string(), 
        |req| {
            // Remove leading /static/ from path
            let file_path = format!("./static{}", req.path.replace("/static", ""));
            
            match fs::read(&file_path) {
                Ok(contents) => HttpResponse {
                    status_code: 200,
                    status_message: "OK".to_string(),
                    headers: [("Content-Type".to_string(), "text/plain".to_string())]
                        .iter().cloned().collect(),
                    body: contents,
                },
                Err(_) => HttpResponse {
                    status_code: 404,
                    status_message: "Not Found".to_string(),
                    headers: HashMap::new(),
                    body: b"File not found".to_vec(),
                }
            }
        }
    );

    // Create and start the server
    let server = HttpServer::new(router);
    server.start("127.0.0.1:8080")
}
