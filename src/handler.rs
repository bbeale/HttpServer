use super::http::{StatusCode, Method, Request, Response};
use super::server::Handler;
use std::fs;

pub struct RequesteHandler {
    public_path: String
}

impl RequesteHandler {
    pub fn new(public_path: String) -> Self {
        Self { public_path }
    }

    fn read_file(&self, file_path: &str) -> Option<String> {
        let path = format!("{}/{}", self.public_path, file_path);

        match fs::canonicalize(path) {
            // This should also strip out other injections,
            // though it would be interesting to test if
            // they can happen if, say, a legitimate directory
            // has an XSS payload for it's name.
            //
            // Maybe this is not relevant if our web app has
            // sufficient protection from files or child
            // directories being given malicous names... any
            // malicious directory names further up will not
            // be valid since they are out of public_path
            //
            // TODO: Add test case and/or additional validation
            Ok(path) => {
                if path.starts_with(&self.public_path) {
                    fs::read_to_string(path).ok()
                } else {
                    println!("[ALERT] Attempted Directory Traversal attack detected: {}", file_path);
                    None
                }
            },
            Err(_) => None,
        }
    }
}

impl Handler for RequesteHandler {
    fn handle_request(&mut self, request: &Request) -> Response {
        //Response::new(StatusCode::Ok, Some("<h1>TEST</h1>".to_string()))

        match request.method() {
            Method::GET => match request.path() {
                "/" => Response::new(StatusCode::Ok, self.read_file("index.html")),
                "/hello" => Response::new(StatusCode::Ok, self.read_file("hello.html")),
                path => match self.read_file(path) {

                     // NONONO! Directory traversal introduced here
                    Some(contents) => Response::new(StatusCode::Ok, Some(contents)),
                    None => Response::new(StatusCode::NotFound, None),
                }
            }
            _ => Response::new(StatusCode::NotFound, None),
        }
    }
}
