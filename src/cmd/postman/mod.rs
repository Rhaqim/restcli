pub mod helper;

use super::ClientProcessor;

// PostmanClient struct for Postman specific operations
pub struct PostmanClient;
impl ClientProcessor for PostmanClient {
    fn process_request(&self, input_file: &str, output_file: &str, _url: &str) {
        println!("Using Postman for the request...");
        println!("Using input file: {}", input_file);
        println!("Using output file: {}", output_file);
    }
}
