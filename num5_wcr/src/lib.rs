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
    let has_arg = cli.count_type.is_some() || cli.lines.is_some() || cli.words.is_some();

    if has_arg {
        let count_type = cli.count_type.unwrap_or(CountType {
            bytes: Some(false),
            chars: Some(false),
        });
        Ok(Config {
            files: cli.files,
            lines: cli.lines.unwrap_or(false),
            words: cli.words.unwrap_or(false),
            bytes: count_type.bytes.unwrap_or(false),
            chars: count_type.chars.unwrap_or(false),
        })
    } else {
        // オプションが指定なしの場合はデフォルト値を渡す
        Ok(Config {
            files: cli.files,
            lines: true,
            words: true,
            bytes: true,
            chars: false,
        })
    }
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
    count_type: Option<CountType>,

    /// Show line count
    #[arg(short, long)]
    lines: Option<bool>,

    /// Show word count
    #[arg(short, long)]
    words: Option<bool>,
}

#[derive(Args, Debug)]
#[group(multiple = false)]
struct CountType {
    /// Show byte count
    #[arg(short('c'), long)]
    bytes: Option<bool>,

    /// Show character count
    #[arg(short('m'), long)]
    chars: Option<bool>,
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

    // TODO: Cli => Configするテストを書く
}
