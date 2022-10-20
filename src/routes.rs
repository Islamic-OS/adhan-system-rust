use std::process::exit;
use chrono;
use serde_json::json;
// use rocket::{
//     serde::json::{json, Value},
//     Request,
// };
use salah::prelude::*;
use warp::Filter;

use crate::methods::*;
use crate::patched_methods::QiblahPatched;



// Routes

// 404 route is redundant
// #[catch(404)]
// pub fn not_found() -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
//     warp::path("*")
//         .map(|| {
//             let res = json!({
//                 "status": 404,
//                 "reason": "Endpoint Not found!"
//             });

//             warp::reply::json(&res)
//         })
// }

// #[get("/")]
pub fn index(
) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    let config = get_config();

    warp::path::end().and(warp::get())
        .map(move || {
            let res = json!({
                "status": 200,
                "message": "Adhan System Online...",
                "data": {
                    "latitude": config.general.latitude.clone(),
                    "longitude": config.general.longitude.clone(),
                    "timezone": chrono::Local::now().offset().to_string(),
                    "method": config.islamic.method.clone(),
                    "madhab": config.islamic.madhab.clone()
                }
            });

            warp::reply::json(&res)
        })
}

// #[get("/today")]
pub fn today_wakt_times(
) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    let config = get_config();

    let lat = config.general.latitude;
    let lon = config.general.longitude;

    // let city = Coordinates::new(lat, lon);
    let city = Coordinates { latitude: lat, longitude: lon };
    let date = Utc::today();
    let params = Configuration::with(
        get_method(&config.islamic.method),
        get_madhab(&config.islamic.madhab),
    );
    let prayers = PrayerSchedule::new()
        .on(date)
        .for_location(city)
        .with_configuration(params)
        .calculate();

    match prayers {
        Ok(prayer) => {
            warp::path("today")
                .and(warp::get())
                .and(warp::path::end())
                .map(move || {
                    let res = json!({
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
                    });

                    warp::reply::json(&res)
                })
        }
        Err(error) => {
            eprintln!("Could not calculate prayer times: {}", error);
            exit(1);
        }
    }
}

// #[get("/current")]
pub fn current_prayer(
) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    let config = get_config();

    let lat = config.general.latitude;
    let lon = config.general.longitude;

    let city = Coordinates::new(lat, lon);
    let date = Utc::today();
    let params = Configuration::with(
        get_method(&config.islamic.method),
        get_madhab(&config.islamic.madhab),
    );
    let prayers = PrayerSchedule::new()
        .on(date)
        .for_location(city)
        .with_configuration(params)
        .calculate();
    
    match prayers {
        Ok(times) => {
            let (hours, mins) = times.time_remaining();
            
            warp::path("current")
                .and(warp::get())
                .and(warp::path::end())
                .map(move || {
                    let res = json!({
                        "status": 200,
                        "message": "Today's Salah Times",
                        "data": {
                            "current": {
                                "name": times.current().name(),
                                "timeRemaining": hours.to_string() + ":" + &mins.to_string()
                            },
                            "next": times.next().name()
                        }
                    });

                    warp::reply::json(&res)
                })
        }
        Err(_) => {
            eprint!("Error fetching PrayerTimes!");

            exit(1);
        }
    }
}

// #[get("/qibla")]
pub fn qibla_direction() -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    let config = get_config();
    
    let lat = config.general.latitude;
    let lon = config.general.longitude;

    let city = Coordinates::new(lat, lon);
    
    let qiblah = QiblahPatched::new(city);
    
    warp::path("qibla")
        .and(warp::get())
        .and(warp::path::end())
        .map(move || {
            let res = json!({
                "status": 200,
                "message": "Direction of the Holy Ka'baa, in degrees from North, from your coordinates",
                "degrees": qiblah.0.clone()
            });

            warp::reply::json(&res)
        })
}

