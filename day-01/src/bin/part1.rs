use std::io::{BufReader, BufRead};
use std::fs::File;

fn main() {
    let reader = BufReader::new(File::open("/home/tegues/advent-of-code-2023/day-01/src/bin/input1.txt").expect("File not found"));
    let strings: Vec<String> = reader.lines().map(|l| l.expect("Could not parse line")).collect();
    let mut total = 0;
    for s in strings {
        let numbers: Vec<char> = s.chars().filter(|d| d.is_digit(10)).collect();
        let len = numbers.len();
        if len == 0 {
            println!("The numbers for this line are: 00");
        }
        println!("The numbers for this line are: {:?}{:?}", numbers[0], numbers[len-1]);
        let suma = numbers[0].to_digit(10).unwrap()*10 + numbers[len-1].to_digit(10).unwrap();
        println!("The numbers for this line are: {:?}{:?} and sums up to {}", numbers[0], numbers[len-1], suma);
        total = total + suma;
    }
    println!("La suma total es de: {}", total);
}

#[cfg(test)]

mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        assert_eq!(2 + 2, 4); 
    }
}