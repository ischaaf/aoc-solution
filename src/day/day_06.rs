use crate::day::{Day, Solution};
use std::collections::HashSet;

pub struct DaySln {}

impl Day for DaySln {
    fn day(&self) -> u32 {
        6
    }

    fn solve(&self) -> Solution {
        let part_1 = self
            .daily_input()
            .split("\n\n")
            .map(|g| {
                g.chars()
                    .filter(|c| *c != '\n')
                    .collect::<HashSet<char>>()
                    .len()
            })
            .sum::<usize>() as i32;

        let part_2 = self
            .daily_input()
            .split("\n\n")
            .map(|g| {
                g.lines()
                    .map(|l| l.chars().collect::<HashSet<char>>())
                    .fold_first(|acc, n| acc.intersection(&n).cloned().collect::<HashSet<char>>())
                    .unwrap()
                    .len()
            })
            .sum::<usize>() as i32;
        (Some(part_1), Some(part_2))
    }
}
