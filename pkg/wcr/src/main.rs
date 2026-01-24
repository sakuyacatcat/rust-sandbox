fn main() {
    let config = wcr::parse_args();
    if let Err(e) = wcr::run(config) {
        eprintln!("{}", e);
        std::process::exit(1);
    }
}
