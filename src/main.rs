extern crate tokio;

mod cli;
mod cmd;
mod utils;

#[tokio::main]
async fn main() {
    cli::parse();
}
