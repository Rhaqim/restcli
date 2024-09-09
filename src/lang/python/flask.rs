use std::collections::HashMap;

use regex::Regex;

use crate::utils;

pub fn flask_method_endpoints(file_path: &str) -> HashMap<String, String> {
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
        expected.insert("POST".to_string(), "/post".to_string());

        assert_eq!(result, expected);
    }
}
