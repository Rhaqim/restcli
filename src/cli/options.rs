use clap::Parser;

#[derive(Parser, Debug, Clone)]
pub struct RestCLIOptions {
    #[clap(help = "Input files containing the request details", required = true)]
    pub file: Vec<String>,

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
        long = "apppend",
        default_value = "false",
        help = "Append to the output file"
    )]
    pub append: bool,

    #[clap(
        short,
        long = "output",
        default_value = "request",
        help = "Output file name"
    )]
    pub output: String,
}
