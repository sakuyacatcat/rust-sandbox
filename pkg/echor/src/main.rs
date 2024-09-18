use clap::{Command, Arg, ArgAction};

fn main() {
    let matches = Command::new("echor")
        .version("0.1.0")
        .author("sakuyacatcat")
        .about("Rust echo")
        .arg(
            Arg::new("text")
                .value_name("TEXT")
                .help("Input text")
                .required(true)
                .num_args(1..),
        )
        .arg(
            Arg::new("omit_newline")
                .short('n')
                .help("Do not print newline")
                .action(ArgAction::SetTrue),
        )
        .get_matches();

    let text: Vec<String> = matches.get_many::<String>("text").unwrap().map(|v| v.to_owned()).collect();
    let omit_newline = matches.get_flag("omit_newline");

    let ending = if omit_newline { "" } else { "\n" };
    println!("{}{}", text.join(" "), ending);
}
