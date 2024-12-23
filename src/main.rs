mod config;

use std::{
    env::{self},
    io::{self, Read},
};

use config::{ArgumentsConfig, DEFAULT_ARGUMENTS_CONFIG, FALSE_ARGUMENTS_CONFIG};
use regex::Regex;

fn assigne_config_option(args_config: &mut ArgumentsConfig, option: char) {
    match option {
        'l' => args_config.display_lines = true,
        'L' => args_config.display_most_chars_argument = true,
        'w' => args_config.display_words = true,
        'c' => args_config.display_characters = true,
        _ => println!("No argument recognised"),
    }
}

fn assigne_config(args: &Vec<String>, args_config: &mut ArgumentsConfig) -> bool {
    assert!(args.len() > 1);
    let mut has_under = false;
    for el in &args[1..] {
        match el.chars().next() {
            Some('-') => {
                let options: String = el.clone().chars().skip(1).collect();
                for opt in options.chars() {
                    assigne_config_option(args_config, opt);
                }
                has_under = true;
            }
            Some(_) => println!("Invalid argument maybe a file"),
            None => println!("The string is empty."),
        }
    }
    return has_under;
}

fn main() {
    let mut input = String::new();
    io::stdin()
        .read_to_string(&mut input)
        .expect("Failed to read line");

    let args_vec = env::args().collect::<Vec<_>>();
    let mut args_config: ArgumentsConfig = FALSE_ARGUMENTS_CONFIG;
    if args_vec.len() > 1 {
        let no_args = !assigne_config(&args_vec, &mut args_config);
        if no_args {
            args_config = DEFAULT_ARGUMENTS_CONFIG;
        }
    } else {
        args_config = DEFAULT_ARGUMENTS_CONFIG;
    }

    let chars = input.len();

    let phrases_vec = input.split("\n").collect::<Vec<_>>();

    let lines = phrases_vec.len() - 1;

    let re = Regex::new(r"\s+").unwrap();
    let formatted = re.replace_all(input.as_str(), " ");

    let words = formatted
        .replace("\n", " ")
        .split(" ")
        .collect::<Vec<_>>()
        .len();

    let mut lines_lens = phrases_vec.iter().map(|&x| x.len()).collect::<Vec<_>>();
    lines_lens.sort();
    lines_lens.reverse();

    if args_config.display_lines {
        print!("       {}", lines);
    }

    if args_config.display_words {
        print!("      {}", words);
    }

    if args_config.display_characters {
        print!("      {}", chars);
    }

    if args_config.display_most_chars_argument {
        print!("      {}", lines_lens[0]);
    }
    print!("\n")
}
