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

    let game_id_re = Regex::new(r"^Game (?<game_id>\d+):").unwrap();
    let ball_count_re = Regex::new(r"((?<count>\d+) (?<color>red|green|blue))").unwrap();

    // task 1
    let task1 = content
        .lines().map(|line| {
            let game_id = &game_id_re.captures_iter(line).nth(0).expect("expected at least one match")["game_id"].parse::<u64>().expect("cannot parse into u64");
            let mut r_acc = u64::MIN;
            let mut g_acc = u64::MIN;
            let mut b_acc = u64::MIN;
            
            ball_count_re.captures_iter(line).for_each(|c| {
                    let count = c["count"].parse::<u64>().expect("cannot parse into u64");
                    match &c["color"] {
                        "red" => {
                            r_acc = r_acc.max(count);
                        },
                        "green" => {
                            g_acc = g_acc.max(count);
                        },
                        "blue" => {
                            b_acc = b_acc.max(count);
                        },
                        _ => panic!("unexpected capture value"),
                    }
                }
            );

            if r_acc <= 12 && g_acc <= 13 && b_acc <= 14 {
                *game_id
            } else {
                0
            }
        }).sum::<u64>();
    println!("task 1 = {task1}");

    // task 2
    let task2 = content
        .lines().map(|line| {
            // let game_id = &game_id_re.captures_iter(line).nth(0).expect("expected at least one match")["game_id"].parse::<u64>().expect("cannot parse into u64");
            let mut r_acc = 0u64;
            let mut g_acc = 0u64;
            let mut b_acc = 0u64;

            ball_count_re.captures_iter(line).for_each(|c| {
                    let count = c["count"].parse::<u64>().expect("cannot parse into u64");
                    match &c["color"] {
                        "red" => {
                            r_acc = r_acc.max(count);
                        },
                        "green" => {
                            g_acc = g_acc.max(count);
                        },
                        "blue" => {
                            b_acc = b_acc.max(count);
                        },
                        _ => panic!("unexpected capture value"),
                    }
                }
            );

            r_acc * g_acc * b_acc
        }).sum::<u64>();
    println!("task 2 = {task2}");
}
