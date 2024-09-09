pub mod helper;

use std::collections::HashMap;

use super::ClientProcessor;

use helper::{
    base_postman_export_json, header_postman_export_json, inner_item_postman_export_json,
    item_postman_export_json, request_postman_export_json, url_postman_export_json,
};

use crate::lang::golang::process as go_process;
use crate::utils::file::{get_file_extension, write_file};
// use crate::lang::python::process as py_process;
// use crate::lang::rust::process as rs_process;

// PostmanClient struct for Postman specific operations
pub struct PostmanClient;
impl ClientProcessor for PostmanClient {
    fn process_request(&self, input_file: &str, output_file: &str, _url: &str) {
        println!("Using Postman for the request...");
        println!("Using input file: {}", input_file);
        println!("Using output file: {}", output_file);

        let extension = get_file_extension(input_file).unwrap();

        let mut input_content = HashMap::new();

        match extension {
            "go" => {
                input_content = go_process(input_file).unwrap();
            }
            _ => {
                eprintln!("File extension not supported, supported extensions are: go, rs, py");
                std::process::exit(1);
            }
        }

        let mut items = Vec::new();

        for (method, endpoint) in input_content.iter() {
            let header = header_postman_export_json("content-type", "application/json");
            let url = url_postman_export_json(
                &endpoint,
                "localhost",
                format!("{}{}", _url, endpoint).as_str(),
            );
            let request = request_postman_export_json(&method, &url, &header);
            let inner_item = inner_item_postman_export_json(&endpoint, &request);
            let item = item_postman_export_json("Request", &inner_item);
            items.push(item);
        }

        let item = items.join(",");

        let postman_json = base_postman_export_json(Some("New Collection"), None, &item);

        write_file(output_file, &postman_json).unwrap();
    }
}
