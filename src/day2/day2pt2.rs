use std::error::Error;
use std::fs::File;
use std::io::{BufRead, BufReader};

fn day2(filepath: &str) -> Result<(), Box<dyn Error>> {
    let file = File::open(filepath)?;
    let reader = BufReader::new(file);

    let mut total_points = 0;

    for line in reader.lines() {
        let cloned_line = line?.clone();
        let inputs: Vec<&str> = cloned_line.split(' ').collect();
        // println!("{} {}", inputs[0], inputs[1]);

        // Calculate how many points for the choice I make.
        let my_points = match inputs[1] {
            "X" => {
                // Need to lose the game
                let mut points = 0;
                match inputs[0] {
                    "A" => points += 3,
                    "B" => points += 1,
                    "C" => points += 2,
                    _ => (),
                }
                points
            },
            "Y" => {
                // Need to tie the game
                let mut points = 3;
                match inputs[0] {
                    "A" => points += 1,
                    "B" => points += 2,
                    "C" => points += 3,
                    _ => (),
                }
                points
            },
            "Z" => {
                // Need to win the game
                let mut points = 6;
                match inputs[0] {
                    "A" => points += 2,
                    "B" => points += 3,
                    "C" => points += 1,
                    _ => (),
                }
                points
            },
            _ => panic!(),
        };
        total_points += my_points;
    }

    println!("Total points: {}", total_points);    

    Ok(())
}

fn main() {
    day2("input.txt");
}
