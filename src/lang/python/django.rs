use std::collections::HashMap;

use regex::Regex;

use crate::utils;

pub fn django_method_endpoints(file_path: &str) -> HashMap<String, String> {
    let content = utils::read_file(file_path).expect("Unable to read file");

    let re = Regex::new(r#"\b(path|re_path|include)\(\"([^\"]+)\"\)"#).unwrap();

    utils::method_endpoints(re, &content, true)
}
