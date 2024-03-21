use crate::constants::*;

use crate::*;
fn sign(val: Float) -> Float {
    if val >= 0.0 {
        1.0
    } else {
        -1.0
    }
}

pub fn doppler_factor(location: Vec3, position: Vec3, velocity: Vec3) -> Float {
    let current_range = position.range(&location);

    let next_pos = position.add(&velocity);

    let next_range = next_pos.range(&location);

    let mut range_rate = next_range - current_range;

    // TODO: is this simply abs()
    range_rate *= sign(range_rate);

    return 1.0 + (range_rate / C);
}
