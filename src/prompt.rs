use anyhow::{anyhow, Result};
use std::str::FromStr;

const DEFAULT_FACILITY_COUNT: usize = 25;

pub struct PromptConfig {
    // We'll validate the bounds here, so need negative values (i64)
    min_world_x: i64,
    max_world_x: i64,
    min_world_y: i64,
    max_world_y: i64,
}

impl PromptConfig {
    pub(crate) fn new(
        min_world_x: i64,
        max_world_x: i64,
        min_world_y: i64,
        max_world_y: i64,
    ) -> PromptConfig {
        PromptConfig {
            min_world_x,
            max_world_x,
            min_world_y,
            max_world_y,
        }
    }

    /// Set up the program's facility count
    pub(crate) fn get_facility_count(&self) -> usize {
        println!("Please Input A Facility Count (or press RET for a default val)");
        self.get_facility(input::<String>())
    }

    fn get_facility(&self, input: Result<String>) -> usize {
        match input {
            Ok(input) => match input.is_empty() {
                false => input
                    .trim()
                    .parse::<usize>()
                    .unwrap_or(DEFAULT_FACILITY_COUNT),
                true => {
                    eprintln!("Empty string, using default");
                    DEFAULT_FACILITY_COUNT
                }
            },
            Err(err) => {
                // Write the error to standard error, and return a default value
                eprintln!("{}", err);
                DEFAULT_FACILITY_COUNT
            }
        }
    }

    /// Get the input coordinates in relative format (\[-x, x\], \[-y,y\])
    ///
    /// Returns the coordinates in absolute \[0, 2 * x\], \[0, 2 * y\]
    ///
    /// Should account for white space in the result. Returns an usize, so [0...u64].
    pub(crate) fn get_input_coordinates(&self) -> Result<(i64, i64)> {
        println!(
            "Please Input Coordinates in the range x: ({}, {}), y: ({}, {}):",
            self.min_world_x, self.max_world_x, self.min_world_y, self.max_world_y
        );
        self.get_coordinates(input::<String>())
    }

    /// Helper function that just parses an input string
    fn get_coordinates(&self, input: Result<String>) -> Result<(i64, i64)> {
        match input {
            Ok(input) => {
                let res = input.split(",").map(|c| c.trim()).collect::<Vec<&str>>();
                Ok((res[0].parse::<i64>()?, res[1].parse::<i64>()?))
            }
            Err(err) => Err(err.into()),
        }
    }
}

/// Function to get the user input in a more ergonomic manner
fn input<T: FromStr>() -> Result<T> {
    let mut input: String = String::with_capacity(64);

    std::io::stdin()
        .read_line(&mut input)
        .expect("Input could not be read");

    match input.trim().parse() {
        Ok(val) => Ok(val),
        Err(err) => Err(anyhow!("Input could not be parsed")),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn valid_get_facility_count() {
        let x_val = 10;
        let y_val = 10;
        let p = PromptConfig::new(-x_val, x_val, -y_val, y_val);

        assert_eq!(DEFAULT_FACILITY_COUNT, p.get_facility(Ok("".to_string())));
        assert_eq!(DEFAULT_FACILITY_COUNT, p.get_facility(Ok("-25".to_string())));
        assert_eq!(35, p.get_facility(Ok("35".to_string())));
    }
}
