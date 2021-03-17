use cpal::traits::{HostTrait, DeviceTrait};

fn main() {
    let host = cpal::default_host();

    let device = host.default_output_device()
        .expect("The default output device is not available.");

    println!("Hello, {}: {}",
        host.id().name(),
        device.name().expect("The Device name is not available"));

    let supported_configs = device.supported_output_configs()
        .expect("There are no supported output configurations.");

    for config in supported_configs {
        println!("Channels: {}, sample rate: {:?}-{:?}, sample format: {:?}, buffer size: {:?}",
            config.channels(),
            config.min_sample_rate(),
            config.max_sample_rate(),
            config.sample_format(),
            config.buffer_size());
    }
}
