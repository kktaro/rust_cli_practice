use std::error::Error;

use clap::{Args, Parser};

type MyResult<T> = Result<T, Box<dyn Error>>;

#[derive(Debug)]
#[allow(dead_code)]
pub struct Config {
    files: Vec<String>,
    lines: bool,
    words: bool,
    bytes: bool,
    chars: bool,
}

pub fn get_args() -> MyResult<Config> {
    let cli = Cli::parse();
    Ok(Config {
        files: cli.files,
        lines: cli.lines,
        words: cli.words,
        bytes: cli.count_type.bytes,
        chars: cli.count_type.chars,
    })
}

pub fn run(config: Config) -> MyResult<()> {
    dbg!(config);
    Ok(())
}

#[derive(Parser, Debug)]
#[command(version)]
#[command(about = "Rust wc")]
struct Cli {
    /// Input file(s)
    #[arg(num_args = 1.., default_values_t = ["-".to_string()])]
    files: Vec<String>,

    #[command(flatten)]
    count_type: CountType,

    /// Show line count
    #[arg(short, long, default_value_t = true)]
    lines: bool,

    /// Show word count
    #[arg(short, long, default_value_t = true)]
    words: bool,
}

#[derive(Args, Debug)]
#[group(multiple = false)]
struct CountType {
    /// Show byte count
    #[arg(short('c'), long, default_value_t = true)]
    bytes: bool,

    /// Show character count
    #[arg(short('m'), long, default_value_t = false)]
    chars: bool,
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn args_verify() {
        use clap::CommandFactory;
        Cli::command().debug_assert()
    }
}
