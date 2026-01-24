use clap::Parser;
use std::io::{BufRead, BufReader};
use std::fs::File;
use std::ops::Add;

#[derive(Debug, clap::Parser)]
pub struct Config {
    #[arg(help = "Input files", default_value = "-")]
    files: Vec<String>,
    #[arg(short = 'l', long = "lines", help = "Count lines")]
    lines: bool,
    #[arg(short = 'w', long = "words", help = "Count words")]
    words: bool,
    #[arg(short = 'c', long = "bytes", help = "Count bytes")]
    bytes: bool,
    #[arg(short = 'm', long = "chars", help = "Count characters")]
    chars: bool,
}

#[derive(Debug, Clone)]
pub struct FileInfo {
    pub lines: usize,
    pub words: usize,
    pub bytes: usize,
    pub chars: usize,
}

impl Config {
    fn is_default(&self) -> bool {
        !self.lines && !self.words && !self.bytes && !self.chars
    }
}

impl Add for FileInfo {
    type Output = FileInfo;

    fn add(self, other: FileInfo) -> FileInfo {
        FileInfo {
            lines: self.lines + other.lines,
            words: self.words + other.words,
            bytes: self.bytes + other.bytes,
            chars: self.chars + other.chars,
        }
    }
}

pub fn parse_args() -> Config{
    Config::parse()
}

pub fn count(mut content: impl BufRead) -> FileInfo {
    let text = std::io::read_to_string(&mut content).unwrap();
    let lines = text.lines().count();
    let words = text.split_whitespace().count();
    let bytes = text.len();
    let chars = text.chars().count();

    FileInfo {
        lines,
        words,
        bytes,
        chars,
    }
}

pub fn run(config: Config) -> Result<(), Box<dyn std::error::Error>> {
    let mut total = FileInfo {
        lines: 0,
        words: 0,
        bytes: 0,
        chars: 0,
    };

    for filename in &config.files {
        let info: FileInfo;
        if filename == "-" {
            info = count(BufReader::new(std::io::stdin()));
        } else {
            let file = File::open(filename)?;
            let mut reader = BufReader::new(file);
            info = count(&mut reader);
        }
        total = total + info.clone();
        println!("{}", format_output(&info, &config, filename));
    }

    if config.files.len() > 1 {
        println!("{}", format_output(&total, &config, "total"));
    }

    Ok(())
}

fn format_output(info: &FileInfo, config: &Config, filename: &str) -> String {
    let mut parts = Vec::new();

    if config.is_default() {
        parts.push(format!("{:>8}", info.lines));
        parts.push(format!("{:>8}", info.words));
        parts.push(format!("{:>8}", info.bytes));
        parts.push(format!("{}", filename));
    } else {
        if config.lines {
            parts.push(format!("{:>8}", info.lines));
        }
        if config.words {
            parts.push(format!("{:>8}", info.words));
        }
        if config.bytes {
            parts.push(format!("{:>8}", info.bytes));
        }
        if config.chars {
            parts.push(format!("{:>8}", info.chars));
        }
        parts.push(format!("{}", filename));
    }

    parts.join(" ")
}
