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

pub const FALSE_ARGUMENTS_CONFIG: ArgumentsConfig = ArgumentsConfig {
    display_most_chars_argument: false,
    display_lines: false,
    display_characters: false,
    display_words: false,
};
