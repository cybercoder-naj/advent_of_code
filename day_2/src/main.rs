use std::{env, fs, error::Error};

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        eprintln!("Not enough arguments provided. Required file for input.");
        return;
    }

    match solve(&args[1]) {
        Ok(answer) => println!("{}", answer),
        Err(err) => eprintln!("Error occured: {}", err)
    }
    match solve_2(&args[1]) {
        Ok(answer) => println!("{}", answer),
        Err(err) => eprintln!("Error occured: {}", err)
    }
}

fn solve(filename: &str) -> Result<i32, Box<dyn Error>> {
    let input = fs::read_to_string(filename)?;
    let mut score = 0;
    for line in input.lines() {
        let mut chars = line.chars();
        let opponent_move = chars.nth(0).expect("Should have opponent's move");
        let my_move = chars.nth(1).expect("Should have my move.");

        score += match (opponent_move, my_move) {
            ('A', 'X') => 3 + 1,
            ('A', 'Y') => 6 + 2,
            ('A', 'Z') => 0 + 3,
            ('B', 'X') => 0 + 1,
            ('B', 'Y') => 3 + 2,
            ('B', 'Z') => 6 + 3,
            ('C', 'X') => 6 + 1,
            ('C', 'Y') => 0 + 2,
            ('C', 'Z') => 3 + 3,
            _ => panic!("unsupported type")
        }
    }

    Ok(score)
}

fn solve_2(filename: &str) -> Result<i32, Box<dyn Error>> {
    let input = fs::read_to_string(filename)?;
    let mut score = 0;
    for line in input.lines() {
        let opponent_move = line.chars().nth(0).expect("Should have opponent's move");
        let strategy = line.chars().nth(2).expect("Should have the strategy.");

        score += match strategy {
            'X' => 0 + match opponent_move {
                'A' => 3,
                'B' => 1,
                'C' => 2,
                _ => panic!("Will never happen")
            },
            'Y' => 3 + match opponent_move {
                'A' => 1,
                'B' => 2,
                'C' => 3,
                _ => panic!("Will never happen")
            },
            'Z' => 6 + match opponent_move {
                'A' => 2,
                'B' => 3,
                'C' => 1,
                _ => panic!("Will never happen")
            },
            _ => panic!("Will never happen")
        }
    }

    Ok(score)
}