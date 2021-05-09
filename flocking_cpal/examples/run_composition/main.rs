use flocking;
use std::{fs, env, process, error::Error};
use cpal::traits::DeviceTrait;


extern crate flocking_cpal;

fn run (composition_file_path: String) -> Result<(), Box<dyn Error>> {
    let compostion_json = fs::read_to_string(composition_file_path)?;

    let composition_spec = flocking::json::parse_composition(&compostion_json)?;

    let environment = flocking_cpal::env::Environment::new(composition_spec.environment);

    println!("Selected host: {:?}", environment.host.id());
    println!("Selected output device: {}",
        environment.host_audio.output.unwrap().name().unwrap());
    println!("Selected input device: {}",
        environment.host_audio.input.unwrap().name().unwrap());

    println!("{:?}", environment.settings);

    Ok(())
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let composition_file_path = args[1].clone();
    println!("{}", composition_file_path);

    if let Err(e) = run(composition_file_path) {
        println!("Error running composition: {}", e);
        process::exit(1);
    }
}
