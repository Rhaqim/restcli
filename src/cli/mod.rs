use clap::Parser;

use crate::cmd;

#[derive(Parser, Debug)]
#[clap(name = "restcli", version = "0.1.0", author = "Rhaqim")]
#[clap(about = "Generate RESTful API endpoints")]
pub struct Opts {
    #[clap(flatten)]
    pub options: RestCLIOptions,
}

#[derive(Parser, Debug, Clone)]
pub struct RestCLIOptions {
    #[clap(help = "Input file containing the request details", required = true)]
    pub file: String,

    #[clap(
        short,
        long = "curl",
        default_value = "false",
        help = "Generate curl requests"
    )]
    pub curl: bool,

    #[clap(
        short,
        long = "postman",
        default_value = "false",
        help = "Create Postman collection"
    )]
    pub postman: bool,

    #[clap(
        short,
        long = "rest_client",
        default_value = "false",
        help = "User rest-client for the requests"
    )]
    pub rest_client: bool,

    #[clap(long = "port", default_value = "8080", help = "Port number")]
    pub port: u16,

    #[clap(long = "url", default_value = "http://localhost", help = "Base URL")]
    pub url: String,

    #[clap(
        short,
        long = "output",
        default_value = "request",
        help = "Output file name"
    )]
    pub output: String,
}

pub fn parse() {
    let opts = Opts::parse();

    let url = format!("{}:{}", opts.options.url, opts.options.port);

    // Handle input file
    let input_file = opts.options.file;
    println!("Using input file: {}", input_file);

    let mut output_file = format! {"{}.sh", opts.options.output};

    // Display the selected client and URL/Port details
    if opts.options.postman {
        println!("Using Postman for the request...");
        output_file = format! {"{}.json", opts.options.output};
    } else if opts.options.rest_client {
        println!("Using rest-client for the request...");
        output_file = format! {"{}.http", opts.options.output};

        cmd::rest_client::rest_client_processor(&input_file, &output_file, &url)
            .expect("Unable to process rest-client");
    } else {
        println!("Using curl for the request...");
    }

    println!("Using output file: {}", output_file);
}
