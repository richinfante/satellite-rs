# sattelite-rs

A port of [sattelite-js](https://github.com/shashwatak/satellite-js) to rust.

Currently unstable, and unsuitable for production use. Zero dependencies.

### Example:
```rust
let tle1 = "1 11801U          80230.29629788  .01431103  00000-0  14311-1        ";
let tle2 = "2 11801  46.7916 230.4354 7318036  47.4722  10.4117  2.28537848      ";

let mut satrec = sattelite_rs::io::twoline2satrec(tle1, tle2);

let result = sattelite_rs::propogation::sgp4::sgp4(&mut satrec, 0.0);

println!("pos @{} {:#?}", 0.0, result.position);
println!("vel @{} {:#?}", 0.0, result.velocity);
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
  - [ ] propogation/dpper.rs
  - [ ] propogation/dscom.rs
  - [ ] propogation/dsinit.rs
  - [ ] propogation/dspace.rs
  - [ ] propogation/gstime.rs
  - [x] propogation/initl.rs (needs deep-space test vectors)
  - [ ] propogation/sgp4.rs
  - [ ] propogation/sgp4init.rs

MIT License. Derivative of [sattelite-js](https://github.com/shashwatak/satellite-js) and [sgp4](https://pypi.org/project/sgp4/)