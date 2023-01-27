#[allow(unused_imports)]
use std::{
    collections::HashMap,
    env::args,
    hash::Hash,
    io::{stdin, BufRead},
    time::{Duration, Instant},
};

#[allow(dead_code)]
fn a(required: i32, lookup: &Vec<i32>) -> i32 {
    if required < 0 {
        i32::MAX - 1
    } else if required == 0 {
        0
    } else {
        let mut values = lookup
            .iter()
            .map(|x| 1 + a(required - x, lookup))
            .collect::<Vec<i32>>();
        values.push(required);

        *values.iter().min().unwrap()
    }
}

#[allow(dead_code)]
fn c_calc(required: i32, lookup: &Vec<i32>, memory: &mut HashMap<i32, i32>) -> i32 {
    if required < 0 {
        i32::MAX - 1
    } else if required == 0 {
        0
    } else if memory.contains_key(&required) {
        return *memory.get(&required).unwrap();
    } else {
        let mut values = lookup
            .iter()
            .map(|x| 1 + c_calc(required - x, lookup, memory))
            .collect::<Vec<i32>>();
        values.push(required);

        let min = *values.iter().min().unwrap();
        memory.insert(required, min);

        min
    }
}

#[allow(dead_code)]
fn c(required: i32, lookup: &Vec<i32>) -> i32 {
    let mut memory = HashMap::new();

    c_calc(required, lookup, &mut memory)
}

fn e(required: i32, lookup: &Vec<i32>) -> i32 {
    let mut memory = Vec::with_capacity(required as usize + 1);
    memory.push(0);

    for req in 1..=required {
        let mut coins = lookup
            .iter()
            .map(|x| req - x)
            .filter(|&x| x > -1)
            .map(|x| 1 + memory[x as usize])
            .collect::<Vec<_>>();
        coins.push(req);

        let minimum = *coins.iter().min().unwrap();

        memory.push(minimum);
    }

    return *memory.get(required as usize).unwrap();
}

fn read_input() -> (i32, Vec<i32>) {
    let mut buffer = String::new();
    let stdin = stdin();
    let mut handle = stdin.lock();

    _ = handle.read_line(&mut buffer);

    let required = str::parse(buffer.trim()).unwrap();

    buffer.clear();

    let mut coin_values = Vec::new();

    while handle.read_line(&mut buffer).unwrap() > 0 {
        coin_values.push(str::parse(buffer.trim()).unwrap());
        buffer.clear();
    }

    (required, coin_values)
}

pub type CoinSolution = fn(i32, &Vec<i32>) -> i32;

fn execution_time(f: CoinSolution, size: i32) -> Duration {
    let coin_values = vec![5, 6, 7];
    let start = Instant::now();
    _ = std::thread::Builder::new()
        .stack_size(4 * 1024 * 1024 * 1024)
        .spawn(move || f(size, &coin_values))
        .unwrap()
        .join();
    Instant::now().duration_since(start)
}

const RETRIES: usize = 3;

fn take_time(f: CoinSolution, size: i32) -> f64 {
    (0..RETRIES)
        .into_iter()
        .map(|_| execution_time(f, size).as_secs_f64())
        .sum::<f64>()
        / RETRIES as f64
}

const MAX_WAIT: f64 = 60.0;

const MAX_SAMPLES: i32 = 10;

fn benchmark(f: CoinSolution) {
    let mut size = 1;
    let mut results = Vec::new();
    loop {
        eprint!("Attempting {}: ", size);
        let time = take_time(f, size);
        eprintln!("{}s", time);
        results.push((size, time));

        if time >= 1.3 {
            break;
        }

        size = i32::max(size + 1, (size as f64 * (1.0 + time)) as i32);
    }

    let mut ceiling = size;
    let mut floor = results.iter().rev().find(|(_, t)| *t < 1.0).unwrap().0;
    loop {
        let next_diff = (ceiling - floor) / 2;
        size = floor + next_diff;

        eprint!("Attempting {}: ", size);
        let time = take_time(f, size);
        eprintln!("{}s", time);
        results.push((size, time));

        if next_diff <= 0 {
            break;
        } else if time > 1.0 {
            ceiling -= next_diff;
        } else {
            floor += next_diff;
        }
    }

    results.sort_unstable_by(|(_, t1), (_, t2)| {
        f64::abs(t1 - 1.0).partial_cmp(&f64::abs(t2 - 1.0)).unwrap()
    });

    let (closest_n, duration) = results.remove(0);

    eprintln!("Found closest N = {} at {}s", closest_n, duration);

    size = closest_n;
    println!("# Linear Increase\n# n, time(s)");
    loop {
        let time = take_time(f, size);
        println!("{}, {}", size, time);

        size += 1;

        if time >= MAX_WAIT || size - closest_n > MAX_SAMPLES {
            break;
        }
    }

    let mut size = closest_n;
    let mut counter = 0;
    println!("# Exponential Increase\n# n, time(s)");
    loop {
        let time = take_time(f, size);
        println!("{}, {}", size, time);

        size *= 2;
        counter += 1;

        if time >= MAX_WAIT || counter >= MAX_SAMPLES {
            break;
        }
    }
}

pub fn main() {
    let f: CoinSolution = e;

    if args().into_iter().any(|arg| arg.contains("-b")) {
        eprintln!("Benchmarking");
        benchmark(f);
    } else {
        let (required, coin_values) = read_input();

        let coins = f(required, &coin_values);
        println!("{}", coins);
    }
}

#[cfg(test)]
mod tests {
    macro_rules! test_part {
        ($x:ident) => {
            mod $x {
                use crate::$x as calc;

                #[test]
                fn example1() {
                    let required = 10;
                    let coin_values = vec![2, 3, 4];

                    let result = calc(required, &coin_values);
                    assert_eq!(result, 3);
                }

                #[test]
                fn example2() {
                    let required = 10;
                    let coin_values = vec![5, 6, 7];

                    let result = calc(required, &coin_values);
                    assert_eq!(result, 2);
                }

                #[test]
                fn example3() {
                    let required = 0;
                    let coin_values = vec![10, 100, 1000];

                    let result = calc(required, &coin_values);
                    assert_eq!(result, 0);
                }
            }
        };
    }

    test_part!(a);

    test_part!(c);
}
