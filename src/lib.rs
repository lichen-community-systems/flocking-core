use cpal::traits::{HostTrait, DeviceTrait};

// CPAL has a bug where it will panic if WASAPI devices are
// enumerated after ASIO devices. Until this is fixed, we
// simply refuse to enumerate WASAPI devices.
// The issue is likely due to ASIO and CPAL's WASAPI
// implementation initializing COM in the same thread using
// incompatible threading models. For more infromation, see:
//   https://github.com/RustAudio/cpal/issues/538
//   https://github.com/RustAudio/cpal/pull/504
//   https://github.com/RustAudio/cpal/pull/441
const HOST_ENUMERATION_BLACKLIST: &'static [&'static str] = if cfg!(windows) {
    &["WASAPI"]
} else {
    &[]
};

pub fn print_audio_system_tree() {
    let default_host = cpal::default_host();

    for host_id in cpal::available_hosts() {
        print_host(host_id, default_host.id());

        if HOST_ENUMERATION_BLACKLIST.contains(&host_id.name()) {
            println!("  Skipping {} devices.",
                host_id.name());
            continue;
        }

        match cpal::host_from_id(host_id) {
            Ok(host)=> {
                print_devices(host);
            },
            Err(e) => {
                println!("  The {:?} host is unvailable. {}", host_id, e);
            }
        }
    }
}

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

fn is_same_device(left: &cpal::Device, right: &cpal::Device) -> bool {
    if let Ok(left_device_name) = left.name() {
        if let Ok(right_device_name) = right.name() {
            return left_device_name == right_device_name;
        }
    }
    false
}

fn label_for_device(device: &cpal::Device, host: &cpal::Host) -> String {
    let mut device_name_label = String::new();

    match host.default_input_device() {
        Some(default_input_device) => {
            if is_same_device(device, &default_input_device) {
                device_name_label.push_str("[default input] ");
            }
        },

        None => println!("  No default input device is available for this host.")
    }

    match host.default_output_device() {
        Some(default_output_device) => {
            if is_same_device(device, &default_output_device) {
                device_name_label.push_str("[default output] ");
            }
        },

        None => println!("  No default output device is available for this host.")
    }

    device_name_label
}

fn print_device(device: &cpal::Device, host: &cpal::Host) {
    let device_name_label = label_for_device(device, host);

    let formatted_device_name = match device.name() {
        Ok(device_name) => device_name,
        Err(_e) => String::from("Unnamed Device")
    };

    println!("  * {} {}", formatted_device_name, device_name_label);
    print_device_configs(device);
}

fn print_devices(host: cpal::Host) {
    match host.devices() {
        Ok(devices) => {
            for device in devices {
                print_device(&device, &host);
            }
        },
        Err(e) => println!("No devices are available for this host. {}", e)
    }
}
