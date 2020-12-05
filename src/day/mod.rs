use aocf::Aoc;

mod day_01;
mod day_02;
mod day_03;
mod day_04;
mod day_05;
mod day_06;
mod day_07;
mod day_08;
mod day_09;
mod day_10;
mod day_11;
mod day_12;
mod day_13;
mod day_14;
mod day_15;
mod day_16;
mod day_17;
mod day_18;
mod day_19;
mod day_20;
mod day_21;
mod day_22;
mod day_23;
mod day_24;
mod day_25;

type Solution = (Option<i32>, Option<i32>);

#[derive(PartialEq)]
pub enum Part {
    ONE,
    TWO,
    BOTH,
}

impl Part {
    fn includes_1(&self) -> bool {
        *self == Part::BOTH || *self == Part::ONE
    }
    fn includes_2(&self) -> bool {
        *self == Part::BOTH || *self == Part::TWO
    }
}

pub trait Day {
    fn day(&self) -> u32;

    fn daily_input(&self) -> String {
        let day = self.day();
        let mut aoc = Aoc::new().year(Some(2020)).day(Some(day)).init().unwrap();
        let input = aoc.get_input(false).unwrap();
        input
    }

    fn solve(&self) -> Solution {
        (None, None)
    }
}

pub fn solve(day: u32, part: Part) {
    let (part_1, part_2) = match day {
        01 => day_01::DaySln {}.solve(),
        02 => day_02::DaySln {}.solve(),
        03 => day_03::DaySln {}.solve(),
        04 => day_04::DaySln {}.solve(),
        05 => day_05::DaySln {}.solve(),
        06 => day_06::DaySln {}.solve(),
        07 => day_07::DaySln {}.solve(),
        08 => day_08::DaySln {}.solve(),
        09 => day_09::DaySln {}.solve(),
        10 => day_10::DaySln {}.solve(),
        11 => day_11::DaySln {}.solve(),
        12 => day_12::DaySln {}.solve(),
        13 => day_13::DaySln {}.solve(),
        14 => day_14::DaySln {}.solve(),
        15 => day_15::DaySln {}.solve(),
        16 => day_16::DaySln {}.solve(),
        17 => day_17::DaySln {}.solve(),
        18 => day_18::DaySln {}.solve(),
        19 => day_19::DaySln {}.solve(),
        20 => day_20::DaySln {}.solve(),
        21 => day_21::DaySln {}.solve(),
        22 => day_22::DaySln {}.solve(),
        23 => day_23::DaySln {}.solve(),
        24 => day_24::DaySln {}.solve(),
        25 => day_25::DaySln {}.solve(),
        _ => panic!("invalid day"),
    };

    if part.includes_1() {
        println!("Day {} Part 1: {:?}", day, part_1);
    }

    if part.includes_2() {
        println!("Day {} Part 2: {:?}", day, part_2);
    }
}
