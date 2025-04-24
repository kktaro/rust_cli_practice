use std::error::Error;

use clap::Parser;

type MyResult<T> = Result<T, Box<dyn Error>>;

#[derive(Debug)]
pub struct Config {
    files: Vec<String>,
    lines: usize,
    bytes: Option<usize>,
}

pub fn get_args() -> MyResult<Config> {
    let args = Args::parse();
    Ok(Config {
        files: args.files,
        lines: args.lines,
        bytes: args.bytes,
    })
}

pub fn run(config: Config) -> MyResult<()> {
    println!("{:#?}", config);
    Ok(())
}

#[derive(Parser, Debug)]
#[command(version)]
#[command(about = "Rust head")]
struct Args {
    /// Target files.
    #[arg(num_args = 1.., default_values_t = ["-".to_string()])]
    files: Vec<String>,

    /// Is display number line.
    #[arg(short('n'), long, default_value_t = 10)]
    lines: usize,

    /// Is display number line if non blank.
    #[arg(short('c'), long)]
    bytes: Option<usize>,
}
