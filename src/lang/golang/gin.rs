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
pub fn gin_method_endpoints(file_path: &str) -> HashMap<String, Vec<String>> {
    let content = utils::read_file(file_path).expect("Unable to read file");

    let re = Regex::new(r#"\b(POST|GET|DELETE|PATCH|PUT)\s*\(\s*\"([^\"]+)\""#).unwrap();

    utils::method_endpoints(re, &content, false)
}
