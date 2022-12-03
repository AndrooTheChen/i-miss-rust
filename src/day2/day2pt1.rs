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
                let mut points = 1;
                match inputs[0] {
                    "C" => points += 6,
                    "A" => points += 3,
                    _ => (),
                };
                points
            },
            "Y" => {
                let mut points = 2;
                match inputs[0] {
                    "A" => points += 6,
                    "B" => points += 3,
                    _ => (),
                };
                points
            },
            "Z" => {
                let mut points = 3;
                match inputs[0] {
                    "B" => points += 6,
                    "C" => points += 3,
                    _ => (),
                };
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
