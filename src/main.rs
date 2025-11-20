use clap::Parser;
use gfy::Converter;
use std::io::Error;

#[derive(Parser, Debug)]
#[command(version, about, long_about=None)]
struct Args {
    #[arg(short)]
    file_name: String,
}

fn main() {
    let args = Args::parse();
    run(&args.file_name).unwrap_or_else(|err| eprintln!("Failed to convert image to ascii: {err}"))
}

fn run(file_name: &str) -> Result<(), Error> {
    Converter::load_image(file_name)?
        .convert_to_ascii()?
        .print()
}
