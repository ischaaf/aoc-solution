use crate::day::{Day,Solution};
use regex::{Captures, Regex};

#[derive(Debug)]
struct PWPolicy {
    letter: char,
    min: usize,
    max: usize,
    pw: String,
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

pub struct DaySln {}

impl Day for DaySln {
    fn day(&self) -> u32 {
        2
    }

    fn solve(&self) -> Solution {
        let part_1 = self.valid_count(|pwp| {
            let count = pwp.pw.chars().filter(|c| *c == pwp.letter).count();
            count >= pwp.min && count <= pwp.max
        });
        let part_2 = self.valid_count(|pwp| {
            let min_char = pwp.pw.chars().nth(pwp.min - 1);
            let max_char = pwp.pw.chars().nth(pwp.max - 1);
            (min_char == Some(pwp.letter)) ^ (max_char == Some(pwp.letter))
        });
        (Some(part_1), Some(part_2))
    }
}

impl DaySln {
    fn valid_count(&self, validate_fn: fn(PWPolicy) -> bool) -> i32 {
        let re = Regex::new(r"([0-9]+)\-([0-9]+) (.): (.+)").unwrap();
        self.daily_input()
            .lines()
            .map(move |l| {
                let cap = re
                    .captures_iter(l)
                    .nth(0)
                    .expect(format!("Failed to match: {}", l).as_str());
                cap.into()
            })
            .map(|pwp| validate_fn(pwp))
            .filter(|v| *v)
            .count() as i32
    }
}
