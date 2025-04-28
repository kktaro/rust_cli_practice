use std::{error::Error, f32::consts::E, ptr::null};

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
    println!("{:#?}", config);
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
