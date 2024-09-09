pub mod helper;

use super::ClientProcessor;

use crate::lang::process;
use crate::utils::file::write_file;

// RestClient struct for rest-client specific operations
pub struct RestClient;
impl ClientProcessor for RestClient {
    fn process_request(&self, input_file: &str, output_file: &str, url: &str) {
        println!("Using rest-client for the request...");
        println!("Using input file: {}", input_file);
        println!("Using output file: {}", output_file);

        let input_content = process(input_file);

        let mut items = Vec::new();

        for (method, endpoint) in input_content.iter() {
            let mut curl_command = format!("\n{} {}{} HTTP/1.1\n", method, url, endpoint);

            if method == "POST" || method == "PUT" {
                curl_command = format!(
                    "{} {}{} HTTP/1.1\ncontent-type: application/json\n\n{{}}\n",
                    method, url, endpoint
                );
            }

            items.push(curl_command);
        }

        let result = items.join("\n###\n");

        write_file(output_file, &result).expect("Unable to process rest-client");
    }
}
