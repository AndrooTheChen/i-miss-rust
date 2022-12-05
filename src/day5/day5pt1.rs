use anyhow::Result;
use std::fs::read_to_string;

fn read_file_string(filepath: &str) -> Result<String> {
    let data = read_to_string(filepath)?;
    Ok(data)
}

fn main() {
    // init this like an idiot bc im still sleepy
    let mut stacks = vec![Vec::new(), Vec::new(), Vec::new(), Vec::new(), Vec::new(), Vec::new(), Vec::new(), Vec::new(), Vec::new()];
    
    stacks[0].push('N');
    stacks[0].push('R');
    stacks[0].push('G');
    stacks[0].push('P');

    stacks[1].push('J');
    stacks[1].push('T');
    stacks[1].push('B');
    stacks[1].push('L');
    stacks[1].push('F');
    stacks[1].push('G');
    stacks[1].push('D');
    stacks[1].push('C');

    stacks[2].push('M');
    stacks[2].push('S');
    stacks[2].push('V');

    stacks[3].push('L');
    stacks[3].push('S');
    stacks[3].push('R');
    stacks[3].push('C');
    stacks[3].push('Z');
    stacks[3].push('P');

    stacks[4].push('P');
    stacks[4].push('S');
    stacks[4].push('L');
    stacks[4].push('V');
    stacks[4].push('C');
    stacks[4].push('W');
    stacks[4].push('D');
    stacks[4].push('Q');

    stacks[5].push('C');
    stacks[5].push('T');
    stacks[5].push('N');
    stacks[5].push('W');
    stacks[5].push('D');
    stacks[5].push('M');
    stacks[5].push('S');

    stacks[6].push('H');
    stacks[6].push('D');
    stacks[6].push('G');
    stacks[6].push('W');
    stacks[6].push('P');

    stacks[7].push('Z');
    stacks[7].push('L');
    stacks[7].push('P');
    stacks[7].push('H');
    stacks[7].push('S');
    stacks[7].push('C');
    stacks[7].push('M');
    stacks[7].push('V');

    stacks[8].push('R');
    stacks[8].push('P');
    stacks[8].push('F');
    stacks[8].push('L');
    stacks[8].push('W');
    stacks[8].push('G');
    stacks[8].push('Z');

    if let Ok(file) = read_file_string("input.txt") {
        for line in file.split("\n") {
            let mut parsed: Vec<&str> = line.split(" ").collect();
            
            let num_crates = parsed[1].parse::<usize>().unwrap();
            let src_stack = parsed[3].parse::<usize>().unwrap() - 1;
            let dst_stack = parsed[5].parse::<usize>().unwrap() - 1;

            // Move the crates.
            for n in 0..num_crates {
                let moved_crate = stacks[src_stack].pop().unwrap();
                stacks[dst_stack].push(moved_crate);
            }
        }

        // Get the topmost crates
        let mut result = String::from("");
        for stack in stacks {
            result.push(*stack.last().unwrap());
        }

        println!("Top of stacks {}", result);
    }
    
}