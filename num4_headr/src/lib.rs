use std::{
    error::Error,
    fs::File,
    io::{self, BufRead, BufReader},
};

use clap::{ArgGroup, Parser};

type MyResult<T> = Result<T, Box<dyn Error>>;

#[derive(Debug)]
#[allow(dead_code)]
pub struct Config {
    files: Vec<String>,
    lines: usize,
    bytes: Option<usize>,
}

pub fn get_args() -> MyResult<Config> {
    let args = Cli::parse();
    Ok(Config {
        files: args.files,
        lines: args.lines,
        bytes: args.bytes,
    })
}

pub fn run(config: Config) -> MyResult<()> {
    let mut is_first_file = true;
    let is_multiple_line = config.files.len() > 1;

    for file_name in config.files {
        match open(&file_name) {
            Err(err) => eprintln!("{}: {}", file_name, err),
            Ok(mut read_buf) => {
                // 二つ目以降のファイル出力の場合は空行を空ける
                if is_first_file {
                    is_first_file = false;
                } else {
                    println!();
                }

                if is_multiple_line {
                    println!("==> {} <==", file_name);
                }
                for _ in 0..config.lines {
                    let mut line = String::new();
                    let bytes = read_buf.read_line(&mut line)?;

                    if bytes == 0 {
                        break;
                    }
                    print!("{}", line);
                }
            }
        }
    }
    Ok(())
}

#[derive(Parser, Debug)]
#[command(version)]
#[command(about = "Rust head")]
#[command(group(ArgGroup::new("inputs").args(["lines", "bytes"])))]
struct Cli {
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

#[allow(dead_code)]
fn parse_positive_int(val: &str) -> MyResult<usize> {
    match val.parse() {
        Ok(n) if n > 0 => Ok(n),
        _ => Err(From::from(val)),
    }
}

fn open(file_name: &str) -> MyResult<Box<dyn BufRead>> {
    match file_name {
        "-" => Ok(Box::new(BufReader::new(io::stdin()))),
        _ => Ok(Box::new(BufReader::new(File::open(file_name)?))),
    }
}

#[test]
fn test_parse_positive_int() {
    let res_positive = parse_positive_int("3");
    assert!(res_positive.is_ok());
    assert_eq!(res_positive.unwrap(), 3);

    let res_string = parse_positive_int("foo");
    assert!(res_string.is_err());
    assert_eq!(res_string.unwrap_err().to_string(), "foo".to_string());

    let res_zero = parse_positive_int("0");
    assert!(res_zero.is_err());
    assert_eq!(res_zero.unwrap_err().to_string(), "0".to_string());
}
