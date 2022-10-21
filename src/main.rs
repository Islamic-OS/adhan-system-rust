mod methods;
mod models;
mod patched_methods;
mod routes;
mod unit_salah;

use methods::{get_config, get_madhab, get_method};

use notify_rust::Notification;
use salah::prelude::*;

use actix_web::{App, HttpServer};

const PORT: u16 = 10295;

async fn daemon_proc() {
    let mut alert = false;
    let mut name: String = String::from("");

    loop {
        if alert {
            if cfg!(target_os = "windows") {
                println!("{} time started!", &name);
            } else {
                Notification::new()
                    .summary(&format!("{} time started!", &name))
                    .body(&format!("{} time started! You may pray now or wait for your nearest Adhan from your mosque!", name))
                    .auto_icon()
                    .show().unwrap();
            }

            alert = false;

            println!("Pings suspended for 1 minute");
            tokio::time::sleep(std::time::Duration::from_secs(60)).await;
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
            }
            Err(_) => {
                eprintln!("Error fetching PrayerTimes!");
            }
        }
    }
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // Create a new thread to run the notification system
    tokio::spawn(async move { daemon_proc().await });

    println!("Started server...");

    HttpServer::new(|| {
        App::new()
            .service(routes::index)
            .service(routes::today_wakt_times)
            .service(routes::current_prayer)
            .service(routes::qiblah_direction)
    })
    .bind(("127.0.0.1", PORT))?
    .run()
    .await
}
