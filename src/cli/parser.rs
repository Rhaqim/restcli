use clap::Parser;

use super::Opts;

use crate::cmd;
use crate::utils::is_supported_extension;

pub fn parse() {
    let opts = Opts::parse();

    let input_file = opts.options.file.clone();

    // check if file is supported
    if !is_supported_extension(&input_file) {
        eprintln!("File extension not supported, supported extensions are: go, rs, py");
        std::process::exit(1);
    }

    let url = format!("{}:{}", opts.options.url.clone(), opts.options.port.clone());

    let mut output_file = format! {"{}.sh", opts.options.output.clone()};

    if opts.options.postman {
        output_file = format! {"{}.json", opts.options.output};
    } else if opts.options.rest_client {
        output_file = format! {"{}.http", opts.options.output};
    }

    let client = cmd::get_client(&opts);
    client.process_request(&input_file, &output_file, &url);
}
