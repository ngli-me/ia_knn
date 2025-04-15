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
}

fn default_world_xy() -> usize {
    20
}

fn main() -> Result<(), Box<dyn Error>> {
    dotenvy::dotenv().ok();
    let config = envy::from_env::<Config>().unwrap();

    // Initialize the relative coordinates for the prompt
    let x_val = (config.world_x % 2) as i64;
    let y_val = (config.world_y % 2) as i64;
    let p = prompt::PromptConfig::new(-x_val, x_val, -y_val, y_val);

    let facility_count = p.get_facility_count();
    println!("Generating {} facilities!", facility_count);

    for x in 0..10 {
        println!("* * * * * * * * * *");
    }


    let (x, y): (usize, usize) = p.get_input_coordinates().expect("TODO: panic message");
    println!("x: {}, y: {}", x, y);

    Ok(())
}
