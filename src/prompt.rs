use anyhow::Result;
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
        match input::<String>() {
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
    pub(crate) fn get_input_coordinates(&self) -> Result<(usize, usize)> {
        println!(
            "Please Input Coordinates in the range x: ({}, {}), y: ({}, {}):",
            self.min_world_x, self.max_world_x, self.min_world_y, self.max_world_y
        );
        match input::<String>() {
            Ok(input) => {
                let res = input.split(",").map(|c| c.trim()).collect::<Vec<&str>>();
                let x = res[0].parse::<i64>()?;
                let y = res[1].parse::<i64>()?;

                if !(self.min_world_x <= x && x <= self.max_world_x)
                    || !(self.min_world_y <= y && y <= self.max_world_y)
                {
                    eprintln!("Invalid coordinates");
                    Err(anyhow::anyhow!("Invalid coordinates (not in range)"))
                } else {
                    // Convert the coordinates back to unsigned
                    println!("Closest Central Fill Facilities to ({}, {}):", x, y);
                    Ok((
                        (x + self.max_world_x) as usize,
                        (y + self.max_world_y) as usize,
                    ))
                }
            }
            Err(err) => Err(err.into()),
        }
    }
}

/// Function to get the user input in a more ergonomic manner
fn input<T: FromStr>() -> Result<T, <T as FromStr>::Err> {
    // Likely some security issues here, needs additional testing
    let mut input: String = String::with_capacity(64);

    std::io::stdin()
        .read_line(&mut input)
        .expect("Input could not be read");

    input.trim().parse()
}
