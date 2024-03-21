use crate::constants::*;
use crate::*;

pub fn radians_to_degrees(radians: Float) -> Float {
    return radians * RAD_TO_DEG;
}

pub fn degrees_to_radians(degrees: Float) -> Float {
    return degrees * DEG_2_RAD;
}

pub fn degrees_lat(radians: Float) -> Float {
    if radians < -FRAC_PI_2 || radians > FRAC_PI_2 {
        panic!("Range error: Latitude radians must be in range [-pi/2; pi/2].")
    }

    radians_to_degrees(radians)
}

pub fn degrees_long(radians: Float) -> Float {
    if radians < -PI || radians > PI {
        panic!("Range error: Longitude radians must be in range [-pi; pi].")
    }

    radians_to_degrees(radians)
}

pub fn radians_lat(degrees: Float) -> Float {
    if degrees < -90.0 || degrees > 90.0 {
        panic!("RangeError: Latitude degrees must be in range [-90; 90].")
    }

    degrees_to_radians(degrees)
}

pub fn radians_long(degrees: Float) -> Float {
    if degrees < -180.0 || degrees > 180.0 {
        panic!("RangeError: Latitude degrees must be in range [-180; 180].")
    }

    degrees_to_radians(degrees)
}

pub fn geodedic_to_ecf(geodetic: &Geodedic) -> Ecf {
    const A: Float = 6378.137;
    const B: Float = 6356.7523142;
    const F: Float = (A - B) / A;
    const E2: Float = (2.0 * F) - (F * F);

    let normal = A / (1.0 - (E2 * geodetic.latitude.sin().powf(2.0))).sqrt();

    let x = (normal + geodetic.height) * geodetic.latitude.cos() * geodetic.longitude.cos();
    let y = (normal + geodetic.height) * geodetic.latitude.cos() * geodetic.longitude.sin();
    let z = ((normal * (1.0 - E2)) + geodetic.height) * geodetic.latitude.sin();

    Vec3 { x, y, z }
}

pub fn eci_to_geodedic(eci: &Eci, gmst: Float) -> Geodedic {
    const A: Float = 6378.137;
    const B: Float = 6356.7523142;
    let r: Float = ((eci.x * eci.x) + (eci.y * eci.y)).sqrt();
    const F: Float = (A - B) / A;
    const E2: Float = (2.0 * F) - (F * F);

    let mut longitude = eci.y.atan2(eci.x) - gmst;

    while longitude < -PI {
        longitude += TWO_PI;
    }

    while longitude > PI {
        longitude -= TWO_PI;
    }

    const KMAX: i32 = 20;

    let mut latitude = eci.z.atan2((eci.x.powi(2) + eci.y.powi(2)).sqrt());

    let mut k: i32 = 0;
    let mut c: Float = 0.0;

    while k < KMAX {
        c = 1.0 / (1.0 - (E2 * latitude.sin().powi(2)));
        latitude = (eci.z + (A * c * E2 * latitude.sin())).atan2(r);
        k += 1;
    }

    let height = (r / latitude.cos()) - (A * c);

    Geodedic {
        latitude,
        longitude,
        height,
    }
}

pub fn ecf_to_eci(ecf: &Ecf, gmst: Float) -> Eci {
    let x = (ecf.x * gmst.cos()) - (ecf.y * gmst.sin());
    let y = (ecf.x * gmst.sin()) + (ecf.y * gmst.cos());
    let z = ecf.z;

    Vec3 { x, y, z }
}

pub fn eci_to_ecf(eci: &Eci, gmst: Float) -> Ecf {
    let x = (eci.x * gmst.cos()) + (eci.y * gmst.sin());
    let y = (eci.x * -gmst.sin()) + (eci.y * gmst.cos());
    let z = eci.z;

    Vec3 { x, y, z }
}

pub fn topocentric(observer: &Geodedic, satellite: &Ecf) -> TopoCentric {
    let observer_ecf = geodedic_to_ecf(&observer);

    let r = satellite.subtract(&observer_ecf);

    let top_s = (observer.latitude.sin() * observer.longitude.cos() * r.x)
        + (observer.latitude.sin() * observer.longitude.sin() * r.y)
        - (observer.latitude.cos() * r.z);

    let top_e = (-observer.longitude.sin() * r.x) + (observer.longitude.cos() * r.y);

    let top_z = (observer.latitude.cos() * observer.longitude.cos() * r.x)
        + (observer.latitude.cos() * observer.longitude.sin() * r.y)
        + (observer.latitude.sin() * r.z);

    TopoCentric {
        top_s,
        top_e,
        top_z,
    }
}

pub fn topocentric_to_look_angles(tc: &TopoCentric) -> Bearing {
    let range_sat = (tc.top_s.powi(2) + tc.top_e.powi(2) + tc.top_z.powi(2)).sqrt();
    let el = (tc.top_z / range_sat).asin();
    let az = (-tc.top_e).atan2(tc.top_s) + PI;

    Bearing {
        range: range_sat,
        elevation: el,
        azimuth: az,
    }
}

pub fn ecf_to_look_angles(observer: &Geodedic, satellite: &Ecf) -> Bearing {
    let topocentric = topocentric(observer, satellite);

    topocentric_to_look_angles(&topocentric)
}

#[cfg(test)]
mod test {
    use crate::tests::*;
    use crate::*;
    #[test]
    fn eci_to_ecf() {
        let position = Eci {
            x: 2328.957357263014,
            y: -5995.219305262678,
            z: 1720.0073114076358,
        };

        let position_ecf = transforms::eci_to_ecf(&position, 0.0);

        assert_eq!(
            position_ecf,
            Ecf {
                x: 2328.957357263014,
                y: -5995.219305262678,
                z: 1720.0073114076358
            }
        );
    }

    #[test]
    fn ecf_to_eci() {
        let position = Eci {
            x: 2328.957357263014,
            y: -5995.219305262678,
            z: 1720.0073114076358,
        };

        let position_ecf = transforms::eci_to_ecf(&position, 0.0);
        let orig_eci = transforms::ecf_to_eci(&position_ecf, 0.0);
        assert_eq!(position, orig_eci);
    }

    #[test]
    fn ecf_to_look_angles() {
        let observer_gd = Geodedic {
            longitude: -122.0308 * constants::DEG_2_RAD,
            latitude: 36.9613422 * constants::DEG_2_RAD,
            height: 0.370,
        };

        let position_ecf = Ecf {
            x: 2328.957357263014,
            y: -5995.219305262678,
            z: 1720.0073114076358,
        };

        let res = transforms::ecf_to_look_angles(&observer_gd, &position_ecf);

        assert_similar(res.azimuth, 1.747132515004105);
        assert_similar(res.elevation, -0.40791001471599636);
        assert_similar(res.range, 5703.24291019934);
    }

    #[test]
    fn topocentric() {
        let observer_gd = Geodedic {
            longitude: -122.0308 * constants::DEG_2_RAD,
            latitude: 36.9613422 * constants::DEG_2_RAD,
            height: 0.370,
        };

        let position_ecf = Ecf {
            x: 2328.957357263014,
            y: -5995.219305262678,
            z: 1720.0073114076358,
        };

        let ts = transforms::topocentric(&observer_gd, &position_ecf);
        assert_similar(ts.top_s, 918.3964944158424);
        assert_similar(ts.top_e, 5154.118963234977);
        assert_similar(ts.top_z, -2262.429067309148);
    }
}
