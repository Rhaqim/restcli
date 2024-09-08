extern crate tokio;

mod cli;

#[tokio::main]
async fn main() {
    cli::restcli::parse();
}
