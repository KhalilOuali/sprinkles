use colored::{Color, Colorize};
use rand::seq::SliceRandom;
use std::{
    env::{args, Args},
    fs, io,
    process::exit,
};

fn print_usage(full: bool) {
    println!();
    println!("Usage:");
    println!("\t- sprinkles");
    if full {
        println!("Reads from stdin or pipe. Press Ctrl+D twice to finish typing.");
    }
    println!();
    println!("\t- sprinkles <input>");
    if full {
        println!("Reads from argument. Make sure to use quotes if it's multiple words.");
    }
    println!();
    println!("\t- sprinkles -f <filename>");
    if full {
        println!("Reads from the file.");
    }
    println!();
    println!("\t- sprinkles --help");
    if full {
        println!("Show this help message.");
    }
}

fn read_from_stdin() -> String {
    io::stdin()
        .lines()
        .map(|line| line.expect("Unexpected error: Couldn't read stdin."))
        .collect::<Vec<_>>()
        .join("\n")
}

fn read_from_arg(mut args: Args) -> String {
    let arg = args
        .next()
        .expect("Unexpected error: Couldn't parse argument.");
    if arg == "--help" {
        println!(
            "{} prints out a colored version of the input text or file.",
            sprinklize("sprinkles")
        );
        print_usage(true);
        exit(0);
    };
    arg
}

fn read_from_file(mut args: Args) -> String {
    let flag = args
        .next()
        .expect("Unexpected error: Couldn't parse arguments.");
    if flag != "-f" {
        eprintln!("Invalid arguments.");
        print_usage(false);
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
        0 => read_from_stdin(),
        1 => read_from_arg(args),
        2 => read_from_file(args),
        _ => {
            eprintln!("Invalid number of arguments.");
            print_usage(false);
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
                .expect("Unexpected error: Chosen color none.");
            c.to_string().color(*color).to_string()
        })
        .collect()
}

fn main() {
    let string = read_string();
    let sprinkles = sprinklize(&string);
    println!("{sprinkles}");
    exit(0);
}
