use crate::constants::*;

pub struct DsinitOptions {
    pub cosim: f64,
    pub argpo: f64,
    pub s1: f64,
    pub s2: f64,
    pub s3: f64,
    pub s4: f64,
    pub s5: f64,
    pub sinim: f64,
    pub ss1: f64,
    pub ss2: f64,
    pub ss3: f64,
    pub ss4: f64,
    pub ss5: f64,
    pub sz1: f64,
    pub sz3: f64,
    pub sz11: f64,
    pub sz13: f64,
    pub sz21: f64,
    pub sz23: f64,
    pub sz31: f64,
    pub sz33: f64,
    pub t: f64,
    pub tc: f64,
    pub gsto: f64,
    pub mo: f64,
    pub mdot: f64,
    pub no: f64,
    pub nodeo: f64,
    pub nodedot: f64,
    pub xpidot: f64,
    pub z1: f64,
    pub z3: f64,
    pub z11: f64,
    pub z13: f64,
    pub z21: f64,
    pub z23: f64,
    pub z31: f64,
    pub z33: f64,
    pub ecco: f64,
    pub eccsq: f64,
    pub emsq: f64,
    pub em: f64,
    pub argpm: f64,
    pub inclm: f64,
    pub mm: f64,
    pub nm: f64,
    pub nodem: f64,
    pub irez: f64,
    pub atime: f64,
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
    pub didt: f64,
    pub dmdt: f64,
    pub dnodt: f64,
    pub domdt: f64,
    pub del1: f64,
    pub del2: f64,
    pub del3: f64,
    pub xfact: f64,
    pub xlamo: f64,
    pub xli: f64,
    pub xni: f64,
}

pub struct DinitResult {
    pub em: f64,
    pub argpm: f64,
    pub inclm: f64,
    pub mm: f64,
    pub nm: f64,
    pub nodem: f64,
    pub irez: f64,
    pub atime: f64,
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
    pub didt: f64,
    pub dmdt: f64,
    pub dndt: f64,
    pub dnodt: f64,
    pub domdt: f64,
    pub del1: f64,
    pub del2: f64,
    pub del3: f64,
    pub xfact: f64,
    pub xlamo: f64,
    pub xli: f64,
    pub xni: f64,
}

/*-----------------------------------------------------------------------------
*
*                           procedure dsinit
*
*  this procedure provides deep space contributions to mean motion dot due
*    to geopotential resonance with half day and one day orbits.
*
*  author        : david vallado                  719-573-2600   28 jun 2005
*
*  inputs        :
*    cosim, sinim-
*    emsq        - eccentricity squared
*    argpo       - argument of perigee
*    s1, s2, s3, s4, s5      -
*    ss1, ss2, ss3, ss4, ss5 -
*    sz1, sz3, sz11, sz13, sz21, sz23, sz31, sz33 -
*    t           - time
*    tc          -
*    gsto        - greenwich sidereal time                   rad
*    mo          - mean anomaly
*    mdot        - mean anomaly dot (rate)
*    no          - mean motion
*    nodeo       - right ascension of ascending node
*    nodedot     - right ascension of ascending node dot (rate)
*    xpidot      -
*    z1, z3, z11, z13, z21, z23, z31, z33 -
*    eccm        - eccentricity
*    argpm       - argument of perigee
*    inclm       - inclination
*    mm          - mean anomaly
*    xn          - mean motion
*    nodem       - right ascension of ascending node
*
*  outputs       :
*    em          - eccentricity
*    argpm       - argument of perigee
*    inclm       - inclination
*    mm          - mean anomaly
*    nm          - mean motion
*    nodem       - right ascension of ascending node
*    irez        - flag for resonance           0-none, 1-one day, 2-half day
*    atime       -
*    d2201, d2211, d3210, d3222, d4410, d4422, d5220, d5232, d5421, d5433    -
*    dedt        -
*    didt        -
*    dmdt        -
*    dndt        -
*    dnodt       -
*    domdt       -
*    del1, del2, del3        -
*    ses  , sghl , sghs , sgs  , shl  , shs  , sis  , sls
*    theta       -
*    xfact       -
*    xlamo       -
*    xli         -
*    xni
*
*  locals        :
*    ainv2       -
*    aonv        -
*    cosisq      -
*    eoc         -
*    f220, f221, f311, f321, f322, f330, f441, f442, f522, f523, f542, f543  -
*    g200, g201, g211, g300, g310, g322, g410, g422, g520, g521, g532, g533  -
*    sini2       -
*    temp        -
*    temp1       -
*    theta       -
*    xno2        -
*
*  coupling      :
*    getgravconst
*
*  references    :
*    hoots, roehrich, norad spacetrack report #3 1980
*    hoots, norad spacetrack report #6 1986
*    hoots, schumacher and glover 2004
*    vallado, crawford, hujsak, kelso  2006
----------------------------------------------------------------------------*/

pub fn dsinit(options: DsinitOptions) -> DinitResult {
    let mut cosim = options.cosim;
    let mut argpo = options.argpo;
    let mut s1 = options.s1;
    let mut s2 = options.s2;
    let mut s3 = options.s3;
    let mut s4 = options.s4;
    let mut s5 = options.s5;
    let mut sinim = options.sinim;
    let mut ss1 = options.ss1;
    let mut ss2 = options.ss2;
    let mut ss3 = options.ss3;
    let mut ss4 = options.ss4;
    let mut ss5 = options.ss5;
    let mut sz1 = options.sz1;
    let mut sz3 = options.sz3;
    let mut sz11 = options.sz11;
    let mut sz13 = options.sz13;
    let mut sz21 = options.sz21;
    let mut sz23 = options.sz23;
    let mut sz31 = options.sz31;
    let mut sz33 = options.sz33;
    let mut t = options.t;
    let mut tc = options.tc;
    let mut gsto = options.gsto;
    let mut mo = options.mo;
    let mut mdot = options.mdot;
    let mut no = options.no;
    let mut nodeo = options.nodeo;
    let mut nodedot = options.nodedot;
    let mut xpidot = options.xpidot;
    let mut z1 = options.z1;
    let mut z3 = options.z3;
    let mut z11 = options.z11;
    let mut z13 = options.z13;
    let mut z21 = options.z21;
    let mut z23 = options.z23;
    let mut z31 = options.z31;
    let mut z33 = options.z33;
    let mut ecco = options.ecco;
    let mut eccsq = options.eccsq;
    let mut emsq = options.emsq;
    let mut em = options.em;
    let mut argpm = options.argpm;
    let mut inclm = options.inclm;
    let mut mm = options.mm;
    let mut nm = options.nm;
    let mut nodem = options.nodem;
    let mut irez = options.irez;
    let mut atime = options.atime;
    let mut d3222 = options.d3222;
    let mut d2201 = options.d2201;
    let mut d4422 = options.d4422;
    let mut d2211 = options.d2211;
    let mut d5232 = options.d5232;
    let mut d3210 = options.d3210;
    let mut d5433 = options.d5433;
    let mut d4410 = options.d4410;
    let mut didt = options.didt;
    let mut d5220 = options.d5220;
    let mut dnodt = options.dnodt;
    let mut d5421 = options.d5421;
    let mut del1 = options.del1;
    let mut dedt = options.dedt;
    let mut del3 = options.del3;
    let mut dmdt = options.dmdt;
    let mut xlamo = options.xlamo;
    let mut domdt = options.domdt;
    let mut xni = options.xni;
    let mut del2 = options.del2;
    let mut xfact = options.xfact;
    let mut xli = options.xli;

    let mut f220: f64;
    let mut f221: f64;
    let mut f311: f64;
    let mut f321: f64;
    let mut f322: f64;
    let mut f330: f64;
    let mut f441: f64;
    let mut f442: f64;
    let mut f522: f64;
    let mut f523: f64;
    let mut f542: f64;
    let mut f543: f64;
    let mut g200: f64;
    let mut g201: f64;
    let mut g211: f64;
    let mut g300: f64;
    let mut g310: f64;
    let mut g322: f64;
    let mut g410: f64;
    let mut g422: f64;
    let mut g520: f64;
    let mut g521: f64;
    let mut g532: f64;
    let mut g533: f64;
    let mut sini2: f64;
    let mut temp: f64;
    let mut temp1: f64;
    let mut xno2: f64;
    let mut ainv2: f64;
    let mut aonv: f64;
    let mut cosisq: f64;
    let mut eoc: f64;

    const q22: f64 = 1.7891679e-6;
    const q31: f64 = 2.1460748e-6;
    const q33: f64 = 2.2123015e-7;
    const root22: f64 = 1.7891679e-6;
    const root44: f64 = 7.3636953e-9;
    const root54: f64 = 2.1765803e-9;
    const rptim: f64 = 4.37526908801129966e-3; // equates to 7.29211514668855e-5 rad/sec
    const root32: f64 = 3.7393792e-7;
    const root52: f64 = 1.1428639e-7;
    const znl: f64 = 1.5835218e-4;
    const zns: f64 = 1.19459e-5;

    // -------------------- deep space initialization ------------
    irez = 0.0;
    if (nm < 0.0052359877) && (nm > 0.0034906585) {
        irez = 1.0;
    }
    if (nm >= 8.26e-3) && (nm <= 9.24e-3) && (em >= 0.5) {
        irez = 2.0;
    }

    // ------------------------ do solar terms -------------------
    let ses = ss1 * zns * ss5;
    let sis = ss2 * zns * (sz11 + sz13);
    let sls = -zns * ss3 * ((sz1 + sz3) - 14.0 - (6.0 * emsq));
    let sghs = ss4 * zns * ((sz31 + sz33) - 6.0);
    let mut shs = -zns * ss2 * (sz21 + sz23);

    // sgp4fix for 180 deg incl
    if inclm < 5.2359877e-2 || inclm > PI - 5.2359877e-2 {
        shs = 0.0;
    }
    if (sinim != 0.0) {
        shs /= sinim;
    }
    let sgs = sghs - (cosim * shs);

    // ------------------------- do lunar terms ------------------
    dedt = ses + (s1 * znl * s5);
    didt = sis + (s2 * znl * (z11 + z13));
    dmdt = sls - (znl * s3 * ((z1 + z3) - 14.0 - (6.0 * emsq)));
    let sghl = s4 * znl * ((z31 + z33) - 6.0);
    let mut shll = -znl * s2 * (z21 + z23);

    // sgp4fix for 180 deg incl
    if (inclm < 5.2359877e-2) || (inclm > (PI - 5.2359877e-2)) {
        shll = 0.0;
    }
    domdt = sgs + sghl;
    dnodt = shs;
    if sinim != 0.0 {
        domdt -= (cosim / sinim) * shll;
        dnodt += shll / sinim;
    }

    // ----------- calculate deep space resonance effects --------
    let dndt = 0.0;
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
    //   inclm  = -inclm;
    //   argpm  = argpm - pi;
    //   nodem = nodem + pi;
    // }

    // -------------- initialize the resonance terms -------------
    if irez != 0.0 {
        aonv = (nm / xke()).powf(X2O3);

        // ---------- geopotential resonance for 12 hour orbits ------
        if irez == 2.0 {
            cosisq = cosim * cosim;
            let emo = em;
            em = ecco;
            let emsqo = emsq;
            emsq = eccsq;
            eoc = em * emsq;
            g201 = -0.306 - ((em - 0.64) * 0.440);

            if em <= 0.65 {
                g211 = (3.616 - (13.2470 * em)) + (16.2900 * emsq);
                g310 = ((-19.302 + (117.3900 * em)) - (228.4190 * emsq)) + (156.5910 * eoc);
                g322 = ((-18.9068 + (109.7927 * em)) - (214.6334 * emsq)) + (146.5816 * eoc);
                g410 = ((-41.122 + (242.6940 * em)) - (471.0940 * emsq)) + (313.9530 * eoc);
                g422 = ((-146.407 + (841.8800 * em)) - (1629.014 * emsq)) + (1083.4350 * eoc);
                g520 = ((-532.114 + (3017.977 * em)) - (5740.032 * emsq)) + (3708.2760 * eoc);
            } else {
                g211 = ((-72.099 + (331.819 * em)) - (508.738 * emsq)) + (266.724 * eoc);
                g310 = ((-346.844 + (1582.851 * em)) - (2415.925 * emsq)) + (1246.113 * eoc);
                g322 = ((-342.585 + (1554.908 * em)) - (2366.899 * emsq)) + (1215.972 * eoc);
                g410 = ((-1052.797 + (4758.686 * em)) - (7193.992 * emsq)) + (3651.957 * eoc);
                g422 = ((-3581.690 + (16178.110 * em)) - (24462.770 * emsq)) + (12422.520 * eoc);
                if (em > 0.715) {
                    g520 = ((-5149.66 + (29936.92 * em)) - (54087.36 * emsq)) + (31324.56 * eoc);
                } else {
                    g520 = (1464.74 - (4664.75 * em)) + (3763.64 * emsq);
                }
            }
            if (em < 0.7) {
                g533 = ((-919.22770 + (4988.6100 * em)) - (9064.7700 * emsq)) + (5542.21 * eoc);
                g521 = ((-822.71072 + (4568.6173 * em)) - (8491.4146 * emsq)) + (5337.524 * eoc);
                g532 = ((-853.66600 + (4690.2500 * em)) - (8624.7700 * emsq)) + (5341.4 * eoc);
            } else {
                g533 = ((-37995.780 + (161616.52 * em)) - (229838.20 * emsq)) + (109377.94 * eoc);
                g521 = ((-51752.104 + (218913.95 * em)) - (309468.16 * emsq)) + (146349.42 * eoc);
                g532 = ((-40023.880 + (170470.89 * em)) - (242699.48 * emsq)) + (115605.82 * eoc);
            }
            sini2 = sinim * sinim;
            f220 = 0.75 * (1.0 + (2.0 * cosim) + cosisq);
            f221 = 1.5 * sini2;
            f321 = 1.875 * sinim * (1.0 - (2.0 * cosim) - (3.0 * cosisq));
            f322 = -1.875 * sinim * ((1.0 + (2.0 * cosim)) - (3.0 * cosisq));
            f441 = 35.0 * sini2 * f220;
            f442 = 39.3750 * sini2 * sini2;

            f522 = 9.84375
                * sinim
                * ((sini2 * (1.0 - (2.0 * cosim) - (5.0 * cosisq)))
                    + (0.33333333 * (-2.0 + (4.0 * cosim) + (6.0 * cosisq))));
            f523 = sinim
                * ((4.92187512 * sini2 * ((-2.0 - (4.0 * cosim)) + (10.0 * cosisq)))
                    + (6.56250012 * ((1.0 + (2.0 * cosim)) - (3.0 * cosisq))));
            f542 = 29.53125
                * sinim
                * ((2.0 - (8.0 * cosim)) + (cosisq * (-12.0 + (8.0 * cosim) + (10.0 * cosisq))));
            f543 = 29.53125
                * sinim
                * ((-2.0 - (8.0 * cosim)) + (cosisq * ((12.0 + (8.0 * cosim)) - (10.0 * cosisq))));

            xno2 = nm * nm;
            ainv2 = aonv * aonv;
            temp1 = 3.0 * xno2 * ainv2;
            temp = temp1 * root22;
            d2201 = temp * f220 * g201;
            d2211 = temp * f221 * g211;
            temp1 *= aonv;
            temp = temp1 * root32;
            d3210 = temp * f321 * g310;
            d3222 = temp * f322 * g322;
            temp1 *= aonv;
            temp = 2.0 * temp1 * root44;
            d4410 = temp * f441 * g410;
            d4422 = temp * f442 * g422;
            temp1 *= aonv;
            temp = temp1 * root52;
            d5220 = temp * f522 * g520;
            d5232 = temp * f523 * g532;
            temp = 2.0 * temp1 * root54;
            d5421 = temp * f542 * g521;
            d5433 = temp * f543 * g533;
            xlamo = ((mo + nodeo + nodeo) - (theta + theta)) % TWO_PI;
            xfact = (mdot + dmdt + (2.0 * ((nodedot + dnodt) - rptim))) - no;
            em = emo;
            emsq = emsqo;
        }

        //  ---------------- synchronous resonance terms --------------
        if irez == 1.0 {
            g200 = 1.0 + (emsq * (-2.5 + (0.8125 * emsq)));
            g310 = 1.0 + (2.0 * emsq);
            g300 = 1.0 + (emsq * (-6.0 + (6.60937 * emsq)));
            f220 = 0.75 * (1.0 + cosim) * (1.0 + cosim);
            f311 = (0.9375 * sinim * sinim * (1.0 + (3.0 * cosim))) - (0.75 * (1.0 + cosim));
            f330 = 1.0 + cosim;
            f330 *= 1.875 * f330 * f330;
            del1 = 3.0 * nm * nm * aonv * aonv;
            del2 = 2.0 * del1 * f220 * g200 * q22;
            del3 = 3.0 * del1 * f330 * g300 * q33 * aonv;
            del1 = del1 * f311 * g310 * q31 * aonv;
            xlamo = ((mo + nodeo + argpo) - theta) % TWO_PI;
            xfact = (mdot + xpidot + dmdt + domdt + dnodt) - (no + rptim);
        }

        //  ------------ for sgp4, initialize the integrator ----------
        xli = xlamo;
        xni = no;
        atime = 0.0;
        nm = no + dndt;
    }

    DinitResult {
        em,
        argpm,
        inclm,
        mm,
        nm,
        nodem,

        irez,
        atime,

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
        didt,
        dmdt,
        dndt,
        dnodt,
        domdt,

        del1,
        del2,
        del3,

        xfact,
        xlamo,
        xli,
        xni,
    }
}
