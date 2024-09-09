use std::collections::HashMap;

use regex::Regex;

pub fn method_endpoints(re: Regex, content: &str) -> HashMap<String, String> {
    let mut methods = HashMap::new();

    for caps in re.captures_iter(&content) {
        let method = caps.get(1).map_or("", |m| m.as_str());
        let endpoint = caps.get(2).map_or("", |e| e.as_str());

        methods.insert(method.to_string(), endpoint.to_string());
    }

    methods
}

mod test {
    #[test]
    fn test_method_endpoint() {
        use regex::Regex;

        let content = r#"
            r.GET("/ping", ping)
            r.POST("/ping", ping)
            r.DELETE("/ping", ping)
            r.PATCH("/ping", ping)
            r.PUT("/ping", ping)
        "#;

        let re = Regex::new(r#"\b(r\.POST|r\.GET|r\.DELETE|r\.PATCH|r\.PUT)\s*\(\s*\"([^\"]+)\""#).unwrap();

        let methods = super::method_endpoints(re, content);

        assert_eq!(methods.get("r.GET").unwrap(), "/ping");
        assert_eq!(methods.get("r.POST").unwrap(), "/ping");
        assert_eq!(methods.get("r.DELETE").unwrap(), "/ping");
        assert_eq!(methods.get("r.PATCH").unwrap(), "/ping");
        assert_eq!(methods.get("r.PUT").unwrap(), "/ping");
    }
}