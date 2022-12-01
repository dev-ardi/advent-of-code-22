#![allow(dead_code)]

use std::cmp::max;

fn main() {
    println!("{}", day1());
}




fn day1() -> String {
    let data: &'static str = include_str!("./day1.txt");

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
            if sto[0] < curr {sto[0] = curr}
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
