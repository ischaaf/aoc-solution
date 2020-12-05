use crate::day::{Day,Solution};

pub struct DaySln {}

impl Day for DaySln {
    fn day(&self) -> u32 {
        5
    }

    fn solve(&self) -> Solution {
        let seats: Vec<_> = self.daily_input()
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

        let max = *seats.iter().max().unwrap();
        let min = *seats.iter().min().unwrap();
        let act_sum: i32 = seats.iter().sum();
        let exp_sum: i32 = (min..max+1).sum();
        let diff = exp_sum - act_sum;


        (Some(max), Some(diff))
    }
}
