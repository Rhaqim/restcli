use clap::Parser;

use super::Opts;

use crate::cmd;
use crate::utils::is_supported_extension;

pub fn parse() {
    let opts = Opts::parse();

    let input_files = opts.options.file.clone();

    // check if file is supported
    if !is_supported_extension(&input_files) {
        eprintln!("File extension not supported, supported extensions are: go, rs, py");
        std::process::exit(1);
    }

    let url = format!("{}:{}", opts.options.url.clone(), opts.options.port.clone());

    let client = cmd::get_client(&opts);

    for (c, output_file) in client {
        c.process_request(&input_files, &url, &output_file, opts.options.append);
    }
}
