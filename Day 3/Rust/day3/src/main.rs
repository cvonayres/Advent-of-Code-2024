use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
use regex::Regex;

fn main() {
    // Get Data from file 
    let file_path = "InputFile.txt";
    let data = match read_file(file_path) {
        Ok(d) => d,
        Err(e) => {
            println!("Error reading file: {}", e);
            return;
        }
    };

    let mul_re = Regex::new(r"mul\((\d+),(\d+)\)").unwrap();
    let do_re = Regex::new(r"^do\(\)$").unwrap();          // Updated regex
    let dont_re = Regex::new(r"^don't\(\)$").unwrap();     // Updated regex

    let mut matches = Vec::new();
    let mut is_enabled = true;

    for token in data.split_whitespace() {
        if do_re.is_match(token) {
            is_enabled = true;
        } else if dont_re.is_match(token) {
            is_enabled = false;
        } else if is_enabled {
            if let Some(caps) = mul_re.captures(token) {
                let first_num = caps.get(1).unwrap().as_str().parse::<i32>().unwrap();
                let second_num = caps.get(2).unwrap().as_str().parse::<i32>().unwrap();
                matches.push((first_num, second_num));
            }
        }
    }

    let total: i32 = matches.iter().map(|(x, y)| x * y).sum();

    println!("Matches {:?}", matches);
    println!("Total {:?}", total);
}

fn read_file<P: AsRef<Path>>(file_path: P) -> io::Result<String> {
    let file = File::open(file_path)?;
    let reader = io::BufReader::new(file);

    let mut data = String::new();
    for line in reader.lines() {
        data.push_str(&line?);
        data.push(' '); // Ensure lines are separated by whitespace
    }

    Ok(data)
}
