use crate::constants::*;
use crate::Bearing;
use crate::Geodedic;
use crate::TopoCentric;
use crate::Vec3;

pub fn radians_to_degrees(radians: f64) -> f64 {
    return radians * RAD_TO_DEG;
}

pub fn degrees_to_radians(degrees: f64) -> f64 {
    return degrees * DEG_2_RAD;
}

pub fn degrees_lat(radians: f64) -> f64 {
    if radians < -std::f64::consts::FRAC_PI_2 || radians > std::f64::consts::FRAC_PI_2 {
        panic!("Range error: Latitude radians must be in range [-pi/2; pi/2].")
    }

    radians_to_degrees(radians)
}

pub fn degrees_long(radians: f64) -> f64 {
    if radians < -std::f64::consts::PI || radians > std::f64::consts::PI {
        panic!("Range error: Longitude radians must be in range [-pi; pi].")
    }

    radians_to_degrees(radians)
}

pub fn radians_lat(degrees: f64) -> f64 {
    if degrees < -90.0 || degrees > 90.0 {
        panic!("RangeError: Latitude degrees must be in range [-90; 90].")
    }

    degrees_to_radians(degrees)
}

pub fn radians_long(degrees: f64) -> f64 {
    if degrees < -180.0 || degrees > 180.0 {
        panic!("RangeError: Latitude degrees must be in range [-180; 180].")
    }

    degrees_to_radians(degrees)
}

pub fn geodedic_to_ecf(geodetic: &Geodedic) -> Vec3 {
    const a: f64 = 6378.137;
    const b: f64 = 6356.7523142;
    const f: f64 = (a - b) / a;
    const e2: f64 = ((2.0 * f) - (f * f));

    let normal = a / (1.0 - (e2 * geodetic.latitude.sin().powf(2.0))).sqrt();

    let x = (normal + geodetic.height) * geodetic.latitude.cos() * geodetic.longitude.cos();
    let y = (normal + geodetic.height) * geodetic.latitude.cos() * geodetic.longitude.sin();
    let z = ((normal * (1.0 - e2)) + geodetic.height) * geodetic.latitude.sin();

    Vec3 { x, y, z }
}

pub fn eci_to_geodedic(eci: &Vec3, gmst: f64) -> Geodedic {
    const a: f64 = 6378.137;
    const b: f64 = 6356.7523142;
    let R: f64 = ((eci.x * eci.x) + (eci.y * eci.y)).sqrt();
    const f: f64 = (a - b) / a;
    const e2: f64 = ((2.0 * f) - (f * f));

    let mut longitude = eci.y.atan2(eci.x) - gmst;

    while longitude < -PI {
        longitude += TWO_PI;
    }

    while longitude > PI {
        longitude -= TWO_PI;
    }

    const kmax: i32 = 20;

    let mut latitude = eci.z.atan2((eci.x.powi(2) + eci.y.powi(2)).sqrt());

    let mut k: i32 = 0;
    let mut c: f64 = 0.0;

    while k < kmax {
        c = 1.0 / (1.0 - (e2 * latitude.sin().powi(2)));
        latitude = (eci.z + (a * c * e2 * latitude.sin())).atan2(R);
        k += 1;
    }

    let height = (R / latitude.cos()) - (a * c);

    Geodedic {
        latitude,
        longitude,
        height,
    }
}

pub fn ecf_to_eci(ecf: &Vec3, gmst: f64) -> Vec3 {
    let X = (ecf.x * gmst.cos()) - (ecf.y * gmst.sin());
    let Y = (ecf.x * gmst.sin()) + (ecf.y * gmst.cos());
    let Z = ecf.z;

    Vec3 { x: X, y: Y, z: Z }
}

pub fn eci_to_ecf(eci: &Vec3, gmst: f64) -> Vec3 {
    let x = (eci.x * gmst.cos()) - (eci.y * gmst.sin());
    let y = (eci.x * -gmst.sin()) + (eci.y * gmst.cos());
    let z = eci.z;

    Vec3 { x, y, z }
}

pub fn topocentric(observer: &Geodedic, satellite_ecf: &Vec3) -> TopoCentric {
    let observer_ecf = geodedic_to_ecf(&observer);

    let r = satellite_ecf.subtract(&observer_ecf);

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
    let az = (-tc.top_e).atan2(tc.top_s) + std::f64::consts::PI;

    Bearing {
        range: range_sat,
        elevation: el,
        azimuth: az,
    }
}

pub fn ecf_to_look_angles(observer: &Geodedic, satellite_ecf: &Vec3) -> Bearing {
    let topocentric = topocentric(observer, satellite_ecf);

    topocentric_to_look_angles(&topocentric)
}


#[cfg(test)]
mod test {
  use crate::*;
  use crate::tests::*;
  #[test]
  fn eci_to_ecf() {
    let position = Vec3 {
        x: 2328.957357263014,
        y: -5995.219305262678,
        z: 1720.0073114076358
    };

    let position_ecf = transforms::eci_to_ecf(&position, 0.0);

    assert_eq!(position_ecf,  Vec3 {
        x: 2328.957357263014,
        y: -5995.219305262678,
        z: 1720.0073114076358
    });
  }

  #[test]
  fn ecf_to_look_angles() {
    let observer_gd = Geodedic {
        longitude: -122.0308 * constants::DEG_2_RAD,
        latitude: 36.9613422 * constants::DEG_2_RAD,
        height: 0.370
    };

    let position_ecf = Vec3 {
        x: 2328.957357263014,
        y: -5995.219305262678,
        z: 1720.0073114076358
    };
    
    let res = transforms::ecf_to_look_angles(&observer_gd, &position_ecf);

    // assert_eq!(res, Bearing {
    //   azimuth: 1.747132515004105,
    //   elevation: -0.40791001471599636,
    //   range: 5703.24291019934
    // });

    assert_diff(res.azimuth, 1.747132515004105, 1e-12);
    assert_diff(res.elevation, -0.40791001471599636, 1e-12);
    assert_diff(res.range, 5703.24291019934, 1e-11);
  }

  #[test]
  fn topocentric() {
        let observer_gd = Geodedic {
        longitude: -122.0308 * constants::DEG_2_RAD,
        latitude: 36.9613422 * constants::DEG_2_RAD,
        height: 0.370
    };

    let position_ecf = Vec3 {
        x: 2328.957357263014,
        y: -5995.219305262678,
        z: 1720.0073114076358
    };
    
    let ts = transforms::topocentric(&observer_gd, &position_ecf);
    assert_diff(ts.top_s, 918.3964944158424, 1e-12);
    assert_diff(ts.top_e, 5154.118963234977, 1e-12);
    assert_diff(ts.top_z, -2262.429067309148, 1e-12);
  }
}