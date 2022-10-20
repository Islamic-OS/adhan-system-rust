mod routes;
mod methods;
mod patched_methods;
mod unit_salah;

use methods::{get_config, get_madhab, get_method};

use warp::Filter;
use notify_rust::Notification;
use salah::prelude::*;

async fn daemon_thread() {
    let mut alert = false;
    let mut name: String = String::from("");

    loop {
        if alert {
            Notification::new()
                .summary(&format!("{} time started!", &name))
                .body(&format!("{} time started! You may pray now or wait for your nearest Adhan from your mosque!", name))
                .auto_icon()
                .show();

            alert = false;

            tokio::time::delay_for(tokio::time::Duration::from_millis(60000));
        }

        let config = get_config();

        let city = Coordinates::new(config.general.latitude, config.general.longitude);
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

                if hours == 0 && mins == 0 {
                    alert = true;
                    name = times.next().name();
                    
                    continue;
                }

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
            }
        }
    }
}


#[tokio::main]
async fn main() {
    tokio::spawn(async move {
        daemon_thread().await;
    });

    println!("Server running on port 10295...");
    
    let index = crate::routes::index();
    let today = crate::routes::today_wakt_times();
    let current = crate::routes::current_prayer();
    let qibla = crate::routes::qibla_direction();

    let routes = index
        .or(today)
        .or(current)
        .or(qibla);

    warp::serve(routes)
        .run(([127, 0, 0, 1], 10295))
        .await;
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

// #[macro_use]
// extern crate rocket;

// Intialize the Rocket framework

// #[launch]
// fn rocket() -> _ {
//     let figment = rocket::Config::figment().merge(("port", 10295));

//     rocket::custom(figment)
//         .mount("/", routes![routes::index])
//         .mount("/", routes![routes::today_wakt_times])
//         .mount("/", routes![routes::current_prayer])
//         .mount("/", routes![routes::qibla_direction])
//         .register("/", catchers![routes::not_found])
// }
