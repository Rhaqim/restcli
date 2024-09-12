use std::collections::HashMap;

use regex::Regex;

use crate::utils;

pub fn fastapi_method_endpoints(file_path: &str) -> HashMap<String, Vec<String>> {
    let content = utils::read_file(file_path).expect("Unable to read file");

    let re = Regex::new(r#"@app\.get\(["'](?P<endpoint>.*?)["']"#).unwrap();

    utils::method_endpoints(re, &content, false)
}
