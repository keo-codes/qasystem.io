use serde::Deserialize;
use std::fs;

#[derive(Deserialize)]
pub struct Config {
    pub vocab_size: usize,
    pub max_len: usize,
    pub d_model: usize,
    pub layers: usize,
    pub batch_size: usize,
    pub epochs: usize,
    pub learning_rate: f64,
    pub checkpoint_dir: String,
}

pub fn load() -> Config {
    let data = fs::read_to_string("config/config.json").unwrap();
    serde_json::from_str(&data).unwrap()
}