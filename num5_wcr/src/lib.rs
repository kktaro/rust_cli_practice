use std::{error::Error, fs::File, io::{self, BufRead, BufReader}};

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

    let is_args_empty = [
        cli.lines,
        cli.words,
        cli.count_type.bytes,
        cli.count_type.chars,
    ]
    .iter()
    .all(|v| !v);

    let (lines, words, bytes, chars) = if is_args_empty {
        (true, true, true, false)
    } else {
        (
            cli.lines,
            cli.words,
            cli.count_type.bytes,
            cli.count_type.chars,
        )
    };

    Ok(Config {
        files: cli.files,
        lines,
        words,
        bytes,
        chars,
    })
}

pub fn run(config: Config) -> MyResult<()> {
    for file_name in config.files {
        match open(&file_name) {
            Err(err) => eprintln!("{}: {}", file_name, err),
            Ok(_) => println!("Opened! {}", file_name),
        }
    }
    Ok(())
}

fn open(file_name: &str) -> MyResult<Box<dyn BufRead>> {
    match file_name {
        "-" => Ok(Box::new(BufReader::new(io::stdin()))),
        _ => Ok(Box::new(BufReader::new(File::open(file_name)?))),
    }
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
    #[arg(short, long)]
    lines: bool,

    /// Show word count
    #[arg(short, long)]
    words: bool,
}

#[derive(Args, Debug)]
#[group(multiple = false)]
struct CountType {
    /// Show byte count
    #[arg(short('c'), long)]
    bytes: bool,

    /// Show character count
    #[arg(short('m'), long)]
    chars: bool,
}

#[cfg(test)]
mod test {
    use anyhow::{Ok, Result};

    use super::*;

    #[test]
    fn args_verify() -> Result<()> {
        use clap::CommandFactory;
        Cli::command().debug_assert();
        Ok(())
    }
}
