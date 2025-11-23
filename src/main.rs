use ab_glyph::FontRef;
use clap::Parser;
use gfy::Converter;
use std::error::Error;

const FONT_DATA: &[u8] = include_bytes!("../assets/DejaVuSansMono.ttf");

#[derive(Parser, Debug)]
#[command(version, about, long_about=None)]
struct Args {
    #[arg(short = 'f')]
    input_file: String,

    #[arg(short = 'o')]
    output_file: String,

    #[arg(short, long, default_value_t = 250)]
    char_width: u32,
}

fn main() {
    let args = Args::parse();
    run(&args).unwrap_or_else(|err| eprintln!("Failed to convert image to ascii: {err}"))
}

fn run(args: &Args) -> Result<(), Box<dyn Error>> {
    // TODO: improve errors
    let font = FontRef::try_from_slice(FONT_DATA)?;
    Converter::load_image(&args.input_file)?
        .downsample(args.char_width)?
        .grayscale()?
        .to_ascii(&font)?
        .save(&args.output_file)
}
