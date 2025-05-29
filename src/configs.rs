use std::{env, fs::File, io::BufReader};

pub fn get_config() -> plume_core::config::Config {
    let config_path = env::var("PLUME_CONFIG").expect("Config env var not set");
    let config_file = File::open(format!("{}/configs.json", config_path)).expect("Eror opening config file");
    let reader = BufReader::new(config_file);

    serde_json::from_reader(reader).expect("Unable to convert this file to json")
}
