use std::error::Error;
use std::{fs, io};
use std::fs::File;
use std::io::BufRead;
use std::path::Path;



pub(crate) fn read_columns_from_file(file_path: &str) -> Result<(Vec<i32>, Vec<i32>), Box<dyn Error>> {
    // Read the entire file content
    let content = fs::read_to_string(file_path)?;

    // Vectors to hold the two columns
    let mut column1 = Vec::new();
    let mut column2 = Vec::new();

    // Process the content line by line
    for line in content.lines() {
        let mut numbers = line.split_whitespace().filter_map(|num| num.parse::<i32>().ok());
        if let Some(first) = numbers.next() {
            column1.push(first);
        }
        if let Some(second) = numbers.next() {
            column2.push(second);
        }
    }

    Ok((column1, column2))
}

pub(crate) fn read_rows_of_numbers_from_file(file_path: &str) -> Result<Vec<Vec<i32>>, Box<dyn Error>> {

    let path = Path::new(file_path);
    let file = File::open(path)?;
    let reader = io::BufReader::new(file);

    // Parse each line into a vector of numbers
    let rows: Vec<Vec<i32>> = reader
        .lines()
        .map(|line| {
            line.unwrap() // Get the string
                .split_whitespace() // Split by whitespace
                .map(|num| num.parse::<i32>().unwrap()) // Parse each number
                .collect() // Collect into Vec<f64>
        })
        .collect();

    Ok(rows)
}
pub(crate) fn read_rows_of_strings_from_file(file_path: &str) -> Result<Vec<String>, Box<dyn Error>> {

    let path = Path::new(file_path);
    let file = File::open(path)?;
    let reader = io::BufReader::new(file);

    // Parse each line into a vector of numbers
    let strings: Vec<_> = reader
        .lines()
        .collect::<Vec<_>>()
        .into_iter()
        .filter_map(|result| match result {
            Ok(string) => Some(string),
            Err(err) => {
                eprintln!("Error: {}", err);
                None
            }
        })
        .collect();

    Ok(strings)
}