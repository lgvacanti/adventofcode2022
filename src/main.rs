use std::cmp;
use std::collections::{HashMap, HashSet, VecDeque};
use std::fmt::Debug;
use std::fs;
use std::str::FromStr;
use std::vec::Vec;

fn main() {
    day_7();
}

fn day_7() {
    let contents =
        fs::read_to_string("input/day7.txt").expect("Should have been able to open te file");

    let mut cwd: Vec<&str> = Vec::new();

    let mut directory_sizes: HashMap<String, u64> = HashMap::new();

    for line in contents.lines() {
        println!("line: {}", line);
        if line.starts_with("$") {
            match line {
                "$ cd /" => {
                    cwd.clear();
                }
                "$ cd .." => {
                    cwd.pop();
                }
                "$ ls" => (),
                x => cwd.push(&x[5..]),
            }
            //println!("{}", cwd.join("/"));
        } else if !line.starts_with("dir") {
            //dbg!(&cwd);
            let mut directories: Vec<String> = Vec::new();
            let mut tmp = cwd.clone();

            for _ in 0..=tmp.len() {
                directories.push(tmp.join("/"));
                tmp.pop();
            }
            //dbg!(&directories);
            for dir in directories {
                if directory_sizes.get(&dir).is_none() {
                    directory_sizes.insert(
                        dir,
                        line.split_ascii_whitespace()
                            .next()
                            .unwrap()
                            .parse()
                            .unwrap(),
                    );
                } else {
                    let x = directory_sizes.get(&dir).unwrap();
                    directory_sizes.insert(
                        dir,
                        line.split_ascii_whitespace()
                            .next()
                            .unwrap()
                            .parse::<u64>()
                            .unwrap()
                            + x,
                    );
                }
            }
        }

        //dbg!(&directory_sizes);
    }

    for (k, v) in directory_sizes.iter().filter(|(k, v)| v <= &&100000u64) {
        println!("{k} {v}");
    }

    let sum: u64 = directory_sizes.values().filter(|x| x <= &&100000u64).sum();
    println!("Sum of sizes of directories at most 100000: {}", sum);

    let need_to_delete = 30_000_000 - (70_000_000 - directory_sizes.get("").unwrap());
    println!("Size to be deleted {need_to_delete}");

    let smallest_enough = directory_sizes
        .values()
        .filter(|x| x >= &&need_to_delete)
        .min()
        .unwrap();

    println!("Smallest that is enough: {}", smallest_enough);
}

fn day_6() {
    let contents =
        fs::read_to_string("input/day6.txt").expect("Should have been able to open te file");

    let mut four = Vec::new();

    for (i, c) in contents.chars().enumerate() {
        four.push(c);
        let set = HashSet::<_>::from_iter(&four[cmp::max(i as i32 - 3, 0) as usize..]);
        if set.len() == 4 {
            println!("{}", i + 1);
            break;
        }
    }

    let mut four = Vec::new();

    for (i, c) in contents.chars().enumerate() {
        four.push(c);
        let set = HashSet::<_>::from_iter(&four[cmp::max(i as i32 - 13, 0) as usize..]);
        if set.len() == 14 {
            println!("{}", i + 1);
            break;
        }
    }

    // println!("{}", contents);
}

fn day_5() {
    let contents =
        fs::read_to_string("input/day5.txt").expect("Should have been able to open te file");

    let mut state: Vec<Vec<char>> = vec![
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
    ];

    load_state(&mut state, &contents);

    // println!("{state:?}");

    for line in contents.lines() {
        if line.starts_with("move") {
            move_crates(line, &mut state);
        }
    }

    println!("CrateMover 9000");
    for mut v in state {
        print!("{}", v.pop().unwrap());
    }
    println!();

    let mut state: Vec<Vec<char>> = vec![
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
    ];

    load_state(&mut state, &contents);

    for line in contents.lines() {
        if line.starts_with("move") {
            move_crates2(line, &mut state);
        }
    }

    println!("CrateMover9001");
    for mut v in state {
        print!("{}", v.pop().unwrap());
    }
}

fn move_crates2(line: &str, state: &mut Vec<Vec<char>>) {
    let mut split = line.split(" ");

    let mut intermediate: Vec<char> = vec![];

    let (n, i1, i2);

    if let (_, Some(a), _, Some(b), _, Some(c)) = (
        split.next(),
        split.next(),
        split.next(),
        split.next(),
        split.next(),
        split.next(),
    ) {
        n = usize::from_str(a).unwrap();
        i1 = usize::from_str(b).unwrap() - 1;
        i2 = usize::from_str(c).unwrap() - 1;
    } else {
        todo!()
    };

    for _ in 0..n {
        let c = state[i1].pop().unwrap();
        intermediate.push(c);
    }

    for _ in 0..n {
        let c = intermediate.pop().unwrap();
        state[i2].push(c);
    }
}

fn move_crates(line: &str, state: &mut Vec<Vec<char>>) {
    let mut split = line.split(" ");

    let (n, i1, i2);

    if let (_, Some(a), _, Some(b), _, Some(c)) = (
        split.next(),
        split.next(),
        split.next(),
        split.next(),
        split.next(),
        split.next(),
    ) {
        n = usize::from_str(a).unwrap();
        i1 = usize::from_str(b).unwrap() - 1;
        i2 = usize::from_str(c).unwrap() - 1;
    } else {
        todo!()
    };

    for _ in 0..n {
        let c = state[i1].pop().unwrap();
        state[i2].push(c);
    }
}

fn load_state(state: &mut Vec<Vec<char>>, contents: &str) {
    for line in contents.lines() {
        for (i, c) in line.chars().enumerate() {
            // 1 -> 1, 5 -> 2, 9 -> 3, 13 -> 4
            if c.is_ascii_alphabetic() && c.is_uppercase() {
                state[(i + 3) / 4 - 1].push(c)
            }
        }
    }

    // Need to reverse the stacks since they were placed upside down.
    for vec in state {
        vec.reverse();
    }
}

fn day_1() {
    let contents =
        fs::read_to_string("input/day1.txt").expect("Should have been able to open te file");

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
    let elf = calorie_counts
        .iter()
        .position(|&x| x == *most_calories)
        .unwrap();
    println!(
        "Elf with most calories has {} calories.",
        calorie_counts[elf]
    );

    calorie_counts.sort();
    let top_three: u32 = calorie_counts.iter().rev().take(3).sum();
    println!("Top three elfs with most calories have {top_three} calories.")
}

fn day_2() {
    let contents =
        fs::read_to_string("input/day2.txt").expect("Should have been able to open te file");

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
        "A X" => 1 + 3,
        "A Y" => 2 + 6,
        "A Z" => 3 + 0,
        "B X" => 1 + 0,
        "B Y" => 2 + 3,
        "B Z" => 3 + 6,
        "C X" => 1 + 6,
        "C Y" => 2 + 0,
        "C Z" => 3 + 3,
        &_ => 0,
    }
}

fn rps_round_score2(s: &str) -> u32 {
    match s {
        "A X" => 0 + 3,
        "A Y" => 3 + 1,
        "A Z" => 6 + 2,
        "B X" => 0 + 1,
        "B Y" => 3 + 2,
        "B Z" => 6 + 3,
        "C X" => 0 + 2,
        "C Y" => 3 + 3,
        "C Z" => 6 + 1,
        &_ => 0,
    }
}

fn day_3() {
    let contents =
        fs::read_to_string("input/day3.txt").expect("Should have been able to open te file");

    let mut sum = 0;

    for line in contents.lines() {
        let first_half = HashSet::<_>::from_iter(line.chars().take(line.len() / 2));
        let second_half = HashSet::<_>::from_iter(line.chars().rev().take(line.len() / 2));
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
            _ => todo!(),
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
    let contents =
        fs::read_to_string("input/day4.txt").expect("Should have been able to open te file");

    #[derive(Debug)]
    struct Assignment {
        start: u32,
        end: u32,
    }

    impl Assignment {
        fn contains(&self, other: &Assignment) -> bool {
            self.start <= other.start && self.end >= other.end
        }

        fn overlaps(&self, other: &Assignment) -> bool {
            (self.start <= other.start && other.start <= self.end)
                || (self.start <= other.end && other.end <= self.end)
                || (other.start <= self.start && other.end >= self.end)
        }
    }

    impl TryFrom<&str> for Assignment {
        type Error = &'static str;

        fn try_from(s: &str) -> Result<Self, Self::Error> {
            // expected input example: "5-20"
            let mut parts = s.split("-");
            let (start, end) = (
                u32::from_str(parts.next().unwrap()).unwrap() as u32,
                u32::from_str(parts.next().unwrap()).unwrap() as u32,
            );
            Ok(Self { start, end })
        }
    }

    let mut to_reconsider = 0;
    let mut overlapping = 0;

    for line in contents.lines() {
        let mut split = line.split(",");
        let (one, two) = (
            Assignment::try_from(split.next().unwrap()),
            Assignment::try_from(split.next().unwrap()),
        );

        match (&one, &two) {
            (Ok(one), Ok(two)) => {
                if one.contains(&two) || two.contains(&one) {
                    to_reconsider += 1
                }
            }
            _ => todo!(),
        }

        match (one, two) {
            (Ok(one), Ok(two)) => {
                if one.overlaps(&two) {
                    overlapping += 1
                }
            }
            _ => todo!(),
        }
    }

    println!("{to_reconsider} pairs fully contain the other.");
    println!("{overlapping} pairs overlap with each the other.");
}
