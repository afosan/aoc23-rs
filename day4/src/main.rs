use std::fs;
use regex::Regex;

fn read_file(file_path: &str) -> String {
    let content = fs::read_to_string(file_path).expect("could not read the file");
    content
}

fn main() {
    // read input file
    let file_path = "input.txt";
    let content = read_file(file_path);

    let number_re = Regex::new(r"(\d+)").unwrap();

    let counts_common = content
        .lines()
        .map(|line| {
            let split = line.split(":").nth(1).unwrap().split("|").collect::<Vec<_>>();
            let winners_split = split[0];
            let numbers_split = split[1];
            let winners = number_re.captures_iter(winners_split).map(|c| c.get(1).unwrap().as_str().parse::<u64>().unwrap()).collect::<Vec<u64>>();
            let numbers = number_re.captures_iter(numbers_split).map(|c| c.get(1).unwrap().as_str().parse::<u64>().unwrap()).collect::<Vec<u64>>();

            let count_common = numbers.iter().filter(|w| winners.iter().any(|n| n == *w)).count() as u64;

            count_common
        })
        .collect::<Vec<u64>>();

    // task 1
    let task1 = counts_common
        .iter()
        .map(|count| if *count == 0 { 0 } else { 2u64.pow(*count as u32 - 1) })
        .sum::<u64>();

    println!("task 1 = {task1}");

    // task 2
    let mut counts_copy = vec![1u64; counts_common.len()];
    counts_common
        .iter()
        .enumerate()
        .for_each(|(i, count)| {
            for j in (i + 1)..=(i + *count as usize) {
                if j < counts_copy.len() {
                    counts_copy[j] += counts_copy[i];
                }
            }
        });

    let task2 = counts_copy.iter().sum::<u64>();
    println!("task 2 = {task2}");
}
