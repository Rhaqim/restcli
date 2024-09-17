use std::collections::HashMap;

use regex::Regex;

use crate::utils;

pub fn nextjs_method_endpoints(file_path: &str) -> HashMap<String, Vec<String>> {
    let content = utils::read_file(file_path).expect("Unable to read file");

    let re = Regex::new(r#"\bfunction\s+(GET|POST|DELETE|PATCH|PUT)\s*\(\s*\"([^\"]+)\""#).unwrap();

    utils::method_endpoints(re, &content, true)
}