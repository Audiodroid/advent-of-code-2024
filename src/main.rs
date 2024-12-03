mod one;
mod two;
mod file_reader;
mod three;

use std::error::Error;
use std::fs;

fn main() -> Result<(), Box<dyn Error>> {

    one()?;
    two()?;
    three()
}

fn one() -> Result<(), Box<dyn Error>> {

    let input_1 = "resources/input_1.txt";

    let (column1, column2) = file_reader::read_columns_from_file(input_1)?;
    let result = one::add_list_distances(column1, column2);
    println!("1 - Part One: {:?}", result);

    let (column1, column2) = file_reader::read_columns_from_file(input_1)?;
    let result = one::add_similarity_score(column1, column2);
    println!("1 - Part Two: {:?}", result);

    Ok(())
}

fn two() -> Result<(), Box<dyn Error>> {

    let input_2 = "resources/input_2.txt";

    let rows= file_reader::read_rows_from_file(input_2)?;
    let result = two::count_valid_reports(rows, false);
    println!("2 - Part One: {:?}", result);

    let rows= file_reader::read_rows_from_file(input_2)?;
    let result = two::count_valid_reports(rows, true);
    println!("2 - Part Two: {:?}", result);

    Ok(())
}
fn three() -> Result<(), Box<dyn Error>> {

    let mut input_3 = fs::read_to_string("resources/input_3.txt").unwrap().to_string();
    let result = three::scan(&mut input_3);
    println!("3 - Part One: {:?}", result);

    Ok(())
}