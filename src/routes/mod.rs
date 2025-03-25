use crate::server::{HttpRequest, HttpResponse, Router};
use std::fs;

pub fn register_routes(router: &mut Router) {
    // Home route
    router.add_route(
        "GET".to_string(), 
        "/".to_string(), 
        |_req| HttpResponse::ok_html("<html><body><h1>Welcome to Rust HTTP Server!</h1></body></html>")
    );

    // Users route
    router.add_route(
        "GET".to_string(), 
        "/users".to_string(), 
        |_req| HttpResponse::ok_json("[{\"id\":1,\"name\":\"Alice\"},{\"id\":2,\"name\":\"Bob\"}]")
    );

    // Static file serving route
    router.add_route(
        "GET".to_string(), 
        "/static/".to_string(), 
        |req| {
            // Remove leading /static/ from path
            let file_path = format!("./static{}", req.path.replace("/static", ""));
            
            match fs::read(&file_path) {
                Ok(contents) => HttpResponse::new(
                    200,
                    "OK".to_string(),
                    Some([("Content-Type".to_string(), "text/plain".to_string())].iter().cloned().collect()),
                    contents
                ),
                Err(_) => HttpResponse::not_found()
            }
        }
    );
}
