use crate::utils;
use regex::{Captures, Regex};

#[derive(Debug)]
struct PWPolicy {
    letter: char,
    min: usize,
    max: usize,
    pw: String,
}

impl PWPolicy {
    fn valid_part_1(&self) -> bool {
        let count = self.pw.chars().filter(|c| *c == self.letter).count();
        count >= self.min && count <= self.max
    }
    fn valid_part_2(&self) -> bool {
        let min_char = self.pw.chars().nth(self.min - 1);
        let max_char = self.pw.chars().nth(self.max - 1);
        (min_char == Some(self.letter)) ^ (max_char == Some(self.letter))
    }
}

impl<'a> From<Captures<'a>> for PWPolicy {
    fn from(cap: Captures) -> PWPolicy {
        PWPolicy {
            letter: cap[3].chars().nth(0).unwrap(),
            min: cap[1].parse::<usize>().unwrap(),
            max: cap[2].parse::<usize>().unwrap(),
            pw: cap[4].to_string(),
        }
    }
}

 fn get_input() -> Box<dyn Iterator<Item=PWPolicy>> {
    let re = Regex::new(r"([0-9]+)\-([0-9]+) (.): (.+)").unwrap();
    Box::new(utils::read_lines("data/day_2/input_big.txt")
        .map(move |l| {
            let cap = re
                .captures_iter(l.as_str())
                .nth(0)
                .expect(format!("Failed to match: {}", l).as_str());
            cap.into()
        }))
}

pub fn solve_part_1() {
    let count = get_input()
        .map(|pwp| pwp.valid_part_1())
        .filter(|v| *v)
        .count();
    println!("Found: {} valid", count);
}

pub fn solve_part_2() {
    let count = get_input()
        .map(|pwp| pwp.valid_part_2())
        .filter(|v| *v)
        .count();
    println!("Found: {} valid", count);
}
