#![allow(unused)]
use std::{error::Error};


// cargo run --quiet -- --path 123 --pattern abc
fn main()  -> Result<(), Box<dyn Error>>{
    use std::env;
    use structopt::StructOpt;
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

    use regex::Regex;
    let result = std::fs::read_to_string("aa.txt");
    match result {
        Err(error) => {
            panic!("err: {}", error)
        }
        Ok(content) => {
            println!("content: {}", content)
        }
    }
    let re = Regex::new(r"^\d{4}-\d{2}-\d{2}$").unwrap();
    println!("Did our date match? {}", re.is_match("2014-01-01"));

    use std::{thread, time::Duration};
    ctrlc::set_handler(move || {
        println!("received Ctrl+C!");
    })
    .expect("Error setting Ctrl-C handler");
    thread::sleep(Duration::from_secs(2));

    use signal_hook::{consts::SIGINT, iterator::Signals};
    use std::{error::Error};
    let mut signals = Signals::new(&[SIGINT])?;
    thread::spawn(move || {
        for sig in signals.forever() {
            println!("Received signal {:?}", sig);
        }
    });
    thread::sleep(Duration::from_secs(2));
    Ok(())
}
