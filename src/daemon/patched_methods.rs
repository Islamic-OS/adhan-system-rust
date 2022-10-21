///
/// ~ DISCLAIMER ~
/// Some of the code is not ours!
/// This file mostly contains patched methods of the
/// crate "salah 0.7.0" (https://github.com/insha/salah), to mainly fix bugs
/// or fix something to fit our needs.
/// All credit goes to Farhan Ahmed (https://github.com/insha/)
///


use salah::Coordinates;
use crate::unit_salah::Angle;

pub struct QiblahPatched(pub f64);

impl QiblahPatched {
    pub fn new(location_coordinates: Coordinates) -> Self {
        // Equation from "Spherical Trigonometry For the use
        // of colleges and schools" page 50
        let makkah_coordinates = Coordinates::new(21.4225241, 39.8261818);
        let term1 = (makkah_coordinates.longitude_angle().radians()
            - location_coordinates.longitude_angle().radians())
        .sin();
        let term2 = makkah_coordinates.latitude_angle().radians().tan()
            * location_coordinates.latitude_angle().radians().cos();
        let term3 = (makkah_coordinates.longitude_angle().radians()
            - location_coordinates.longitude_angle().radians())
        .cos()
            * location_coordinates.latitude_angle().radians().sin();
        let term4 = term1.atan2(term2 - term3);

        QiblahPatched(Angle::from_radians(term4).unwound().degrees)
    }
}

