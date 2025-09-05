# HTTP Server with Router - Code Documentation

## Module Structure

```
src/
├── main.rs              // Server initialization and startup
├── routes/
│   └── mod.rs          // Route definitions and handlers
└── server/
    ├── mod.rs          // Module re-exports
    ├── http.rs         // HTTP request/response structures
    ├── router.rs       // Routing logic and handler dispatch
    └── handler.rs      // Connection handling and request processing
```

## HTTP Module (`src/server/http.rs`)

### Dependencies and Imports
```rust
use std::collections::HashMap;  // For storing headers as key-value pairs
```

### HTTP Request Structure
```rust
#[derive(Debug)]
pub struct HttpRequest {
    pub method: String,                    // HTTP method (GET, POST, etc.)
    pub path: String,                      // Request path (/users, /api/data)
    pub headers: HashMap<String, String>,  // HTTP headers as key-value pairs
    pub body: String,                      // Request body content
}
```

**Design Decisions**:
- **Public Fields**: Direct access for simplicity
- **String Types**: UTF-8 text handling, owned data
- **HashMap Headers**: Flexible header storage, case-sensitive keys
- **Debug Trait**: Enables debugging output for troubleshooting

### HTTP Response Structure
```rust
#[derive(Debug)]
pub struct HttpResponse {
    pub status_code: u16,                  // HTTP status code (200, 404, etc.)
    pub status_message: String,            // Status message ("OK", "Not Found")
    pub headers: HashMap<String, String>,  // Response headers
    pub body: Vec<u8>,                     // Response body as bytes
}
```

**Key Differences from Request**:
- **Vec<u8> Body**: Supports binary content (images, files)
- **u16 Status Code**: Standard HTTP status code range
- **Owned Headers**: Response controls header content

### Response Builder Implementation
```rust
impl HttpResponse {
    pub fn new(
        status_code: u16, 
        status_message: String, 
        headers: Option<HashMap<String, String>>, 
        body: Vec<u8>
    ) -> Self {
        HttpResponse {
            status_code,
            status_message,
            headers: headers.unwrap_or_default(),  // Default empty HashMap
            body,
        }
    }
```

**Constructor Pattern**:
- **Optional Headers**: Use `Option<HashMap>` for flexibility
- **Default Fallback**: