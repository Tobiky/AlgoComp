#[allow(unused_imports)]
use std::{
    collections::HashMap,
    env::args,
    hash::Hash,
    io::{stdin, BufRead},
    time::{Duration, Instant},
};

type Solution = fn(i32, i32, f64) -> f64;

fn a_calc(x: i32, y: i32, k: i32, p: f64, memory: &mut HashMap<(i32, i32), f64>) -> f64 {
    if y == 0 {
        return 1.0;
    } else if x == 0 && y > 0 {
        return 0.0;
    } else if memory.contains_key(&(x, y)) {
        return *memory.get(&(x, y)).unwrap();
    }

    let first = p * a_calc(x - 1, y - 1, k, p, memory);
    let second = (1.0 - p) * a_calc(x - 1, k, k, p, memory);
    memory.insert((x, y), first + second);
    return first + second;
}

fn a(n: i32, k: i32, p: f64) -> f64 {
    let mut memory = HashMap::new();

    a_calc(n, k, k, p, &mut memory)
}

fn c_calc(x: i32, k: i32, p: f64, memory: &mut HashMap<i32, f64>) -> f64 {
    if x < k {
        return 0.0;
    } else if x == k {
        return p.powi(k);
    } else if memory.contains_key(&x) {
        return *memory.get(&x).unwrap();
    }
    let first = c_calc(x - 1, k, p, memory);
    let second = p.powi(k) * (1.0 - p) * (1.0 - c_calc(x - k - 1, k, p, memory));
    memory.insert(x, first + second);

    return first + second;
}

fn c(n: i32, k: i32, p: f64) -> f64 {
    let mut memory = HashMap::new();

    c_calc(n, k, p, &mut memory)
}

fn execution_time(f: Solution, size: i32) -> Duration {
    let wins = size / 2;
    let probability = 0.99;
    let start = Instant::now();
    _ = std::thread::Builder::new()
        .stack_size(4 * 1024 * 1024 * 1024)
        .spawn(move || f(size, wins, probability))
        .unwrap()
        .join();
    Instant::now().duration_since(start)
}

const RETRIES: usize = 3;

fn take_time(f: Solution, size: i32) -> f64 {
    (0..RETRIES)
        .into_iter()
        .map(|_| execution_time(f, size).as_secs_f64())
        .sum::<f64>()
        / RETRIES as f64
}

const MAX_WAIT: f64 = 60.0;

const MAX_SAMPLES: i32 = 10;

fn benchmark(f: Solution) {
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

fn read_input() -> (i32, i32, f64) {
    let mut buffer = String::new();
    let stdin = stdin();
    let mut handle = stdin.lock();

    _ = handle.read_line(&mut buffer);
    let games = str::parse(buffer.trim()).unwrap();
    buffer.clear();

    _ = handle.read_line(&mut buffer);
    let wins = str::parse(buffer.trim()).unwrap();
    buffer.clear();

    _ = handle.read_line(&mut buffer);
    let probability = str::parse(buffer.trim()).unwrap();

    (games, wins, probability)
}

pub fn main() {
    let f: Solution = c;

    if args().into_iter().any(|arg| arg.contains("-b")) {
        eprintln!("Benchmarking");
        benchmark(f);
    } else {
        let (g, w, p) = read_input();

        let likelihood = f(g, w, p);
        println!("{}", likelihood);
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
                    const ANSWER: f64 = 0.784;
                    let lambda: f64 = 10f64.powi(-6);
                    let (games, wins, probability) = (4, 2, 0.7);

                    let result = calc(games, wins, probability);
                    assert!(result - lambda < ANSWER && ANSWER < result + lambda);
                }

                #[test]
                fn example2() {
                    const ANSWER: f64 = 0.2922575722241294;
                    let lambda: f64 = 10f64.powi(-6);
                    let (games, wins, probability) = (20, 4, 0.42);

                    let result = calc(games, wins, probability);
                    assert!(result - lambda < ANSWER && ANSWER < result + lambda);
                }
            }
        };
    }

    test_part!(a);
    test_part!(c);
}
