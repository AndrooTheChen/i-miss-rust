mod challenge;

use challenge::{Challenge, ChallengeRunner};
use std::env;

// TODO define error types

fn main() {
    // Get commandline args.
    let args: Vec<String> = env::args().collect();

    // TODO: handle these and log better.
    let day = if let Ok(day_val) = args[1].parse::<u32>() { day_val } else { panic!() };
    let part = if let Ok(part_val) = args[2].parse::<u32>() {part_val } else { panic!() };
    println!("Running challenge for day {} part {}", day, part);

    // Run the challenge.
    let challenge = Challenge::new(day, part);
    challenge.run();
}
