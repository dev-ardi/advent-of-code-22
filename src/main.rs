#![allow(dead_code)]

use std::{cmp::max, fmt::Debug, str::FromStr, vec};

fn main() {
    day5();
    //day4();
    //day3();
    //day2();
    //day1();
}

fn day5() {
    let data: Vec<Vec<usize>> = numberify::<usize>(include_str!("./day5.txt"));
    let mut hardcoded: Vec<Vec<char>> = vec![
        vec!['N', 'W', 'B'],
        vec!['B', 'M', 'D', 'T', 'P', 'S', 'Z', 'L'],
        vec!['R', 'W', 'Z', 'H', 'Q'],
        vec!['R', 'Z', 'J', 'V', 'D', 'W'],
        vec!['B', 'M', 'H', 'S'],
        vec!['B', 'P', 'V', 'H', 'J', 'N', 'G', 'L'],
        vec!['S', 'L', 'D', 'H', 'F', 'Z', 'Q', 'J'],
        vec!['B', 'Q', 'G', 'J', 'F', 'S', 'W'],
        vec!['J', 'D', 'C', 'S', 'M', 'W', 'Z'],
    ];
    //let mut hardcoded = vec![vec!['N', 'Z'], vec!['D', 'C', 'M'], vec!['P']];

    for i in 0..hardcoded.len() {
        hardcoded[i].reverse();
    }

    //    let problem1 = || {
    //for line in data {
    //let [qty, from, to]: [usize; 3] = line.try_into().unwrap();
    //for _ in 0..qty {
    //let x = hardcoded[from - 1].pop().unwrap();
    //hardcoded[to - 1].push(x);
    //}
    //}
    //let str: String = hardcoded.iter().map(|x| x.last().unwrap()).collect();
    //println!(" {}", str);
    //};
    let problem2 = || {
        for line in data {
            let [qty, from, to]: [usize; 3] = line.try_into().unwrap();
            let mut a = Vec::new();
            for _ in 0..qty {
                let x = hardcoded[from - 1].pop().unwrap();
                a.push(x);
            }
            a.reverse();
            hardcoded[to - 1].append(&mut a);
        }
        let str: String = hardcoded.iter().map(|x| x.last().unwrap()).collect();
        println!(" {}", str);
    };
    problem2();
}

fn day4() {
    let data: Vec<Vec<i32>> = numberify(include_str!("./day4.txt"));

    let problem1 = || {
        let mut accumulator = 0;

        for word in &data {
            if word[0] <= word[2] && word[1] >= word[3] || word[0] >= word[2] && word[1] <= word[3]
            {
                accumulator += 1;
            }
        }
        dbg!(accumulator);
    };

    let problem2 = || {
        let mut accumulator = 0;
        for word in &data {
            if word[0] > word[3] || word[1] < word[2] {
                accumulator += 1;
            }
        }
        println!("{}", data.len() - accumulator);
    };
    problem1();
    problem2();
}

fn day3() {
    let data = include_str!("./day3.txt");

    let problem1 = || {
        let res: u32 = data
            .lines()
            .map(|line| {
                let s1 = &line[0..line.len() / 2];
                let s2 = &line[line.len() / 2..];

                let mut a = s1.chars().collect::<Vec<char>>();
                a.sort_unstable();
                a.dedup();
                a.iter()
                    .filter_map(|x| if s2.contains(*x) { Some(*x) } else { None })
                    .map(transform)
                    .sum::<u32>()
            })
            .sum();

        println!("{}", res);
    };
    let problem2 = || {
        let iter: Vec<&str> = data.lines().collect();
        let mut accumulator = 0;
        for i in iter.chunks(3) {
            for j in i[0].chars() {
                if i[1].contains(j) && i[2].contains(j) {
                    accumulator += transform(j);
                    break;
                }
            }
        }

        println!("{}", accumulator)
    };

    problem1();
    problem2();

    fn transform(input: char) -> u32 {
        let ascii = input as u32;
        if input.is_ascii_lowercase() {
            // lowercase
            return ascii - 0x60;
        }
        (ascii - 0x40) + 26
    }
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
    problem1();
    problem2();
}

fn day1() -> String {
    let data = include_str!("./day1.txt");

    // This could also be solved as problem2's `sto.iter().max().unwrap()`
    let problem1 = || {
        let mut biggest = 0;
        let mut curr = 0;
        for i in data.lines() {
            if !i.is_empty() {
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
            if !line.is_empty() {
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

fn numberify<T: std::str::FromStr>(str: &str) -> Vec<Vec<T>>
where
    <T as FromStr>::Err: Debug,
{
    str.lines()
        .map(|line| {
            let mut tmp = String::new();
            let mut word = Vec::new();
            for ch in line.chars() {
                if !ch.is_numeric() {
                    if !tmp.is_empty() {
                        word.push(tmp.parse::<T>().unwrap());
                        tmp = String::new();
                    }
                } else {
                    tmp.push(ch)
                }
            }

            word.push(tmp.parse::<T>().unwrap());
            word
        })
        .collect()
}
