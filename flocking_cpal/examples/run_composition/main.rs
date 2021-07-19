use flocking;
use std::{fs, env, process, error::Error};

extern crate flocking_cpal;

fn run(composition_file_path: String) -> Result<(), Box<dyn Error>> {
    let compostion_json = fs::read_to_string(composition_file_path)?;

    let composition_spec = flocking::json::parse_composition(&compostion_json)?;

    let environment = flocking_cpal::env::Environment::new(composition_spec.environment);

    println!("Selected host: {:?}", environment.host.id());

    if let Some(output_device) = environment.host_audio.output {
        println!("Selected output device: {}",
            flocking_cpal::utils::device_display_name(&output_device));
    }

    if let Some(input_device) = environment.host_audio.input {
        println!("Selected input device: {}",
        flocking_cpal::utils::device_display_name(&input_device));
    }

    println!("{:?}", environment.settings);

    Ok(())
}

fn check_arguments(args: &Vec<String>) {
    if args.len() < 2 {
        println!("Error: No composition file was specified as an argument.");
        process::exit(1);
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();
    check_arguments(&args);

    let composition_file_path = args[1].clone();
    println!("{}", composition_file_path);

    if let Err(e) = run(composition_file_path) {
        println!("Error running composition: {}", e);
        process::exit(1);
    }
}
