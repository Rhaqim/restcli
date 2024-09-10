pub mod helper;

use super::ClientProcessor;

use crate::lang::process;

use crate::utils::write_file;

// CurlClient struct for curl specific operations
pub struct CurlClient;
impl ClientProcessor for CurlClient {
    fn process_request(&self, input_file: &str, output_file: &str, url: &str) {
        println!("Using curl for the request...");
        println!("Using output file: {}", output_file);

        let input_content = process(input_file);

        let mut items = Vec::new();

        for (method, endpoint) in input_content.iter() {
            let mut curl_command = format!("\ncurl -X {} {}{}\n", method, url, endpoint);

            if method == "POST" || method == "PUT" {
                curl_command = format!(
                    "curl -X {} {}{} -H \"content-type: application/json\" -d '{{}}'\n",
                    method, url, endpoint
                );
            }

            items.push(curl_command);
        }

        let result = items.join("\n###\n");

        write_file(output_file, &result).expect("Unable to process rest-client");
    }
}
