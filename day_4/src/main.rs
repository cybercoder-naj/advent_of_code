use std::{env, fs};

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        eprintln!("Supply your input file.");
        return;
    }

    let input = fs::read_to_string(&args[1]).unwrap_or_default();
    println!("{}", solve(&input));

    println!("{}", solve_2(&input));
}

fn solve(input: &str) -> u32 {
    let lines: Vec<_> = input.lines().collect();

    let mut count = 0;
    for line in lines {
        let pair: Vec<_> = line.split(",").collect();

        let range1: Vec<u8> = pair[0].split("-").map(|num| num.parse().expect("Need a number!")).collect();
        let range2: Vec<u8> = pair[1].split("-").map(|num| num.parse().expect("Need a number!")).collect();
    
        if range1[0] >= range2[0] && range1[1] <= range2[1] {
            count += 1;
        } else if range1[0] <= range2[0] && range1[1] >= range2[1] {
            count += 1;
        }
    }

    count
}

fn solve_2(input: &str) -> u32 {
    let lines: Vec<_> = input.lines().collect();

    let mut count = 0;
    for line in lines {
        let pair: Vec<_> = line.split(",").collect();

        let range1: Vec<u8> = pair[0].split("-").map(|num| num.parse().expect("Need a number!")).collect();
        let range2: Vec<u8> = pair[1].split("-").map(|num| num.parse().expect("Need a number!")).collect();

        if range1[1] >= range2[0] && range1[0] <= range2[1] {
            count += 1;
        } else if range1[0] <= range2[1] && range1[1] >= range2[0] {
            count += 1;
        }
    }

    count
}
