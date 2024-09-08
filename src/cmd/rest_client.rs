use crate::utils;

use super::ClientProcessor;

// RestClient struct for rest-client specific operations
pub struct RestClient;
impl ClientProcessor for RestClient {
    fn process_request(&self, input_file: &str, output_file: &str, url: &str) {
        println!("Using rest-client for the request...");
        println!("Using input file: {}", input_file);
        println!("Using output file: {}", output_file);
        // Call the actual rest-client processor here
        let content = utils::detect_methods_in_file(input_file, url);

        match content {
            Ok(content) => {
                utils::write_file(output_file, &content).expect("Unable to process rest-client");
            }
            Err(e) => {
                eprintln!("Error: {}", e);
            }
        }
    }
}
