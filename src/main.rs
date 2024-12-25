mod config;
mod counters;
mod files;

use std::{
    env::{self},
    fs,
    io::{self, Read},
    process::exit,
};

#[derive(PartialEq)]
pub struct FileResults {
    pub total_lines: usize,
    pub total_words: usize,
    pub total_chars: usize,
    pub most_chars_line: usize,
}

pub const EMPTY_FILE_RESULTS: FileResults = FileResults {
    most_chars_line: 0,
    total_chars: 0,
    total_lines: 0,
    total_words: 0,
};

fn main() {
    let args_vec = env::args().collect::<Vec<_>>();
    let mut args_config: config::ArgumentsConfig = config::EMPTY_ARGUMENTS_CONFIG;
    if args_vec.len() > 1 {
        let no_args = !config::assigne_config(&args_vec, &mut args_config);
        if no_args {
            args_config = config::DEFAULT_ARGUMENTS_CONFIG;
        }
    } else {
        args_config = config::DEFAULT_ARGUMENTS_CONFIG;
    }

    let mut files: Vec<String> = vec![];
    if args_vec.len() > 1 {
        files.append(&mut files::get_files(&args_vec));
    }

    if files.is_empty() {
        let mut stdin_input = String::new();
        io::stdin()
            .read_to_string(&mut stdin_input)
            .expect("Failed to read line");
        print_result(stdin_input, &args_config, None);
    } else {
        let mut tot_file_res = EMPTY_FILE_RESULTS;
        for file in files {
            let file_name = file.clone();
            match fs::read_to_string(file) {
                Ok(c) => {
                    if let Some(res) = print_result(c, &args_config, Some(file_name)) {
                        tot_file_res.total_chars += res.total_chars;
                        tot_file_res.total_lines += res.total_lines;
                        tot_file_res.total_words += res.total_words;
                        if tot_file_res.most_chars_line < res.most_chars_line {
                            tot_file_res.most_chars_line = res.most_chars_line;
                        }
                    }
                }
                Err(error) => {
                    eprintln!("file {}: {}", file_name, error);
                    exit(-1);
                }
            };
        }
        if tot_file_res != EMPTY_FILE_RESULTS {
            print_total(tot_file_res, &args_config);
        }
    }
}

fn print_result(
    input: String,
    args_config: &config::ArgumentsConfig,
    file_name: Option<String>,
) -> Option<FileResults> {
    let mut file_result = EMPTY_FILE_RESULTS;
    if args_config.display_lines {
        file_result.total_lines = counters::get_lines(input.clone());
        print!("       {}", file_result.total_lines);
    }

    if args_config.display_words {
        file_result.total_words = counters::get_words(input.clone());
        print!("      {}", file_result.total_words);
    }

    if args_config.display_characters {
        file_result.total_chars = counters::get_char(input.clone());
        print!("      {}", file_result.total_chars);
    }

    if args_config.display_most_chars_argument {
        file_result.most_chars_line = counters::get_most_char_line(input.clone());
        print!("      {}", file_result.most_chars_line);
    }
    if let Some(file) = file_name {
        print!(" {}", file);
        print!("\n");
        Some(file_result)
    } else {
        print!("\n");
        None
    }
}

fn print_total(file_result: FileResults, args_config: &config::ArgumentsConfig) {
    if args_config.display_lines {
        print!("       {}", file_result.total_lines);
    }

    if args_config.display_words {
        print!("      {}", file_result.total_words);
    }

    if args_config.display_characters {
        print!("      {}", file_result.total_chars);
    }

    if args_config.display_most_chars_argument {
        print!("      {}", file_result.most_chars_line);
    }
    print!(" total");
    print!("\n");
}
