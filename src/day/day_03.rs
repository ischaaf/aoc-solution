use crate::day::Day;

type Forest = Vec<Vec<u32>>;

fn count_trees(forest: &Forest, over: usize, down: usize) -> u32 {
    forest
        .iter()
        .step_by(down)
        .enumerate()
        .map(|(i, row)| row[(i * over) % row.len()])
        .sum()
}

pub struct DaySln {}

impl Day for DaySln {
    fn day(&self) -> u32 { 3 }
    fn solve_part_1(&self) {
        let forest = self.build_input();
        let trees = count_trees(&forest, 3, 1);
        println!("Encountered {} trees", trees);
    }

    fn solve_part_2(&self) {
        let forest = self.build_input();
        let slopes: [(usize, usize); 5] = [(1, 1), (3, 1), (5, 1), (7, 1), (1, 2)];
        let prod: u32 = slopes
            .iter()
            .map(|(o, d)| count_trees(&forest, *o, *d))
            .product();
        println!("combined values are: {}", prod);
    }
}

impl DaySln {
    fn build_input(&self) -> Vec<Vec<u32>> {
        self.daily_input()
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
            .collect()
    }
}
