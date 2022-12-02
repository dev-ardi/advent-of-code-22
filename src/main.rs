#![allow(dead_code)]

use std::{cmp::max, fmt::format};

fn main() {
    day2();
}

fn day2() {
    // A: Rock(1) B: Paper(2) C: scissors(3)
    let data = include_str!("./day2.txt");

    let problem1 = || {
        let mut total = 0;
        for i in data.lines() {
            let mut line = i.chars();

            match line.nth(0).unwrap() {
                'A' => match line.nth(1).unwrap() {
                    'X' => total += 3 + 1,
                    'Y' => total += 6 + 2,
                    'Z' => total += 0 + 3,
                    _ => (),
                },
                'B' => match line.nth(1).unwrap() {
                    'X' => total += 0 + 1,
                    'Y' => total += 3 + 2,
                    'Z' => total += 6 + 3,
                    _ => (),
                },
                'C' => match line.nth(1).unwrap() {
                    'X' => total += 6 + 1,
                    'Y' => total += 0 + 2,
                    'Z' => total += 3 + 3,
                    _ => (),
                },
                _ => (),
            }
        }
        println!("{}", total)
    };
    let problem2 = || {
        //X: lose Y: draw Z: win
        let mut total = 0;
        for i in data.lines() {
            let mut line = i.chars();

            match line.nth(0).unwrap() {
                'A' => match line.nth(1).unwrap() {
                    'X' => total += 0 + 3,
                    'Y' => total += 3 + 1,
                    'Z' => total += 6 + 2,
                    _ => (),
                },
                'B' => match line.nth(1).unwrap() {
                    'X' => total += 0 + 1,
                    'Y' => total += 3 + 2,
                    'Z' => total += 6 + 3,
                    _ => (),
                },
                'C' => match line.nth(1).unwrap() {
                    'X' => total += 0 + 2,
                    'Y' => total += 3 + 3,
                    'Z' => total += 6 + 1,
                    _ => (),
                },
                _ => (),
            }
        }
        println!("{}", total)
    };

    problem2();
}

fn day1() -> String {
    let data = include_str!("./day1.txt");

    // This could also be solved as problem2's `sto.iter().max().unwrap()`
    let problem1 = || {
        let mut biggest = 0;
        let mut curr = 0;
        for i in data.lines() {
            if i != "" {
                curr += i.parse::<usize>().unwrap();
                continue;
            }
            biggest = max(biggest, curr);
            curr = 0;
        }
        format!("Solution to problem #1: {}", biggest)
    };

    let problem2 = || {
        let mut sto: Vec<usize> = vec![0, 0, 0];
        let mut curr = 0;
        for line in data.lines() {
            if line != "" {
                curr += line.parse::<usize>().unwrap();
                continue;
            }
            sto.sort();
            if sto[0] < curr {
                sto[0] = curr
            }
            curr = 0;
        }

        let sol: usize = sto.into_iter().sum();
        format!("Solution to problem #2: {}", sol)
    };

    format!(
        "
    {}
    {}
    ",
        problem1(),
        problem2()
    )
}
