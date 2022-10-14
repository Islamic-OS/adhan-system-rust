use salah::prelude::*;

fn main() {
    let lat = 23.7231;
    let lon = 90.4086;

    let dhaka_city = Coordinates::new(lat, lon);
    let date = Utc::today();
    let params = Configuration::with(Method::Karachi, Madhab::Hanafi);
    let prayers = PrayerSchedule::new()
        .on(date)
        .for_location(dhaka_city)
        .with_configuration(params)
        .calculate();

    match prayers
    {
        Ok(prayer) => {
            println!("{}: {}", Prayer::Fajr.name(), prayer.time(Prayer::Fajr).with_timezone(&Local).format("%-l:%M %p").to_string());
            println!("{}: {}", Prayer::Sunrise.name(), prayer.time(Prayer::Sunrise).with_timezone(&Local).format("%-l:%M %p").to_string());
            println!("{}: {}", Prayer::Dhuhr.name(), prayer.time(Prayer::Dhuhr).with_timezone(&Local).format("%-l:%M %p").to_string());
            println!("{}: {}", Prayer::Asr.name(), prayer.time(Prayer::Asr).with_timezone(&Local).format("%-l:%M %p").to_string());
            println!("{}: {}", Prayer::Maghrib.name(), prayer.time(Prayer::Maghrib).with_timezone(&Local).format("%-l:%M %p").to_string());
            println!("{}: {}", Prayer::Isha.name(), prayer.time(Prayer::Isha).with_timezone(&Local).format("%-l:%M %p").to_string());
            println!("{}: {}", Prayer::Qiyam.name(), prayer.time(Prayer::Qiyam).with_timezone(&Local).format("%-l:%M %p").to_string());
        },
        Err(error) => println!("Could not calculate prayer times: {}", error)
    }
}
