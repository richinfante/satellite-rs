use crate::*;

/// 2 * π
pub static TWO_PI: Float = PI * 2.0;

/// Conversion factor for Degrees -> Radians
pub static DEG_2_RAD: Float = PI / 180.0;

/// Conversion factorfor Radians -> Degrees
pub static RAD_TO_DEG: Float = 180.0 / PI;

/// Minutes per day constant.
pub static MINUTES_PER_DAY: Float = 1440.0;

pub static MU: Float = 398600.5; // in km3 / s2

/// Earth's Radiys (km)
pub static EARTH_RADIUS: Float = 6378.137;

// Two-thirds precomputed constant.
pub static X2O3: Float = 2.0 / 3.0;

/// Speed of light in km/s
pub static C: Float = 299792.458;

/// Km to mi conversion
pub static KM_TO_MI: Float = 0.62137;

// Mi to Km conversion
pub static MI_TO_KM: Float = 1.0 / 0.62137;

pub static J2: Float = 0.00108262998905;
pub static J3: Float = -0.00000253215306;
pub static J4: Float = -0.00000161098761;
pub static J3OJ2: Float = J3 / J2;
pub static XKE: Float = 0.07436685316871385;
pub static TUMIN: Float = 13.446851082044981;
pub static XPDOTP: Float = 1440.0 / (2.0 * PI);

#[cfg(test)]
mod test {
    use crate::constants::*;

    pub fn xke() -> Float {
        60.0 / ((EARTH_RADIUS * EARTH_RADIUS * EARTH_RADIUS) / MU).sqrt()
    }

    pub fn tumin() -> Float {
        1.0 / xke()
    }

    #[test]
    fn test() {
        assert_eq!(PI, 3.141592653589793);
        assert_eq!(TWO_PI, 6.283185307179586);
        assert_eq!(DEG_2_RAD, 0.017453292519943295);
        assert_eq!(RAD_TO_DEG, 57.29577951308232);
        assert_eq!(MINUTES_PER_DAY, 1440.0);
        assert_eq!(MU, 398600.5);
        assert_eq!(EARTH_RADIUS, 6378.137);
        assert_eq!(XKE, xke());
        assert_eq!(TUMIN, tumin());
        assert_eq!(xke(), 0.07436685316871385);
        assert_eq!(tumin(), 13.446851082044981);
        assert_eq!(J2, 0.00108262998905);
        assert_eq!(J3, -0.00000253215306);
        assert_eq!(J4, -0.00000161098761);
        assert_eq!(J3OJ2, -0.0023388905587420003);
        assert_eq!(X2O3, 0.6666666666666666);
    }
}
