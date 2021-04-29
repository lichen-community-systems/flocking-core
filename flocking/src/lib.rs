pub mod json;

use serde::{Deserialize, Serialize};

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
    pub environment: EnvironmentSettings,
    pub signals: Option<HashMap<String, SignalSpec>>,
    pub connections: Option<HashMap<String, ConnectionSpec>>
}
