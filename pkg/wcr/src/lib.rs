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

#[derive(Debug, Clone, PartialEq, Default)]
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
    let mut lines = 0;
    let mut words = 0;
    let mut bytes = 0;
    let mut chars = 0;

    let mut line = String::new();

    while content.read_line(&mut line).unwrap() > 0 {
        lines += 1;
        words += line.split_whitespace().count();
        bytes += line.len();
        chars += line.chars().count();
        line.clear();
    }

    FileInfo {
        lines,
        words,
        bytes,
        chars,
    }
}

pub fn run(config: Config) -> Result<(), Box<dyn std::error::Error>> {
    let mut total = FileInfo::default();

    for filename in &config.files {
        match open(filename) {
            Ok(reader) => {
                let info: FileInfo = count(reader);
                total = total + info.clone();
                println!("{}", format_output(&info, &config, filename));
            }
            Err(e) => {
                eprintln!("Failed to open {}: {}", filename, e);
            }
        }
    }

    if config.files.len() > 1 {
        println!("{}", format_output(&total, &config, "total"));
    }

    Ok(())
}

fn open(filename: &str) -> Result<Box<dyn BufRead>, Box<dyn std::error::Error>> {
    match filename {
        // "-" means standard input
        "-" => Ok(Box::new(BufReader::new(std::io::stdin()))),
        // _ means wildcard (match anything)
        _ => Ok(Box::new(BufReader::new(File::open(filename)?))),
    }
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

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::Cursor;

    #[test]
    fn test_count_simple() {
        let input = "Hello World\nThis is a test.\n";
        let cursor = Cursor::new(input);

        let info = count(cursor);

        assert_eq!(info, FileInfo {
            lines: 2,
            words: 6,
            bytes: 28,
            chars: 28,
        });
    }

    #[test]
    fn test_count_multibyte() {
        let input = "こんにちは 世界\nこれはテストです。\n";
        let cursor = Cursor::new(input);

        let info = count(cursor);

        assert_eq!(info, FileInfo {
            lines: 2,
            words: 3,
            bytes: 51,
            chars: 19,
        });
    }
}
