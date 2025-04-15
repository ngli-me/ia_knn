use anyhow::Result;
use serde::Deserialize;
use std::error::Error;
use crate::facility::FacilityMap;

mod facility;
mod medication;
mod prompt;

// Helper struct to hold some of our config values
#[derive(Debug, Deserialize)]
pub struct Config {
    #[serde(default = "default_world_xy")]
    world_x: usize,
    #[serde(default = "default_world_xy")]
    world_y: usize,
}

fn default_world_xy() -> usize {
    21
}

fn main() -> Result<(), Box<dyn Error>> {
    dotenvy::dotenv().ok();
    let config = envy::from_env::<Config>().unwrap();
    println!("{} {}", config.world_x, config.world_y);

    // Initialize the relative coordinates for the prompt
    let x_val = (config.world_x % 2) as i64;
    let y_val = (config.world_y % 2) as i64;
    let p = prompt::PromptConfig::new(-x_val, x_val, -y_val, y_val);

    let facility_count = p.get_facility_count();
    println!("Generating {} facilities!", facility_count);
    let map = FacilityMap::new(facility_count, config.world_x, config.world_y);

    let mut count: usize = 1;
    for val in &map.location {
        print!("{:4} ", val);
        if count % config.world_x == 0 {
            println!();
        }
        count += 1;
    }

    // Prompt for the first user input before entering the loop
    let mut ret: Result<(usize, usize)> = p.get_input_coordinates();
    while ret.is_ok() {
        let (x, y): (usize, usize) = ret.unwrap();
        map.calc_distance(x, y);
        ret = p.get_input_coordinates();

    }

    Ok(())
}
