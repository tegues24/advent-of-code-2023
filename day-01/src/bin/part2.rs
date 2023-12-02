use std::io::{BufReader, BufRead};
use std::fs::File;
use std::collections::HashMap;

fn main() {
    let spelled: HashMap<&str, char> = HashMap::from([
        ("one", '1'),
        ("two", '2'),
        ("three", '3'),
        ("four", '4'),
        ("five", '5'),
        ("six", '6'),
        ("seven", '7'),
        ("eight", '8'),
        ("nine", '9')
    ]);
    let reader = BufReader::new(File::open("/home/tegues/advent-of-code-2023/day-01/src/bin/input2.txt").expect("File not found"));
    let mut strings: Vec<String> = reader.lines().map(|l| l.expect("Could not parse line")).collect();
    let mut correct_list = Vec::new();
    let mut tmp = String::new();
    for s in &strings {
        tmp = s.clone();
        for (number, value) in &spelled {
            tmp = tmp.replace(*number, &value.to_string());
        }
        correct_list.push(tmp.clone());
    }
    println!("The strings are: {:?}", correct_list);
    let suma = part1_func(correct_list);
    println!("The total sum is: {}", suma);
}

fn part1_func(correct_list: Vec<String>) -> u32{
    let mut total = 0;
    for s in correct_list {
        let numbers: Vec<char> = s.chars().filter(|d| d.is_digit(10)).collect();
        let len = numbers.len();
        if len == 0 {
            println!("The numbers for this line are: 00");
        }
        let suma = numbers[0].to_digit(10).unwrap()*10 + numbers[len-1].to_digit(10).unwrap();
        println!("The numbers for this line are: {:?}{:?} and sums up to {}", numbers[0], numbers[len-1], suma);
        total = total + suma;
    }
    total
}