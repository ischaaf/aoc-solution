use crate::utils;
use regex::Regex;
use crate::day::Day;
use std::collections::HashMap;

const REQUIRED: [&'static str; 7] = ["byr", "iyr", "eyr", "hgt", "hcl", "ecl", "pid"];

lazy_static! {
    static ref PID_RE: Regex = Regex::new(r"^[0-9]{9}$").unwrap();
    static ref ECL_RE: Regex = Regex::new(r"^amb|blu|brn|gry|grn|hzl|oth$").unwrap();
    static ref HCL_RE: Regex = Regex::new(r"^#[0-9a-f]{6}$").unwrap();
    static ref HGT_RE: Regex = Regex::new(r"^([0-9]+)(cm|in)$").unwrap();
}

fn digit_match(val: &String, min: u32, max: u32) -> bool {
    match val.parse::<u32>() {
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

fn run_check<F: FnMut(&HashMap<String, String>) -> bool>(validate: F) -> usize {
    utils::read_lines("data/day_4/input.txt")
        .map(|l| {
            l.split_whitespace()
                .map(|e| {
                    let mut items = e.split(":");
                    (
                        items.next().unwrap().to_string(),
                        items.next().unwrap().to_string(),
                    )
                })
                .collect::<Vec<(String, String)>>()
        })
        .fold(vec![HashMap::new()], |mut acc, eles| {
            if eles.len() == 0 {
                acc.push(HashMap::new());
            } else {
                let len = acc.len();
                let map = &mut acc[len - 1];
                for (key, val) in eles {
                    map.insert(key, val);
                }
            }
            acc
        })
        .iter()
        .map(validate)
        .filter(|v| *v)
        .count()
}

pub struct DaySln {}

impl Day for DaySln {
    fn day(&self) -> u32 { 4 }
    fn solve_part_1(&self) {
        let result = run_check(|pport| {
            REQUIRED
                .iter()
                .map(|key| pport.contains_key(&key.to_string()))
                .all(|v| v)
        });
        println!("Found {} valid passports", result);
    }

    fn solve_part_2(&self) {
        let result: usize = run_check(|pport| {
            REQUIRED
                .iter()
                .map(|key| is_valid(key, pport.get(&key.to_string())))
                .all(|v| v)
        });
        println!("Found {} valid passports", result);
    }
}