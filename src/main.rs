use std::fmt::Debug;
use std::fs;
use std::vec::Vec;
use std::collections::HashSet;
use std::str::FromStr;

fn main() {
    day_4();
}

fn day_1() {
    let contents = fs::read_to_string("input/day1.txt")
        .expect("Should have been able to open te file");

    let mut current_calories: u32 = 0;
    let mut calorie_counts = Vec::new();
    for line in contents.lines() {
        if line == "" {
            calorie_counts.push(current_calories);
            current_calories = 0;
        } else {
            current_calories += line.parse::<u32>().unwrap();
            // println!("{current_calories}");
        }
    }

    let most_calories = calorie_counts.iter().max().unwrap();
    let elf = calorie_counts.iter().position(|&x| x == *most_calories).unwrap();
    println!("Elf with most calories has {} calories.", calorie_counts[elf]);

    calorie_counts.sort();
    let top_three: u32 = calorie_counts.iter().rev().take(3).sum();
    println!("Top three elfs with most calories have {top_three} calories.")
}

fn day_2() {
    let contents = fs::read_to_string("input/day2.txt")
        .expect("Should have been able to open te file");

    let mut score = 0;
    for line in contents.lines() {
        score += rps_round_score1(line);
    }

    println!("Following this strategy you score {score} points.");

    let mut score = 0;
    for line in contents.lines() {
        score += rps_round_score2(line);
    }

    println!("Following the second strategy you score {score} points.");

}

fn rps_round_score1(s: &str) -> u32 {
    match s {
        "A X" => 1+3,
        "A Y" => 2+6,
        "A Z" => 3+0,
        "B X" => 1+0,
        "B Y" => 2+3,
        "B Z" => 3+6,
        "C X" => 1+6,
        "C Y" => 2+0,
        "C Z" => 3+3,
        &_ => 0
    }
}

fn rps_round_score2(s: &str) -> u32 {
    match s {
        "A X" => 0+3,
        "A Y" => 3+1,
        "A Z" => 6+2,
        "B X" => 0+1,
        "B Y" => 3+2,
        "B Z" => 6+3,
        "C X" => 0+2,
        "C Y" => 3+3,
        "C Z" => 6+1,
        &_ => 0
    }
}

fn day_3() {
    let contents = fs::read_to_string("input/day3.txt")
        .expect("Should have been able to open te file");

    let mut sum = 0;

    for line in contents.lines() {
        

        let first_half = HashSet::<_>::from_iter(line.chars().take(line.len()/2));
        let second_half = HashSet::<_>::from_iter(line.chars().rev().take(line.len()/2));
        let mut intersection: Vec<&char> = first_half.intersection(&second_half).collect();
        let ascii_value: u32 = *intersection.pop().unwrap() as u32;
        let priority: u32;
        if ascii_value >= 97 {
            priority = ascii_value - 96; 
        } else {
            priority = ascii_value - 38;
        }
        
        sum += priority;
        
    }

    println!("Sum of priorities is {sum}");

    let mut c: u32 = 0;
    sum = 0;
    let mut elf_1: HashSet<char> = HashSet::new();
    let mut elf_2: HashSet<char> = HashSet::new();
    let mut elf_3: HashSet<char> = HashSet::new();
    for line in contents.lines() {
        match c % 3 {
            0 => elf_1 = HashSet::<_>::from_iter(line.chars()),
            1 => elf_2 = HashSet::<_>::from_iter(line.chars()),
            2 => elf_3 = HashSet::<_>::from_iter(line.chars()),
            _ => todo!()
        }

        if c % 3 == 2 {
            let mut intersection = &(&elf_1 & &elf_2) & &elf_3;
            let ascii_value: u32 = intersection.drain().last().unwrap() as u32;
            let priority: u32;
            if ascii_value >= 97 {
                priority = ascii_value - 96; 
            } else {
                priority = ascii_value - 38;
            }

            sum += priority;
        }

        c += 1;
    }

    println!("Sum of priorities is {sum}");

}

fn day_4() {
    let contents = fs::read_to_string("input/day4.txt")
        .expect("Should have been able to open te file");

    #[derive(Debug)]
    struct Assignment {
        start: u32,
        end: u32
    }

    impl Assignment {
        fn contains(&self, other: &Assignment) -> bool {
            self.start <= other.start && self.end >= other.end
        }

        fn overlaps(&self, other: &Assignment) -> bool {
            (self.start <= other.start && other.start <= self.end) || (self.start <= other.end && other.end <= self.end) || (other.start <= self.start && other.end >= self.end)
        }
    }

    impl TryFrom<&str> for Assignment {
        type Error = &'static str;

        fn try_from(s: &str) -> Result<Self, Self::Error> {
            // expected input example: "5-20"
            let mut parts = s.split("-");
            let (start, end) = (u32::from_str(parts.next().unwrap()).unwrap() as u32, u32::from_str(parts.next().unwrap()).unwrap() as u32);
            Ok(Self {
                start,
                end
            })
        }
    }

    let mut to_reconsider = 0;
    let mut overlapping = 0;

    for line in contents.lines() {
        let mut split = line.split(",");
        let (one, two) = (Assignment::try_from(split.next().unwrap()), Assignment::try_from(split.next().unwrap()));

        match (&one, &two) {
            (Ok(one), Ok(two)) => if one.contains(&two) || two.contains(&one) {to_reconsider += 1},
            _ => todo!()
        }

        match (one, two) {
            (Ok(one), Ok(two)) => if one.overlaps(&two)  {overlapping += 1},
            _ => todo!()
        }
    }

    println!("{to_reconsider} pairs fully contain the other.");
    println!("{overlapping} pairs overlap with each the other.");


}