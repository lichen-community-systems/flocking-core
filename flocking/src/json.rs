use super::*;

pub fn parse_composition(composition_spec_json: &str) -> Result<CompositionSpec, serde_json::Error> {
    serde_json::from_str::<CompositionSpec>(composition_spec_json)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse() {
        let composition_spec_json = r#"{
            "environment": {
                "host": "CoreAudio",
                "input_device": "MacBook Pro Microphone",
                "output_device": "MacBook Pro Speakers",
                "num_input_channels": 2,
                "num_output_channels": 2,
                "sample_rate": 44100,
                "buffer_size": 128,
                "block_size": 4
            },
            "signals": {}
        }"#;

        let expected = CompositionSpec {
            environment: EnvironmentSettings {
                host: Some("CoreAudio".to_string()),
                input_device: Some(
                    "MacBook Pro Microphone".to_string()),
                output_device: Some(
                    "MacBook Pro Speakers".to_string()),
                num_input_channels: Some(2),
                num_output_channels: Some(2),
                sample_rate: Some(44100),
                buffer_size: Some(128),
                block_size: Some(4)
            },
            signals: Some(
                HashMap::<String, SignalSpec>::new()
            ),
            connections: None
        };

        let actual = json::parse_composition(composition_spec_json).unwrap();
        assert_eq!(expected, actual);
    }
}
