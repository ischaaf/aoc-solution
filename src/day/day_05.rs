use crate::day::{Day,Solution};

pub struct DaySln {}

impl Day for DaySln {
    fn day(&self) -> u32 {
        5
    }

    fn solve(&self) -> Solution {
        let mut seats: Vec<i32> = self.daily_input()
            .lines()
            .map(|l| {
                l.chars()
                    .map(|c| match c {
                        'B' | 'R' => '1',
                        'F' | 'L' => '0',
                        _ => panic!(),
                    })
                    .collect::<String>()
            })
            .map(|l| i32::from_str_radix(l.as_str(), 2).unwrap())
            .collect();

        seats.sort_unstable();
        let part_1 = *seats.last().unwrap();

        let part_2 = seats
            .windows(2)
            .filter(|p| p[0] + 2 == p[1])
            .next()
            .unwrap()[0];

        (Some(part_1), Some(part_2))
    }
}
