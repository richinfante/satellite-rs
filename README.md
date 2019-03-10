# sattelite-rs

A port of [sattelite-js](https://github.com/shashwatak/satellite-js) to rust.

Currently unstable, and unsuitable for production use. Zero dependencies.


### Cargo.toml
Since this library is currently unstable, you may want to simply grab the latest version from `master`.
```bash
sattelite-rs = { git = "https://github.com/richinfante/satellite-rs.git", branch = "master" }
```

### Example:
```rust
  let tle1 = "1 88888U          80275.98708465  .00073094  13844-3  66816-4 0    8";
  let tle2 = "2 88888  72.8435 115.9689 0086731  52.6988 110.5714 16.05824518  105";

  let mut satrec = sattelite_rs::io::twoline2satrec(tle1, tle2);

  let result = sattelite_rs::propogation::sgp4::sgp4(&mut satrec, 0.0);

  println!("pos @{} {:#?}", 0.0, result.position);
  println!("vel @{} {:#?}", 0.0, result.velocity);

  // Set the Observer at 122.03 West by 36.96 North, in RADIANS
  let observer = sattelite_rs::Geodedic {
      longitude: -122.0308 * sattelite_rs::constants::DEG_2_RAD,
      latitude: 36.9613422 * sattelite_rs::constants::DEG_2_RAD,
      height: 0.370
  };

  let position_ecf = sattelite_rs::transforms::eci_to_ecf(&result.position, 0.0);
  let look_angles = sattelite_rs::transforms::ecf_to_look_angles(&observer, &position_ecf);

  println!("{:#?}", look_angles);
```

### Known Issues:
- Deep-space calculations appear to be slightly off. Cause is currently unknown
- Many portions are currently untested.

### Todo List:
- Refactor code to allow for easier testing
- Add tests for remaining modules to ensure proper port
  - [ ] constants.rs
  - [ ] ext.rs
  - [ ] doppler_factor.rs
  - [ ] transforms.rs
  - [x] io.rs
  - [x] propogation/dpper.rs
  - [x] propogation/dscom.rs
  - [ ] propogation/dsinit.rs
  - [x] propogation/dspace.rs
  - [ ] propogation/gstime.rs
  - [x] propogation/initl.rs
  - [ ] propogation/sgp4.rs
  - [ ] propogation/sgp4init.rs

MIT License. Derivative of [sattelite-js](https://github.com/shashwatak/satellite-js) and [sgp4](https://pypi.org/project/sgp4/)