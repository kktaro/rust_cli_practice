use std::{
    error::Error,
    fs::File,
    io::{self, BufRead, BufReader},
};

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
    number: bool,

    /// Is display number line if non blank.
    #[arg(short('b'), long, default_value_t = false)]
    number_nonblank: bool,
}

type MyResult<T> = Result<T, Box<dyn Error>>;

pub fn run(config: Config) -> MyResult<()> {
    for file_name in config.files {
        match open(&file_name) {
            Err(err) => eprintln!("Failed to open! {}: {}", file_name, err),
            Ok(buf_read) => {
                let mut line_count = 1;
                for line in buf_read.lines() {
                    let line_string = line.unwrap();
                    if config.number_nonblank_lines && line_string.is_empty() {
                        println!();
                        continue;
                    }
                    println!(
                        "{}{}",
                        if config.number_lines || config.number_nonblank_lines {
                            format!("{0: >6}\t", line_count)
                        } else {
                            "".to_string()
                        },
                        line_string
                    );
                    line_count += 1;
                }
            }
        }
    }
    Ok(())
}

pub fn get_args() -> MyResult<Config> {
    let args = Cli::parse();
    Ok(Config {
        files: args.files,
        number_lines: args.number,
        number_nonblank_lines: args.number_nonblank,
    })
}

fn open(file_name: &str) -> MyResult<Box<dyn BufRead>> {
    match file_name {
        "-" => Ok(Box::new(BufReader::new(io::stdin()))),
        _ => Ok(Box::new(BufReader::new(File::open(file_name)?))),
    }
}
