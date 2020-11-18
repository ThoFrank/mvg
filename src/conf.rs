use serde::{Deserialize, Serialize};

use lazy_static::lazy_static;

lazy_static! {
    pub static ref DEFAULT_LOCATION: String = {
        let mut home_dir =
            std::path::PathBuf::from(std::env::var("HOME").unwrap_or(String::from("")));
        home_dir.push(".mvg.conf");
        String::from(home_dir.to_str().unwrap())
    };
}

pub fn load_config(location: &str) -> Config {
    match std::fs::File::open(location) {
        Ok(file) => match serde_yaml::from_reader(file) {
            Ok(conf) => return conf,
            Err(e) => println!("Failed to parse config file!\n{}", e),
        },
        Err(e) => println!("Failed to open config file!\n{}", e),
    }
    if let Ok(file) = std::fs::File::open(location) {
        if let Ok(conf) = serde_yaml::from_reader(file) {
            return conf;
        }
    }
    Config::default()
}

#[derive(Serialize, Deserialize, Default, Debug)]
pub struct Config {
    #[serde(default)]
    pub color_option: ColorOption,
    #[serde(default)]
    pub default_station: Option<String>,
}

#[derive(Serialize, Deserialize, Debug)]
pub enum ColorOption {
    TrueColor,
    Ansi,
    No,
}

impl Default for ColorOption {
    fn default() -> Self {
        let color_env_var = std::env::var("COLORTERM").unwrap_or(String::new());
        if color_env_var.contains("truecolor") || color_env_var.contains("24bit") {
            ColorOption::TrueColor
        } else {
            ColorOption::Ansi
        }
    }
}
