use clap::Parser;

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
