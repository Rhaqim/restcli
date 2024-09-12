pub mod curl;
pub mod postman;
pub mod rest_client;

pub use curl::CurlClient;
pub use postman::PostmanClient;
pub use rest_client::RestClient;

use crate::cli;

pub trait ClientProcessor {
    fn process_request(&self, input_file: &Vec<String>, output_file: &str, url: &str);
}

// Factory function to select the appropriate client based on options
pub fn get_client(opts: &cli::Opts) -> Vec<(Box<dyn ClientProcessor>, String)> {
    let mut clients: Vec<(Box<dyn ClientProcessor>, String)> = Vec::new();

    if opts.options.postman {
        clients.push((
            Box::new(PostmanClient),
            format!("{}.postman_collection.json", opts.options.output),
        ));
    }

    if opts.options.rest_client {
        clients.push((
            Box::new(RestClient),
            format!("{}.http", opts.options.output),
        ));
    }

    if opts.options.curl {
        clients.push((Box::new(CurlClient), format!("{}.sh", opts.options.output)));
    }

    clients
}
