use std::fs;
use std::vec::Vec;

fn main() {
    day_1();
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