use std::fs::File;
use std::fs::read_to_string;
use std::io::{BufRead, BufReader};

use priority_queue::PriorityQueue;

// Convert file input to string.
fn read_file_string(filepath: &str) -> Result<String, Box<dyn std::error::Error>> {
    let data = read_to_string(filepath)?;
    Ok(data)
}

fn main() {
    let mut max_calories = 0;
    let mut calories = 0;

    // This is complete overkill for a problem like this but I want to see
    // what PQs are like in this language.
    let mut pq = PriorityQueue::new();

    if let Ok(data) = read_file_string("input.txt") {
        for line in data.split('\n') {
            // Increment this elf's calorie count.
            if let Ok(calories_line) = line.parse::<i32>() {
                calories += calories_line;
            } else {
                // Push this calorie count on our priority queue.
                pq.push(calories, calories);

                // Reset count to zero for the next elf's calorie count.
                calories = 0;
            }
        }
    }

    // Get the top 3 items off of our priority queue and return the sum.
    let mut top_3 = 0;
    for n in 0..3 {
        if let Some(calories) = pq.pop() {
            top_3 += calories.0;
        }
    }

    println!("Sum of calories for top 3 elves found: {}", top_3);
}
