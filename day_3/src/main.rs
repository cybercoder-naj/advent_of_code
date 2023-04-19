use std::{fs, env, collections::BTreeSet};

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        eprintln!("You need to supply the input file.");
        return;
    }

    match solve_1(&args[1]) {
        Ok(result) => println!("{}", result),
        Err(message) => eprintln!("An error occured: {}", message)
    }

    match solve_2(&args[0]) {
        Ok(result) => println!("{}", result),
        Err(message) => eprintln!("An error occured: {}", message)
    }
}

fn solve_1(filename: &str) -> Result<i32, &'static str> {
    let input = fs::read_to_string(filename);
    if let Err(_) = input {
        return Err("Error reading from the file");
    }
    let input = input.unwrap();

    let rucksacks: Vec<(&str, &str)> = input.lines()
        .map(|line| {
            let mid = line.len() / 2;
            (&line[..mid], &line[mid..])
        }).collect();

    let mut score = 0;
    for rucksack in rucksacks {
        'outer:
        for comp1 in rucksack.0.chars() {
            for comp2 in rucksack.1.chars() {
                if comp1 == comp2 {
                    score += convert_to_score(comp1);
                    break 'outer;
                }
            }
        }
    }

    Ok(score)
}

fn solve_2(filename: &str) -> Result<i32, &'static str> {
    let input = fs::read_to_string(filename);
    if let Err(_) = input {
        return Err("Error reading from the file");
    }
    let input = input.unwrap();
    let lines: Vec<_> = input.lines().collect();

    let mut vec: Vec<BTreeSet<char>> = vec![];
    let mut score = 0;
    for i in 0..lines.len() {
        match i % 3 {
            2 => {
                let mut set: BTreeSet<char> = BTreeSet::new();
                lines[i].chars().for_each(|c| { set.insert(c); });

                vec.push(set);

                let (intersection, others) = vec.split_at_mut(1);
                let intersection = &mut intersection[0];
                for other in others {
                    intersection.retain(|e| other.contains(e));
                }

                let chrs = intersection.first().unwrap();
                score += convert_to_score(*chrs);

                vec.clear();
            },
            _ => {
                let mut set: BTreeSet<char> = BTreeSet::new();
                lines[i].chars().for_each(|c| { set.insert(c); });

                vec.push(set);
            }
        }
    }

    Ok(score)
}

fn convert_to_score(ch: char) -> i32 {
    if ch.is_lowercase()  {
        (ch as i32) - 96
    } else {
        (ch as i32) - 64 + 26
    }
}