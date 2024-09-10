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
    fn process_request(&self, input_file: &str, output_file: &str, url: &str) {
        println!("Using Postman for the request...");
        println!("Using input file: {}", input_file);
        println!("Using output file: {}", output_file);

        let input_content = process(input_file);

        let mut items = Vec::new();

        for (method, endpoint) in input_content.iter() {
            let header = header_postman_export_json("content-type", "application/json");

            let split_endpoint: Vec<&str> = endpoint.split("/").collect();
            let url = url_postman_export_json(
                format!("{}{}", url, endpoint).as_str(),
                "localhost",
                split_endpoint,
            );
            let request = request_postman_export_json(&method, &url, &header);
            let inner_item = inner_item_postman_export_json(&endpoint, &request);
            let item =
                item_postman_export_json(format!("{} request", &endpoint).as_str(), &inner_item);
            items.push(item);
        }

        let item = items.join(",");

        let postman_json = base_postman_export_json(Some("New Collection"), None, &item);

        write_file(output_file, &postman_json).unwrap();
    }
}
