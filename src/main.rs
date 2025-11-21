use clap::Parser;
use gfy::Converter;
use std::error::Error;

#[derive(Parser, Debug)]
#[command(version, about, long_about=None)]
struct Args {
    #[arg(short = 'f')]
    input_file: String,

    #[arg(short = 'o')]
    output_file: String,
}

fn main() {
    let args = Args::parse();
    run(&args).unwrap_or_else(|err| eprintln!("Failed to convert image to ascii: {err}"))
}

fn run(args: &Args) -> Result<(), Box<dyn Error>> {
    Converter::load_image(&args.input_file)?
        .convert_to_ascii()?
        .write_file(&args.output_file)
}
