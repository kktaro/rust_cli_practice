fn main() {
    if let Err(e) = num3_catr::get_args().and_then(num3_catr::run) {
        eprintln!("{}", e);
        std::process::exit(1);
    }
}
