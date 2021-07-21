pub mod json;
pub mod utils;

use serde::{Deserialize, Serialize};
use merge::Merge;

// TODO: Replace this with an equivalent no-std alloc collection.
use std::collections::HashMap;

#[derive(Clone, Merge, Serialize, Deserialize, Eq, PartialEq, Debug)]
pub struct EnvironmentSettings {
    pub host: Option<String>,
    pub input_device: Option<String>,
    pub output_device: Option<String>,
    pub num_input_channels: Option<u32>,
    pub num_output_channels: Option<u32>,
    pub sample_rate: Option<u32>,
    pub buffer_size: Option<u32>,
    pub block_size: Option<u32>
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
