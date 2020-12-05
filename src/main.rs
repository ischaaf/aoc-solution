extern crate regex;
#[macro_use]
extern crate lazy_static;

extern crate clap;

use clap::{Arg, App};

mod utils;
mod day;

use day::Part;



fn main() {

    let matches = App::new("Advent of Code Solution")
        .version("1.0")
        .author("Isaac S. <zeekus99@gmail.com>")
        .arg(Arg::with_name("day")
            .short("d")
            .long("day")
            .takes_value(true)
            .required(true)
        )
        .arg(Arg::with_name("part")
            .short("p")
            .long("part")
            .takes_value(true)
        )
        .get_matches();
    let day = matches.value_of("day").unwrap().parse::<u32>().unwrap();
    let part = match matches.value_of("part") {
        Some("1") => Part::ONE,
        Some("2") => Part::TWO,
        Some("both") | None => Part::BOTH,
        _ => panic!("Unknown part"),
    };
    day::solve(day, part);
}
