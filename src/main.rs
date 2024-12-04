mod day3;

fn main() {
    // Day 1
    /*
     * let path = "input.txt";
    let Ok((mut input0, mut input2)) = parse_txt_to_vecs(path) else {todo!()};
    input0.sort();
    input1.sort();

    println!("Total distance: {}", get_distances(&input0, &input2));
    println!("Similarity scores: {}", get_similarity_scores(&input0, &input2));
}
     */

    // Day 2
    // let path = "day2.txt";
    // let Ok(mut vectors) = parse_file_to_vectors(path) else {todo!()};
    // println!("Safety amount: {}", check_safety_amount(&vectors));
    // println!("Safety amount: {}", safety_with_problem_dampener(&mut vectors));

    day3::foo();

}
// This should be day 1, at least
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

fn parse_txt_to_vecs(file_path: &str) -> Result<(Vec<i64>, Vec<i64>), Box<dyn std::error::Error>> {
    let path = Path::new(file_path);
    let file = File::open(&path)?;
    let reader = io::BufReader::new(file);

    let mut column1 = Vec::new();
    let mut column2 = Vec::new();

    for line in reader.lines() {
        let line = line?;
        let mut parts = line.split_whitespace();
        if let (Some(col1), Some(col2)) = (parts.next(), parts.next()) {
            column1.push(col1.parse::<i64>()?);
            column2.push(col2.parse::<i64>()?);
        }
    }

    Ok((column1, column2))
}

fn get_distances(l1: &Vec<i64>, l2: &Vec<i64>) ->i64{
    let mut total_distance = 0;
    for i in 0..l1.len() {
        total_distance += (l1[i] - l2[i]).abs();
    }
    total_distance
}

fn get_similarity_scores(l1: &Vec<i64>, l2: &Vec<i64>) ->i64 {
    let mut similarity_scores: Vec<i64> = vec!();
    l1.into_iter().for_each(|x| similarity_scores.push(<usize as TryInto<i64>>::try_into(l2.into_iter().filter(|y| *y == x).collect::<Vec<&i64>>().len()).unwrap() * x));
    similarity_scores.iter().sum()
}
// Day 2
fn parse_file_to_vectors(file_path: &str) -> io::Result<Vec<Vec<i32>>> {
    // Open the file for reading
    let file = File::open(file_path)?;
    let reader = io::BufReader::new(file);

    // Parse each line into a vector of integers
    let parsed_lines: Vec<Vec<i32>> = reader
        .lines()
        .filter_map(|line| line.ok()) // Handle any IO errors
        .map(|line| {
             line.split_whitespace() // Split by whitespace
                .filter_map(|num| num.parse::<i32>().ok()) // Parse each number
                .collect() // Collect into a Vec<i32>
        })
        .collect();

    Ok(parsed_lines)
}

fn is_safe(vector: &Vec<i32>) -> bool {
    let is_decreasing = vector[0] < vector[1];
    for i in 0..vector.len() {
        if i + 1 >= vector.len() {
            return true;
        }
        if is_decreasing && vector[i] > vector[i + 1] {
            return false;
        }
        if !is_decreasing && vector[i] < vector[i + 1] {
            return false;
        }
        let difference = (vector[i] - vector[i + 1]).abs();
        if difference < 1 || difference > 3 {
            return false;
        }
    }
    true
}

fn safety_with_problem_dampener(vectors: &mut Vec<Vec<i32>>) -> i32 {
    let mut count = 0;

    for vector in vectors.iter_mut() {
        if is_safe_after_removal(vector) {
            count += 1;
        }
    }

    count
}




fn is_safe_after_removal(vector: &mut Vec<i32>) -> bool {
    for i in 0..(vector).len() {
        let removed = (vector)[i];
        (vector).remove(i);

        let is_safe_now = is_safe(vector);

        (vector).insert(i, removed);

        if is_safe_now {
            return true;
        }
    }
    false
}

