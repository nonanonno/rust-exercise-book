use clap::{App, Arg};
use std::error::Error;
use std::fs::File;
use std::io::{self, BufRead, BufReader};

#[derive(Debug)]
pub struct Config {
    files: Vec<String>,
    number_lines: bool,
    number_nonblank_lines: bool,
}

type MyResult<T> = Result<T, Box<dyn Error>>;

fn open(filename: &str) -> MyResult<Box<dyn BufRead>> {
    match filename {
        "-" => Ok(Box::new(BufReader::new(io::stdin()))),
        _ => Ok(Box::new(BufReader::new(File::open(filename)?))),
    }
}

pub fn get_args() -> MyResult<Config> {
    let matches = App::new("catr")
        .version("0.1.0")
        .author("nonanonno <as.nonanonno@gmail.com>")
        .about("rust cat")
        .arg(
            Arg::with_name("files")
                .value_name("FILE")
                .multiple(true)
                .help("Files to read")
                .default_value("-"),
        )
        .arg(
            Arg::with_name("number_lines")
                .short("n")
                .long("number")
                .help("Number of output lines")
                .takes_value(false)
                .conflicts_with("number_nonblank_lines"),
        )
        .arg(
            Arg::with_name("number_nonblank_lines")
                .short("b")
                .long("number-nonblank")
                .help("Number of non-blank output lines")
                .takes_value(false),
        )
        .get_matches();

    Ok(Config {
        files: matches.values_of_lossy("files").unwrap(),
        number_lines: matches.is_present("number_lines"),
        number_nonblank_lines: matches.is_present("number_nonblank_lines"),
    })
}

pub fn run(config: Config) -> MyResult<()> {
    for filename in config.files {
        match open(&filename) {
            Err(err) => eprintln!("{}: {}", filename, err),
            Ok(buf) => {
                let mut number = 1;

                for line in buf.lines() {
                    let line = line?;
                    if config.number_nonblank_lines && line.trim().is_empty() {
                        println!("{}", line);
                        continue;
                    }
                    let head = if config.number_lines || config.number_nonblank_lines {
                        format!("{:>6}\t", number)
                    } else {
                        String::new()
                    };
                    println!("{}{}", head, line);
                    number += 1;
                }
            }
        }
    }
    Ok(())
}
