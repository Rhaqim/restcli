pub mod curl;
pub mod postman;
pub mod rest_client;

pub use curl::CurlClient;
pub use postman::PostmanClient;
pub use rest_client::RestClient;

use crate::cli;

pub trait ClientProcessor {
    fn process_request(&self, input_file: &str, output_file: &str, url: &str);
}

// Factory function to select the appropriate client based on options
pub fn get_client(opts: &cli::Opts) -> Box<dyn ClientProcessor> {
    if opts.options.postman {
        Box::new(PostmanClient)
    } else if opts.options.rest_client {
        Box::new(RestClient)
    } else {
        Box::new(CurlClient)
    }
}
