fn main() {
    if let Err(e) = catr::parse_args().and_then(catr::run) {
        eprintln!("{}", e);
        std::process::exit(1);
    }
}
