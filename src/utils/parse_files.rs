use std::{fs::File, io::Read};

use regex::Regex;

fn read_file(file: &str) -> Result<String, std::io::Error> {
    let mut file = File::open(file)?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;
    Ok(contents)
}

pub fn write_file(file: &str, content: &str) -> Result<(), std::io::Error> {
    std::fs::write(file, content)
}

pub fn detect_methods_in_file(file_path: &str, url: &str) -> Result<String, std::io::Error> {
    let content = read_file(file_path).expect("Unable to read file");

    // Update patterns to match method and endpoint
    let re = Regex::new(r#"\b(POST|GET|DELETE|PATCH|PUT)\s*\(\s*\"([^\"]+)\""#).unwrap();

    let mut curl_commands = Vec::new();

    for caps in re.captures_iter(&content) {
        let method = caps.get(1).map_or("", |m| m.as_str());
        let endpoint = caps.get(2).map_or("", |e| e.as_str());

        // Construct the curl command
        let mut curl_command = format!("\n{} {}{} HTTP/1.1\n", url, method, endpoint);

        if method == "POST" || method == "PUT" {
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
