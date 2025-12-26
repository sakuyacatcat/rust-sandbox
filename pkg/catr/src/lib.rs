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
                .action(clap::ArgAction::SetTrue)
                .help("Number all output lines")
                .conflicts_with("number_nonblank_lines"),
        )
        .arg(
            Arg::new("number_nonblank_lines")
                .short('b')
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
        println!("{}", filename);
    }
    Ok(())
}
