use anyhow::{anyhow, Result};

fn dummy() {
    println!("moshi mosh")
}

pub struct Challenge {
    challenge: fn()
}

impl Challenge {
    pub fn new(day: u32, part: u32) -> Self {
        match (day, part) {
            // (1, 1) => Ok(Self { challenge: day1pt1 }),
            (1, 1) => Self { challenge: dummy },
            _ => panic!()
        }
    }
}

pub trait ChallengeRunner {
    fn run(&self);
}

impl ChallengeRunner for Challenge {
    fn run(&self) {
        (self.challenge)()
    }
}