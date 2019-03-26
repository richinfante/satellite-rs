pub mod dpper;
pub mod dscom;
pub mod dsinit;
pub mod dspace;
pub mod gstime;
pub mod initl;
pub mod sgp4;
pub mod sgp4init;


use crate::constants::*;
use crate::io::*;
use crate::ext::*;
use crate::propogation::sgp4::*;

use chrono::prelude::*;
use chrono::DateTime;

/// Propogate changes to a satrec for a specific datetime.
pub fn propogate_datetime(satrec: &Satrec, time: DateTime<Utc>) -> Result<SGP4Result, SGP4Error> {
  let j = jday(
    time.year() as f64,
    time.month() as f64,
    time.day() as f64,
    time.hour() as f64,
    time.minute() as f64,
    time.second() as f64,
    0 as f64
  );

  let m = (j - satrec.jdsatepoch) * MINUTES_PER_DAY;
  let mut rec = satrec.clone();
  return sgp4(&mut rec, m);
}
