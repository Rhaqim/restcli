pub mod gin;

pub use gin::gin_method_endpoints;

use std::collections::HashMap;

use regex::Regex;

use crate::utils;

/// Golang files
/// Gin framework
/// method is in the form of r.POST, r.GET, r.DELETE, r.PATCH, r.PUT
/// gin framework Regex::new(r#"\b(r\.POST|r\.GET|r\.DELETE|r\.PATCH|r\.PUT)\s*\(\s*\"([^\"]+)\""#).unwrap(); || Regex::new(r#"\b(POST|GET|DELETE|PATCH|PUT)\s*\(\s*\"([^\"]+)\""#).unwrap();
///
/// regular http
/// method is in the form of http.MethodPost, http.MethodGet, http.MethodDelete, http.MethodPatch, http.MethodPut
/// regular http Regex::new(r#"\b(http\.MethodPost|http\.MethodGet|http\.MethodDelete|http\.MethodPatch|http\.MethodPut)\s*\(\s*\"([^\"]+)\""#).unwrap();
///
pub fn detect_methods_in_go_file(file_path: &str, url: &str) -> String {
    let content = utils::read_file(file_path).expect("Unable to read file");

    println!("Content: {}", content);

    // Update patterns to match method and endpoint
    let re = Regex::new(r#"\b(http\.MethodPost|http\.MethodGet|http\.MethodDelete|http\.MethodPatch|http\.MethodPut)\s*"#).unwrap();

    let mut curl_commands = Vec::new();

    for caps in re.captures_iter(&content) {
        let method = caps.get(1).map_or("", |m| m.as_str());
        let endpoint = caps.get(2).map_or("", |e| e.as_str());

        // Construct the curl command
        let mut curl_command = format!("\n{} {}{} HTTP/1.1\n", url, method, endpoint);

        if method == "http.MethodPost" || method == "http.MethodPut" {
            curl_command = format!(
                "{} {}{} HTTP/1.1\ncontent-type: application/json\n\n{{}}\n",
                url, method, endpoint
            );
        }
        curl_commands.push(curl_command);
    }

    if curl_commands.is_empty() {
        "No methods found".to_string()
    } else {
        curl_commands.join("\n###\n")
    }
}

fn gin_rest_client(file_path: &str, url: &str) -> String {
    let methods = gin_method_endpoints(file_path);

    let mut curl_commands = Vec::new();

    for (method, endpoint) in methods {
        let mut curl_command = format!("\n{} {}{} HTTP/1.1\n", method, url, endpoint);

        if method == "POST" || method == "PUT" {
            curl_command = format!(
                "{} {}{} HTTP/1.1\ncontent-type: application/json\n\n{{}}\n",
                method, url, endpoint
            );
        }
        curl_commands.push(curl_command);
    }

    if curl_commands.is_empty() {
        "No methods found".to_string()
    } else {
        curl_commands.join("\n###\n")
    }
}

mod tests {
    #[test]
    fn test_detect_methods_in_go_file() {
        let file_path = "test_files/input/main.go";
        let url = "http://localhost:8080";
        let result = super::detect_methods_in_go_file(file_path, url);
        let expected = r#"
http://localhost:8080 POST/hello HTTP/1.1
"#;
        assert_eq!(result, expected);
    }

    #[test]
    fn test_gin_rest_client() {
        let file_path = "test_files/input/main.go";
        let url = "http://localhost:8080";
        let result = super::gin_rest_client(file_path, url);
        let expected = r#"
.POST http://localhost:8080/hello HTTP/1.1
content-type: application/json

{}
"#;
        assert_eq!(result, expected);
    }
}
