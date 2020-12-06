use crate::day::{Day, Solution};
use regex::Regex;
use std::collections::HashMap;

const REQUIRED: [&'static str; 7] = ["byr", "iyr", "eyr", "hgt", "hcl", "ecl", "pid"];

lazy_static! {
    static ref PID_RE: Regex = Regex::new(r"^[0-9]{9}$").unwrap();
    static ref ECL_RE: Regex = Regex::new(r"^amb|blu|brn|gry|grn|hzl|oth$").unwrap();
    static ref HCL_RE: Regex = Regex::new(r"^#[0-9a-f]{6}$").unwrap();
    static ref HGT_RE: Regex = Regex::new(r"^([0-9]+)(cm|in)$").unwrap();
}

fn digit_match(val: &String, min: i32, max: i32) -> bool {
    match val.parse::<i32>() {
        Ok(num) => num >= min && num <= max,
        Err(_) => false,
    }
}

fn height_check(val: &String) -> bool {
    let cap_res = HGT_RE.captures_iter(val.as_str()).next();
    if cap_res.is_none() {
        return false;
    }

    let cap = cap_res.unwrap();
    let num = cap[1].to_string();
    match &cap[2] {
        "in" => digit_match(&num, 59, 76),
        "cm" => digit_match(&num, 150, 193),
        _ => false,
    }
}

fn is_valid(key: &str, val: Option<&String>) -> bool {
    if val.is_none() {
        return false;
    }
    let val = val.unwrap();
    match key {
        "byr" => digit_match(&val, 1920, 2002),
        "iyr" => digit_match(&val, 2010, 2020),
        "eyr" => digit_match(&val, 2020, 2030),
        "hgt" => height_check(&val),
        "hcl" => HCL_RE.is_match(val.as_str()),
        "ecl" => ECL_RE.is_match(val.as_str()),
        "pid" => PID_RE.is_match(val.as_str()),
        "cid" => true,
        _ => panic!("Unknown key"),
    }
}

pub struct DaySln {}

impl Day for DaySln {
    fn day(&self) -> u32 {
        4
    }

    fn solve(&self) -> Solution {
        let part_1 = self.run_check(|pport| {
            REQUIRED
                .iter()
                .map(|key| pport.contains_key(&key.to_string()))
                .all(|v| v)
        });

        let part_2: i32 = self.run_check(|pport| {
            REQUIRED
                .iter()
                .map(|key| is_valid(key, pport.get(&key.to_string())))
                .all(|v| v)
        });
        (Some(part_1), Some(part_2))
    }
}

impl DaySln {
    fn run_check<F: FnMut(HashMap<String, String>) -> bool>(&self, validate: F) -> i32 {
        self.daily_input()
            .split("\n\n")
            .map(|l| {
                l.split_whitespace()
                    .map(|e| {
                        let mut items = e.split(":").map(|v| v.to_string());
                        (items.next().unwrap(), items.next().unwrap())
                    })
                    .collect::<HashMap<String, String>>()
            })
            .map(validate)
            .filter(|v| *v)
            .count() as i32
    }
}
