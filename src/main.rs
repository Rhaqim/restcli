extern crate tokio;

mod cli;
mod cmd;
mod lang;
mod utils;

#[tokio::main]
async fn main() {
    cli::parse();
}
