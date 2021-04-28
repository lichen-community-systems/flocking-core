use serde::{Deserialize, Serialize};
use serde_json::Result;

 // TODO: Replace this with an equivalent no-std alloc collection.
use std::collections::HashMap;

#[derive(Serialize, Deserialize, Eq, PartialEq, Debug)]
pub struct EnvironmentSettings {
    host: Option<String>,
    input_device: Option<String>,
    output_device: Option<String>,
    num_input_channels: Option<u32>,
    num_output_channels: Option<u32>,
    sample_rate: Option<u32>,
    buffer_size: Option<u32>,
    block_size: Option<u32>
}

#[derive(Serialize, Deserialize, Eq, PartialEq, Debug)]
pub struct SignalSpec {}

#[derive(Serialize, Deserialize, Eq, PartialEq, Debug)]
pub struct ConnectionSpec {}

#[derive(Serialize, Deserialize, Eq, PartialEq, Debug)]
pub struct CompositionSpec {
    environment: EnvironmentSettings,
    signals: Option<HashMap<String, SignalSpec>>,
    connections: Option<HashMap<String, ConnectionSpec>>
}

pub struct CompositionSpecParser {}

impl CompositionSpecParser {
    pub fn new() -> CompositionSpecParser {
        CompositionSpecParser{}
    }

    pub fn parse(self, app_spec_json: &str) -> Result<CompositionSpec> {
        serde_json::from_str::<CompositionSpec>(app_spec_json)
    }
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

        let parser = CompositionSpecParser::new();
        let actual = parser.parse(composition_spec_json).unwrap();
        assert_eq!(expected, actual);
    }
}
