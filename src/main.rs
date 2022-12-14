mod cli;



use std::env;
use crate::cli::cli::Cli;


fn main() {

   let mut cli =  Cli::new(env::args().collect());
   cli.execute()
}
