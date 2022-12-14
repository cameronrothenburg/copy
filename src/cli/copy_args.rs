pub struct CopyArgs {
    pub copy_to: String,
    pub copy_from: String,
}


pub fn parse_args(args: Vec<String>) -> Result<CopyArgs, &'static str> {
    if args.len() < 3 {
        panic!("Usage: <input> <output>")
    } else {
        Ok(CopyArgs {
            copy_to: args[2].clone(),
            copy_from: args[1].clone(),
        })
    }

}