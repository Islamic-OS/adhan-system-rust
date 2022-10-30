use chrono;
// use serde_json::json;
use std::process::exit;
// use rocket::{
//     serde::json::{json, Value},
//     Request,
// };
use salah::prelude::*;
// use warp::Filter;
use actix_web::{get, web, Responder, Result};

use crate::methods::*;
use crate::models::{
    CurrentCurrentData, CurrentData, CurrentMod, IndexData, IndexMod, QiblahData, QiblahMod,
    TodayData, TodayMod,
};
use crate::patched_methods::QiblahPatched;

// Routes

#[get("/")]
pub async fn index() -> Result<impl Responder, actix_web::Error> {
    let config = get_config();

    // let res = json!({
    //     "status": 200,
    //     "message": "Adhan System Online...",
    //     "data": {
    //         "latitude": config.latitude.clone(),
    //         "longitude": config.longitude.clone(),
    //         "timezone": chrono::Local::now().offset().to_string(),
    //         "method": config.method.clone(),
    //         "madhab": config.madhab.clone()
    //     }
    // });

    let res = IndexMod {
        status: 200,
        message: "Adhan System Online...".to_string(),
        data: IndexData {
            latitude: config.latitude.clone(),
            longitude: config.longitude.clone(),
            timezone: chrono::Local::now().offset().to_string(),
            method: config.method.clone(),
            madhab: config.madhab.clone(),
        },
    };

    Ok(web::Json(res))

    // warp::path::end().and(warp::get()).map(move || {
    //     let res = json!({
    //         "status": 200,
    //         "message": "Adhan System Online...",
    //         "data": {
    //             "latitude": config.latitude.clone(),
    //             "longitude": config.longitude.clone(),
    //             "timezone": chrono::Local::now().offset().to_string(),
    //             "method": config.method.clone(),
    //             "madhab": config.madhab.clone()
    //         }
    //     });

    //     warp::reply::json(&res)
    // })
}

#[get("/today")]
pub async fn today_wakt_times() -> Result<impl Responder, actix_web::Error> {
    let config = get_config();

    let lat = config.latitude;
    let lon = config.longitude;

    // let city = Coordinates::new(lat, lon);
    let city = Coordinates {
        latitude: lat,
        longitude: lon,
    };
    let date = Utc::today();
    let params = Configuration::with(
        get_method(&config.method),
        get_madhab(&config.madhab),
    );
    let prayers = PrayerSchedule::new()
        .on(date)
        .for_location(city)
        .with_configuration(params)
        .calculate();

    match prayers {
        Ok(prayer) => {
            let res = TodayMod {
                status: 200,
                message: "Today's Salah Times".to_string(),
                data: TodayData {
                    fajr: prayer
                        .time(Prayer::Fajr)
                        .with_timezone(&chrono::Local)
                        .format("%-l:%M %p")
                        .to_string(),
                    sunrise: prayer
                        .time(Prayer::Sunrise)
                        .with_timezone(&chrono::Local)
                        .format("%-l:%M %p")
                        .to_string(),
                    dhuhr: prayer
                        .time(Prayer::Dhuhr)
                        .with_timezone(&chrono::Local)
                        .format("%-l:%M %p")
                        .to_string(),
                    asr: prayer
                        .time(Prayer::Asr)
                        .with_timezone(&chrono::Local)
                        .format("%-l:%M %p")
                        .to_string(),
                    maghrib: prayer
                        .time(Prayer::Maghrib)
                        .with_timezone(&chrono::Local)
                        .format("%-l:%M %p")
                        .to_string(),
                    isha: prayer
                        .time(Prayer::Isha)
                        .with_timezone(&chrono::Local)
                        .format("%-l:%M %p")
                        .to_string(),
                    qiyam: prayer
                        .time(Prayer::Qiyam)
                        .with_timezone(&chrono::Local)
                        .format("%-l:%M %p")
                        .to_string(),
                },
            };

            Ok(web::Json(res))

            // warp::path("today")
            //     .and(warp::get())
            //     .and(warp::path::end())
            //     .map(move || {
            //         let res = json!({
            //             "status": 200,
            //             "message": "Today's Salah Times",
            //             "data": {
            //                 "fajr": prayer.time(Prayer::Fajr).with_timezone(&chrono::Local).format("%-l:%M %p").to_string(),
            //                 "sunrise": prayer.time(Prayer::Sunrise).with_timezone(&chrono::Local).format("%-l:%M %p").to_string(),
            //                 "dhuhr": prayer.time(Prayer::Dhuhr).with_timezone(&chrono::Local).format("%-l:%M %p").to_string(),
            //                 "asr": prayer.time(Prayer::Asr).with_timezone(&chrono::Local).format("%-l:%M %p").to_string(),
            //                 "maghrib": prayer.time(Prayer::Maghrib).with_timezone(&chrono::Local).format("%-l:%M %p").to_string(),
            //                 "isha": prayer.time(Prayer::Isha).with_timezone(&chrono::Local).format("%-l:%M %p").to_string(),
            //                 "qiyam": prayer.time(Prayer::Qiyam).with_timezone(&chrono::Local).format("%-l:%M %p").to_string()
            //             }
            //         });

            //         warp::reply::json(&res)
            //     })
        }
        Err(error) => {
            eprintln!("Could not calculate prayer times: {}", error);

            exit(1);
        }
    }
}

#[get("/current")]
pub async fn current_prayer() -> Result<impl Responder, actix_web::Error> {
    let config = get_config();

    let lat = config.latitude;
    let lon = config.longitude;

    let city = Coordinates::new(lat, lon);
    let date = Utc::today();
    let params = Configuration::with(
        get_method(&config.method),
        get_madhab(&config.madhab),
    );
    let prayers = PrayerSchedule::new()
        .on(date)
        .for_location(city)
        .with_configuration(params)
        .calculate();

    match prayers {
        Ok(times) => {
            let (hours, mins) = times.time_remaining();

            let res = CurrentMod {
                status: 200,
                message: "Currently going-on and next Salah(s)".to_string(),
                data: CurrentData {
                    current: CurrentCurrentData {
                        name: times.current().name(),
                        time_remaining: hours.to_string() + ":" + &mins.to_string(),
                    },
                    next: times.next().name(),
                },
            };

            Ok(web::Json(res))

            // warp::path("current")
            //     .and(warp::get())
            //     .and(warp::path::end())
            //     .map(move || {
            //         let res = json!({
            //             "status": 200,
            //             "message": "Today's Salah Times",
            //             "data": {
            //                 "current": {
            //                     "name": times.current().name(),
            //                     "timeRemaining": hours.to_string() + ":" + &mins.to_string()
            //                 },
            //                 "next": times.next().name()
            //             }
            //         });

            //         warp::reply::json(&res)
            //     })
        }
        Err(_) => {
            eprint!("Error fetching PrayerTimes!");

            exit(1);
        }
    }
}

#[get("/qiblah")]
pub async fn qiblah_direction() -> Result<impl Responder, actix_web::Error> {
    let config = get_config();

    let lat = config.latitude;
    let lon = config.longitude;

    let city = Coordinates::new(lat, lon);

    let qiblah = QiblahPatched::new(city);

    let res = QiblahMod {
        status: 200,
        message: "Direction of the Holy Ka'baa, in degrees from North, from your coordinates"
            .to_string(),
        data: QiblahData {
            degrees: qiblah.0.clone(),
        },
    };

    Ok(web::Json(res))

    // warp::path("qibla")
    //     .and(warp::get())
    //     .and(warp::path::end())
    //     .map(move || {
    //         let res = json!({
    //             "status": 200,
    //             "message": "Direction of the Holy Ka'baa, in degrees from North, from your coordinates",
    //             "degrees": qiblah.0.clone()
    //         });

    //         warp::reply::json(&res)
    //     })
}
