use anyhow::Result;
use std::collections::HashSet;
use std::fs::read_to_string;

// Convert file input to string.
fn read_file_string(filepath: &str) -> Result<String> {
    let data = read_to_string(filepath)?;
    Ok(data)
}

fn find_badge(groups: [&str; 3]) -> u32 {
    // Place all items in a hashset.
    let mut item_list_set: Vec<HashSet<char>> = vec![HashSet::new(), HashSet::new(), HashSet::new()];
    for n in 0..3 {
        println!("{}", groups[n]);
        groups[n].chars().for_each(|item| {
            item_list_set[n].insert(item);
        });
    }

    let int1: HashSet<char> = item_list_set[0].intersection(&item_list_set[1]).map(|i| i.clone()).collect();
    let int2: HashSet<_> = int1.intersection(&item_list_set[2]).collect();

    let badge_char: char =*int2.into_iter().next().unwrap();
    let badge_priority: u32 = if badge_char as u32 > 90 { badge_char as u32 - 'a' as u32 + 1 } else { badge_char as u32 - 'A' as u32 + 27 };
    println!("badge priority: {} {}", badge_char, badge_priority);

    badge_priority
}

fn main() {
    let mut priority_sum = 0;

    if let Ok(file) = read_file_string("input2.txt") {
        let mut line_ctr = 0;
        let mut groups: [&str; 3] = [""; 3];

        for line in file.split('\n') {
            // println!("{}", line);
            if line_ctr % 3 == 0 && line_ctr > 0 {
                // Reset the group index.
                line_ctr = 0;
                
                // Every three lines we should have a group fully initialized. Find the badge.
                priority_sum += find_badge(groups);
            }
            groups[line_ctr % 3] = line;
            line_ctr += 1;
        }

        // Do this for the last trio.
        priority_sum += find_badge(groups);
    }

    println!("Sum of priorites as {}", priority_sum);
}