use crate::constants::*;
use crate::*;
pub struct DsinitOptions {
    pub cosim: Float,
    pub argpo: Float,
    pub s1: Float,
    pub s2: Float,
    pub s3: Float,
    pub s4: Float,
    pub s5: Float,
    pub sinim: Float,
    pub ss1: Float,
    pub ss2: Float,
    pub ss3: Float,
    pub ss4: Float,
    pub ss5: Float,
    pub sz1: Float,
    pub sz3: Float,
    pub sz11: Float,
    pub sz13: Float,
    pub sz21: Float,
    pub sz23: Float,
    pub sz31: Float,
    pub sz33: Float,
    pub t: Float,
    pub tc: Float,
    pub gsto: Float,
    pub mo: Float,
    pub mdot: Float,
    pub no: Float,
    pub nodeo: Float,
    pub nodedot: Float,
    pub xpidot: Float,
    pub z1: Float,
    pub z3: Float,
    pub z11: Float,
    pub z13: Float,
    pub z21: Float,
    pub z23: Float,
    pub z31: Float,
    pub z33: Float,
    pub ecco: Float,
    pub eccsq: Float,
    pub emsq: Float,
    pub em: Float,
    pub argpm: Float,
    pub inclm: Float,
    pub mm: Float,
    pub nm: Float,
    pub nodem: Float,
    pub irez: Float,
    pub atime: Float,
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
    pub didt: Float,
    pub dmdt: Float,
    pub dnodt: Float,
    pub domdt: Float,
    pub del1: Float,
    pub del2: Float,
    pub del3: Float,
    pub xfact: Float,
    pub xlamo: Float,
    pub xli: Float,
    pub xni: Float,
}

pub struct DinitResult {
    pub em: Float,
    pub argpm: Float,
    pub inclm: Float,
    pub mm: Float,
    pub nm: Float,
    pub nodem: Float,
    pub irez: Float,
    pub atime: Float,
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
    pub didt: Float,
    pub dmdt: Float,
    pub dndt: Float,
    pub dnodt: Float,
    pub domdt: Float,
    pub del1: Float,
    pub del2: Float,
    pub del3: Float,
    pub xfact: Float,
    pub xlamo: Float,
    pub xli: Float,
    pub xni: Float,
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
    let cosim = options.cosim;
    let argpo = options.argpo;
    let s1 = options.s1;
    let s2 = options.s2;
    let s3 = options.s3;
    let s4 = options.s4;
    let s5 = options.s5;
    let sinim = options.sinim;
    let ss1 = options.ss1;
    let ss2 = options.ss2;
    let ss3 = options.ss3;
    let ss4 = options.ss4;
    let ss5 = options.ss5;
    let sz1 = options.sz1;
    let sz3 = options.sz3;
    let sz11 = options.sz11;
    let sz13 = options.sz13;
    let sz21 = options.sz21;
    let sz23 = options.sz23;
    let sz31 = options.sz31;
    let sz33 = options.sz33;
    let t = options.t;
    let tc = options.tc;
    let gsto = options.gsto;
    let mo = options.mo;
    let mdot = options.mdot;
    let no = options.no;
    let nodeo = options.nodeo;
    let nodedot = options.nodedot;
    let xpidot = options.xpidot;
    let z1 = options.z1;
    let z3 = options.z3;
    let z11 = options.z11;
    let z13 = options.z13;
    let z21 = options.z21;
    let z23 = options.z23;
    let z31 = options.z31;
    let z33 = options.z33;
    let ecco = options.ecco;
    let eccsq = options.eccsq;
    let mut emsq = options.emsq;
    let mut em = options.em;
    let mut argpm = options.argpm;
    let mut inclm = options.inclm;
    let mut mm = options.mm;
    let mut nm = options.nm;
    let mut nodem = options.nodem;
    let mut _irez = options.irez;
    let mut atime = options.atime;
    let mut d3222 = options.d3222;
    let mut d2201 = options.d2201;
    let mut d4422 = options.d4422;
    let mut d2211 = options.d2211;
    let mut d5232 = options.d5232;
    let mut d3210 = options.d3210;
    let mut d5433 = options.d5433;
    let mut d4410 = options.d4410;
    let mut _didt = options.didt;
    let mut d5220 = options.d5220;
    let mut _dnodt = options.dnodt;
    let mut d5421 = options.d5421;
    let mut del1 = options.del1;
    let mut _dedt = options.dedt;
    let mut del3 = options.del3;
    let mut _dmdt = options.dmdt;
    let mut xlamo = options.xlamo;
    let mut _domdt = options.domdt;
    let mut xni = options.xni;
    let mut del2 = options.del2;
    let mut xfact = options.xfact;
    let mut xli = options.xli;

    let mut f220: Float;
    let f221: Float;
    let f311: Float;
    let f321: Float;
    let f322: Float;
    let mut f330: Float;
    let f441: Float;
    let f442: Float;
    let f522: Float;
    let f523: Float;
    let f542: Float;
    let f543: Float;
    let g200: Float;
    let g201: Float;
    let g211: Float;
    let g300: Float;
    let mut g310: Float;
    let g322: Float;
    let g410: Float;
    let g422: Float;
    let g520: Float;
    let g521: Float;
    let g532: Float;
    let g533: Float;
    let sini2: Float;
    let mut temp: Float;
    let mut temp1: Float;
    let xno2: Float;
    let ainv2: Float;
    let aonv: Float;
    let cosisq: Float;
    let eoc: Float;

    const Q22: Float = 1.7891679e-6;
    const Q31: Float = 2.1460748e-6;
    const Q33: Float = 2.2123015e-7;
    const ROOT22: Float = 1.7891679e-6;
    const ROOT44: Float = 7.3636953e-9;
    const ROOT54: Float = 2.1765803e-9;
    const RPTIM: Float = 4.37526908801129966e-3; // equates to 7.29211514668855e-5 rad/sec
    const ROOT32: Float = 3.7393792e-7;
    const ROOT52: Float = 1.1428639e-7;
    const ZNL: Float = 1.5835218e-4;
    const ZNS: Float = 1.19459e-5;

    // -------------------- deep space initialization ------------
    _irez = 0.0;
    if (nm < 0.0052359877) && (nm > 0.0034906585) {
        _irez = 1.0;
    }
    if (nm >= 8.26e-3) && (nm <= 9.24e-3) && (em >= 0.5) {
        _irez = 2.0;
    }

    // ------------------------ do solar terms -------------------
    let ses = ss1 * ZNS * ss5;
    let sis = ss2 * ZNS * (sz11 + sz13);
    let sls = -ZNS * ss3 * ((sz1 + sz3) - 14.0 - (6.0 * emsq));
    let sghs = ss4 * ZNS * ((sz31 + sz33) - 6.0);
    let mut shs = -ZNS * ss2 * (sz21 + sz23);

    // sgp4fix for 180 deg incl
    if inclm < 5.2359877e-2 || inclm > PI - 5.2359877e-2 {
        shs = 0.0;
    }
    if sinim != 0.0 {
        shs /= sinim;
    }
    let sgs = sghs - (cosim * shs);

    // ------------------------- do lunar terms ------------------
    _dedt = ses + (s1 * ZNL * s5);
    _didt = sis + (s2 * ZNL * (z11 + z13));
    _dmdt = sls - (ZNL * s3 * ((z1 + z3) - 14.0 - (6.0 * emsq)));
    let sghl = s4 * ZNL * ((z31 + z33) - 6.0);
    let mut shll = -ZNL * s2 * (z21 + z23);

    // sgp4fix for 180 deg incl
    if (inclm < 5.2359877e-2) || (inclm > (PI - 5.2359877e-2)) {
        shll = 0.0;
    }
    _domdt = sgs + sghl;
    _dnodt = shs;
    if sinim != 0.0 {
        _domdt -= (cosim / sinim) * shll;
        _dnodt += shll / sinim;
    }

    // ----------- calculate deep space resonance effects --------
    let dndt = 0.0;
    let theta = (gsto + (tc * RPTIM)) % TWO_PI;
    em += _dedt * t;
    inclm += _didt * t;
    argpm += _domdt * t;
    nodem += _dnodt * t;
    mm += _dmdt * t;

    // sgp4fix for negative inclinations
    // the following if statement should be commented out
    // if (inclm < 0.0)
    // {
    //   inclm  = -inclm;
    //   argpm  = argpm - pi;
    //   nodem = nodem + pi;
    // }

    // -------------- initialize the resonance terms -------------
    if _irez != 0.0 {
        aonv = (nm / XKE).powf(X2O3);

        // ---------- geopotential resonance for 12 hour orbits ------
        if _irez == 2.0 {
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
                if em > 0.715 {
                    g520 = ((-5149.66 + (29936.92 * em)) - (54087.36 * emsq)) + (31324.56 * eoc);
                } else {
                    g520 = (1464.74 - (4664.75 * em)) + (3763.64 * emsq);
                }
            }
            if em < 0.7 {
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
            temp = temp1 * ROOT22;
            d2201 = temp * f220 * g201;
            d2211 = temp * f221 * g211;
            temp1 *= aonv;
            temp = temp1 * ROOT32;
            d3210 = temp * f321 * g310;
            d3222 = temp * f322 * g322;
            temp1 *= aonv;
            temp = 2.0 * temp1 * ROOT44;
            d4410 = temp * f441 * g410;
            d4422 = temp * f442 * g422;
            temp1 *= aonv;
            temp = temp1 * ROOT52;
            d5220 = temp * f522 * g520;
            d5232 = temp * f523 * g532;
            temp = 2.0 * temp1 * ROOT54;
            d5421 = temp * f542 * g521;
            d5433 = temp * f543 * g533;
            xlamo = ((mo + nodeo + nodeo) - (theta + theta)) % TWO_PI;
            xfact = (mdot + _dmdt + (2.0 * ((nodedot + _dnodt) - RPTIM))) - no;
            em = emo;
            emsq = emsqo;
        }

        //  ---------------- synchronous resonance terms --------------
        if _irez == 1.0 {
            g200 = 1.0 + (emsq * (-2.5 + (0.8125 * emsq)));
            g310 = 1.0 + (2.0 * emsq);
            g300 = 1.0 + (emsq * (-6.0 + (6.60937 * emsq)));
            f220 = 0.75 * (1.0 + cosim) * (1.0 + cosim);
            f311 = (0.9375 * sinim * sinim * (1.0 + (3.0 * cosim))) - (0.75 * (1.0 + cosim));
            f330 = 1.0 + cosim;
            f330 *= 1.875 * f330 * f330;
            del1 = 3.0 * nm * nm * aonv * aonv;
            del2 = 2.0 * del1 * f220 * g200 * Q22;
            del3 = 3.0 * del1 * f330 * g300 * Q33 * aonv;
            del1 = del1 * f311 * g310 * Q31 * aonv;
            xlamo = ((mo + nodeo + argpo) - theta) % TWO_PI;
            xfact = (mdot + xpidot + _dmdt + _domdt + _dnodt) - (no + RPTIM);
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

        irez: _irez,
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

        dedt: _dedt,
        didt: _didt,
        dmdt: _dmdt,
        dndt,
        dnodt: _dnodt,
        domdt: _domdt,

        del1,
        del2,
        del3,

        xfact,
        xlamo,
        xli,
        xni,
    }
}

#[cfg(test)]
mod tests {
    use crate::propogation::dsinit::*;
    use crate::tests::*;
    #[test]
    fn test_dsinit() {
        let opts = DsinitOptions {
            cosim: 0.6846539709541596,
            emsq: 0.53553650897296,
            argpo: 0.8285461931652521,
            s1: -0.0003598896387548552,
            s2: -0.000035294088067045225,
            s3: 0.00004810694207075695,
            s4: 0.000032785630712471235,
            s5: -0.32951417210709383,
            sinim: 0.7288682597401953,
            ss1: -0.0022406638674745552,
            ss2: -0.00021974010738653476,
            ss3: 0.00029951261516050645,
            ss4: 0.0002041225147908132,
            ss5: -0.2842063666667719,
            sz1: 7.241600464426519,
            sz3: 1.6686076878687435,
            sz11: 5.923151674966536,
            sz13: -1.5569425931864267,
            sz21: -5.5489812856673435,
            sz23: 0.16121448720509934,
            sz31: 3.273877043602411,
            sz33: -2.1469592167364815,
            t: 0.0,
            tc: 0.0,
            gsto: 1.265125075734467,
            mo: 0.1817184457298936,
            mdot: 0.009971844927637492,
            no: 0.009971131594572634,
            nodeo: 4.021856443150141,
            nodedot: -0.0000035284989537160483,
            xpidot: -6.772658213883069e-8,
            z1: 2.5573881535383824,
            z3: 8.004285429240719,
            z11: -1.949045345610987,
            z13: 6.119118527261723,
            z21: 0.6841370517901615,
            z23: -6.022288391897364,
            z31: -1.633514023053113,
            z33: 3.7885830019843842,
            ecco: 0.7318036,
            eccsq: 0.53553650897296,
            em: 0.7318036,
            argpm: 0.0,
            inclm: 0.8166674822761788,
            mm: 0.0,
            nm: 0.009971131594572634,
            nodem: 0.0,
            irez: 0.0,
            atime: 0.0,
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
            dedt: 0.0,
            didt: 0.0,
            dmdt: 0.0,
            dnodt: 0.0,
            domdt: 0.0,
            del1: 0.0,
            del2: 0.0,
            del3: 0.0,
            xfact: 0.0,
            xlamo: 0.0,
            xli: 0.0,
            xni: 0.0,
        };

        let res = dsinit(opts);

        assert_eq!(res.em, 0.7318036);
        assert_eq!(res.argpm, 0.0);
        assert_eq!(res.inclm, 0.8166674822761788);
        assert_eq!(res.mm, 0.0);
        assert_eq!(res.nm, 0.009971131594572634);
        assert_eq!(res.nodem, 0.0);
        assert_eq!(res.irez, 0.0);
        assert_eq!(res.atime, 0.0);
        assert_eq!(res.d2201, 0.0);
        assert_eq!(res.d2211, 0.0);
        assert_eq!(res.d3210, 0.0);
        assert_eq!(res.d3222, 0.0);
        assert_eq!(res.d4410, 0.0);
        assert_eq!(res.d4422, 0.0);
        assert_eq!(res.d5220, 0.0);
        assert_eq!(res.d5232, 0.0);
        assert_eq!(res.d5421, 0.0);
        assert_eq!(res.d5433, 0.0);
        assert_similar(res.dedt, 2.63860646954029e-8);
        assert_similar(res.didt, -3.4767374233712414e-8);
        assert_similar(res.dmdt, 8.037814266648781e-8);
        assert_similar(res.dndt, 0.0);
        assert_similar(res.dnodt, -6.033631312091549e-8);
        assert_similar(res.domdt, 9.465204025716937e-9);
        assert_eq!(res.del1, 0.0);
        assert_eq!(res.del2, 0.0);
        assert_eq!(res.del3, 0.0);
        assert_eq!(res.xfact, 0.0);
        assert_eq!(res.xlamo, 0.0);
        assert_eq!(res.xli, 0.0);
        assert_eq!(res.xni, 0.0);
    }
}
