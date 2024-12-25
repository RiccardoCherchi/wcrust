use regex::Regex;

pub fn get_char(inp: String) -> usize {
    return inp.len();
}

pub fn get_lines(inp: String) -> usize {
    return inp.split("\n").collect::<Vec<_>>().len() - 1;
}

pub fn get_words(inp: String) -> usize {
    let re = Regex::new(r"\s+").unwrap();
    let formatted = re.replace_all(inp.as_str(), " ");

    return formatted
        .replace("\n", " ")
        .split(" ")
        .collect::<Vec<_>>()
        .len();
}

pub fn get_most_char_line(inp: String) -> usize {
    let lines_vec = inp.split("\n").collect::<Vec<_>>();
    let mut lines_lens = lines_vec.iter().map(|&x| x.len()).collect::<Vec<_>>();
    lines_lens.sort();
    lines_lens.reverse();
    return lines_lens[0];
}
