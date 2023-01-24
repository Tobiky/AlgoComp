use std::collections::HashMap;

fn calc(required: i32, lookup: &Vec<i32>, memory: &mut HashMap<i32, i32>) -> i32 {
    if required < 0 {
        i32::MAX - 1
    } else if required == 0 {
        0
    } else if memory.contains_key(&required) {
        return *memory.get(&required).unwrap()
    } else {
        let mut values = lookup.iter().map(|x| 1 + calc(required - x, lookup, memory)).collect::<Vec<i32>>();
        values.push(required);

        let min = *values.iter().min().unwrap();
        memory.insert(required, min);

        min
    }
}

fn main() {
    let mut coin_values = std::io::stdin().lines().map(|x| str::parse(x.unwrap().as_str()).unwrap()).collect::<Vec<i32>>();
    let required = coin_values.remove(0);
    let mut memory = HashMap::new();

    let coins = calc(required, &coin_values, &mut memory);
    println!("{}", coins);
}
