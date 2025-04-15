use clap::Parser;

#[derive(Parser, Debug)]
#[command(version)]
#[command(about = "Rust echo")]
struct Cli {
    /// Input text
    #[arg(required = true, num_args = 1..)]
    text: Vec<String>,

    #[arg(short, num_args = 0)]
    /// Do not print newline
    newline: bool,
}

fn main() {
    let args = Cli::parse();
    println!("{:#?}", args);
}
