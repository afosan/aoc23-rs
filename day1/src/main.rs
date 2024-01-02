use std::fs;
use aho_corasick::AhoCorasick;

fn read_file(file_path: &str) -> String {
    let content = fs::read_to_string(file_path).expect("could not read the file");
    content
}

fn main() {
    // read input file
    let file_path = "input.txt";
    let content = read_file(file_path);

    // task 1
    let task1 = content
        .lines().map(|line| {
            let first = line.chars().filter(|c| c.is_digit(10)).nth(0).expect("expected at least one digit").to_digit(10).unwrap() as u64;
            let last = line.chars().filter(|c| c.is_digit(10)).last().expect("expected at least one digit").to_digit(10).unwrap() as u64;
            10 * first + last
        }).sum::<u64>();
    println!("task 1 = {task1}");

    let patterns = [
        "0",
        "1",
        "2",
        "3",
        "4",
        "5",
        "6",
        "7",
        "8",
        "9",
        "zero",
        "one",
        "two",
        "three",
        "four",
        "five",
        "six",
        "seven",
        "eight",
        "nine",
    ];
    let ac = AhoCorasick::new(patterns).unwrap();

    // task 2
    let task2 = content
        .lines().map(|line| {
            let matches = ac.find_overlapping_iter(line).collect::<Vec<_>>();
            let first = matches.get(0).expect("expected at least one match").pattern().as_u64() % 10;
            let last = matches.get(matches.len() - 1).expect("expected at least one match").pattern().as_u64() % 10;
            10 * first + last
        }).sum::<u64>();
    println!("task 2 = {task2}");
}
