use hello_rust_shere::http_response;

#[test]
fn test_http_response_format() {
    let body = "Hello from config!";
    let response = http_response(body);

    assert!(response.starts_with("HTTP/1.1 200 OK"));
    assert!(response.contains("Content-Length: 20"));
    assert!(response.ends_with(&format!("\r\n\r\n{}", body)));
}
