pub mod helper;

use super::ClientProcessor;

use crate::lang::process;

use crate::utils::write_file;

// CurlClient struct for curl specific operations
pub struct CurlClient;
impl ClientProcessor for CurlClient {
    fn process_request(&self, input_files: &Vec<String>, output_file: &str, url: &str) {
        println!("Using curl for the request...");
        println!("Using output file: {}", output_file);

        let input_content = process(input_files);

        let mut items = Vec::new();

        for (method, endpoint) in input_content.iter() {
            for e in endpoint {
                let mut curl_command = format!("\ncurl -X {} {}{}", method, url, e);

                if method == "POST" || method == "PUT" {
                    curl_command = format!(
                        "curl -X {} {}{} -H \"content-type: application/json\" -d '{{}}'",
                        method, url, e
                    );
                }

                items.push(curl_command);
            }
        }

        let result = items.join("\n");

        write_file(output_file, &result).expect("Unable to process rest-client");
    }
}
