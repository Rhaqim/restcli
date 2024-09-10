use uuid::Uuid;

pub fn base_postman_export_json(name: Option<&str>, schema: Option<&str>, item: &str) -> String {
    let name = name.unwrap_or("New Collection");
    let schema =
        schema.unwrap_or("https://schema.getpostman.com/json/collection/v2.1.0/collection.json");

    format!(
        r#"{{
    "info": {{
        "_postman_id": "{}",
        "name": "{}",
        "schema": "{}"
    }},
    "item": [{}]
}}"#,
        Uuid::new_v4().to_string(),
        name,
        schema,
        item
    )
}

pub fn item_postman_export_json(name: &str, item: &str) -> String {
    format!(
        r#"{{
    "name": "{}",
    "item": [{}]
}}"#,
        name, item
    )
}

pub fn inner_item_postman_export_json(name: &str, request: &str) -> String {
    format!(
        r#"{{
    "name": "{}",
    "request": {},
    "response": []
}}"#,
        name, request,
    )
}

pub fn request_postman_export_json(method: &str, url: &str, header: &str) -> String {
    format!(
        r#"{{
    "method": "{}",
    "header": {},
    "url": {}
}}"#,
        method, header, url
    )
}

pub fn url_postman_export_json(raw: &str, host: &str, path: Vec<&str>) -> String {
    // convert path to json array
    let path = path
        .iter()
        .filter(|x| !x.is_empty()) // Ignore empty strings
        .map(|x| format!(r#""{}""#, x))
        .collect::<Vec<String>>()
        .join(",");

    format!(
        r#"{{
    "raw": "{}",
    "host": ["{}"],
    "path": [{}]
}}"#,
        raw, host, path
    )
}

pub fn header_postman_export_json(key: &str, value: &str) -> String {
    format!(
        r#"{{
    "key": "{}",
    "value": "{}",
    "description": ""
}}"#,
        key, value
    )
}

// pub fn body_postman_export_json(mode: &str, raw: &str) -> String {
//     format!(
//         r#"{{
//     "mode": "{}",
//     "raw": "{}"
//     "options": {{
//         "raw": {{
//             "language": "json"
//         }}
//     }}
// }}"#,
//         mode, raw
//     )
// }
