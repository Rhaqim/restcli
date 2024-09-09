use crate::utils;
use regex::Regex;

pub fn detect_methods_in_file(file_path: &str, url: &str) -> Result<String, std::io::Error> {
    let content = utils::read_file(file_path).expect("Unable to read file");

    // Update patterns to match method and endpoint
    let re = Regex::new(r#"\b(POST|GET|DELETE|PATCH|PUT)\s*\(\s*\"([^\"]+)\""#).unwrap();

    let mut curl_commands = Vec::new();

    for caps in re.captures_iter(&content) {
        let method = caps.get(1).map_or("", |m| m.as_str());
        let endpoint = caps.get(2).map_or("", |e| e.as_str());

        // Construct the curl command
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
        Err(std::io::Error::new(
            std::io::ErrorKind::Other,
            "No methods found in file",
        ))
    } else {
        let result = curl_commands.join("\n###\n");
        Ok(result)
    }
}
