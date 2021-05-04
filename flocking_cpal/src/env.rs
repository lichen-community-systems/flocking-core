// TODO: Export Environment to the root of flocking_cpal.

use flocking::{EnvironmentSettings};
use cpal;

pub fn find_host(settings: &EnvironmentSettings) -> cpal::Host {
    let host: cpal::Host = match &settings.host {
        Some(host_name) => {
            let host_ids = cpal::available_hosts();
            let mut host_iter = host_ids.iter();

            if let Some(matched_id) = host_iter.find(|&host_id| host_id.name() == host_name) {
                cpal::host_from_id(*matched_id).unwrap()
            } else {
                cpal::default_host()
            }
        },

        None => cpal::default_host()
    };

    host
}

pub struct Environment {
    pub settings: EnvironmentSettings,
    pub host: cpal::Host
}

impl Environment {
    pub fn new(options: EnvironmentSettings) -> Environment {
        // TODO: Find a better way to store defaults.
        let defaults = EnvironmentSettings {
            host: None,
            input_device: None,
            output_device: None,
            num_input_channels: Some(2),
            num_output_channels: Some(2),
            sample_rate: Some(44100),
            buffer_size: Some(128),
            block_size: Some(64)
        };

        let settings = flocking::utils::merge(
            &defaults, Some(&options));
        let host = find_host(&settings);

        Environment {
            settings: settings,
            host: host
        }
    }
}
