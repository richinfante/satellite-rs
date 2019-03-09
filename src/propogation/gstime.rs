use crate::constants::*;

pub fn gstime(jdut1: f64) -> f64{
  let tut1 = (jdut1 - 2451545.0) / 36525.0;

  let mut temp = (-6.2e-6 * tut1 * tut1 * tut1)
    + (0.093104 * tut1 * tut1)
    + (((876600.0 * 3600.0) + 8640184.812866) * tut1) + 67310.54841; // # sec
  temp = ((temp * DEG_2_RAD) / 240.0) % TWO_PI; // 360/86400 = 1/240, to deg, to rad

  //  ------------------------ check quadrants ---------------------
  if temp < 0.0 {
    temp += TWO_PI;
  }

  return temp;
}


// TODO: does this mean anything?
// export default function gstime(...args) {
//   if (args[0] instanceof Date || args.length > 1) {
//     return gstimeInternal(jday(...args));
//   }
//   return gstimeInternal(...args);
// }
