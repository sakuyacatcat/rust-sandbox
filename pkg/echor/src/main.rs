use echor::{build_cli, run};

fn main() {
    let matches = build_cli().get_matches();

    let text: Vec<String> = matches.get_many("text").unwrap().cloned().collect();
    let omit_newline = matches.get_flag("omit_newline");

    print!("{}", run(text, omit_newline));
}
