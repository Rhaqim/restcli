pub mod gin;

use std::collections::HashMap;

use regex::Regex;

pub use gin::{detect_framework, gin_method_endpoints};

use crate::utils;

pub fn process(input_file: &str) -> Result<HashMap<String, String>, String> {
    let framework = detect_framework(&input_file);

    match framework.as_str() {
        "gin" => {
            let methods = gin_method_endpoints(input_file);
            Ok(methods)
        }
        _ => Err("Framework not supported".to_string()),

    }
}

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
