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


#[cfg(test)]
mod test {
  use crate::constants::*;
  #[test]
  fn test() {
    assert_eq!(PI, 3.141592653589793);
    assert_eq!(TWO_PI, 6.283185307179586);
    assert_eq!(DEG_2_RAD, 0.017453292519943295);
    assert_eq!(RAD_TO_DEG, 57.29577951308232);
    assert_eq!(MINUTES_PER_DAY, 1440.0);
    assert_eq!(MU, 398600.5);
    assert_eq!(EARTH_RADIUS, 6378.137);
    assert_eq!(xke(), 0.07436685316871385);
    assert_eq!(tumin(), 13.446851082044981);
    assert_eq!(J2, 0.00108262998905);
    assert_eq!(J3, -0.00000253215306);
    assert_eq!(J4, -0.00000161098761);
    assert_eq!(J3OJ2, -0.0023388905587420003);
    assert_eq!(X2O3, 0.6666666666666666);
  }
}
