use clap::{Command, Arg};
use std::error::Error;
use std::fs::File;
use std::io::{self, BufRead, BufReader};

#[derive(Debug)]
pub struct Config {
    files: Vec<String>,
    number_lines: bool,
    number_nonblank_lines: bool,
}

pub fn parse_args() -> MyResult<Config> {
    let matches = Command::new("catr")
        .version("0.1.0")
        .author("ss <hoge@example.com>")
        .about("Rust cat command")
        .arg(
            Arg::new("files")
                .help("Input files")
                .num_args(0..)
                .default_value("-"),
        )
        .arg(
            Arg::new("number_lines")
                .short('n')
                .long("number")
                .action(clap::ArgAction::SetTrue)
                .help("Number all output lines")
                .conflicts_with("number_nonblank_lines"),
        )
        .arg(
            Arg::new("number_nonblank_lines")
                .short('b')
                .long("number-nonblank")
                .action(clap::ArgAction::SetTrue)
                .help("Number nonempty output lines")
                .conflicts_with("number_lines"),
        )
        .get_matches();

    Ok(Config {
        files: matches.get_many("files").unwrap().cloned().collect(),
        number_lines: matches.get_one("number_lines").copied().unwrap_or(false),
        number_nonblank_lines: matches.get_one("number_nonblank_lines").copied().unwrap_or(false),
    })
}

type MyResult<T> = Result<T, Box<dyn Error>>;

pub fn run(config: Config) -> MyResult<()> {
    for filename in config.files {
        match open(&filename) {
            Err(e) => eprintln!("Failed to open {}: {}", filename, e),
            Ok(file) => {
                let mut last_num = 0;
                for (line_num, line) in file.lines().enumerate() {
                    let line = line?;

                    if config.number_lines {
                        println!("{:>6}\t{}", line_num + 1, line);
                    } else if config.number_nonblank_lines {
                        if !line.is_empty() {
                            last_num += 1;
                            println!("{:>6}\t{}", last_num, line);
                        } else {
                            println!();
                        }
                    } else {
                        println!("{}", line);
                    }
                }
            }
        }
    }
    Ok(())
}

fn open(filename: &str) -> MyResult<Box<dyn BufRead>> {
    match filename {
        // "-" means standard input
        "-" => Ok(Box::new(BufReader::new(io::stdin()))),
        // _ means wildcard (match anything)
        _ => Ok(Box::new(BufReader::new(File::open(filename)?)))
    }
}
