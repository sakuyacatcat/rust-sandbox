use clap::{Command, Arg};

fn main() {
    let matches = Command::new("echor")
        .version("1.0")
        .author("Satoshi Nakamoto <hogehoge@example.com>")
        .about("A simple echo program written in Rust")
        .arg(
            Arg::new("text")
                .required(true)
                .num_args(1..)
                .help("Input text")
        )
        .arg(
            Arg::new("omit_newline")
                .short('n')
                .action(clap::ArgAction::SetTrue)
                .help("Do not print newline")
        )
        .get_matches();

    let text: Vec<String> = matches.get_many("text").unwrap().cloned().collect();
    let omit_newline = matches.get_flag("omit_newline");

    print!("{}{}", text.join(" "), if omit_newline { "" } else { "\n" });
}
