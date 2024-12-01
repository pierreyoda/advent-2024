use std::fmt::Debug;
use std::str::from_utf8;
use std::{
    fmt::Display,
    fs::File,
    io::{BufRead, BufReader},
    path::Path,
    time::Instant,
};

use anyhow::Result;

pub enum DatumType {
    Integer,
    String,
}

impl DatumType {
    pub fn get_number_lines_transformer(&self) -> impl Fn(&[u8]) -> Result<u32> {
        |raw_datum| {
            let string = from_utf8(raw_datum).unwrap();
            let number: u32 = string.parse().unwrap();
            Ok(number)
        }
    }

    pub fn get_string_lines_transformer(&self) -> impl Fn(&[u8]) -> Result<String> {
        |raw_datum| {
            let string = from_utf8(raw_datum).unwrap();
            Ok(string.into())
        }
    }
}

pub fn load_inputs_from_file<O, P, F>(path: P, separator: u8, transform: F) -> Result<Vec<O>>
where
    O: Debug,
    P: AsRef<Path>,
    F: Fn(&[u8]) -> Result<O>,
{
    let file = File::open(path)?;
    let lines = BufReader::new(file);
    Ok(lines
        .split(separator)
        .into_iter()
        // TODO: avoid unwrap
        .map(|l| {
            let raw = l.unwrap();
            transform(&raw).expect("input transformation should not fail")
        })
        .collect())
}

pub fn run_with_scaffolding_integers<O, F>(
    label: &'static str,
    inputs_separator: u8,
    compute: F,
) -> Result<O>
where
    O: Clone + Display,
    F: Fn(Vec<u32>) -> Result<O>,
{
    // Read input(s)
    let input_start = Instant::now();
    let input = load_inputs_from_file(
        format!("./src/{}/input.txt", label),
        inputs_separator,
        DatumType::Integer.get_number_lines_transformer(),
    )?;
    let input_time = input_start.elapsed();
    println!("Inputs read in {:?}", input_time);

    // Run computing function
    let compute_start = Instant::now();
    let output = compute(input)?;
    let compute_time = compute_start.elapsed();
    println!("Computing done in {:?}", compute_time);

    // Output
    println!("Result = {}", output);
    Ok(output)
}

pub fn run_with_scaffolding_strings<O, F>(
    label: &'static str,
    inputs_separator: u8,
    compute: F,
) -> Result<O>
where
    O: Clone + Display,
    F: Fn(Vec<String>) -> Result<O>,
{
    // Read input(s)
    let input_start = Instant::now();
    let input = load_inputs_from_file(
        format!("./src/{}/input.txt", label),
        inputs_separator,
        DatumType::Integer.get_string_lines_transformer(),
    )?;
    let input_time = input_start.elapsed();
    println!("Inputs read in {:?}", input_time);

    // Run computing function
    let compute_start = Instant::now();
    let output = compute(input)?;
    let compute_time = compute_start.elapsed();
    println!("Computing done in {:?}", compute_time);

    // Output
    println!("Result = {}", output);
    Ok(output)
}
