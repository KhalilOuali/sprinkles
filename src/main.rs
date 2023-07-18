use std::{
    error::Error,
    io::{self, Read, Write},
};

use clap::Parser;
use sprinkles::sprinklize;

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// Gimme BIG sprinkles
    #[arg(short = 'g', long, action)]
    bold: bool,

    /// More on the side
    #[arg(short, long, action)]
    italic: bool,

    /// Make em tilt
    #[arg(short, long, action, conflicts_with = "bright")]
    normal: bool,

    /// Ooh, shiny
    #[arg(short, long, action, conflicts_with = "normal")]
    bright: bool,

    /// I'd like a cup of hot cocoa
    #[arg(short, long, action, conflicts_with = "cold")]
    warm: bool,

    /// I need something refreshing
    #[arg(short, long, action, conflicts_with = "warm")]
    cold: bool,
}

fn read_text() -> Result<String, io::Error> {
    let mut buf = String::new();
    io::stdin().read_to_string(&mut buf)?;
    Ok(buf)
}

fn main() -> Result<(), Box<dyn Error>> {
    let args = Args::parse();
    let text = read_text()?;

    // args indicate "only", that's why we pass them with !
    let sprinkles = sprinklize(&text, args.bold, args.italic, !args.bright, !args.normal, !args.cold, !args.warm);
    
    print!("{sprinkles}");
    std::io::stdout().flush().unwrap();
    Ok(())
}
