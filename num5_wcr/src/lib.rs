use std::{
    error::Error,
    fs::File,
    io::{self, BufRead, BufReader},
};

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
            Ok(file) => match count(file) {
                Err(err) => eprintln!("{}: {}", file_name, err),
                Ok(file_info) => println!(
                    "\t{}\t{}\t{} {}",
                    file_info.num_lines, file_info.num_words, file_info.num_chars, file_name
                ),
            },
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

#[derive(Debug, PartialEq)]
struct FileInfo {
    num_lines: usize,
    num_words: usize,
    num_bytes: usize,
    num_chars: usize,
}

fn count(mut file: impl BufRead) -> MyResult<FileInfo> {
    let mut num_lines = 0;
    let mut num_words = 0;
    let mut num_bytes = 0;
    let mut num_chars = 0;

    loop {
        let mut buf = String::new();
        let bytes = file.read_line(&mut buf)?;
        if bytes == 0 {
            // 読み取りバイト数が0の場合はEOF
            break;
        }

        num_lines += 1;
        num_words += buf.split_whitespace().count();
        num_bytes += bytes;
        num_chars += buf.chars().count();
    }

    Ok(FileInfo {
        num_lines,
        num_words,
        num_bytes,
        num_chars,
    })
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
    use std::io::Cursor;

    use anyhow::{Ok, Result};

    use super::*;

    #[test]
    fn args_verify() -> Result<()> {
        use clap::CommandFactory;
        Cli::command().debug_assert();
        Ok(())
    }

    fn generate_info(text: &str) -> FileInfo {
        let info = count(Cursor::new(text));
        assert!(info.is_ok());
        info.unwrap()
    }

    #[test]
    fn test_count() -> Result<()> {
        let info = generate_info("I don't want the world. I just want your half.\r\n");
        let expected = FileInfo {
            num_lines: 1,
            num_words: 10,
            num_chars: 48,
            num_bytes: 48,
        };
        assert_eq!(info, expected);
        Ok(())
    }

    #[test]
    fn test_count_empty() -> Result<()> {
        let info = generate_info("");
        let expected = FileInfo {
            num_lines: 0,
            num_words: 0,
            num_chars: 0,
            num_bytes: 0,
        };
        assert_eq!(info, expected);
        Ok(())
    }

    #[test]
    fn test_count_single_line() -> Result<()> {
        let info = generate_info("aaa");
        let expected = FileInfo {
            num_lines: 1,
            num_words: 1,
            num_chars: 3,
            num_bytes: 3,
        };
        assert_eq!(info, expected);
        Ok(())
    }

    #[test]
    fn test_count_fox() -> Result<()> {
        let info = generate_info(
            r"The  quick brown fox	jumps over   the lazy dog.
",
        );
        let expected = FileInfo {
            num_lines: 1,
            num_words: 9,
            num_chars: 48,
            num_bytes: 48,
        };
        assert_eq!(info, expected);
        Ok(())
    }
}
