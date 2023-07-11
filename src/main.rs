use colored::{Color, Colorize};
use rand::seq::SliceRandom;
use std::{
    env::{args, Args},
    fs, io,
    process::exit,
};

fn read_from_io() -> String {
    io::stdin()
        .lines()
        .map(|line| line.expect("Unexpected error: Couldn't read stdin.") + "\n")
        .collect()
}

fn read_from_arg(mut args: Args) -> String {
    let arg = args
        .next()
        .expect("Unexpected error: Couldn't parse argument.");
    if arg == "-f" {
        eprintln!("Invalid arguments. Usage:");
        eprintln!("1. sprinkles");
        eprintln!("2. sprinkles <input>");
        eprintln!("3. sprinkles -f <filename>");
        exit(1);
    };
    arg
}

fn read_from_file(mut args: Args) -> String {
    let flag = args
        .next()
        .expect("Unexpected error: Couldn't parse arguments.");
    if flag != "-f" {
        eprintln!("Invalid arguments. Usage:");
        eprintln!("1. sprinkles");
        eprintln!("2. sprinkles <input>");
        eprintln!("3. sprinkles -f <filename>");
        exit(1);
    };
    let filename = args
        .next()
        .expect("Unexpected error: Couldn't parse filename.");
    match fs::read_to_string(filename) {
        Ok(content) => content,
        Err(error) => {
            eprintln!("Couldn't read file due to the following error:");
            eprintln!("{error}");
            exit(2);
        }
    }
}

fn read_string() -> String {
    let mut args = args();
    args.next();
    let len = args.len();

    match len {
        0 => read_from_io(),
        1 => read_from_arg(args),
        2 => read_from_file(args),
        _ => {
            eprintln!("Invalid number of arguments. Usage:");
            eprintln!("1. sprinkles");
            eprintln!("2. sprinkles <input>");
            eprintln!("3. sprinkles -f <filename>");
            exit(1);
        }
    }
}

fn sprinklize(string: &str) -> String {
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
    string
        .chars()
        .map(|c| {
            let color = colors
                .choose(&mut rng)
                .expect("Unexpected error: Chosen color null.");
            c.to_string().color(*color).to_string()
        }).collect()
}

fn main() {
    let string = read_string();
    let sprinkles = sprinklize(&string);
    println!("{sprinkles}");
}
