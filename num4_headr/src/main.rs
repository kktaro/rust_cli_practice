fn main() {
    if let Err(e) = num4_headr::get_args().and_then(num4_headr::run) {
        eprintln!("{}", e);
        std::process::exit(1);
    }
}
