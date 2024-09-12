use regex::Regex;
use std::collections::HashMap;

pub fn method_endpoints(re: Regex, content: &str, flip: bool) -> HashMap<String, Vec<String>> {
    let mut methods = HashMap::new();

    for caps in re.captures_iter(content) {
        let (method, endpoint) = if flip {
            let endpoint = caps.get(1).map_or("", |e| e.as_str());
            let method = caps.get(2).map_or("", |m| m.as_str());
            (method, endpoint)
        } else {
            let method = caps.get(1).map_or("", |m| m.as_str());
            let endpoint = caps.get(2).map_or("", |e| e.as_str());
            (method, endpoint)
        };

        // Add endpoint to the method's list of endpoints
        methods
            .entry(method.to_string())
            .or_insert_with(Vec::new)
            .push(endpoint.to_string());
    }

    methods
}

// pub fn method_endpoints(re: Regex, content: &str, flip: bool) -> HashMap<String, String> {
//     let mut methods = HashMap::new();

//     for caps in re.captures_iter(&content) {
//         if flip {
//             let endpoint = caps.get(1).map_or("", |e| e.as_str());
//             let method = caps.get(2).map_or("", |m| m.as_str());

//             methods.insert(method.to_string(), endpoint.to_string());
//             continue;
//         }
//         let method = caps.get(1).map_or("", |m| m.as_str());
//         let endpoint = caps.get(2).map_or("", |e| e.as_str());

//         println!("Inserting {} {}", method, endpoint);

//         methods.insert(method.to_string(), endpoint.to_string());
//     }

//     methods
// }

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

        let re = Regex::new(r#"\b(r\.POST|r\.GET|r\.DELETE|r\.PATCH|r\.PUT)\s*\(\s*\"([^\"]+)\""#)
            .unwrap();

        let methods = super::method_endpoints(re, content, false);

        assert_eq!(methods.get("r.GET").unwrap(), &vec!["/ping"]);
        assert_eq!(methods.get("r.POST").unwrap(), &vec!["/ping"]);
        assert_eq!(methods.get("r.DELETE").unwrap(), &vec!["/ping"]);
        assert_eq!(methods.get("r.PATCH").unwrap(), &vec!["/ping"]);
        assert_eq!(methods.get("r.PUT").unwrap(), &vec!["/ping"]);
    }
}
