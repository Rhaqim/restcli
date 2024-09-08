mod options;
mod parser;

pub use options::RestCLIOptions;
pub use parser::parse;

use clap::Parser;

#[derive(Parser, Debug)]
#[clap(name = "restcli", version = "0.1.0", author = "Rhaqim")]
#[clap(about = "Generate RESTful API endpoints")]
pub struct Opts {
    #[clap(flatten)]
    pub options: RestCLIOptions,
}
