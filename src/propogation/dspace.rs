use crate::*;
const TWO_PI: Float = PI * 2.0;

pub struct DspaceOptions {
    pub irez: Float,
    pub d2201: Float,
    pub d2211: Float,
    pub d3210: Float,
    pub d3222: Float,
    pub d4410: Float,
    pub d4422: Float,
    pub d5220: Float,
    pub d5232: Float,
    pub d5421: Float,
    pub d5433: Float,
    pub dedt: Float,
    pub del1: Float,
    pub del2: Float,
    pub del3: Float,
    pub didt: Float,
    pub dmdt: Float,
    pub dnodt: Float,
    pub domdt: Float,
    pub argpo: Float,
    pub argpdot: Float,
    pub t: Float,
    pub tc: Float,
    pub gsto: Float,
    pub xfact: Float,
    pub xlamo: Float,
    pub no: Float,
    pub atime: Float,
    pub em: Float,
    pub argpm: Float,
    pub inclm: Float,
    pub xli: Float,
    pub mm: Float,
    pub xni: Float,
    pub nodem: Float,
    pub nm: Float,
}

pub struct DspaceResult {
    pub atime: Float,
    pub em: Float,
    pub argpm: Float,
    pub inclm: Float,
    pub xli: Float,
    pub mm: Float,
    pub xni: Float,
    pub nodem: Float,
    pub dndt: Float,
    pub nm: Float,
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

    const FASX2: Float = 0.13130908;
    const FASX4: Float = 2.8843198;
    const FASX6: Float = 0.37448087;
    const G22: Float = 5.7686396;
    const G32: Float = 0.95240898;
    const G44: Float = 1.8014998;
    const G52: Float = 1.0508330;
    const G54: Float = 4.4108898;
    const RPTIM: Float = 4.37526908801129966e-3; // equates to 7.29211514668855e-5 rad/sec
    const STEPP: Float = 720.0;
    const STEPN: Float = -720.0;
    const STEP2: Float = 259200.0;

    let delt;
    let mut x2li;
    let mut x2omi;
    let xl;
    let mut xldot = 0.0;
    let mut xnddt = 0.0;
    let mut xndt = 0.0;
    let mut xomi;
    let mut dndt = 0.0;
    let mut ft = 0.0;

    //  ----------- calculate deep space resonance effects -----------
    let theta = (gsto + (tc * RPTIM)) % TWO_PI;
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
        if t > 0.0 {
            delt = STEPP;
        } else {
            delt = STEPN;
        }

        let mut iretn = 381; // added for do loop
        while iretn == 381 {
            //  ------------------- dot terms calculated -------------
            //  ----------- near - synchronous resonance terms -------
            if irez != 2.0 {
                xndt = (del1 * (xli - FASX2).sin())
                    + (del2 * (2.0 * (xli - FASX4)).sin())
                    + (del3 * (3.0 * (xli - FASX6)).sin());
                xldot = xni + xfact;
                xnddt = (del1 * (xli - FASX2).cos())
                    + (2.0 * del2 * (2.0 * (xli - FASX4)).cos())
                    + (3.0 * del3 * (3.0 * (xli - FASX6)).cos());
                xnddt *= xldot;
            } else {
                // --------- near - half-day resonance terms --------
                xomi = argpo + (argpdot * atime);
                x2omi = xomi + xomi;
                x2li = xli + xli;
                xndt = (d2201 * ((x2omi + xli) - G22).sin())
                    + (d2211 * (xli - G22).sin())
                    + (d3210 * ((xomi + xli) - G32).sin())
                    + (d3222 * ((-xomi + xli) - G32).sin())
                    + (d4410 * ((x2omi + x2li) - G44).sin())
                    + (d4422 * (x2li - G44).sin())
                    + (d5220 * ((xomi + xli) - G52).sin())
                    + (d5232 * ((-xomi + xli) - G52).sin())
                    + (d5421 * ((xomi + x2li) - G54).sin())
                    + (d5433 * ((-xomi + x2li) - G54).sin());
                xldot = xni + xfact;
                xnddt = (d2201 * ((x2omi + xli) - G22).cos())
                    + (d2211 * (xli - G22).cos())
                    + (d3210 * ((xomi + xli) - G32).cos())
                    + (d3222 * ((-xomi + xli) - G32).cos())
                    + (d5220 * ((xomi + xli) - G52).cos())
                    + (d5232 * ((-xomi + xli) - G52).cos())
                    + (2.0 * d4410 * ((x2omi + x2li) - G44).cos())
                    + (d4422 * (x2li - G44).cos())
                    + (d5421 * ((xomi + x2li) - G54).cos())
                    + (d5433 * ((-xomi + x2li) - G54).cos());
                xnddt *= xldot;
            }

            //  ----------------------- integrator -------------------
            //  sgp4fix move end checks to end of routine
            if (t - atime).abs() >= STEPP {
                iretn = 381;
            } else {
                ft = t - atime;
                iretn = 0;
            }

            if iretn == 381 {
                xli += (xldot * delt) + (xndt * STEP2);
                xni += (xndt * delt) + (xnddt * STEP2);
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

#[cfg(test)]
mod tests {
    use crate::propogation::dspace::*;

    #[test]
    fn test_dspace() {
        let opts = DspaceOptions {
            irez: 0.0,
            d2201: 0.0,
            d2211: 0.0,
            d3210: 0.0,
            d3222: 0.0,
            d4410: 0.0,
            d4422: 0.0,
            d5220: 0.0,
            d5232: 0.0,
            d5421: 0.0,
            d5433: 0.0,
            dedt: 2.63860646954029e-8,
            del1: 0.0,
            del2: 0.0,
            del3: 0.0,
            didt: -3.4767374233712414e-8,
            dmdt: 8.037814266648781e-8,
            dnodt: -6.033631312091549e-8,
            domdt: 9.465204025716937e-9,
            argpo: 0.8285461931652521,
            argpdot: 0.0000034607723715772176,
            t: 0.0,
            tc: 0.0,
            gsto: 1.265125075734467,
            xfact: 0.0,
            xlamo: 0.0,
            no: 0.009971131594572634,
            atime: 0.0,
            em: 0.7318036,
            argpm: 0.8285461931652521,
            inclm: 0.8166674822761788,
            xli: 0.0,
            mm: 0.1817184457298936,
            xni: 0.0,
            nodem: 4.021856443150141,
            nm: 0.009971131594572634,
        };

        let res = dspace(opts);

        assert_eq!(res.atime, 0.0);
        assert_eq!(res.em, 0.7318036);
        assert_eq!(res.argpm, 0.8285461931652521);
        assert_eq!(res.inclm, 0.8166674822761788);
        assert_eq!(res.xli, 0.0);
        assert_eq!(res.mm, 0.1817184457298936);
        assert_eq!(res.xni, 0.0);
        assert_eq!(res.nodem, 4.021856443150141);
        assert_eq!(res.dndt, 0.0);
        assert_eq!(res.nm, 0.009971131594572634);
    }
}
