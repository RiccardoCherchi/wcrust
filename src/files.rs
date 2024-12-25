pub fn get_files(args: &Vec<String>) -> Vec<String> {
    assert!(args.len() > 1);
    let mut files = vec![];
    for arg in &args[1..] {
        let first_char = arg.chars().next();
        match first_char {
            Some('-') => continue,
            Some(_) => files.push(arg.clone()),
            None => println!("The string is empty."),
        }
    }
    return files;
}
