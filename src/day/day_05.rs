use crate::day::Day;

pub struct DaySln {}

impl Day for DaySln {
    fn day(&self) -> u32 {
        5
    }

    fn solve_part_1(&self) {
        let max = self.seats().into_iter().max().unwrap();
        println!("Max value is: {}", max);
    }

    fn solve_part_2(&self) {
        let mut seats = self.seats();
        seats.sort();
        let mut prev = seats[0];
        let mut gap = None;
        for s in seats[1..].iter() {
            if prev + 2 == *s {
                println!("Found gap: {} and {}", prev, s);
                gap = Some(prev + 1);
            }
            prev = *s;
        }
        match gap {
            Some(val) => println!("Found Gap: {}", val),
            None => println!("Did not find a gap"),
        }
    }
}

impl DaySln {
    #[allow(dead_code)]
    fn fake_input(&self) -> String {
        return String::from("FBFBBFFRLR");
    }

    fn seats(&self) -> Vec<u32> {
        self.daily_input()
            .lines()
            .map(|l| {
                l.chars()
                    .map(|c| match c {
                        'B' | 'R' => '1',
                        'F' | 'L' => '0',
                        _ => panic!(),
                    })
                    .collect::<String>()
            })
            .map(|l| u32::from_str_radix(l.as_str(), 2).unwrap())
            .collect()
    }
}
