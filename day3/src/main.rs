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

    let symbol_locations = content
        .lines()
        .enumerate()
        .map(|(row, line)| {
            line
                .chars()
                .enumerate()
                .filter(|(_, c)| !c.is_digit(10) && *c != '.' && *c != '\n')
                .map(|(column, _)| (row, column))
                .collect::<Vec<(usize, usize)>>()
        })
        .flatten().collect::<Vec<(usize, usize)>>();

    let number_re = Regex::new(r"(?<num>\d+)").unwrap();

    let numbers_and_locations_neighboring_a_symbol = content
        .lines()
        .enumerate()
        .map(|(row, line)| {
            number_re
                .captures_iter(line)
                .map(|c| {
                    let mtch = c.get(1).unwrap();
                    let column_start = mtch.start();
                    let column_end = mtch.end() - 1;
                    let num = mtch.as_str().parse::<u64>().unwrap();

                    (row, column_start, column_end, num)
                })
                .filter(|(row, column_start, column_end, _)| {
                    symbol_locations.iter().any(|(s_row, s_column)| {
                        (if *row >= 1 { *row - 1 } else { 0 }) <= *s_row && *s_row <= *row + 1 && (if *column_start >= 1 { *column_start - 1 } else { 0 }) <= *s_column && *s_column <= column_end + 1
                    })
                })
                .collect::<Vec<(usize, usize, usize, u64)>>()
        })
        .flatten()
        .collect::<Vec<(usize, usize, usize, u64)>>();

    // task 1
    let task1 = numbers_and_locations_neighboring_a_symbol
        .iter()
        .map(|(_, _, _, num)| num)
        .sum::<u64>();

    println!("task 1 = {task1}");

    // task 2
    let task2 = symbol_locations
        .iter()
        .map(|(s_row, s_column)| {
            numbers_and_locations_neighboring_a_symbol
                .iter()
                .filter(|(row, column_start, column_end, _)| {
                    (if *row >= 1 { *row - 1 } else { 0 }) <= *s_row && *s_row <= *row + 1 && (if *column_start >= 1 { *column_start - 1 } else { 0 }) <= *s_column && *s_column <= *column_end + 1
                })
                .map(|(_, _, _, num)| *num)
                .collect::<Vec<u64>>()
        })
        .filter(|nums| nums.len() == 2)
        .map(|nums| nums.iter().product::<u64>())
        .sum::<u64>();
    println!("task 2 = {task2}");
}
