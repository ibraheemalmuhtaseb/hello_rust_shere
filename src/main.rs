use std::fs;
use std::io::{Read, Write};
use std::net::{TcpListener, TcpStream};

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
    let address = format!("127.0.0.1:{}", config.port);
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
    let mut buffer = [0; 1024];
    let bytes_read = stream.read(&mut buffer)?;
    let request = String::from_utf8_lossy(&buffer[..bytes_read]);

    println!("Request:\n{}", request);

    let response = if request.starts_with("GET") {
        // Serve static HTML file instead of plain message
        let html = fs::read_to_string("static/index.html")
            .unwrap_or_else(|_| "<h1>404 Not Found</h1>".to_string());
        http_html_response(&html)
    } else if request.starts_with("POST") {
        http_response(&format!("POST: {}", config.message))
    } else if request.starts_with("DELETE") {
        http_response(&format!("DELETE: {}", config.message))
    } else {
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
