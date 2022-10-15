use salah::prelude::*;
use rocket::serde::{ Serialize, json::{json, Value} };
use toml;
use std::{fs, process::exit};
use serde_derive::Deserialize;


#[derive(Deserialize)]
struct ConfigIsm {
    config: SubConfig
}

#[derive(Deserialize)]
struct SubConfig {
    ip: String,
    port: u16
}

// fn main() {
//     let lat = 23.7231;
//     let lon = 90.4086;

//     let dhaka_city = Coordinates::new(lat, lon);
//     let date = Utc::today();
//     let params = Configuration::with(Method::Karachi, Madhab::Hanafi);
//     let prayers = PrayerSchedule::new()
//         .on(date)
//         .for_location(dhaka_city)
//         .with_configuration(params)
//         .calculate();

//     match prayers
//     {
//         Ok(prayer) => {
//             println!("{}: {}", Prayer::Fajr.name(), prayer.time(Prayer::Fajr).with_timezone(&Local).format("%-l:%M %p").to_string());
//             println!("{}: {}", Prayer::Sunrise.name(), prayer.time(Prayer::Sunrise).with_timezone(&Local).format("%-l:%M %p").to_string());
//             println!("{}: {}", Prayer::Dhuhr.name(), prayer.time(Prayer::Dhuhr).with_timezone(&Local).format("%-l:%M %p").to_string());
//             println!("{}: {}", Prayer::Asr.name(), prayer.time(Prayer::Asr).with_timezone(&Local).format("%-l:%M %p").to_string());
//             println!("{}: {}", Prayer::Maghrib.name(), prayer.time(Prayer::Maghrib).with_timezone(&Local).format("%-l:%M %p").to_string());
//             println!("{}: {}", Prayer::Isha.name(), prayer.time(Prayer::Isha).with_timezone(&Local).format("%-l:%M %p").to_string());
//             println!("{}: {}", Prayer::Qiyam.name(), prayer.time(Prayer::Qiyam).with_timezone(&Local).format("%-l:%M %p").to_string());
//         },
//         Err(error) => println!("Could not calculate prayer times: {}", error)
//     }
// }

#[macro_use] extern crate rocket;

#[get("/")]
fn index() -> Value {
    let filecontent = match fs::read_to_string("./testing/ismconf.toml") {
        Ok(c) => c,
        Err(_) => {
            eprintln!("Could not find or read file!");

            exit(1);
        }
    };

    let config: ConfigIsm = match toml::from_str(&filecontent) {
        // If successful, return data as `Data` struct.
        // `d` is a local variable.
        Ok(d) => d,
        // Handle the `error` case.
        Err(_) => {
            // Write `msg` to `stderr`.
            eprintln!("Unable to load data from config file!");
            // E
            exit(1);
        }
    };
    
    json!({
        "status": 200,
        "message": "Adhan System Online...".to_string() + &config.config.ip
    })
}

#[launch]
fn rocket() -> _ {
    rocket::build().mount("/", routes![index])
}


