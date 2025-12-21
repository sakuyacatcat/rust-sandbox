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
