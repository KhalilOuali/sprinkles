use colored::{Color, Colorize};
use rand::seq::SliceRandom;
use std::{env::args, fs, process::exit};

fn main() {
    let mut args = args();
    let exe_name = args.next().unwrap_or("sprinkles".to_string());

    if args.len() != 1 {
        eprintln!("Invalid number of arguments.");
        eprintln!("Usage: {exe_name} <filename>");
        exit(1);
    }

    let filename = args
        .next()
        .expect("Unexpected error: Couldn't parse filename.");

    let content = match fs::read_to_string(filename) {
        Ok(s) => s,
        Err(error) => {
            eprintln!("Couldn't read file due to the following error:");
            eprintln!("{error}");
            exit(2);
        }
    };

    let colors = [
        Color::Red,
        Color::Green,
        Color::Yellow,
        Color::Blue,
        Color::Magenta,
        Color::Cyan,
        Color::White,
        Color::BrightRed,
        Color::BrightGreen,
        Color::BrightYellow,
        Color::BrightBlue,
        Color::BrightMagenta,
        Color::BrightCyan,
        Color::BrightWhite,
    ];

    let mut rng = rand::thread_rng();
    content
        .chars()
        .map(|c| {
            let color = colors
                .choose(&mut rng)
                .expect("Unexpected error: Chosen color null.");
            c.to_string().color(*color)
        })
        .for_each(|c| print!("{c}"));
}
