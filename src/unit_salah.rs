///
/// ~ DISCLAIMER ~
/// Some of the code is not ours!
/// This file mostly contains patched methods of the
/// crate "salah 0.7.0" (https://github.com/insha/salah), to mainly fix bugs
/// or fix something to fit our needs.
/// All credit goes to Farhan Ahmed (https://github.com/insha/)
///


// Salah
//
// See LICENSE for more details.
// Copyright (c) 2019-2022 Farhan Ahmed. All rights reserved.
//

use std::f64::consts::PI;
use std::ops::{Add, Div, Mul, Sub};

// use crate::models::rounding::Rounding;
use chrono::{DateTime, Datelike, Duration, TimeZone};

pub trait Normalize {
    fn normalized_to_scale(&self, max: f64) -> f64;
}

impl Normalize for f64 {
    fn normalized_to_scale(&self, max: f64) -> f64 {
        self - (max * (self / max).floor())
    }
}

/// Convenience methods for the DateTime type.
pub trait Stride {
    fn tomorrow(&self) -> Self;
    fn yesterday(&self) -> Self;
    // fn julian_day(&self) -> f64;
    fn adjust_time(&self, minutes: i64) -> Self;
    fn next_date(&self, fwd: bool) -> Self;
	// fn rounded_minute(&self, rounding: Rounding) -> Self;
}

impl<Tz: TimeZone> Stride for DateTime<Tz> {
    /// Returns the date/time for tomorrow.
    fn tomorrow(&self) -> Self {
        self.next_date(true)
    }

    /// Returns the date/time for yesterday.
    fn yesterday(&self) -> Self {
        self.next_date(false)
    }

    /// Returns the Julian day.
    // fn julian_day(&self) -> f64 {
    //     ops::julian_day(
    //         self.year() as i32,
    //         self.month() as i32,
    //         self.day() as i32,
    //         0.0,
    //     )
    // }

	// fn rounded_minute(&self, rounding: Rounding) -> Self {
	// 	let adjusted = self.clone();
	// 	let seconds = adjusted.second();
	// 	
	// 	match rounding {
	// 		Rounding::Nearest => {
	// 			let rounded = ((seconds as f64)/60.0).round() as i64;
	// 			let adjusted_seconds = seconds as i64;
	// 			
	// 			if rounded == 1 {
	// 				adjusted + Duration::seconds(60 - adjusted_seconds)
	// 			} else {
	// 				adjusted + Duration::seconds(adjusted_seconds * -1)
	// 			}
	// 		},
	// 		Rounding::Up => {
	// 			let adjusted_seconds = seconds as i64;
	// 			
	// 			adjusted + Duration::seconds(60 - adjusted_seconds)
	// 		},
	// 		Rounding::None => adjusted,
	// 	}
	// }

    fn adjust_time(&self, minutes: i64) -> Self {
        let some_date = self.clone();
        some_date
            .checked_add_signed(Duration::seconds(minutes * 60))
            .unwrap()
    }

    fn next_date(&self, fwd: bool) -> Self {
        let ordinal = if fwd {
            self.ordinal() + 1
        } else {
            self.ordinal() - 1
        };

        match self.with_ordinal(ordinal) {
            Some(dt) => dt,
            None => {
                if fwd {
                    self.with_year(self.year() + 1)
                        .unwrap()
                        .with_ordinal(1)
                        .unwrap()
                } else {
                    self.with_year(self.year() - 1)
                        .unwrap()
                        .with_month(12)
                        .unwrap()
                        .with_day(31)
                        .unwrap()
                }
            }
        }
    }
}

#[derive(PartialEq, Debug, Copy, Clone)]
pub struct Angle {
    pub degrees: f64,
}

impl Angle {
    // pub fn new(value: f64) -> Self {
    //     Angle { degrees: value }
    // }

    pub fn from_radians(value: f64) -> Self {
        Angle {
            degrees: (value * 180.0) / PI,
        }
    }

    // pub fn radians(&self) -> f64 {
    //     (self.degrees * PI) / 180.0
    // }

    pub fn unwound(&self) -> Angle {
        Angle {
            degrees: self.degrees.normalized_to_scale(360.0),
        }
    }

    // pub fn quadrant_shifted(&self) -> Angle {
    //     let angle: Angle;

    //     if self.degrees >= -180.0 && self.degrees <= 180.0 {
    //         // Nothing to do. Already initialized
    //         // to the default value.
    //         angle = self.clone();
    //     } else {
    //         let value = self.degrees - (360.0 * (self.degrees / 360.0).round());
    //         angle = Angle { degrees: value };
    //     }

    //     angle
    // }
}

impl Add for Angle {
    type Output = Angle;

    fn add(self, rhs: Angle) -> Angle {
        Angle {
            degrees: self.degrees + rhs.degrees,
        }
    }
}

impl Sub for Angle {
    type Output = Angle;

    fn sub(self, rhs: Angle) -> Angle {
        Angle {
            degrees: self.degrees - rhs.degrees,
        }
    }
}

impl Mul for Angle {
    type Output = Angle;

    fn mul(self, rhs: Angle) -> Angle {
        Angle {
            degrees: self.degrees * rhs.degrees,
        }
    }
}

impl Div for Angle {
    type Output = Angle;

    fn div(self, rhs: Angle) -> Angle {
        if rhs.degrees == 0.0 {
            panic!("Cannot divide by zero.");
        }

        Angle {
            degrees: self.degrees / rhs.degrees,
        }
    }
}

