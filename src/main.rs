use cpal::traits::{HostTrait, DeviceTrait};

fn print_host(host_id: cpal::HostId, default_host_id: cpal::HostId) {
    let host_label = {
        if host_id == default_host_id {
            "[default host]"
        } else {
            ""
        }
    };

    println!("* {} {}", host_id.name(), host_label);
}

fn print_device_config(config: cpal::SupportedStreamConfigRange) {
    let buffer_size = config.buffer_size();

    let buffer_sizes_display = match buffer_size {
        cpal::SupportedBufferSize::Range{min, max} => {
            format!("{}-{}", *min, *max)
        },
        cpal::SupportedBufferSize::Unknown => "unknown".to_string()
    };

    println!("    * {} KHz, {} channels, {:?}, {} buffer size range",
        config.max_sample_rate().0,
        config.channels(),
        config.sample_format(),
        buffer_sizes_display
    );
}

fn print_device_configs(device: &cpal::Device) {
    match device.supported_input_configs() {
        Ok(supported_input_configs) => {
            for input_config in supported_input_configs {
                print_device_config(input_config);
            }
        },
        Err(e) => {
            println!("      No output configurations found. {}", e);
        }
    };

    match device.supported_output_configs() {
        Ok(supported_output_configs) => {
            for output_config in supported_output_configs {
                print_device_config(output_config);
            }
        },
        Err(e) => {
            println!("      No output configurations found. {}", e);
        }
    };
}

fn print_device(device: &cpal::Device,
    default_input_name: &String, default_output_name: &String) {
    let device_name = device.name()
        .expect("The device name is not available");

    let device_name_label = {
        let mut label = String::new();
        if device_name == *default_input_name {
            label.push_str("[default input] ");
        }

        if device_name == *default_output_name {
            label.push_str("[default output]");
        }

        label
    };

    println!("  * {} {}", device_name, device_name_label);
    print_device_configs(device);
}

fn main() {
    let default_host = cpal::default_host();

    println!("Hosts");
    println!("-----");
    for host_id in cpal::available_hosts() {
        let host = cpal::host_from_id(host_id).expect("Host is not available.");
        print_host(host_id, default_host.id());

        let default_input = host.default_input_device()
            .expect("The default input device is not available.");

        let default_input_name = default_input.name()
            .expect("The default input name is not available.");

        let default_output = host.default_output_device()
            .expect("The host's default output device is not available.");

        let default_output_name = default_output.name()
            .expect("The default output name is not available.");

        let devices = host.devices()
            .expect("The host's devices are not available.");

        for device in devices {
            print_device(&device, &default_input_name, &default_output_name);
        }
    }


}
