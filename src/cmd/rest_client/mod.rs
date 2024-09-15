pub mod helper;

use super::ClientProcessor;

use crate::lang::process;
use crate::utils::write_file;

// RestClient struct for rest-client specific operations
pub struct RestClient;
impl ClientProcessor for RestClient {
    fn process_request(
        &self,
        input_files: &Vec<String>,
        url: &str,
        output_file: &str,
        append: bool,
    ) {
        println!("Using rest-client for the request...");
        println!("Using input file: {}", input_files.join(", "));
        println!("Using output file: {}", output_file);

        let input_content = process(input_files);

        let mut items = Vec::new();

        for (method, endpoint) in input_content.iter() {
            for e in endpoint {
                let mut curl_command = format!("\n{} {}{} HTTP/1.1\n", method, url, e);

                if method == "POST" || method == "PUT" {
                    curl_command = format!(
                        "{} {}{} HTTP/1.1\ncontent-type: application/json\n\n{{}}\n",
                        method, url, e
                    );
                }

                items.push(curl_command);
            }
        }

        let result = items.join("\n###\n");

        write_file(output_file, &result, append).expect("Unable to process rest-client");
    }
}
