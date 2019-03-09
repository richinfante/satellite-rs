pub use std::f64::consts::PI;
pub static TWO_PI: f64 = PI * 2.0;
pub static DEG_2_RAD: f64 = PI / 180.0;
pub static RAD_TO_DEG: f64 = 180.0 / PI;
pub static MINUTES_PER_DAY: f64 = 1440.0;
pub static MU: f64 = 398600.5; // in km3 / s2
pub static EARTH_RADIUS: f64 = 6378.137; // in km
pub static J2: f64 = 0.00108262998905;
pub static J3: f64 = -0.00000253215306;
pub static J4: f64 = -0.00000161098761;
pub static J3OJ2: f64 = J3 / J2;
pub static X2O3: f64 = 2.0 / 3.0;
pub static C: f64 = 299792.458; // Speed of light in km/s

// TODO: precalculate these.
pub fn xke() -> f64 {
    60.0 / ((EARTH_RADIUS * EARTH_RADIUS * EARTH_RADIUS) / MU).sqrt()
}

pub fn tumin() -> f64 {
    1.0 / xke()
}
