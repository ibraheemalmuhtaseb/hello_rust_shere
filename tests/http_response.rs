use hello_rust_shere::http_response;

#[test]
fn test_http_response_format() {
    let body = "Hello from config!";
let response = http_response(body);

let expected_length = body.len().to_string();
assert!(response.starts_with("HTTP/1.1 200 OK"));
assert!(response.contains(&format!("Content-Length: {}", expected_length)));
assert!(response.ends_with(&format!("\r\n\r\n{}", body)));

}
