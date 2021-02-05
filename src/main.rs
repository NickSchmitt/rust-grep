#![feature(option_result_contains)]

use structopt::StructOpt;
use std::io::prelude::*;
use std::io::BufReader;
use std::fs::File;
use std::io::Result;
#[derive(StructOpt, Debug)]
struct Cli {
    /// The pattern to look for
    pattern: String,
    /// The path to the file to read
    #[structopt(parse(from_os_str))]
    path: std::path::PathBuf,
}


fn main() -> Result<()> {
    let args = Cli::from_args();
    let content = File::open(&args.path).expect("Unable to open");
    let reader = BufReader::new(content);
    
    for line in reader.lines() {
        if line.contains(&args.pattern) {
            println!("{:?}", line);
        }
    }
    Ok(())
}

