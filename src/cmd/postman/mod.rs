pub mod helper;

use super::ClientProcessor;

use helper::{
    base_postman_export_json, header_postman_export_json, inner_item_postman_export_json,
    item_postman_export_json, request_postman_export_json, url_postman_export_json,
};

use crate::lang::process;
use crate::utils::write_file;

// PostmanClient struct for Postman specific operations
pub struct PostmanClient;
impl ClientProcessor for PostmanClient {
    fn process_request(&self, input_files: &Vec<String>, output_file: &str, url: &str) {
        println!("Using Postman for the request...");
        println!("Using input file: {}", input_files.join(", "));
        println!("Using output file: {}", output_file);

        let input_content = process(input_files);

        let mut items = Vec::new();

        for (method, endpoint) in input_content.iter() {
            let header = header_postman_export_json("content-type", "application/json");

            for e in endpoint {
                let split_endpoint: Vec<&str> = e.split("/").collect();
                let url = url_postman_export_json(
                    format!("{}{}", url, e).as_str(),
                    "localhost",
                    split_endpoint,
                );
                let request = request_postman_export_json(&method, &url, &header);
                let inner_item = inner_item_postman_export_json(&e, &request);
                let item =
                    item_postman_export_json(format!("{} request", &e).as_str(), &inner_item);
                items.push(item);
            }
        }

        let item = items.join(",");

        let postman_json = base_postman_export_json(Some("New Collection"), None, &item);

        write_file(output_file, &postman_json).unwrap();
    }
}
