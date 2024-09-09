use regex::Regex;

use crate::utils;

/// Rust file
/// Actix-web framework
/// method is in the form of .route
/// Actix-web framework Regex::new(r#"\.route\(\"([^\"]+)\"\)"#).unwrap();
///
/// Rocket framework
/// method is in the form of get, post, delete, patch, put
/// Rocket framework Regex::new(r#"\b(get|post|delete|patch|put)\(\"([^\"]+)\"\)"#).unwrap();
///
/// Warp framework
/// method is in the form of warp::path
/// Warp framework Regex::new(r#"\bwarp::path\(\"([^\"]+)\"\)"#).unwrap();
///
/// Tide framework
/// method is in the form of .at
/// Tide framework Regex::new(r#"\.at\(\"([^\"]+)\"\)"#).unwrap();
pub fn detect_methods_in_rs_file(file_path: &str, url: &str) -> Result<String, std::io::Error> {
    let content = utils::read_file(file_path).expect("Unable to read file");

    // Update patterns to match method and endpoint
    let re = Regex::new(r#"\.route\(\"([^\"]+)\"\)"#).unwrap();

    let mut curl_commands = Vec::new();

    for caps in re.captures_iter(&content) {
        let endpoint = caps.get(1).map_or("", |e| e.as_str());

        // Construct the curl command
        let mut curl_command = format!("\n{} {}{} HTTP/1.1\n", url, "GET", endpoint);

        curl_commands.push(curl_command);
    }

    if curl_commands.is_empty() {
        Err(std::io::Error::new(
            std::io::ErrorKind::Other,
            "No methods found in file",
        ))
    } else {
        let result = curl_commands.join("\n###\n");
        Ok(result)
    }
}
