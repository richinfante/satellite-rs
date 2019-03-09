use std::f64::consts::PI;
const TWO_PI: f64 = PI * 2.0;

pub struct DspaceOptions {
    pub irez: f64,
    pub d2201: f64,
    pub d2211: f64,
    pub d3210: f64,
    pub d3222: f64,
    pub d4410: f64,
    pub d4422: f64,
    pub d5220: f64,
    pub d5232: f64,
    pub d5421: f64,
    pub d5433: f64,
    pub dedt: f64,
    pub del1: f64,
    pub del2: f64,
    pub del3: f64,
    pub didt: f64,
    pub dmdt: f64,
    pub dnodt: f64,
    pub domdt: f64,
    pub argpo: f64,
    pub argpdot: f64,
    pub t: f64,
    pub tc: f64,
    pub gsto: f64,
    pub xfact: f64,
    pub xlamo: f64,
    pub no: f64,
    pub atime: f64,
    pub em: f64,
    pub argpm: f64,
    pub inclm: f64,
    pub xli: f64,
    pub mm: f64,
    pub xni: f64,
    pub nodem: f64,
    pub nm: f64,
}

pub struct DspaceResult {
    pub atime: f64,
    pub em: f64,
    pub argpm: f64,
    pub inclm: f64,
    pub xli: f64,
    pub mm: f64,
    pub xni: f64,
    pub nodem: f64,
    pub dndt: f64,
    pub nm: f64,
}
/*-----------------------------------------------------------------------------
*
*                           procedure dspace
*
*  this procedure provides deep space contributions to mean elements for
*    perturbing third body.  these effects have been averaged over one
*    revolution of the sun and moon.  for earth resonance effects, the
*    effects have been averaged over no revolutions of the satellite.
*    (mean motion)
*
*  author        : david vallado                  719-573-2600   28 jun 2005
*
*  inputs        :
*    d2201, d2211, d3210, d3222, d4410, d4422, d5220, d5232, d5421, d5433 -
*    dedt        -
*    del1, del2, del3  -
*    didt        -
*    dmdt        -
*    dnodt       -
*    domdt       -
*    irez        - flag for resonance           0-none, 1-one day, 2-half day
*    argpo       - argument of perigee
*    argpdot     - argument of perigee dot (rate)
*    t           - time
*    tc          -
*    gsto        - gst
*    xfact       -
*    xlamo       -
*    no          - mean motion
*    atime       -
*    em          - eccentricity
*    ft          -
*    argpm       - argument of perigee
*    inclm       - inclination
*    xli         -
*    mm          - mean anomaly
*    xni         - mean motion
*    nodem       - right ascension of ascending node
*
*  outputs       :
*    atime       -
*    em          - eccentricity
*    argpm       - argument of perigee
*    inclm       - inclination
*    xli         -
*    mm          - mean anomaly
*    xni         -
*    nodem       - right ascension of ascending node
*    dndt        -
*    nm          - mean motion
*
*  locals        :
*    delt        -
*    ft          -
*    theta       -
*    x2li        -
*    x2omi       -
*    xl          -
*    xldot       -
*    xnddt       -
*    xndt        -
*    xomi        -
*
*  coupling      :
*    none        -
*
*  references    :
*    hoots, roehrich, norad spacetrack report #3 1980
*    hoots, norad spacetrack report #6 1986
*    hoots, schumacher and glover 2004
*    vallado, crawford, hujsak, kelso  2006
----------------------------------------------------------------------------*/
pub fn dspace(options: DspaceOptions) -> DspaceResult {
    let DspaceOptions {
        irez,
        d2201,
        d2211,
        d3210,
        d3222,
        d4410,
        d4422,
        d5220,
        d5232,
        d5421,
        d5433,
        dedt,
        del1,
        del2,
        del3,
        didt,
        dmdt,
        dnodt,
        domdt,
        argpo,
        argpdot,
        t,
        tc,
        gsto,
        xfact,
        xlamo,
        no,
        ..
    } = options;

    let DspaceOptions {
        mut atime,
        mut em,
        mut argpm,
        mut inclm,
        mut xli,
        mut mm,
        mut xni,
        mut nodem,
        mut nm,
        ..
    } = options;

    const fasx2: f64 = 0.13130908;
    const fasx4: f64 = 2.8843198;
    const fasx6: f64 = 0.37448087;
    const g22: f64 = 5.7686396;
    const g32: f64 = 0.95240898;
    const g44: f64 = 1.8014998;
    const g52: f64 = 1.0508330;
    const g54: f64 = 4.4108898;
    const rptim: f64 = 4.37526908801129966e-3; // equates to 7.29211514668855e-5 rad/sec
    const stepp: f64 = 720.0;
    const stepn: f64 = -720.0;
    const step2: f64 = 259200.0;

    let mut delt;
    let mut x2li;
    let mut x2omi;
    let mut xl;
    let mut xldot = 0.0;
    let mut xnddt = 0.0;
    let mut xndt = 0.0;
    let mut xomi;
    let mut dndt = 0.0;
    let mut ft = 0.0;

    //  ----------- calculate deep space resonance effects -----------
    let theta = (gsto + (tc * rptim)) % TWO_PI;
    em += dedt * t;

    inclm += didt * t;
    argpm += domdt * t;
    nodem += dnodt * t;
    mm += dmdt * t;

    // sgp4fix for negative inclinations
    // the following if statement should be commented out
    // if (inclm < 0.0)
    // {
    //   inclm = -inclm;
    //   argpm = argpm - pi;
    //   nodem = nodem + pi;
    // }

    /* - update resonances : numerical (euler-maclaurin) integration - */
    /* ------------------------- epoch restart ----------------------  */
    //   sgp4fix for propagator problems
    //   the following integration works for negative time steps and periods
    //   the specific changes are unknown because the original code was so convoluted

    // sgp4fix take out atime = 0.0 and fix for faster operation

    if irez != 0.0 {
        //  sgp4fix streamline check
        if atime == 0.0 || t * atime <= 0.0 || t.abs() < atime.abs() {
            atime = 0.0;
            xni = no;
            xli = xlamo;
        }

        // sgp4fix move check outside loop
        if (t > 0.0) {
            delt = stepp;
        } else {
            delt = stepn;
        }

        let mut iretn = 381; // added for do loop
        while iretn == 381 {
            //  ------------------- dot terms calculated -------------
            //  ----------- near - synchronous resonance terms -------
            if irez != 2.0 {
                xndt = (del1 * (xli - fasx2).sin())
                    + (del2 * (2.0 * (xli - fasx4)).sin())
                    + (del3 * (3.0 * (xli - fasx6)).sin());
                xldot = xni + xfact;
                xnddt = (del1 * (xli - fasx2).cos())
                    + (2.0 * del2 * (2.0 * (xli - fasx4)).cos())
                    + (3.0 * del3 * (3.0 * (xli - fasx6)).cos());
                xnddt *= xldot;
            } else {
                // --------- near - half-day resonance terms --------
                xomi = argpo + (argpdot * atime);
                x2omi = xomi + xomi;
                x2li = xli + xli;
                xndt = (d2201 * ((x2omi + xli) - g22).sin())
                    + (d2211 * (xli - g22).sin())
                    + (d3210 * ((xomi + xli) - g32).sin())
                    + (d3222 * ((-xomi + xli) - g32).sin())
                    + (d4410 * ((x2omi + x2li) - g44).sin())
                    + (d4422 * (x2li - g44).sin())
                    + (d5220 * ((xomi + xli) - g52).sin())
                    + (d5232 * ((-xomi + xli) - g52).sin())
                    + (d5421 * ((xomi + x2li) - g54).sin())
                    + (d5433 * ((-xomi + x2li) - g54).sin());
                xldot = xni + xfact;
                xnddt = (d2201 * ((x2omi + xli) - g22).cos())
                    + (d2211 * (xli - g22).cos())
                    + (d3210 * ((xomi + xli) - g32).cos())
                    + (d3222 * ((-xomi + xli) - g32).cos())
                    + (d5220 * ((xomi + xli) - g52).cos())
                    + (d5232 * ((-xomi + xli) - g52).cos())
                    + (2.0 * d4410 * ((x2omi + x2li) - g44).cos())
                    + (d4422 * (x2li - g44).cos())
                    + (d5421 * ((xomi + x2li) - g54).cos())
                    + (d5433 * ((-xomi + x2li) - g54).cos());
                xnddt *= xldot;
            }

            //  ----------------------- integrator -------------------
            //  sgp4fix move end checks to end of routine
            if (t - atime).abs() >= stepp {
                iretn = 381;
            } else {
                ft = t - atime;
                iretn = 0;
            }

            if iretn == 381 {
                xli += (xldot * delt) + (xndt * step2);
                xni += (xndt * delt) + (xnddt * step2);
                atime += delt;
            }
        }

        nm = xni + (xndt * ft) + (xnddt * ft * ft * 0.5);
        xl = xli + (xldot * ft) + (xndt * ft * ft * 0.5);
        if irez != 1.0 {
            mm = (xl - (2.0 * nodem)) + (2.0 * theta);
            dndt = nm - no;
        } else {
            mm = (xl - nodem - argpm) + theta;
            dndt = nm - no;
        }
        nm = no + dndt;
    }

    DspaceResult {
        atime,
        em,
        argpm,
        inclm,
        xli,
        mm,
        xni,
        nodem,
        dndt,
        nm,
    }
}
