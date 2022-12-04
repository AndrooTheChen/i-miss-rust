use anyhow::{anyhow, Error, Result};
use std::fs::File;
use std::collections::HashSet;
use std::io::{BufRead, BufReader};

// Split the rucksack down the middle and find the element in common.
fn get_duped_letter(rucksack: String) -> Result<char> {
    // Convert input to list of chars.
    let rucksack_as_chars: Vec<char> = rucksack.chars().collect();

    // Split the rucksack into the two compartments.
    let compartment_size = rucksack.len() / 2;
    let compartment1: HashSet<char> = rucksack_as_chars
        .iter()
        .enumerate()
        .filter(|(index, _)| index < &compartment_size)
        .map(|(_, item)| item.clone())
        .collect();
    let compartment2: HashSet<char> = rucksack_as_chars
        .iter()
        .enumerate()
        .filter(|(index, _)| index >= &compartment_size)
        .map(|(_, item)| item.clone())
        .collect();

    // Find the item that is in common in both compartments.
    let mut common_item = compartment1.intersection(&compartment2);

    // This is jank and would be nicer with `ok_or_else` but I can't seem
    // to return `char` from there instead of `&char`.
    if let Some(item) = common_item.next() {
        return Ok(*item);
    } else {
        return Err(anyhow!("failed to get duped item"));
    }
}

fn read_file_line_by_line(filepath: &str) -> Result<()> {
    let file = File::open(filepath)?;
    let reader = BufReader::new(file);

    let mut dup_items: Vec<char> = Vec::new();
    for line in reader.lines() {
        dup_items.push(get_duped_letter(line?)?);
    }

    // Sum up the point values for all the duplicated items.
    let mut total_points = 0;
    dup_items
        .iter()
        .for_each(|dup_item| {
            let dup_value = *dup_item as u32;
            let inc = if dup_value > 90 { dup_value - 'a' as u32 + 1 } else { dup_value - 'A' as u32 + 27 };
            total_points += inc;
        }
    ); 
    println!("Got total value as {}", total_points);

    Ok(())
}

fn main() {
    read_file_line_by_line("input.txt");
}