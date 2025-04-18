use std::error::Error;

use clap::Parser;

#[derive(Debug)]
pub struct Config {
    files: Vec<String>,
    number_lines: bool,
    number_nonblank_lines: bool,
}

#[derive(Parser, Debug)]
#[command(version)]
#[command(about = "Rust cat")]
struct Cli {
    /// Target files.
    #[arg(num_args = 1.., default_values_t = ["-".to_string()])]
    files: Vec<String>,

    /// Is display number line.
    #[arg(short, long, default_value_t = false)]
    number_lines: bool,

    /// Is display number line if non blank.
    #[arg(short('b'), long, default_value_t = false)]
    number_nonblank_lines: bool,
}

type MyResult<T> = Result<T, Box<dyn Error>>;

pub fn run(config: Config) -> MyResult<()> {
    dbg!(config);
    Ok(())
}

pub fn get_args() -> MyResult<Config> {
    let args = Cli::parse();
    Ok(Config {
        files: args.files,
        number_lines: args.number_lines,
        number_nonblank_lines: args.number_nonblank_lines,
    })
}
