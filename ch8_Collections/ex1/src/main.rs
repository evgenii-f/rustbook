use std::io;
use std::collections::HashMap;

fn parse_line(line: &str) -> Vec<u32> {
    let vec: Vec<u32> = line.split_whitespace()
        .map(|x| x.parse().expect("Not an integer!"))
        .collect();
    return vec
}

fn input_vec() -> Vec<u32> {
    println!("Input your array separating integers by space");
    let mut line = String::new();
    io::stdin()
        .read_line(&mut line)
        .expect("Failed to read line");
    let trimmed = line.trim();

    return parse_line(trimmed);
}

fn test_parse_line() {
    let test_line = String::from("1 2 3 43");
    println!("Your line: {}", test_line);
    
    let vec: Vec<u32> = parse_line(&test_line);
    println!("The parsed vector: {:?}", vec);
}

fn count_entries(vec: &Vec<u32>) -> HashMap<u32, u32> {
    let mut entry_count = HashMap::new();
    for entry in &*vec{
        let n = entry_count.entry(*entry).or_insert(0);
        *n += 1;
    }
    
    return entry_count;
}

fn get_mean(vec: &Vec<u32>) -> f32 {
    let length: usize = vec.len();
    let sum: u32 = vec.iter().sum();

    return (sum as f32) / (length as f32);
}

fn get_most_frequent_val(vec: &Vec<u32>) -> Option<u32> {
    let entry_count: HashMap<u32, u32> = count_entries(vec);
    let max_key = entry_count
        .iter()
        .max_by_key(|&(_, val)| val)
        .map(|(key, _)| *key);

    return max_key;
}

fn main() {
    test_parse_line();
    let vec: Vec<u32> = input_vec();
    println!("Your vector: {:?}", vec);

    let entry_count = count_entries(&vec);
    println!("Count entries:\n{:?}", entry_count);

    let mean: f32 = get_mean(&vec);
    println!("Mean val = {}", mean);

    let most_freq_val: Option<u32> = get_most_frequent_val(&vec);
    match most_freq_val {
        Some(k) => println!("The most frequent val = {}", k),
        None => println!("Empty vec given!"),
    }
}