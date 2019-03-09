# sattelite-rs

A port of [sattelite-js](https://github.com/shashwatak/satellite-js) to rust.

Currently unstable, and unsuitable for production use. Zero dependencies.

Known Issues:
- Deep-space calculations appear to be slightly off. Cause is currently unknown
- Many portions are currently untested.

Todo List:
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