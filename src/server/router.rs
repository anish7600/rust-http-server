use std::collections::HashMap;
use std::sync::Arc;
use super::http::{HttpRequest, HttpResponse};

pub struct Router {
    routes: HashMap<(String, String), Box<dyn Fn(&HttpRequest) -> HttpResponse + Send + Sync>>,
}

impl Router {
    pub fn new() -> Self {
        Router {
            routes: HashMap::new(),
        }
    }

    pub fn add_route<F>(&mut self, method: String, path: String, handler: F)
    where
        F: Fn(&HttpRequest) -> HttpResponse + 'static + Send + Sync,
    {
        self.routes.insert((method, path), Box::new(handler));
    }

    pub fn handle_request(&self, request: &HttpRequest) -> HttpResponse {
        self.routes
            .get(&(request.method.clone(), request.path.clone()))
            .map_or_else(
                || HttpResponse::not_found(),
                |handler| handler(request)
            )
    }
}
