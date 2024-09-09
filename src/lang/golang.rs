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
pub fn detect_methods_in_go_file(file_path: &str, url: &str) -> Result<String, std::io::Error> {
    let content = utils::read_file(file_path).expect("Unable to read file");

    // Update patterns to match method and endpoint
    let re = Regex::new(r#"\b(http\.MethodPost|http\.MethodGet|http\.MethodDelete|http\.MethodPatch|http\.MethodPut)\s*\(\s*\"([^\"]+)\""#).unwrap();

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
        Err(std::io::Error::new(
            std::io::ErrorKind::Other,
            "No methods found in file",
        ))
    } else {
        let result = curl_commands.join("\n###\n");
        Ok(result)
    }
}
