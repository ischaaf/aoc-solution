use crate::day::Day;
use std::collections::HashSet;

pub struct DaySln {}

impl Day for DaySln {
    fn day(&self) -> u32 {
        1
    }

    fn solve_part_1(&self) {
        let lines = self.get_input_ints();
        let mut seen: HashSet<i32> = HashSet::new();
        for num in lines {
            let comp = 2020 - num;
            if seen.contains(&comp) {
                println!("Found {}", num * comp);
                return;
            }
            seen.insert(num);
        }
        println!("Did not find match");
    }

    fn solve_part_2(&self) {
        let lines = self.get_input_ints();
        let mut seen: HashSet<i32> = HashSet::new();
        for num_1 in &lines {
            for num_2 in &lines {
                let comp = 2020 - num_1 - num_2;
                if seen.contains(&comp) {
                    println!("Found {}", num_1 * num_2 * comp);
                    return;
                }
                seen.insert(*num_1);
                seen.insert(*num_2);
            }
        }
        println!("Did not find match");
    }
}

impl DaySln {
    fn get_input_ints(&self) -> Vec<i32> {
        self.daily_input()
            .lines()
            .map(|e| e.parse::<i32>().unwrap())
            .collect()
    }
}
