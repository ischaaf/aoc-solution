use crate::day::{Day, Solution};
use regex::Regex;
use std::collections::{HashMap,HashSet};

pub struct DaySln {}

impl Day for DaySln {
    fn day(&self) -> u32 {
        7
    }

    fn solve(&self) -> Solution {
        let rules = self.build_rules();

        // Part 1
        let inversion: HashMap<String, Vec<String>> = rules
            .iter()
            .flat_map(|(k, v)| {
                v.keys()
                    .map(|k2| (k2.clone(), k.clone()))
                    .collect::<Vec<(String, String)>>()
            })
            .fold(HashMap::new(), |mut acc, p| {
                if acc.contains_key(&p.0) {
                    acc.get_mut(&p.0).unwrap().push(p.1);
                } else {
                    acc.insert(p.0, vec![p.1]);
                }
                acc
            });

        let search = "shiny gold".to_string();
        let mut queue = vec![&search];
        let mut result = HashSet::new();
        while let Some(val) = queue.pop() {
            result.insert(val);
            if let Some(containers) = inversion.get(val) {
                queue.append(&mut containers.iter().collect())
            }
        }
        result.remove(&search);
        let part_1 = result.len() as i32;

        // Part 2
        let mut sum = 0;
        let mut queue = vec![(&search, 1)];
        while let Some((bag, count)) = queue.pop() {
            sum += count; // add this bag
            if let Some(content) = rules.get(bag) {
                queue.append(&mut content.iter().map(|(k, v)| (k, count * v)).collect());
            }
        }
        sum -= 1; // subtract the shiny gold bag

        (Some(part_1), Some(sum))
    }
}

impl DaySln {
    fn build_rules(&self) -> HashMap<String, HashMap<String, i32>> {
        let re = Regex::new(r"([0-9]+) (\w+ \w+) bags?\.?").unwrap();
        self
            .daily_input()
            .lines()
            .map(|l| l.split(" bags contain "))
            .map(|mut p| {
                (
                    p.next().unwrap().to_string(),
                    p.next()
                        .unwrap()
                        .split(", ")
                        .filter(|e| e != &"no other bags.")
                        .map(|r| re.captures_iter(r).next().unwrap())
                        .map(|cap| {
                            (
                                cap[2].to_string(),
                                cap[1].to_string().parse::<i32>().unwrap(),
                            )
                        })
                        .collect(),
                )
            })
            .collect()

    }
}
