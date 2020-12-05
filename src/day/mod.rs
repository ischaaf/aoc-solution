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

pub enum Part {
    ONE,
    TWO,
    BOTH,
}

pub trait Day {

    fn day(&self) -> u32;

    fn daily_input(&self) -> String {
        let day = self.day();
        let mut aoc = Aoc::new().year(Some(2020)).day(Some(day)).init().unwrap();
        let input = aoc.get_input(false).unwrap();
        input
    }

    fn solve_part_1(&self) {
        println!("Day {} Part 1 not implemented", self.day());
    }
    fn solve_part_2(&self) {
        println!("Day {} Part 2 not implemented", self.day());
    }

    fn solve(&self, part: Part) {
        match part {
            Part::ONE => self.solve_part_1(),
            Part::TWO => self.solve_part_2(),
            Part::BOTH => {
                self.solve_part_1();
                self.solve_part_2();
            }
        }
    }
}

pub fn solve(day: u32, part: Part) {
    match day {
        01 => day_01::DaySln{}.solve(part),
        02 => day_02::DaySln{}.solve(part),
        03 => day_03::DaySln{}.solve(part),
        04 => day_04::DaySln{}.solve(part),
        05 => day_05::DaySln{}.solve(part),
        06 => day_06::DaySln{}.solve(part),
        07 => day_07::DaySln{}.solve(part),
        08 => day_08::DaySln{}.solve(part),
        09 => day_09::DaySln{}.solve(part),
        10 => day_10::DaySln{}.solve(part),
        11 => day_11::DaySln{}.solve(part),
        12 => day_12::DaySln{}.solve(part),
        13 => day_13::DaySln{}.solve(part),
        14 => day_14::DaySln{}.solve(part),
        15 => day_15::DaySln{}.solve(part),
        16 => day_16::DaySln{}.solve(part),
        17 => day_17::DaySln{}.solve(part),
        18 => day_18::DaySln{}.solve(part),
        19 => day_19::DaySln{}.solve(part),
        20 => day_20::DaySln{}.solve(part),
        21 => day_21::DaySln{}.solve(part),
        22 => day_22::DaySln{}.solve(part),
        23 => day_23::DaySln{}.solve(part),
        24 => day_24::DaySln{}.solve(part),
        25 => day_25::DaySln{}.solve(part),
        _ => println!("Day {} not implemented", day),
    }
}

