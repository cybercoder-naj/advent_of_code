use std::{env, io::Error, fs};


fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        eprintln!("You need to supply the input file. No arguments were passeed");
        return;
    }

    match process(&args[1]) {
        Ok(()) => return,
        Err(msg) => eprintln!("Error encountered: {}", msg)
    }
}

fn process(filename: &str) -> Result<(), Error> {
    let input = fs::read_to_string(filename)?;
    let lines: Vec<&str> = input.lines().collect();

    let mut calories: Vec<usize> = vec![0];

    for line in lines {
        let calorie: usize = line.parse().unwrap_or_else(|_e| {
            calories.push(0);
            0
        });

        let last = calories.last_mut().expect("calories should have atleast one element");
        *last += calorie;
    }

    let max = calories.iter().max().expect("calories can never be empty");
    println!("{max}");

    Ok(())
}
