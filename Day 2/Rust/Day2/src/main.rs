use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
use std::env;

fn main() {
    // Handle Debug
    let args: Vec<String> = env::args().collect();
    let debug = args.contains(&String::from("d"));

   // Get Data from file 
   let file_path = "InputFile.txt";
   let data = match read_file_to_vectors(file_path) {
        Ok(d) => d,
        Err(e) => {
            println!("Error reading file: {}", e);
            return;
        }
    };

   // Part 1
    if debug { println!("Part 1 : "); } 
    let mut part1_answer = 0;
    
    for (i, floor) in data.iter().enumerate() {
        let result = check_floor(floor);
        if result {
            part1_answer += 1;
        }
        print_result(i, result, debug);
    }
    println!("Part 1 Answer is {}", part1_answer);


   // Part 2
   if debug { println!("Part 2 : "); } 
   let mut part2_answer = 0;
   
    for (i, floor) in data.iter().enumerate() {
        let result = check_floor_with_dampener(floor);
        if result {
            part2_answer += 1;
        }
        print_result(i, result, debug);
    }
    println!("Part 2 Answer is {}", part2_answer);
}

fn read_file_to_vectors<P: AsRef<Path>>(file_path: P) -> io::Result<Vec<Vec<i32>>> {
    let file = File::open(file_path)?;
    let reader = io::BufReader::new(file);

    let mut data: Vec<Vec<i32>> = Vec::new();
   
    for line in reader.lines() {
        let line = line?;
        let row: Vec<i32> = line
            .split_whitespace()
            .filter_map(|s| s.parse::<i32>().ok())
            .collect();
        data.push(row);
    }

    Ok(data)
}

fn print_result (i : usize, result : bool, debug : bool) {
    if !debug { return; }

    if result {
        println!("Floor {} is Safe", i);
    } else {
        println!("Floor {} is Unsafe", i);
    }
}

fn check_floor(floor: &Vec<i32>) -> bool {
    if floor.len() < 2 { return false; }

    let mut is_ascending = true;
    let mut is_descending = true;
    let mut is_valid_range = true;

    for window in floor.windows(2) {
        let diff = (window[1] - window[0]).abs();

        if diff < 1 || diff > 3 { is_valid_range = false; }

        if window[1] < window[0] { is_ascending = false; }

        if window[1] > window[0] { is_descending = false; }
    }

    is_valid_range && (is_ascending || is_descending)
}

fn check_floor_with_dampener(floor: &Vec<i32>) -> bool {
    if check_floor(floor) { return true; }

    for i in 0..floor.len() {
        let mut temp_floor = floor.to_vec();
        temp_floor.remove(i);

        if check_floor(&temp_floor) { return true; }
    }

    false
}
