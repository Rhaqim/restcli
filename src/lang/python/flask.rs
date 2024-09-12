use std::collections::HashMap;

use regex::Regex;

use crate::utils;

/// Python file
/// Flask framework
/// method is in the form of @app.route
/// Flask framework Regex::new(r#"\b@app\.route\(\"([^\"]+)\"\)"#).unwrap();
///
/// Django framework
/// method is in the form of path, re_path, include
/// Django framework Regex::new(r#"\b(path|re_path|include)\(\"([^\"]+)\"\)"#).unwrap();
///
/// FastAPI framework
/// method is in the form of @app.get, @app.post, @app.delete, @app.put
/// FastAPI framework Regex::new(r#"\b@app\.get\(\"([^\"]+)\"\)"#).unwrap(); || Regex::new(r#"\b@app\.post\(\"([^\"]+)\"\)"#).unwrap(); || Regex::new(r#"\b@app\.delete\(\"([^\"]+)\"\)"#).unwrap(); || Regex::new(r#"\b@app\.put\(\"([^\"]+)\"\)"#).unwrap();
pub fn flask_method_endpoints(file_path: &str) -> HashMap<String, Vec<String>> {
    let content = utils::read_file(file_path).expect("Unable to read file");

    let re = Regex::new(r#"route\('(/[^']*)',\s*methods=\['([A-Z]+)'\]"#).unwrap();

    utils::method_endpoints(re, &content, true)
}

mod test {
    #[test]
    fn test_flask_method_endpoints() {
        let file_path = "test_files/input/main.py";
        let result = super::flask_method_endpoints(file_path);

        let mut expected = std::collections::HashMap::new();
        expected.insert("POST".to_string(), vec!["/ping".to_string()]);

        assert_eq!(result, expected);
    }
}
