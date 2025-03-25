mod server;
mod routes;

use std::sync::Arc;
use server::{Router, handle_connection};

fn main() -> std::io::Result<()> {
    // Create a new router
    let router = Arc::new({
        let mut router = Router::new();
        
        // Register routes
        routes::register_routes(&mut router);
        
        router
    });

    // Create and start the server
    let listener = std::net::TcpListener::bind("127.0.0.1:8080")?;
    println!("Server listening on 127.0.0.1:8080");

    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                let router = Arc::clone(&router);
                std::thread::spawn(move || {
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
