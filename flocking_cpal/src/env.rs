// TODO: Export Environment to the root of flocking_cpal.

use flocking::EnvironmentSettings;
use cpal::Host;
use cpal::traits::DeviceTrait;
use cpal::traits::HostTrait;

pub fn find_host(settings: &EnvironmentSettings) -> cpal::Host {
    let host: cpal::Host = match &settings.host {
        Some(host_name) => {
            let host_ids = cpal::available_hosts();
            let mut host_iter = host_ids.iter();

            if let Some(matched_id) = host_iter.find(
                |&host_id| host_id.name().to_lowercase() ==
                host_name.to_lowercase()) {
                // TODO: Properly handle this error.
                cpal::host_from_id(*matched_id).unwrap()
            } else {
                cpal::default_host()
            }
        },

        None => cpal::default_host()
    };

    host
}

pub fn match_device_name(requested_name: &String, device: &cpal::Device) -> bool {
    match device.name() {
        Ok(device_name) => device_name.to_lowercase() ==
            requested_name.to_lowercase(),

        // Here we're assuming that a device name error
        // signifies that this a device we can never use
        // because we can't refer to it by name.
        Err(_e) => false
    }
}

// TODO: This function likely will need to verify that
// this device has an appropriate (or just any?) configuration
// as well. This is particularly important when the user doesn't
// specify a device name, but does request e.g. a particular
// channel count, sample rate, or buffer size. As a result, this
// function will need to be refactored to also take the
// EnvironmentSettings object as an argument.
pub fn find_device(
    requested_device: &Option<String>,
    mut device_iter: std::iter::Filter<cpal::Devices, for<'r> fn(&'r cpal::Device) -> bool>,
    get_default: &dyn Fn() -> Option<cpal::Device>
) -> Option<cpal::Device> {
    match requested_device {
        Some(requested_name) => {
            match device_iter.find(|device|
                match_device_name(requested_name, &device)) {
                Some(matched_device) => Some(matched_device),
                None => get_default()
            }
        },
        None => get_default()
    }
}

pub fn find_output_device(
    requested_device: &Option<String>,
    host: &cpal::Host
) -> Option<cpal::Device> {
    match host.output_devices() {
        Ok(device_iter) => find_device(
            requested_device, device_iter, &|| host.default_output_device()),

        // We assume an error while accessing output devices
        // means that the requested device was not found.
        Err(_e) => None
    }
}

pub fn find_input_device(
    requested_device: &Option<String>,
    host: &cpal::Host
) -> Option<cpal::Device> {
    match host.input_devices() {
        Ok(device_iter) => find_device(
            requested_device, device_iter, &|| host.default_input_device()),

        // We assume an error while accessing input devices
        // means that the requested device was not found.

        Err(_e) => None
    }
}

// TODO: Implement.
pub fn find_stream_config(
    device: &cpal::Device,
    settings: &EnvironmentSettings
) -> Option<cpal::SupportedStreamConfig> {
    let configs_range = device.supported_output_configs();
    None
}

pub struct HostAudio {
    pub output: Option<cpal::Device>,
    pub input: Option<cpal::Device>
}

impl HostAudio {
    pub fn new(
        settings: &EnvironmentSettings,
        host: &Host
    ) -> HostAudio {
        return HostAudio {
            output: find_output_device(&settings.output_device, host),
            input: find_input_device(&settings.input_device, host)
        }
    }
}

#[derive(Debug, Clone)]
pub struct AudioConnectionError;

pub struct AudioConnection {
    pub config: cpal::SupportedStreamConfig,
    pub stream: cpal::Stream
}

impl AudioConnection {
    // TODO: Implement.
    pub fn new(
        host_audio: &HostAudio,
        settings: &EnvironmentSettings
    ) -> Result<AudioConnection, AudioConnectionError> {
        Err(AudioConnectionError)
    }
}

pub struct AudioConnections {
    settings: EnvironmentSettings,
    pub output: Option<AudioConnection>,
    pub input: Option<AudioConnection>
}

pub struct Environment {
    pub settings: EnvironmentSettings,
    pub host: cpal::Host,
    pub host_audio: HostAudio
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

        let settings = flocking::utils::merge_options(
            &defaults, Some(&options));
        let host = find_host(&settings);
        let host_audio = HostAudio::new(
            &settings, &host);

        Environment {
            settings: settings,
            host: host,
            host_audio: host_audio
        }
    }

    // TODO: Implement.
    pub fn connect() -> AudioConnections {
        // This is also where we'll have to examine
        // the connections and produce an updated EnvironmentSettings
        // object that represents the actual state of the
        // audio connections. For example, we may end up with a
        // stream config that has a different sample rate or number of
        // channels than was specified.
        AudioConnections {
            settings: EnvironmentSettings {
                // TODO: Fill this with the host's "display name."
                host: None,

                // TODO: Fill these with the input/output devices'
                // display names.
                input_device: None,
                output_device: None,

                // TODO: These can be sourced from
                // the AudioConnection's config object.
                // But what if the input and output values don't
                // match?
                // e.g. we connect to 4-channel EIE at 44100 for
                // output, but we connect to the built-in microphone
                // (2 channel 48000) for input.
                num_input_channels: None,
                num_output_channels: None,
                sample_rate: None,
                buffer_size: None,
                block_size: None
            },
            output: None,
            input: None
        }
    }
}
