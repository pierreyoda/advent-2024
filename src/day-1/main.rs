use std::{collections::HashMap, ops::Deref};

use anyhow::{anyhow, Context, Ok, Result};

use advent_2024_common::run_with_scaffolding_strings;

type LocationID = u32;

struct LocationIDsPair {
    left: LocationID,
    right: LocationID,
}

impl LocationIDsPair {
    pub fn new(left: LocationID, right: LocationID) -> Self {
        Self { left, right }
    }

    pub fn distance(&self) -> u32 {
        self.right.abs_diff(self.left)
    }
}

fn input_line_to_pair(line: &str) -> Result<(LocationID, LocationID)> {
    let mut parts = line.split_ascii_whitespace();
    let left = parts
        .next()
        .with_context(|| format!("Failed to parse input line (left): {}", line))?;
    let right = parts
        .next()
        .with_context(|| format!("Failed to parse input line (right): {}", line))?;

    fn convert_string_to_location_id(string: &str) -> Result<LocationID> {
        string
            .parse()
            .with_context(|| format!("Failed to parse Location ID"))
    }

    Ok((
        convert_string_to_location_id(left)?,
        convert_string_to_location_id(right)?,
    ))
}

fn parse_input(input: &[String]) -> Result<(Vec<LocationID>, Vec<LocationID>)> {
    let (mut ids_left, mut ids_right) = (
        Vec::<LocationID>::with_capacity(input.len()),
        Vec::<LocationID>::with_capacity(input.len()),
    );
    for line in input {
        if line.trim().is_empty() {
            continue;
        }
        let (left, right) = input_line_to_pair(line)?;
        ids_left.push(left);
        ids_right.push(right);
    }
    if ids_left.len() != ids_right.len() {
        return Err(anyhow!("Parsed Location ID lists length mismatch"));
    }

    Ok((ids_left, ids_right))
}

// TODO: use &[&str]
fn compute_solution_1(input: &[String]) -> Result<u32> {
    let (mut ids_left, mut ids_right) = parse_input(input)?;
    ids_left.sort();
    ids_right.sort();
    let mut location_id_pairs = Vec::with_capacity(ids_left.len());
    for i in 0..ids_left.len() {
        location_id_pairs.push(LocationIDsPair::new(ids_left[i], ids_right[i]));
    }

    let total_distance = location_id_pairs
        .iter()
        .fold(0, |sum, pair| sum + pair.distance());
    Ok(total_distance)
}

// TODO: use &[&str]
fn compute_solution_2(input: &[String]) -> Result<u32> {
    let (ids_left, ids_right) = parse_input(input)?;
    let mut right_occurrences = HashMap::new();
    for right_id in &ids_right {
        if let Some(occurrences) = right_occurrences.get_mut(right_id) {
            *occurrences += 1;
        } else {
            right_occurrences.insert(right_id, 1);
        }
    }

    let similarity_score = ids_left.iter().fold(0, |score, left_id| {
        score + left_id * right_occurrences.get(left_id).unwrap_or(&0)
    });

    Ok(similarity_score)
}

fn main() -> Result<()> {
    // Part 1
    run_with_scaffolding_strings("day-1", b'\n', |inputs| Ok(compute_solution_1(&inputs)?))?;
    // Part 2
    run_with_scaffolding_strings("day-1", b'\n', |inputs| Ok(compute_solution_2(&inputs)?))?;

    Ok(())
}

#[cfg(test)]
mod tests {
    use crate::{compute_solution_1, compute_solution_2};
    const TEST_INPUT: &str = r#"
3 4
4 3
2 5
1 3
3 9
3 3
"#;

    fn test_input_to_lines(input: &str) -> Vec<String> {
        TEST_INPUT.lines().map(|l| l.to_string()).collect()
    }

    #[test]
    fn test_day_1_compute_solution_1() {
        assert_eq!(
            compute_solution_1(&test_input_to_lines(TEST_INPUT)).unwrap(),
            11
        );
    }

    #[test]
    fn test_day_1_compute_solution_2() {
        assert_eq!(
            compute_solution_2(&test_input_to_lines(TEST_INPUT)).unwrap(),
            31
        );
    }
}
