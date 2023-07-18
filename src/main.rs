use std::{
    error::Error,
    io::{self, Read},
};

use clap::Parser;
use sprinkles::sprinklize;
use termcolor::{BufferWriter, ColorChoice};

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
#[command(about = "Randomly colors input text and outputs it.")]
struct Args {
    /// something warm
    #[arg(short, long, action, conflicts_with = "cold")]
    warm: bool,

    /// something refreshing
    #[arg(short, long, action, conflicts_with = "warm")]
    cold: bool,

    /// a bit brighter
    #[arg(short, long, action)]
    shiny: bool,

    /// BIG sprinkles
    #[arg(short, long, action)]
    bold: bool,

    /// slightly off
    #[arg(short, long, action)]
    italic: bool,

    /// only the good stuff
    #[arg(short, long, action)]
    no_white: bool,
}

fn read_text() -> Result<String, io::Error> {
    let mut buf = String::new();
    io::stdin().read_to_string(&mut buf)?;
    Ok(buf)
}

fn main() -> Result<(), Box<dyn Error>> {
    let args = Args::parse();
    let text = read_text()?;

    let writer = BufferWriter::stdout(ColorChoice::Always);
    let mut buffer = writer.buffer();

    // cold and warm args indicate "only", that's why we pass them with '!'
    sprinklize(
        &text,
        &mut buffer,
        !args.cold,
        !args.warm,
        args.shiny,
        args.bold,
        args.italic,
        !args.no_white,
    )?;

    writer.print(&buffer)?;
    Ok(())
}
