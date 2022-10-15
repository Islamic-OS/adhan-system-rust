use chrono;
use rocket::{
    serde::json::{json, Value},
    Request,
};
use salah::prelude::*;
use serde_derive::Deserialize;
use std::{fs, process::exit};
use toml;

// Data structure for the configuration file

#[derive(Deserialize)]
pub struct ConfigIsm {
    general: GeneralConfig,
    islamic: IslamicConfig,
}

#[derive(Deserialize)]
struct GeneralConfig {
    latitude: f64,
    longitude: f64,
}

#[derive(Deserialize)]
struct IslamicConfig {
    method: String,
    madhab: String,
}

// Dependency methods

fn get_method(method: &str) -> Method {
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

fn get_madhab(madhab: &str) -> Madhab {
    match madhab {
        "Hanafi" => Madhab::Hanafi,
        "Shafi" => Madhab::Shafi,
        &_ => Madhab::Hanafi,
    }
}

fn get_config() -> ConfigIsm {
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

// Routes

#[catch(404)]
pub fn not_found(req: &Request) -> Value {
    json!({
        "status": 404,
        "reason": "Endpoint Not found! Path: ".to_string() + &req.uri().path().to_string()
    })
}

#[get("/")]
pub fn index() -> Value {
    let config = get_config();

    json!({
        "status": 200,
        "message": "Adhan System Online...",
        "data": {
          "latitude": config.general.latitude,
          "longitude": config.general.longitude,
          "timezone": chrono::Local::now().offset().to_string(),
          "method": config.islamic.method,
          "madhab": config.islamic.madhab
        }
    })
}

#[get("/today")]
pub fn today_wakt_times() -> Value {
    let config = get_config();

    let lat = config.general.latitude;
    let lon = config.general.longitude;

    let dhaka_city = Coordinates::new(lat, lon);
    let date = Utc::today();
    let params = Configuration::with(
        get_method(&config.islamic.method),
        get_madhab(&config.islamic.madhab),
    );
    let prayers = PrayerSchedule::new()
        .on(date)
        .for_location(dhaka_city)
        .with_configuration(params)
        .calculate();

    match prayers {
        Ok(prayer) => {
            json!({
                "status": 200,
                "message": "Today's Salah Times",
                "data": {
                    "fajr": prayer.time(Prayer::Fajr).with_timezone(&Local).format("%-l:%M %p").to_string(),
                    "sunrise": prayer.time(Prayer::Sunrise).with_timezone(&Local).format("%-l:%M %p").to_string(),
                    "dhuhr": prayer.time(Prayer::Dhuhr).with_timezone(&Local).format("%-l:%M %p").to_string(),
                    "asr": prayer.time(Prayer::Asr).with_timezone(&Local).format("%-l:%M %p").to_string(),
                    "maghrib": prayer.time(Prayer::Maghrib).with_timezone(&Local).format("%-l:%M %p").to_string(),
                    "isha": prayer.time(Prayer::Isha).with_timezone(&Local).format("%-l:%M %p").to_string(),
                    "qiyam": prayer.time(Prayer::Qiyam).with_timezone(&Local).format("%-l:%M %p").to_string()
                }
            })
        }
        Err(error) => {
            eprintln!("Could not calculate prayer times: {}", error);
            exit(1);
        }
    }
}
