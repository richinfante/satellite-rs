#![cfg_attr(not(feature = "std"), no_std)]
use cfg_if::cfg_if;
cfg_if! {
    if #[cfg(feature = "std")]
    {
        extern crate std;
        use std::f64::consts::PI;
        use std::f64::consts::FRAC_PI_2;
        pub type Float = f64;
    }else{
        extern crate core as std;
        extern crate alloc;
        use alloc::{string::{String,ToString},format,vec,vec::Vec,boxed::Box};
        use micromath::F32Ext;
        use std::f32::consts::PI;
        use std::f32::consts::FRAC_PI_2;
        pub type Float = f32;
    }
}
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
    pub x: Float,
    pub y: Float,
    pub z: Float,
}

type Ecf = Vec3;
type Eci = Vec3;

#[derive(Debug, PartialEq)]
pub struct TopoCentric {
    /// Positive horizontal vector S due south.
    pub top_s: Float,

    /// Positive horizontal vector E due east.
    pub top_e: Float,

    /// Vector Z normal to the surface of the earth (up).
    pub top_z: Float,
}

#[derive(Debug, PartialEq)]
/// Latitude/Longitude/Height based position
pub struct Geodedic {
    /// Longitude, in radians.
    pub longitude: Float,

    /// Longitude, in radians.
    pub latitude: Float,

    /// Altitude, in Km.
    pub height: Float,
}

#[derive(Debug, PartialEq)]
/// Relative position vector
pub struct Bearing {
    /// Aizmuth in radians
    pub azimuth: Float,

    /// Elevation in radians
    pub elevation: Float,

    /// Range in km
    pub range: Float,
}

impl Vec3 {
    pub fn range(&self, to: &Vec3) -> Float {
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
    pub fn assert_similar(lhs: Float, rhs: Float) {
        assert_diff(lhs, rhs, 1e-13);
    }

    pub fn assert_diff(lhs: Float, rhs: Float, epsilon: Float) {
        if (lhs - rhs).abs() > epsilon {
            panic!(
                "Assertion failed: diff between {} - {} > {}",
                lhs, rhs, epsilon
            );
        }
    }

    use crate::Vec3;
    struct TrackEntry {
        time: Float,
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
                Err(err) => panic!("Propogation Error: {:#?} (code {})", err, err.code()),
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
                Err(err) => panic!("Propogation Error: {:#?} (code {})", err, err.code()),
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

    #[test]
    fn test_parse_invalid_tle() {
        use crate::io::SatrecParseError;
        use chrono::prelude::*;

        let (sats, errors) = crate::io::parse_multiple("Test");
        assert_eq!(
            errors,
            vec![SatrecParseError::SatrecMultiError(
                0,
                Box::new(SatrecParseError::InvalidTLEBadLineCount)
            )]
        );
    }
}
