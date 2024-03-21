# satellite-rs
![build status](https://api.travis-ci.org/richinfante/satellite-rs.svg?branch=master)

A port of [satellite-js](https://github.com/shashwatak/satellite-js) to rust.

Currently unstable, and unsuitable for production use.


### Cargo.toml
Since this library is currently unstable, you may want to simply grab the latest version from `master`.
```bash
satellite = { git = "https://github.com/richinfante/satellite-rs.git", branch = "master" }
```
This crate can be used without the standard library (`#![no_std]`) by disabling
the default `std` feature. Use this in `Cargo.toml`:

```toml
[dependencies]
satellite = { git = "https://github.com/richinfante/satellite-rs.git", branch = "master" , default-features = false} 
```


### How to use
1. Get updated tles from a source such as [celestrak](https://celestrak.com/NORAD/elements/stations.txt).
2. Run predictions for a specific location / time (see below.)

### Example:
```rust
extern crate satellite;
extern crate chrono;
use chrono::prelude::*;

fn main() {
    // ISS TLE's
    let tle1 = "1 25544U 98067A   19070.20068744  .00000619  00000-0  17310-4 0  9990";
    let tle2 = "2 25544  51.6414 128.3903 0004102  93.2843   5.7821 15.52799004160030";

    let mut satrec = satellite::io::twoline2satrec(tle1, tle2).unwrap();

    let time = Utc::now();
    let result = satellite::propogation::propogate_datetime(&mut satrec, time).unwrap();

    println!("Position {:#?}", result.position);
    println!("Velocity {:#?}", result.velocity);
    
    // Set the Observer at 122.03 West by 36.96 North, in RADIANS
    let observer = satellite::Geodedic {
        longitude: -122.0308 * satellite::constants::DEG_2_RAD,
        latitude: 36.9613422 * satellite::constants::DEG_2_RAD,
        height: 0.370
    };

    let gmst = satellite::propogation::gstime::gstime_datetime(time);
    let sat_pos = satellite::transforms::eci_to_geodedic(&result.position, gmst);
    let position_ecf = satellite::transforms::eci_to_ecf(&result.position, gmst);
    let look_angles = satellite::transforms::ecf_to_look_angles(&observer, &position_ecf);

    println!("longitude = {}", sat_pos.latitude * satellite::constants::RAD_TO_DEG);
    println!("latitude = {}", sat_pos.longitude * satellite::constants::RAD_TO_DEG);
    println!("alt = {}", sat_pos.height * satellite::constants::KM_TO_MI);
    println!("aizmuth = {}", look_angles.azimuth * satellite::constants::RAD_TO_DEG);
    println!("elevation = {}", look_angles.elevation * satellite::constants::RAD_TO_DEG);
    println!("range = {}", look_angles.range * satellite::constants::KM_TO_MI);
}
```

### Known Issues:
- Deep-space calculations appear to be slightly off. Cause is currently unknown
- Many portions are currently untested.

### Todo List:
- Refactor code to allow for easier testing
- Add tests for remaining modules to ensure proper port
  - [x] constants.rs
  - [ ] ext.rs
  - [ ] doppler_factor.rs
  - [ ] transforms.rs _partial_
  - [x] io.rs
  - [x] propogation/dpper.rs
  - [x] propogation/dscom.rs
  - [x] propogation/dsinit.rs
  - [x] propogation/dspace.rs
  - [x] propogation/gstime.rs
  - [x] propogation/initl.rs
  - [ ] propogation/sgp4.rs
  - [ ] propogation/sgp4init.rs

MIT License. Derivative of [satellite-js](https://github.com/shashwatak/satellite-js) and [sgp4](https://pypi.org/project/sgp4/)
