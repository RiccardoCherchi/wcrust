use rayon::{iter::ParallelIterator, str::ParallelString};

pub fn get_char(inp: &String) -> usize {
    return inp.par_chars().count();
}

pub fn get_lines(inp: &String) -> usize {
    return inp.par_lines().count();
}

pub fn get_words(inp: &String) -> usize {
    return inp.par_split_whitespace().count();
}

pub fn get_most_char_line(inp: &String) -> usize {
    return inp.par_lines().map(|line| line.len()).max().unwrap_or(0);
}
