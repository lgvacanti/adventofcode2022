use std::fs;
use std::vec::Vec;

fn main() {
    day_2();
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