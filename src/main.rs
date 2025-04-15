use crate::facility::FacilityMap;
use anyhow::Result;
use serde::Deserialize;
use std::error::Error;

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
    #[serde(default = "default_k")]
    k: usize,
}

fn default_world_xy() -> usize {
    21
}

fn default_k() -> usize {
    3
}

fn main() -> Result<(), Box<dyn Error>> {
    dotenvy::dotenv().ok();
    let config = envy::from_env::<Config>().unwrap();
    println!("{} {}", config.world_x, config.world_y);

    // Initialize the relative coordinates for the prompt
    let x_val = ((config.world_x - 1) / 2) as i64;
    let y_val = ((config.world_y - 1) / 2) as i64;
    let p = prompt::PromptConfig::new(-x_val, x_val, -y_val, y_val);

    let facility_count = p.get_facility_count();
    println!("Generating {} facilities!", facility_count);
    let map = FacilityMap::new(facility_count, config.world_x, config.world_y);
    map.print();

    // Prompt for the first user input before entering the loop
    let mut ret: Result<(usize, usize)> = p.get_input_coordinates();
    while ret.is_ok() {
        let (x, y): (usize, usize) = ret.unwrap();

        map.knn(config.k, x, y);
        ret = p.get_input_coordinates();
    }

    Ok(())
}
