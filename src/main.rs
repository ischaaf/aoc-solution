extern crate regex;
#[macro_use]
extern crate lazy_static;

mod utils;
mod day_1;
mod day_2;
mod day_3;
mod day_4;

fn main() {
    println!(">>> Day 1 Part 1");
    day_1::solve_part_1();
    println!(">>> Day 1 Part 2");
    day_1::solve_part_2();

    println!(">>> Day 2 Part 1");
    day_2::solve_part_1();
    println!(">>> Day 2 Part 2");
    day_2::solve_part_2();

    println!(">>> Day 3 Part 1");
    day_3::solve_part_1();
    println!(">>> Day 3 Part 2");
    day_3::solve_part_2();

    println!(">>> Day 4 Part 1");
    day_4::solve_part_1();
    println!(">>> Day 4 Part 2");
    day_4::solve_part_2();
}
