use serde_derive::Deserialize;
use std::{fs, process::exit};
use toml;
use salah::prelude::*;

// Data structure for the configuration file

#[derive(Deserialize)]
pub struct ConfigIsm {
    pub general: GeneralConfig,
    pub islamic: IslamicConfig,
}

#[derive(Deserialize)]
pub struct GeneralConfig {
    pub latitude: f64,
    pub longitude: f64,
}

#[derive(Deserialize)]
pub struct IslamicConfig {
    pub method: String,
    pub madhab: String,
}

// Dependency methods

pub fn get_method(method: &str) -> Method {
    match method {
        "MWL" => Method::MuslimWorldLeague,
        "Egyptian" => Method::Egyptian,
        "Karachi" => Method::Karachi,
        "UmmAlQura" => Method::UmmAlQura,
        "Dubai" => Method::Dubai,
        "Qatar" => Method::Qatar,
        "Kuwait" => Method::Kuwait,
        "MoonsightingCommittee" => Method::MoonsightingCommittee,
        "Singapore" => Method::Singapore,
        "Turkey" => Method::Turkey,
        "Tehran" => Method::Tehran,
        "ISNA" => Method::NorthAmerica,
        "Other" => Method::Other,
        &_ => Method::Other,
    }
}

pub fn get_madhab(madhab: &str) -> Madhab {
    match madhab {
        "Hanafi" => Madhab::Hanafi,
        "Shafi" => Madhab::Shafi,
        &_ => Madhab::Hanafi,
    }
}

pub fn get_config() -> ConfigIsm {
    let filecontent = match fs::read_to_string("./testing/ismconf.toml") {
        Ok(c) => c,
        Err(_) => {
            eprintln!("Could not find or read file!");

            exit(1);
        }
    };

    let config: ConfigIsm = match toml::from_str(&filecontent) {
        Ok(d) => d,
        Err(_) => {
            eprintln!("Unable to load data from config file!");

            exit(1);
        }
    };

    config
}

