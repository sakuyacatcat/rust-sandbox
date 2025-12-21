use clap::{Command, Arg};

pub fn build_cli() -> Command {
    Command::new("echor")
        .version("1.0")
        .author("Satoshi Nakamoto <hogehoge@example.com>")
        .about("A simple echo program written in Rust")
        .arg(
            Arg::new("text")
                .required(true)
                .num_args(1..)
                .help("Input text"),
        )
        .arg(
            Arg::new("omit_newline")
                .short('n')
                .action(clap::ArgAction::SetTrue)
                .help("Do not print newline"),
        )
}

pub fn run(text: Vec<String>, omit_newline: bool) -> String {
    let ending = if omit_newline { "" } else { "\n" };
    format!("{}{}", text.join(" "), ending)
}

#[cfg(test)]
mod tests {
    use super::*;
    use rstest::rstest;

    #[rstest]
    #[case(vec!["Hello,", "world!"], false, "Hello, world!\n")]
    #[case(vec!["Hello,", "world!"], true, "Hello, world!")]
    #[case(vec!["This", "is", "a", "test."], false, "This is a test.\n")]
    fn test_run(#[case] input: Vec<&str>, #[case] omit_newline: bool, #[case] expected: &str) {
        let input_strings: Vec<String> = input.iter().map(|s| s.to_string()).collect();
        let result = run(input_strings, omit_newline);
        assert_eq!(result, expected);
    }
}
