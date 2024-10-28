use std::error::Error;
use std::fs::File;
use std::io::prelude::*;

#[derive(Debug)]
pub struct Config {
    pub query: String,
    pub filename: String,
}

impl Config {
    pub fn new(args: &[String]) -> Result<Config, &'static str> {
        if args.len() < 3 {
            return Err("not enough arguments");
        }
        let query = args[1].clone();
        let filename = args[2].clone();

        Ok(Config {query, filename})
    }
}

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let mut f = File::open(config.filename)?;

    let mut contents = String::new();
    f.read_to_string(&mut contents)?;

    println!("With text:\n{}", contents);

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn config_new_with_two_args() {
        let args = vec![String::from("minigrep"), String::from("query"), String::from("filename")];
        let config = Config::new(&args).unwrap();
        assert_eq!(config.query, "query");
        assert_eq!(config.filename, "filename");
    }

    #[test]
    fn config_new_with_one_arg() {
        let args = vec![String::from("minigrep"), String::from("query")];
        let result = Config::new(&args);
        assert!(result.is_err());
        assert_eq!(result.unwrap_err(), "not enough arguments");
    }

    #[test]
    fn config_new_with_no_args() {
        let args = vec![String::from("minigrep")];
        let result = Config::new(&args);
        assert!(result.is_err());
        assert_eq!(result.unwrap_err(), "not enough arguments");
    }
}
