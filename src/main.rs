mod cli;

use crate::cli::cli::Cli;
use std::env;

fn main() {
    let mut cli = Cli::new(env::args().collect());
    cli.execute()
}
