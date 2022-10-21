use serde_derive::Serialize;

#[derive(Serialize)]
pub struct IndexMod {
    pub status: i32,
    pub message: String,
    pub data: IndexData,
}

#[derive(Serialize)]
pub struct IndexData {
    pub latitude: f64,
    pub longitude: f64,
    pub timezone: String,
    pub method: String,
    pub madhab: String,
}

#[derive(Serialize)]
pub struct TodayMod {
    pub status: i32,
    pub message: String,
    pub data: TodayData,
}

#[derive(Serialize)]
pub struct TodayData {
    pub fajr: String,
    pub sunrise: String,
    pub dhuhr: String,
    pub asr: String,
    pub maghrib: String,
    pub isha: String,
    pub qiyam: String,
}

#[derive(Serialize)]
pub struct CurrentMod {
    pub status: i32,
    pub message: String,
    pub data: CurrentData,
}

#[derive(Serialize)]
pub struct CurrentData {
    pub current: CurrentCurrentData,
    pub next: String,
}

#[derive(Serialize)]
pub struct CurrentCurrentData {
    pub name: String,
    pub time_remaining: String,
}

#[derive(Serialize)]
pub struct QiblahMod {
    pub status: i32,
    pub message: String,
    pub data: QiblahData,
}

#[derive(Serialize)]
pub struct QiblahData {
    pub degrees: f64,
}
