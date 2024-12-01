mod one;

use std::error::Error;
use std::fs;

fn main() -> Result<(), Box<dyn Error>> {

    let file_path = "input.txt";

    let (column1, column2) = read_columns_from_file(file_path)?;

    let result = one::add_list_distances(column1, column2);

    println!("Result: {:?}", result);

    Ok(())
}

// Subroutine to read numbers from a file and separate into two columns
fn read_columns_from_file(file_path: &str) -> Result<(Vec<i32>, Vec<i32>), Box<dyn Error>> {
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
