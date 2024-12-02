use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

fn main() {
    // Define file
    let file_path = "InputFile.txt";

    // Get Data from file as two vectors
    let (left_values, right_values) = match read_file_to_vectors(file_path) {
        Ok((l, r)) => (l, r),
        Err(e) => {
            println!("Error reading file: {}", e);
            return;
        }
    };

    // Create sorted copies
    let left_sorted = get_sorted_vec(&left_values);
    let right_sorted = get_sorted_vec(&right_values);

    // Part 1
    let part1_answer: i32 = left_sorted
    .iter()
    .zip(right_sorted.iter())
    .map(|(l, r)| (l - r).abs())
    .sum();

    println!("Part 1 Answer is {}", part1_answer);

    // Part 2
    let part2_answer: i32 = left_values
    .iter()
    .map(|left| {
        right_values
            .iter()
            .filter(|&&right| right == *left)
            .count() as i32 * left
    })
    .sum();

    println!("Part 2 Answer is {}", part2_answer);
}

fn read_file_to_vectors<P: AsRef<Path>>(file_path: P) -> io::Result<(Vec<i32>, Vec<i32>)> {
    let file = File::open(file_path)?;
    let reader = io::BufReader::new(file);

    let mut left_values = Vec::new();
    let mut right_values = Vec::new();

    for line in reader.lines().flatten() {
        let mut columns = line.split_whitespace();
        let left = columns.next().unwrap().parse::<i32>().unwrap();
        let right = columns.next().unwrap().parse::<i32>().unwrap();
        left_values.push(left);
        right_values.push(right);
    }

    Ok((left_values, right_values))
}

fn get_sorted_vec(vec: &Vec<i32>) -> Vec<i32> {
    let mut sorted_vec = vec.clone();
    sorted_vec.sort();
    sorted_vec
}