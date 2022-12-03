use std::fs::File;
use std::fs::read_to_string;
use std::io::{BufRead, BufReader};

// Convert file input to string.
fn read_file_string(filepath: &str) -> Result<String, Box<dyn std::error::Error>> {
    let data = read_to_string(filepath)?;
    Ok(data)
}

fn main() {
    let mut max_calories = 0;
    let mut calories = 0;

    if let Ok(data) = read_file_string("day1/input.txt") {
        for line in data.split('\n') {
            // Increment this elf's calorie count.
            if let Ok(calories_line) = line.parse::<i32>() {
                calories += calories_line;
            } else {
                // Update the max calorie count we've found so far.
                max_calories = if calories > max_calories { calories } else { max_calories };

                // Reset count to zero for the next elf's calorie count.
                calories = 0;
            }
        }
    }

    println!("Max calories found: {}", max_calories);
}
