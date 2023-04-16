use std::{env, io::Error, fs};


fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        eprintln!("You need to supply the input file. No arguments were passeed");
        return;
    }

    match part_1(&args[1]) {
        Ok(result) => println!("{}", result),
        Err(msg) => eprintln!("Error encountered: {}", msg)
    }

    match part_2(&args[1]) {
        Ok(result) => println!("{}", result),
        Err(msg) => eprintln!("Error encountered: {}", msg)
    }
}

fn part_1(filename: &str) -> Result<i32, Error> {
    let calories = get_calories_list(filename)?;

    let max = calories.iter().max().expect("calories can never be empty");
    Ok(*max)
}

fn part_2(filename: &str) -> Result<i32, Error> {
    let mut calories = get_calories_list(filename)?;
    calories.sort();

    let mut sum = 0;
    let i = calories.len();
    for j in 1..4 { // the range is end exclusive!
        sum += calories[i - j];
    }

    Ok(sum)
}

fn get_calories_list(filename: &str) -> Result<Vec<i32>, Error> {
    let input = fs::read_to_string(filename)?;
    let lines: Vec<&str> = input.lines().collect();

    let mut calories: Vec<i32> = vec![0];

    for line in lines {
        let calorie: i32 = line.parse().unwrap_or_else(|_e| {
            calories.push(0);
            0
        });

        let last = calories.last_mut().expect("calories should have atleast one element");
        *last += calorie;
    }

    Ok(calories)
}