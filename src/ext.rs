use chrono::prelude::*;
use chrono::DateTime;

pub struct MDHMS {
    pub month: f64,
    pub day: f64,
    pub hour: f64,
    pub minute: f64,
    pub second: f64,
}

/* -----------------------------------------------------------------------------
 *
 *                           procedure days2mdhms
 *
 *  this procedure converts the day of the year, days, to the equivalent month
 *    day, hour, minute and second.
 *
 *  algorithm     : set up array for the number of days per month
 *                  find leap year - use 1900 because 2000 is a leap year
 *                  loop through a temp value while the value is < the days
 *                  perform int conversions to the correct day and month
 *                  convert remainder into h m s using type conversions
 *
 *  author        : david vallado                  719-573-2600    1 mar 2001
 *
 *  inputs          description                    range / units
 *    year        - year                           1900 .. 2100
 *    days        - julian day of the year         0.0  .. 366.0
 *
 *  outputs       :
 *    mon         - month                          1 .. 12
 *    day         - day                            1 .. 28,29,30,31
 *    hr          - hour                           0 .. 23
 *    min         - minute                         0 .. 59
 *    sec         - second                         0.0 .. 59.999
 *
 *  locals        :
 *    dayofyr     - day of year
 *    temp        - temporary extended values
 *    inttemp     - temporary int value
 *    i           - index
 *    lmonth[12]  - int array containing the number of days per month
 *
 *  coupling      :
 *    none.
 * --------------------------------------------------------------------------- */
pub fn days2mdhms(year: u64, days: f64) -> MDHMS {
    let mut lmonth: [u64; 12] = [31, 28, 31, 30, 31, 30, 31, 31, 30, 31, 30, 31];
    if year % 4 == 0 {
        lmonth[1] = 29
    }

    let dayofyr = days.floor() as u64;

    let mut i: u64 = 1;
    let mut inttemp: u64 = 0;

    while dayofyr > (inttemp + lmonth[(i - 1) as usize]) && i < 12 {
        inttemp += lmonth[(i - 1) as usize];
        i += 1;
    }

    let month = i as f64;
    let day = (dayofyr - inttemp) as f64;
    let mut temp = (days - dayofyr as f64) * 24.0;
    let hour = temp.floor();
    temp = (temp - hour as f64) * 60.0;
    let minute = temp.floor();
    let second = (temp - minute as f64) * 60.0;

    MDHMS {
        month,
        hour,
        day,
        minute,
        second,
    }
}

pub fn jday(year: f64, mon: f64, day: f64, hr: f64, minute: f64, sec: f64, msec: f64) -> f64 {
    ((367.0 * year) - ((7.0 * (year + ((mon + 9.0) / 12.0).floor())) * 0.25).floor())
        + ((275.0 * mon) / 9.0).floor()
        + day
        + 1721013.5
        + (((((msec / 60000.0) + (sec / 60.0) + minute) / 60.0) + hr) / 24.0) // ut in days
}

pub fn jday_datetime(time: DateTime<Utc>) -> f64 {
    jday(
        time.year() as f64,
        time.month() as f64,
        time.day() as f64,
        time.hour() as f64,
        time.minute() as f64,
        time.second() as f64,
        0 as f64
    )
}
