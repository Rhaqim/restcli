use std::collections::HashMap;

use regex::Regex;

use crate::utils;

pub fn gin_method_endpoints(file_path: &str) -> HashMap<String, String> {
    let content = utils::read_file(file_path).expect("Unable to read file");

    let re = Regex::new(r#"\b(POST|GET|DELETE|PATCH|PUT)\s*\(\s*\"([^\"]+)\""#).unwrap();

    utils::method_endpoints(re, &content, false)
}

pub fn detect_framework(file_path: &str) -> String {
    let content = utils::read_file(file_path).expect("Unable to read file");

    if content.contains("gin.Default()") {
        return "gin".to_string();
    }

    "unknown".to_string()
}