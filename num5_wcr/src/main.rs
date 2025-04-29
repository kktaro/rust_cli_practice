fn main() {
    if let Err(e) = num5_wcr::get_args().and_then(num5_wcr::run) {
        eprintln!("{}", e);
        std::process::exit(1);
    }
}
