pub mod dpper;
pub mod dscom;
pub mod dsinit;
pub mod dspace;
pub mod gstime;
pub mod initl;
pub mod sgp4;
pub mod sgp4init;

use crate::constants::*;
use crate::ext::*;
use crate::io::*;
use crate::propogation::sgp4::*;
use crate::*;
use chrono::prelude::*;

/// Propogate changes to a satrec for a specific datetime.
pub fn propogate_datetime(satrec: &Satrec, time: DateTime<Utc>) -> Result<SGP4Result, SGP4Error> {
    let j = jday(
        time.year() as Float,
        time.month() as Float,
        time.day() as Float,
        time.hour() as Float,
        time.minute() as Float,
        time.second() as Float,
        0 as Float,
    );

    let m = (j - satrec.jdsatepoch) * MINUTES_PER_DAY;
    let mut rec = satrec.clone();
    return sgp4(&mut rec, m);
}
