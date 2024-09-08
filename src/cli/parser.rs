use super::Opts;

use clap::Parser;

use crate::cmd;

pub fn parse() {
    let opts = Opts::parse();

    let url = format!("{}:{}", opts.options.url.clone(), opts.options.port.clone());
    let input_file = opts.options.file.clone();
    let mut output_file = format! {"{}.sh", opts.options.output.clone()};

    if opts.options.postman {
        output_file = format! {"{}.json", opts.options.output};
    } else if opts.options.rest_client {
        output_file = format! {"{}.http", opts.options.output};
    }

    let client = cmd::get_client(&opts);
    client.process_request(&input_file, &output_file, &url);
}
