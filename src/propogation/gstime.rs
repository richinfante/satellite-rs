use crate::ext;
use crate::constants::*;
use chrono::prelude::*;

/// Convert a julian date to GMST (Greenwich Mean Sidereal Time)
pub fn gstime(jdut1: f64) -> f64 {
    let tut1 = (jdut1 - 2451545.0) / 36525.0;

    let mut temp = (-6.2e-6 * tut1 * tut1 * tut1)
        + (0.093104 * tut1 * tut1)
        + (((876600.0 * 3600.0) + 8640184.812866) * tut1)
        + 67310.54841; // # sec
    temp = ((temp * DEG_2_RAD) / 240.0) % TWO_PI; // 360/86400 = 1/240, to deg, to rad

    //  ------------------------ check quadrants ---------------------
    if temp < 0.0 {
        temp += TWO_PI;
    }

    return temp;
}

/// Convert a datetime to GMST (Greenwich Mean Sidereal Time)
pub fn gstime_datetime(datetime: DateTime<Utc>) -> f64 {
    let jday = ext::jday_datetime(datetime);
    return gstime(jday)
}

#[cfg(test)]
mod tests {
    use crate::propogation::gstime::*;
    #[test]
    fn test_gst() {
        let res = gstime(2444468.79629788);
        assert_eq!(res, 1.265125075734467);
    }
}
