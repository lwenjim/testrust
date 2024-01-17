#![allow(unused)]
use std::env;
use structopt::StructOpt;

fn main() {
    #[derive(Debug)]
    // #[allow(dead_code)]
    struct Cli {
        pattern: String,
        path: std::path::PathBuf,
    }
    let pattern = env::args().nth(1).expect("no pattern given");
    let path = env::args().nth(1).expect("no path given");
    let args = Cli {
        pattern,
        path: std::path::PathBuf::from(path),
    };
    println!("{:?}", args);

    #[derive(Debug, StructOpt)]
    // #[allow(dead_code)]
    struct Cli2 {
        #[structopt(short = "t", long = "pattern")]
        pattern: String,
        #[structopt(parse(from_os_str))]
        #[structopt(short = "p", long = "path")]
        path: std::path::PathBuf,
    }
    let args = Cli2::from_args();
    println!("{:?}", args);
}
