use super::ClientProcessor;

// CurlClient struct for curl specific operations
pub struct CurlClient;
impl ClientProcessor for CurlClient {
    fn process_request(&self, _input_file: &str, output_file: &str, _url: &str) {
        println!("Using curl for the request...");
        println!("Using output file: {}", output_file);
    }
}
