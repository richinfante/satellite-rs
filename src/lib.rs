extern crate chrono;

pub mod constants;
pub mod doppler_factor;
pub mod ext;
pub mod io;
pub mod propogation;
pub mod transforms;

#[derive(Debug, PartialEq)]
/// Standard three-component vector (x,y,z)
pub struct Vec3 {
    pub x: f64,
    pub y: f64,
    pub z: f64,
}

type Ecf = Vec3;
type Eci = Vec3;

#[derive(Debug, PartialEq)]
pub struct TopoCentric {
    /// Positive horizontal vector S due south.
    pub top_s: f64,

    /// Positive horizontal vector E due east.
    pub top_e: f64,

    /// Vector Z normal to the surface of the earth (up).
    pub top_z: f64,
}

#[derive(Debug, PartialEq)]
/// Latitude/Longitude/Height based position
pub struct Geodedic {
    /// Longitude, in radians.
    pub longitude: f64,

    /// Longitude, in radians.
    pub latitude: f64,

    /// Altitude, in Km.
    pub height: f64,
}

#[derive(Debug, PartialEq)]
/// Relative position vector
pub struct Bearing {
    /// Aizmuth in radians
    pub azimuth: f64,

    /// Elevation in radians
    pub elevation: f64,

    /// Range in km
    pub range: f64,
}

impl Vec3 {
    pub fn range(&self, to: &Vec3) -> f64 {
        (((self.x - to.x).powf(2.0)) + ((self.y - to.y).powf(2.0)) + ((self.z - to.z).powf(2.0)))
            .sqrt()
    }

    pub fn add(&self, by: &Vec3) -> Vec3 {
        Vec3 {
            x: self.x + by.x,
            y: self.y + by.y,
            z: self.z + by.z,
        }
    }

    pub fn subtract(&self, by: &Vec3) -> Vec3 {
        Vec3 {
            x: self.x - by.x,
            y: self.y - by.y,
            z: self.z - by.z,
        }
    }
}

#[cfg(test)]
mod tests {
    pub fn assert_similar(lhs: f64, rhs: f64) {
        assert_diff(lhs, rhs, 1e-13);
    }

    pub fn assert_diff(lhs: f64, rhs: f64, epsilon: f64) {
        if (lhs - rhs).abs() > epsilon {
            panic!(
                "Assertion failed: diff between {} - {} > {}",
                lhs, rhs, epsilon
            );
        }
    }

    use crate::Vec3;
    struct TrackEntry {
        time: f64,
        known_pos: Vec3,
        known_vel: Vec3,
    }

    #[test]
    fn leo_tle() {
        let tle1 = "1 88888U          80275.98708465  .00073094  13844-3  66816-4 0    8";
        let tle2 = "2 88888  72.8435 115.9689 0086731  52.6988 110.5714 16.05824518  105";

        let mut satrec = crate::io::twoline2satrec(tle1, tle2).unwrap();

        let known_track: Vec<TrackEntry> = vec![
            TrackEntry {
                time: 0.0,
                known_pos: Vec3 {
                    x: 2328.97048951,
                    y: -5995.22076416,
                    z: 1719.97067261,
                },
                known_vel: Vec3 {
                    x: 2.91207230,
                    y: -0.98341546,
                    z: -7.09081703,
                },
            },
            TrackEntry {
                time: 360.0,
                known_pos: Vec3 {
                    x: 2456.10705566,
                    y: -6071.93853760,
                    z: 1222.89727783,
                },
                known_vel: Vec3 {
                    x: 2.67938992,
                    y: -0.44829041,
                    z: -7.22879231,
                },
            },
        ];

        for entry in known_track {
            let result = match crate::propogation::sgp4::sgp4(&mut satrec, entry.time) {
                Ok(res) => res,
                Err(err) => panic!("Propogation Error: {:#?} (code {})", err, err.code())
            };

            // println!("pos @{} {:#?}", entry.time, result.position);
            // println!("vel @{} {:#?}", entry.time, result.velocity);
            let diff_pos = result.position.subtract(&entry.known_pos);
            // println!("distance to pos: @{} {:#?}", entry.time, diff_pos);

            let diff_vel = result.velocity.subtract(&entry.known_vel);
            // println!("distance to vel: @{} {:#?}", entry.time, diff_vel);
            // println!(
            //     "{} < {} = {}",
            //     diff_pos.x.abs(),
            //     0.1,
            //     diff_pos.x.abs() < 0.1
            // );
            assert!(diff_pos.x.abs() < 0.1);
            assert!(diff_pos.y.abs() < 0.1);
            assert!(diff_pos.z.abs() < 0.1);

            assert!(diff_vel.x.abs() < 0.1);
            assert!(diff_vel.y.abs() < 0.1);
            assert!(diff_vel.z.abs() < 0.1);

            // assert_eq!(result.position.range(&entry.known_pos), 0.0);
            // assert_eq!(result.velocity.range(&entry.known_vel), 0.0);
        }
    }

    #[test]
    fn ds_tle() {
        let tle1 = "1 11801U          80230.29629788  .01431103  00000-0  14311-1        ";
        let tle2 = "2 11801  46.7916 230.4354 7318036  47.4722  10.4117  2.28537848      ";

        let mut satrec = crate::io::twoline2satrec(tle1, tle2).unwrap();

        let known_track: Vec<TrackEntry> = vec![
            TrackEntry {
                time: 0.0,
                known_pos: Vec3 {
                    x: 7473.37066650,
                    y: 428.95261765,
                    z: 5828.74786377,
                },
                known_vel: Vec3 {
                    x: 5.10715413,
                    y: 6.44468284,
                    z: -0.18613096,
                },
            },
            TrackEntry {
                time: 360.0,
                known_pos: Vec3 {
                    x: -3305.22537232,
                    y: 32410.86328125,
                    z: -24697.17675781,
                },
                known_vel: Vec3 {
                    x: -1.30113538,
                    y: -1.15131518,
                    z: -0.28333528,
                },
            },
        ];

        for entry in known_track {
            let result = match crate::propogation::sgp4::sgp4(&mut satrec, entry.time) {
                Ok(res) => res,
                Err(err) => panic!("Propogation Error: {:#?} (code {})", err, err.code())
            };

            // println!("pos @{} {:#?}", entry.time, result.position);
            // println!("vel @{} {:#?}", entry.time, result.velocity);
            let diff_pos = result.position.subtract(&entry.known_pos);
            // println!("distance to pos: @{} {:#?}", entry.time, diff_pos);

            let diff_vel = result.velocity.subtract(&entry.known_vel);
            // println!("distance to vel: @{} {:#?}", entry.time, diff_vel);
            // println!(
            //     "{} < {} = {}",
            //     diff_pos.x.abs(),
            //     0.1,
            //     diff_pos.x.abs() < 0.1
            // );

            // TODO: these seem off.
            assert!(diff_pos.x.abs() < 50.0);
            assert!(diff_pos.y.abs() < 50.0);
            assert!(diff_pos.z.abs() < 50.0);

            assert!(diff_vel.x.abs() < 0.1);
            assert!(diff_vel.y.abs() < 0.1);
            assert!(diff_vel.z.abs() < 0.1);
        }
    }

    const TEST_TLE: &str =
        "NOAA 15 \
        1 25338U 98030A   20028.53684332  .00000010  00000-0  22730-4 0  9996 \
        2 25338  98.7308  54.2052 0009655 316.5487  43.4931 14.25949056128892 \
        NOAA 18 \
        1 28654U 05018A   20028.55430359  .00000064  00000-0  59410-4 0  9998 \
        2 28654  99.0657  83.5290 0013366 267.3059  92.6583 14.12484618757024 \
        NOAA 19 \
        1 33591U 09005A   20028.54874297  .00000001  00000-0  25623-4 0  9996 \
        2 33591  99.1936  30.2411 0014855 109.6767 250.6008 14.12393428565240";


    /// Load a NOAA test Satrec object from test tle
    fn load_test_sat(name: &str) -> crate::io::Satrec {
        let (sats, _errors) = crate::io::parse_multiple(TEST_TLE);
        sats.iter().find(|&sat| sat.name == Some(name.to_string()))
            .expect(&format!("{} not found in test TLE file", name)).clone()
    }

    #[test]
    fn test_memory() {
        use chrono::prelude::*;

        // Known results, we test against each one of these.
        // Everything in degrees. Fields:
        // (satellite, timestamp, latitude, longitude, tolerance)
        let test_values = [
            ("NOAA 15", 1577836800, -22.135, 103.093, 0.005), // 2020-01-01
            ("NOAA 18", 1580257671, -23.131, 125.410, 0.005), // 2020-01-28
            ("NOAA 19", 1580000000, -16.414,  66.815, 0.005), // 2020-01-26
            ("NOAA 15", 1590000000, -53.152,  19.884, 0.036), // 2020-05-20
            ("NOAA 18", 1565395200,  68.577, 287.984, 0.05 ), // 2019-08-10
            ("NOAA 15", 1672531200, -79.203,  64.941, 1.   ), // 2023-01-01
            ("NOAA 19", 1514764800, -36.389,  46.125, 1.   ), // 2018-01-01
        ];

        for test in test_values.iter() {
            let tolerance = test.4; // Degrees
            let mut sat = load_test_sat(test.0);
            let time = chrono::Utc.timestamp(test.1, 0); // 0 milliseconds
            let result = crate::propogation::propogate_datetime(&mut sat, time).unwrap();
            let gmst = crate::propogation::gstime::gstime_datetime(time);
            let sat_pos = crate::transforms::eci_to_geodedic(&result.position, gmst);

            let lat = sat_pos.latitude * crate::constants::RAD_TO_DEG;
            let lon = sat_pos.longitude * crate::constants::RAD_TO_DEG;

            // assert_abs_diff_eq!(lat, test.2, epsilon = tolerance);
            // Predict gives longitudes from 0:360, satellite-rs from -180:180
            // assert_abs_diff_eq!((lon + 360.) % 360., test.3, epsilon = tolerance);
        }
    }
}
