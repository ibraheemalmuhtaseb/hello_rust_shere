use std::fs;
use std::io::{Read, Write};
use std::net::{TcpListener, TcpStream};
use std::process::{Command, Stdio};
use serde::Deserialize;

#[derive(Debug, Deserialize)]
struct Config {
    port: u16,
    message: String,
}

fn load_config() -> Config {
    let config_data = fs::read_to_string("config.toml").expect("Failed to read config.toml");
    toml::from_str(&config_data).expect("Invalid config format")
}

fn main() -> std::io::Result<()> {
    let config = load_config();
    let address = format!("0.0.0.0:{}", config.port);
    let listener = TcpListener::bind(&address)?;
    println!("Server listening on http://{}", address);

    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                handle_client(stream, &config)?;
            }
            Err(e) => {
                eprintln!("Connection error: {}", e);
            }
        }
    }

    Ok(())
}

fn handle_client(mut stream: TcpStream, config: &Config) -> std::io::Result<()> {
    let mut buffer = [0; 4096];
    let bytes_read = stream.read(&mut buffer)?;
    let request = String::from_utf8_lossy(&buffer[..bytes_read]);

    println!("Request:\n{}", request);
    let path_line = request.lines().next().unwrap_or("");
    let mut parts = path_line.split_whitespace();
    let method = parts.next().unwrap_or("");
    let path = parts.next().unwrap_or("/");

    if method == "POST" {
    let script_path = "hi.py";
    let path_info = path;
    let body = request.split("\r\n\r\n").nth(1).unwrap_or("");

    if fs::metadata(script_path).is_ok() {
        let output = Command::new("python3")
            .arg(script_path)
            .env("PATH_INFO", path_info)
            .stdin(Stdio::piped())
            .stdout(Stdio::piped())
            .spawn()
            .and_then(|mut child| {
                if let Some(stdin) = child.stdin.as_mut() {
                    stdin.write_all(body.as_bytes())?;
                }
                child.wait_with_output()
            });

        match output {
            Ok(output) => {
                let response = format!(
                    "HTTP/1.1 200 OK\r\nContent-Type: text/plain\r\n\r\n{}",
                    String::from_utf8_lossy(&output.stdout)
                );
                stream.write_all(response.as_bytes())?;
            }
            Err(e) => {
                let response = format!(
                    "HTTP/1.1 500 Internal Server Error\r\n\r\nCGI error: {}",
                    e
                );
                stream.write_all(response.as_bytes())?;
            }
        }

        return Ok(());
        }
    }

    if method == "GET" && path.starts_with("/stayle/") {
        let file_path = &path[1..];
        match fs::read(file_path) {
            Ok(contents) => {
                let content_type = if file_path.ends_with(".css") {
                    "text/css"
                } else if file_path.ends_with(".jpg") || file_path.ends_with(".jpeg") {
                    "image/jpeg"
                } else if file_path.ends_with(".png") {
                    "image/png"
                } else {
                    "application/octet-stream"
                };

                let response = format!(
                    "HTTP/1.1 200 OK\r\nContent-Length: {}\r\nContent-Type: {}\r\n\r\n",
                    contents.len(),
                    content_type
                );
                stream.write_all(response.as_bytes())?;
                stream.write_all(&contents)?;
                return Ok(());
            }
            Err(_) => {
                let response = http_response("Static file not found");
                stream.write_all(response.as_bytes())?;
                return Ok(());
            }
        }
    }

    // Serve static HTML for GET
    let response = if request.starts_with("GET") {
        let html = fs::read_to_string("static/aboutme.html")
            .unwrap_or_else(|_| "<h1>404 Not Found</h1>".to_string());
        http_html_response(&html)
    }
    // Respond to POST with message
    else if request.starts_with("POST") {
        http_response(&format!("POST: {}", config.message))
    }
    // Respond to DELETE with message
    else if request.starts_with("DELETE") {
        http_response(&format!("DELETE: {}", config.message))
    }
    // Fallback
    else {
        http_response("Unknown method")
    };

    stream.write_all(response.as_bytes())?;
    stream.flush()?;
    Ok(())
}

fn http_response(body: &str) -> String {
    format!(
        "HTTP/1.1 200 OK\r\nContent-Length: {}\r\nContent-Type: text/plain\r\n\r\n{}",
        body.len(),
        body
    )
}

fn http_html_response(body: &str) -> String {
    format!(
        "HTTP/1.1 200 OK\r\nContent-Length: {}\r\nContent-Type: text/html\r\n\r\n{}",
        body.len(),
        body
    )
}
