use std::{env, fs};
use std::io::{BufReader, BufRead};

fn main() {
    let args: Vec<String> = env::args().collect();
    let input = read_input(&args[1]);
    let day = &args[2].parse::<u8>().expect("Couldn't parse day");
    let part = &args[3].parse::<u8>().expect("Couldn't parse part");
    let day_part = (day, part);
    match day_part {
        (1, 1) => solve_day1_part1(input),
        (1, 2) => solve_day1_part2(input),
        _ => panic!("Day and part not implemented")
    }
}

fn read_input(filename: &String) -> Vec<String> {    
    let file = fs::File::open(filename).expect("Unable to open file");
    let reader = BufReader::new(file);
    
    reader
        .lines()
        .map(|line| line.expect("Failed to read line"))
        .collect::<Vec<String>>()
}

fn get_lists(input: Vec<String>) -> (Vec<u64>, Vec<u64>) {
    let mut left_list = Vec::<u64>::new();
    let mut right_list = Vec::<u64>::new();
    for line in input {
        let parts = line.split("   ").collect::<Vec<&str>>();
        let left = parts[0].parse::<u64>().expect("Couldn't parse left");
        let right = parts[1].parse::<u64>().expect("Couldn't parse right");
        left_list.push(left);
        right_list.push(right);
    }
    (left_list, right_list)
}

fn solve_day1_part1(input: Vec<String>) {
    let (mut left_list, mut right_list) = get_lists(input);
    left_list.sort();
    right_list.sort();
    let diffs = left_list
        .iter()
        .zip(
            right_list.iter()
        )
        .map(|(&left, &right)| (left as i64 - right as i64).abs())
        .collect::<Vec<i64>>();
    let sum = diffs.iter().sum::<i64>();
    println!("Day 1 Part 1: {:?}", sum);
}

fn solve_day1_part2(input: Vec<String>) {
    let (left_list, right_list) = get_lists(input);
    // Loop through the left list
    let mut sum = 0;
    for left in left_list {
        // Check how many elements in right list is equal to current element in left list
        let equal_elements = right_list
            .iter()
            .filter(|&right| right == &left)
            .count();
        // Multiply the number of equal elements by the current element in left list
        sum += equal_elements as u64 * left;
}   println!("Day 1 Part 2: {:?}", sum);
}