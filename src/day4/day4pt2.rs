use anyhow::Result;
use std::fs::read_to_string;

fn read_file_string(filepath: &str) -> Result<String> {
    let data = read_to_string(filepath)?;
    Ok(data)
}

fn pair_overlaps(line: &str) -> bool {
    let mut pairs = line.split(',');
    let pair1 = pairs.next().unwrap();
    let pair2 = pairs.next().unwrap();
    
    let mut range1 = pair1.split('-');
    let mut range2 = pair2.split('-');
    
    // jank asf but im tired
    let r1v1 = range1.next().unwrap().parse::<u32>().unwrap();
    let r1v2 = range1.next().unwrap().parse::<u32>().unwrap();
    let r2v1 = range2.next().unwrap().parse::<u32>().unwrap();
    let r2v2 = range2.next().unwrap().parse::<u32>().unwrap();

    !(r1v2 < r2v1 || r2v2 < r1v1)
}

fn main() {
    if let Ok(data) = read_file_string("input.txt") {
        let mut overlapping_pairs = 0;
        for line in data.split('\n') {
            overlapping_pairs += if pair_overlaps(line) { 1 } else { 0 };
        }

        println!("Number of pairs that have a range containing another is {}", overlapping_pairs);
    }
}