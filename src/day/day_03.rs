use crate::day::{Day, Solution};

type Forest = Vec<Vec<i32>>;

fn count_trees(forest: &Forest, over: usize, down: usize) -> i32 {
    forest
        .iter()
        .step_by(down)
        .enumerate()
        .map(|(i, row)| row[(i * over) % row.len()])
        .sum()
}

pub struct DaySln {}

impl Day for DaySln {
    fn day(&self) -> u32 {
        3
    }

    fn solve(&self) -> Solution {
        let forest = self
            .daily_input()
            .lines()
            .map(|l| {
                l.chars()
                    .map(|c| match c {
                        '.' => 0,
                        '#' => 1,
                        _ => panic!(),
                    })
                    .collect()
            })
            .collect();

        let part_1 = count_trees(&forest, 3, 1);

        let slopes: [(usize, usize); 5] = [(1, 1), (3, 1), (5, 1), (7, 1), (1, 2)];
        let part_2: i32 = slopes
            .iter()
            .map(|(o, d)| count_trees(&forest, *o, *d))
            .product();
        (Some(part_1), Some(part_2))
    }
}
