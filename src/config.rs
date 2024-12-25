#[derive(Debug)]
pub struct ArgumentsConfig {
    pub display_most_chars_argument: bool,
    pub display_lines: bool,
    pub display_characters: bool,
    pub display_words: bool,
}

pub const DEFAULT_ARGUMENTS_CONFIG: ArgumentsConfig = ArgumentsConfig {
    display_most_chars_argument: false,
    display_lines: true,
    display_characters: true,
    display_words: true,
};

pub const EMPTY_ARGUMENTS_CONFIG: ArgumentsConfig = ArgumentsConfig {
    display_most_chars_argument: false,
    display_lines: false,
    display_characters: false,
    display_words: false,
};

fn assigne_config_option(args_config: &mut ArgumentsConfig, option: char) {
    match option {
        'l' => args_config.display_lines = true,
        'L' => args_config.display_most_chars_argument = true,
        'w' => args_config.display_words = true,
        'c' => args_config.display_characters = true,
        _ => println!("No argument recognised"),
    }
}

pub fn assigne_config(args: &Vec<String>, args_config: &mut ArgumentsConfig) -> bool {
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
            Some(_) => continue,
            None => println!("The string is empty."),
        }
    }
    return has_under;
}
