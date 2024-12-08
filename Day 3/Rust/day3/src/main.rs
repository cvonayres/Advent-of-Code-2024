use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
use regex::Regex;

fn main() {
    let file_path = "InputFile.txt";
    let data = match read_file(file_path) {
        Ok(d) => d,
        Err(e) => {
            println!("Error reading file: {}", e);
            return;
        }
    };

    let instructions_re = Regex::new(r"do\(\)|don't\(\)|mul\(\d+,\d+\)").unwrap();
    let mul_extract_re = Regex::new(r"mul\((\d+),(\d+)\)").unwrap();

    let mut matches = Vec::new();
    let mut is_enabled = true;

    for cap in instructions_re.find_iter(&data) {
        let instr = cap.as_str();
        if instr == "do()" {
            is_enabled = true;
        } else if instr == "don't()" {
            is_enabled = false;
        } else if instr.starts_with("mul(") && is_enabled {
            if let Some(m) = mul_extract_re.captures(instr) {
                let x = m[1].parse::<i32>().unwrap();
                let y = m[2].parse::<i32>().unwrap();
                matches.push((x, y));
            }
        }
    }

    let total: i32 = matches.iter().map(|(x, y)| x * y).sum();

    println!("Matches: {:?}", matches);
    println!("Total: {}", total);
}

fn read_file<P: AsRef<Path>>(file_path: P) -> io::Result<String> {
    let file = File::open(file_path)?;
    let reader = io::BufReader::new(file);

    let mut data = String::new();
    for line in reader.lines() {
        data.push_str(&line?);
        data.push(' ');
    }

    Ok(data)
}