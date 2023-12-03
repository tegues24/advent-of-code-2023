use std::{io::{BufReader, BufRead}, fs::File, collections::HashMap};

use regex::Regex;

#[derive(Debug)]
struct Game {
    game_num: u32,
    color_num: Vec<ColorNum>
}

#[derive(Debug)]
struct ColorNum{
    num: u32,
    color: String
}

fn main() {
    let reader = BufReader::new(File::open("/home/tegues/advent-of-code-2023/day-02/src/bin/input1.txt").unwrap());
    let lines: Vec<String> = reader.lines().map(|l| l.unwrap()).collect();
    let regex = Regex::new("([0-9]{2})+\\s+(blue|green|red)").unwrap();
    let mut game_vec: Vec<Game> = vec![];
    let mut total = 0;
    let mut flag = false;

    for line in lines {
        let split: Vec<&str> = line.split(": ").collect();
        let split_again: Vec<&str> = split[0].split(" ").collect();
        let color_num = split_again[1].parse::<u32>().unwrap();
        let list: Vec<ColorNum> = regex.captures_iter(split[1]).map(|caps| {
            let (_, [n, c]) = caps.extract();
            ColorNum{
                num: n.parse::<u32>().unwrap(),
                color: c.to_string()}
        }).collect();
        game_vec.push(
            Game{
                game_num: color_num,
                color_num: list
            }
        )
    }

    println!("{:?}", game_vec);

    for game in game_vec {
        game.color_num.iter().for_each(|n| {
            if n.color == "blue" && n.num > 14 || n.color == "green" && n.num > 13 || n.color == "red" && n.num > 12 {
                flag = true;
            }
        });
        if !flag {
            total = total + game.game_num;
        } else {
            flag = false;
        }
    }

    println!("Total: {}", total);
}