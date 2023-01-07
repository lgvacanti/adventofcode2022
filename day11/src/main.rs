use std::fs;

fn main() {
    day_11();
}

struct Monkey {
    items: Vec<u32>,
    operation: fn(u32) -> bool,
}

fn day_11() {
    let contents =
        fs::read_to_string("../input/day11.txt").expect("Should have been able to open te file");
    println!("{contents:?}");
}
