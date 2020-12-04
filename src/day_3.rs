use crate::utils;

const MASK: u32 = 0x00000001;

#[derive(Debug)]
struct Row {
    d: u32,
}

impl Row {
    pub fn from_line(line: String) -> Self {
        let b2_str: String = line.replace(".", "0").replace("#", "1").chars().rev().collect();
        Self {
            d: u32::from_str_radix(b2_str.as_str(), 2).unwrap(),
        }
    }

    #[inline(always)]
    pub fn at(&self, idx: u8) -> u32 {
        (self.d >> idx) & MASK
    }
}

#[derive(Debug)]
struct Forest {
    rows: Vec<Row>,
    row_len: u8,
}

impl Forest {
    pub fn traverse(&self, over: u8, down: u8, row_offset: usize, col_offset: u8) -> u32 {
        if row_offset >= self.rows.len() {
            return 0;
        }
        self.rows[row_offset].at(col_offset)
            + self.traverse(
                over,
                down,
                row_offset + down as usize,
                (col_offset + over) % self.row_len,
            )
    }
}

fn build_input() -> Forest {
    let mut lines = utils::read_lines("data/day_3/input.txt").peekable();
    let size = lines.peek().unwrap().len();
    let rows = lines.map(|l| Row::from_line(l)).collect();
    Forest { rows: rows, row_len: size as u8}
}

pub fn solve_part_1() {
    let forest = build_input();
    let trees = forest.traverse(3, 1, 0, 0);
    println!("Encountered {} trees", trees);
}

pub fn solve_part_2() {
    let forest = build_input();
    let slopes: [(u8,u8);5] = [(1, 1), (3, 1), (5, 1), (7, 1), (1, 2)];
    let prod: u32 = slopes.iter().map(|(o, d)| forest.traverse(*o, *d, 0, 0)).product();
    println!("combined values are: {}", prod);
}
