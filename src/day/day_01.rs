use crate::day::{Day,Solution};
use std::collections::HashSet;

pub struct DaySln {}

impl Day for DaySln {
    fn day(&self) -> u32 {
        1
    }

    fn solve(&self) -> Solution {
        let lines = self.daily_input()
            .lines()
            .map(|e| e.parse::<i32>().unwrap())
            .collect();

        (Some(self.part_1(&lines)), Some(self.part_2(&lines)))
    }

}

impl DaySln {
    fn part_1(&self, lines: &Vec<i32>) -> i32 {
        let mut seen: HashSet<i32> = HashSet::new();
        for num in lines {
            let comp = 2020i32.checked_sub(*num);
            if comp.is_some() && seen.contains(&comp.unwrap()) {
                return num * comp.unwrap();
            }
            seen.insert(*num);
        }
        panic!("Did not find match");
    }

    fn part_2(&self, lines: &Vec<i32>) -> i32 {
        let mut seen: HashSet<i32> = HashSet::new();
        for num_1 in lines {
            for num_2 in lines {
                let comp = 2020 - num_1 - num_2;
                if seen.contains(&comp) {
                    return num_1 * num_2 * comp;
                }
                seen.insert(*num_1);
                seen.insert(*num_2);
            }
        }

        panic!("Did not find match");
    }
}
