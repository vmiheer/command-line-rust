use clap::{App, Arg};
use std::error::Error;
use std::fs::File;
use std::io::{BufRead, BufReader};

type MyResult<T> = Result<T, Box<dyn Error>>;

#[derive(Debug)]
pub struct Config {
    files: Vec<String>,
    line_numbers: bool,
    line_numbers_no_blanks: bool,
}

pub fn get_args() -> MyResult<Config> {
    let matches = App::new("catr")
        .version("0.0.1")
        .about("Rust cat")
        .arg(
            Arg::with_name("number")
                .short("n")
                .long("number")
                .takes_value(false)
                .help("Print line numbers"),
        )
        .arg(
            Arg::with_name("number-nonblank")
                .short("b")
                .long("number-nonblank")
                .takes_value(false)
                .help("Print line numbers for non-blank lines")
                .conflicts_with("number"),
        )
        .arg(
            Arg::with_name("files")
                .help("Input files(s)")
                .multiple(true)
                .value_name("FILE")
                .default_value("-"),
        )
        .get_matches();
    let config = Config {
        files: matches.values_of_lossy("files").unwrap(),
        line_numbers: matches.is_present("number"),
        line_numbers_no_blanks: matches.is_present("number-nonblank"),
    };
    Ok(config)
}

fn open(filename: &str) -> MyResult<Box<dyn BufRead>> {
    match filename {
        "-" => Ok(Box::new(BufReader::new(std::io::stdin()))),
        _ => Ok(Box::new(BufReader::new(File::open(filename)?))),
    }
}

pub fn run(config: Config) -> MyResult<()> {
    let mut line_number: u32 = 1;
    for f in config.files {
        match open(&f) {
            Ok(file_handle) =>
            //format!("{}",
            {
                for l in file_handle.lines().map(|l| l.unwrap()) {
                    if config.line_numbers {
                        println!("{:6}\t{}", line_number, l);
                        line_number += 1;
                    } else if config.line_numbers_no_blanks {
                        if l.is_empty() {
                            println!("")
                        } else {
                            println!("{:6}\t{}", line_number, l);
                            line_number += 1;
                        }
                    } else {
                        println!("{}", l);
                    }
                }
            }
            Err(e) => {
                eprintln!("Filed to open {}: {}", f, e)
            }
        };
    }
    Ok(())
}
