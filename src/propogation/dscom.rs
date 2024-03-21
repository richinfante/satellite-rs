use crate::constants::*;
use crate::*;
pub struct DscomOptions {
    pub epoch: Float,
    pub ep: Float,
    pub argpp: Float,
    pub tc: Float,
    pub inclp: Float,
    pub nodep: Float,
    pub np: Float,
}

pub struct DscomResult {
    pub snodm: Float,
    pub cnodm: Float,
    pub sinim: Float,
    pub cosim: Float,
    pub sinomm: Float,
    pub cosomm: Float,
    pub day: Float,
    pub e3: Float,
    pub ee2: Float,
    pub em: Float,
    pub emsq: Float,
    pub gam: Float,
    pub peo: Float,
    pub pgho: Float,
    pub pho: Float,
    pub pinco: Float,
    pub plo: Float,
    pub rtemsq: Float,
    pub se2: Float,
    pub se3: Float,
    pub sgh2: Float,
    pub sgh3: Float,
    pub sgh4: Float,
    pub sh2: Float,
    pub sh3: Float,
    pub si2: Float,
    pub si3: Float,
    pub sl2: Float,
    pub sl3: Float,
    pub sl4: Float,
    pub s1: Float,
    pub s2: Float,
    pub s3: Float,
    pub s4: Float,
    pub s5: Float,
    pub s6: Float,
    pub s7: Float,
    pub ss1: Float,
    pub ss2: Float,
    pub ss3: Float,
    pub ss4: Float,
    pub ss5: Float,
    pub ss6: Float,
    pub ss7: Float,
    pub sz1: Float,
    pub sz2: Float,
    pub sz3: Float,
    pub sz11: Float,
    pub sz12: Float,
    pub sz13: Float,
    pub sz21: Float,
    pub sz22: Float,
    pub sz23: Float,
    pub sz31: Float,
    pub sz32: Float,
    pub sz33: Float,
    pub xgh2: Float,
    pub xgh3: Float,
    pub xgh4: Float,
    pub xh2: Float,
    pub xh3: Float,
    pub xi2: Float,
    pub xi3: Float,
    pub xl2: Float,
    pub xl3: Float,
    pub xl4: Float,
    pub nm: Float,
    pub z1: Float,
    pub z2: Float,
    pub z3: Float,
    pub z11: Float,
    pub z12: Float,
    pub z13: Float,
    pub z21: Float,
    pub z22: Float,
    pub z23: Float,
    pub z31: Float,
    pub z32: Float,
    pub z33: Float,
    pub zmol: Float,
    pub zmos: Float,
}

/*-----------------------------------------------------------------------------
*
*                           procedure dscom
*
*  this procedure provides deep space common items used by both the secular
*    and periodics subroutines.  input is provided as shown. this routine
*    used to be called dpper, but the functions inside weren't well organized.
*
*  author        : david vallado                  719-573-2600   28 jun 2005
*
*  inputs        :
*    epoch       -
*    ep          - eccentricity
*    argpp       - argument of perigee
*    tc          -
*    inclp       - inclination
*    nodep       - right ascension of ascending node
*    np          - mean motion
*
*  outputs       :
*    sinim  , cosim  , sinomm , cosomm , snodm  , cnodm
*    day         -
*    e3          -
*    ee2         -
*    em          - eccentricity
*    emsq        - eccentricity squared
*    gam         -
*    peo         -
*    pgho        -
*    pho         -
*    pinco       -
*    plo         -
*    rtemsq      -
*    se2, se3         -
*    sgh2, sgh3, sgh4        -
*    sh2, sh3, si2, si3, sl2, sl3, sl4         -
*    s1, s2, s3, s4, s5, s6, s7          -
*    ss1, ss2, ss3, ss4, ss5, ss6, ss7, sz1, sz2, sz3         -
*    sz11, sz12, sz13, sz21, sz22, sz23, sz31, sz32, sz33        -
*    xgh2, xgh3, xgh4, xh2, xh3, xi2, xi3, xl2, xl3, xl4         -
*    nm          - mean motion
*    z1, z2, z3, z11, z12, z13, z21, z22, z23, z31, z32, z33         -
*    zmol        -
*    zmos        -
*
*  locals        :
*    a1, a2, a3, a4, a5, a6, a7, a8, a9, a10         -
*    betasq      -
*    cc          -
*    ctem, stem        -
*    x1, x2, x3, x4, x5, x6, x7, x8          -
*    xnodce      -
*    xnoi        -
*    zcosg  , zsing  , zcosgl , zsingl , zcosh  , zsinh  , zcoshl , zsinhl ,
*    zcosi  , zsini  , zcosil , zsinil ,
*    zx          -
*    zy          -
*
*  coupling      :
*    none.
*
*  references    :
*    hoots, roehrich, norad spacetrack report #3 1980
*    hoots, norad spacetrack report #6 1986
*    hoots, schumacher and glover 2004
*    vallado, crawford, hujsak, kelso  2006
----------------------------------------------------------------------------*/
pub fn dscom(options: DscomOptions) -> DscomResult {
    let mut a1;
    let mut a2;
    let mut a3;
    let mut a4;
    let mut a5;
    let mut a6;
    let mut a7;
    let mut a8;
    let mut a9;
    let mut a10;
    let mut cc;
    let mut x1;
    let mut x2;
    let mut x3;
    let mut x4;
    let mut x5;
    let mut x6;
    let mut x7;
    let mut x8;
    let mut zcosg;
    let mut zsing;
    let mut zcosh;
    let mut zsinh;
    let mut zcosi;
    let mut zsini;
    let mut ss1 = 0.0;
    let mut ss2 = 0.0;
    let mut ss3 = 0.0;
    let mut ss4 = 0.0;
    let mut ss5 = 0.0; // TODO: unused?
    let mut ss6 = 0.0;
    let mut ss7 = 0.0;
    let mut sz1 = 0.0;
    let mut sz2 = 0.0;
    let mut sz3 = 0.0;
    let mut sz11 = 0.0;
    let mut sz12 = 0.0;
    let mut sz13 = 0.0;
    let mut sz21 = 0.0;
    let mut sz22 = 0.0;
    let mut sz23 = 0.0;
    let mut sz31 = 0.0;
    let mut sz32 = 0.0;
    let mut sz33 = 0.0;
    let mut s1 = 0.0;
    let mut s2 = 0.0;
    let mut s3 = 0.0;
    let mut s4 = 0.0;
    let mut s5 = 0.0; // TODO: unused?
    let mut s6 = 0.0;
    let mut s7 = 0.0;
    let mut z1 = 0.0;
    let mut z2 = 0.0;
    let mut z3 = 0.0;
    let mut z11 = 0.0;
    let mut z12 = 0.0;
    let mut z13 = 0.0;
    let mut z21 = 0.0;
    let mut z22 = 0.0;
    let mut z23 = 0.0;
    let mut z31 = 0.0;
    let mut z32 = 0.0;
    let mut z33 = 0.0;

    // -------------------------- constants -------------------------
    const ZES: Float = 0.01675;
    const ZEL: Float = 0.05490;
    const C1SS: Float = 2.9864797e-6;
    const C1L: Float = 4.7968065e-7;
    const ZSINIS: Float = 0.39785416;
    const ZCOSIS: Float = 0.91744867;
    const ZCOSGS: Float = 0.1945905;
    const ZSINGS: Float = -0.98088458;

    //  --------------------- local variables ------------------------
    let nm = options.np;
    let em = options.ep;
    let snodm = options.nodep.sin();
    let cnodm = options.nodep.cos();
    let sinomm = options.argpp.sin();
    let cosomm = options.argpp.cos();
    let sinim = options.inclp.sin();
    let cosim = options.inclp.cos();
    let emsq = em.powi(2);
    let betasq = 1.0 - emsq;
    let rtemsq = betasq.sqrt();

    //  ----------------- initialize lunar solar terms ---------------
    let peo = 0.0;
    let pinco = 0.0;
    let plo = 0.0;
    let pgho = 0.0;
    let pho = 0.0;
    let day = options.epoch + 18261.5 + (options.tc / 1440.0);
    let xnodce = (4.5236020 - (9.2422029e-4 * day)) % TWO_PI;
    let stem = xnodce.sin();
    let ctem = xnodce.cos();
    let zcosil = 0.91375164 - (0.03568096 * ctem);
    let zsinil = (1.0 - (zcosil * zcosil)).sqrt();
    let zsinhl = (0.089683511 * stem) / zsinil;
    let zcoshl = (1.0 - (zsinhl * zsinhl)).sqrt();
    let gam = 5.8351514 + (0.0019443680 * day);
    let mut zx = (0.39785416 * stem) / zsinil;
    let zy = (zcoshl * ctem) + (0.91744867 * zsinhl * stem);
    zx = zx.atan2(zy);
    zx += gam - xnodce;
    let zcosgl = zx.cos();
    let zsingl = zx.sin();

    //  ------------------------- do solar terms ---------------------
    zcosg = ZCOSGS;
    zsing = ZSINGS;
    zcosi = ZCOSIS;
    zsini = ZSINIS;
    zcosh = cnodm;
    zsinh = snodm;
    cc = C1SS;
    let xnoi = 1.0 / nm;

    let mut lsflg = 0;
    while lsflg < 2 {
        lsflg += 1;
        a1 = (zcosg * zcosh) + (zsing * zcosi * zsinh);
        a3 = (-zsing * zcosh) + (zcosg * zcosi * zsinh);
        a7 = (-zcosg * zsinh) + (zsing * zcosi * zcosh);
        a8 = zsing * zsini;
        a9 = (zsing * zsinh) + (zcosg * zcosi * zcosh);
        a10 = zcosg * zsini;
        a2 = (cosim * a7) + (sinim * a8);
        a4 = (cosim * a9) + (sinim * a10);
        a5 = (-sinim * a7) + (cosim * a8);
        a6 = (-sinim * a9) + (cosim * a10);

        x1 = (a1 * cosomm) + (a2 * sinomm);
        x2 = (a3 * cosomm) + (a4 * sinomm);
        x3 = (-a1 * sinomm) + (a2 * cosomm);
        x4 = (-a3 * sinomm) + (a4 * cosomm);
        x5 = a5 * sinomm;
        x6 = a6 * sinomm;
        x7 = a5 * cosomm;
        x8 = a6 * cosomm;

        z31 = (12.0 * x1 * x1) - (3.0 * x3 * x3);
        z32 = (24.0 * x1 * x2) - (6.0 * x3 * x4);
        z33 = (12.0 * x2 * x2) - (3.0 * x4 * x4);

        z1 = (3.0 * ((a1 * a1) + (a2 * a2))) + (z31 * emsq);
        z2 = (6.0 * ((a1 * a3) + (a2 * a4))) + (z32 * emsq);
        z3 = (3.0 * ((a3 * a3) + (a4 * a4))) + (z33 * emsq);

        z11 = (-6.0 * a1 * a5) + (emsq * ((-24.0 * x1 * x7) - (6.0 * x3 * x5)));
        z12 = (-6.0 * ((a1 * a6) + (a3 * a5)))
            + (emsq * ((-24.0 * ((x2 * x7) + (x1 * x8))) + (-6.0 * ((x3 * x6) + (x4 * x5)))));

        z13 = (-6.0 * a3 * a6) + (emsq * ((-24.0 * x2 * x8) - (6.0 * x4 * x6)));

        z21 = (6.0 * a2 * a5) + (emsq * ((24.0 * x1 * x5) - (6.0 * x3 * x7)));
        z22 = (6.0 * ((a4 * a5) + (a2 * a6)))
            + (emsq * ((24.0 * ((x2 * x5) + (x1 * x6))) - (6.0 * ((x4 * x7) + (x3 * x8)))));
        z23 = (6.0 * a4 * a6) + (emsq * ((24.0 * x2 * x6) - (6.0 * x4 * x8)));

        z1 = z1 + z1 + (betasq * z31);
        z2 = z2 + z2 + (betasq * z32);
        z3 = z3 + z3 + (betasq * z33);
        s3 = cc * xnoi;
        s2 = (-0.5 * s3) / rtemsq;
        s4 = s3 * rtemsq;
        s1 = -15.0 * em * s4;
        s5 = (x1 * x3) + (x2 * x4);
        s6 = (x2 * x3) + (x1 * x4);
        s7 = (x2 * x4) - (x1 * x3);

        //  ----------------------- do lunar terms -------------------
        if lsflg == 1 {
            ss1 = s1;
            ss2 = s2;
            ss3 = s3;
            ss4 = s4;
            ss5 = s5;
            ss6 = s6;
            ss7 = s7;
            sz1 = z1;
            sz2 = z2;
            sz3 = z3;
            sz11 = z11;
            sz12 = z12;
            sz13 = z13;
            sz21 = z21;
            sz22 = z22;
            sz23 = z23;
            sz31 = z31;
            sz32 = z32;
            sz33 = z33;
            zcosg = zcosgl;
            zsing = zsingl;
            zcosi = zcosil;
            zsini = zsinil;
            zcosh = (zcoshl * cnodm) + (zsinhl * snodm);
            zsinh = (snodm * zcoshl) - (cnodm * zsinhl);
            cc = C1L;
        }
    }

    let zmol = (4.7199672 + ((0.22997150 * day) - gam)) % TWO_PI;
    let zmos = (6.2565837 + (0.017201977 * day)) % TWO_PI;

    //  ------------------------ do solar terms ----------------------
    let se2 = 2.0 * ss1 * ss6;
    let se3 = 2.0 * ss1 * ss7;
    let si2 = 2.0 * ss2 * sz12;
    let si3 = 2.0 * ss2 * (sz13 - sz11);
    let sl2 = -2.0 * ss3 * sz2;
    let sl3 = -2.0 * ss3 * (sz3 - sz1);
    let sl4 = -2.0 * ss3 * (-21.0 - (9.0 * emsq)) * ZES;
    let sgh2 = 2.0 * ss4 * sz32;
    let sgh3 = 2.0 * ss4 * (sz33 - sz31);
    let sgh4 = -18.0 * ss4 * ZES;
    let sh2 = -2.0 * ss2 * sz22;
    let sh3 = -2.0 * ss2 * (sz23 - sz21);

    //  ------------------------ do lunar terms ----------------------
    let ee2 = 2.0 * s1 * s6;
    let e3 = 2.0 * s1 * s7;
    let xi2 = 2.0 * s2 * z12;
    let xi3 = 2.0 * s2 * (z13 - z11);
    let xl2 = -2.0 * s3 * z2;
    let xl3 = -2.0 * s3 * (z3 - z1);
    let xl4 = -2.0 * s3 * (-21.0 - (9.0 * emsq)) * ZEL;
    let xgh2 = 2.0 * s4 * z32;
    let xgh3 = 2.0 * s4 * (z33 - z31);
    let xgh4 = -18.0 * s4 * ZEL;
    let xh2 = -2.0 * s2 * z22;
    let xh3 = -2.0 * s2 * (z23 - z21);

    DscomResult {
        snodm,
        cnodm,
        sinim,
        cosim,
        sinomm,

        cosomm,
        day,
        e3,
        ee2,
        em,

        emsq,
        gam,
        peo,
        pgho,
        pho,

        pinco,
        plo,
        rtemsq,
        se2,
        se3,

        sgh2,
        sgh3,
        sgh4,
        sh2,
        sh3,

        si2,
        si3,
        sl2,
        sl3,
        sl4,

        s1,
        s2,
        s3,
        s4,
        s5,

        s6,
        s7,
        ss1,
        ss2,
        ss3,

        ss4,
        ss5,
        ss6,
        ss7,
        sz1,

        sz2,
        sz3,
        sz11,
        sz12,
        sz13,

        sz21,
        sz22,
        sz23,
        sz31,
        sz32,

        sz33,
        xgh2,
        xgh3,
        xgh4,
        xh2,

        xh3,
        xi2,
        xi3,
        xl2,
        xl3,

        xl4,
        nm,
        z1,
        z2,
        z3,

        z11,
        z12,
        z13,
        z21,
        z22,

        z23,
        z31,
        z32,
        z33,
        zmol,

        zmos,
    }
}

#[cfg(test)]
mod tests {
    use crate::propogation::dscom::*;
    use crate::tests::assert_similar;
    #[test]
    fn test() {
        let opts = DscomOptions {
            epoch: 11187.29629787989,
            ep: 0.7318036,
            argpp: 0.8285461931652521,
            tc: 0.0,
            inclp: 0.8166674822761788,
            nodep: 4.021856443150141,
            np: 0.009971131594572634,
        };

        let res = dscom(opts);

        assert_similar(res.snodm, -0.7709069259013173);
        assert_similar(res.cnodm, -0.6369478091628709);
        assert_similar(res.sinim, 0.7288682597401953);
        assert_similar(res.cosim, 0.6846539709541596);
        assert_similar(res.sinomm, 0.7369494526341018);
        assert_similar(res.cosomm, 0.6759478561710938);
        assert_similar(res.day, 29448.79629787989);
        assert_similar(res.e3, -0.00007499513323066247);
        assert_similar(res.ee2, 0.0003984687913511968);
        assert_similar(res.em, 0.7318036);
        assert_similar(res.emsq, 0.53553650897296);
        assert_similar(res.gam, 63.09444856011612);
        assert_similar(res.peo, 0.0);
        assert_similar(res.pgho, 0.0);
        assert_similar(res.pho, 0.0);
        assert_similar(res.pinco, 0.0);
        assert_similar(res.plo, 0.0);
        assert_similar(res.rtemsq, 0.6815155838475302);
        assert_similar(res.se2, -0.0023592253136306925);
        assert_similar(res.se3, -0.00007047176334622737);
        assert_similar(res.sgh2, -0.00018225669552040974);
        assert_similar(res.sgh3, -0.0022130294594592042);
        assert_similar(res.sgh4, -0.00006154293820943018);
        assert_similar(res.sh2, -0.0011394073362677001);
        assert_similar(res.sh3, 0.002509518064658255);
        assert_similar(res.si2, -0.00005208303756792119);
        assert_similar(res.si3, 0.0032873534354906702);
        assert_similar(res.sl2, 0.0027816840121748848);
        assert_similar(res.sl3, 0.0033383632815548628);
        assert_similar(res.sl4, 0.00025906770677081676);
        assert_similar(res.s1, -0.0003598896387548552);
        assert_similar(res.s2, -0.000035294088067045225);
        assert_similar(res.s3, 0.00004810694207075695);
        assert_similar(res.s4, 0.000032785630712471235);
        assert_similar(res.s5, -0.32951417210709383);
        assert_similar(res.s6, -0.5535985875139634);
        assert_similar(res.s7, 0.10419184821509497);
        assert_similar(res.ss1, -0.0022406638674745552);
        assert_similar(res.ss2, -0.00021974010738653476);
        assert_similar(res.ss3, 0.00029951261516050645);
        assert_similar(res.ss4, 0.0002041225147908132);
        assert_similar(res.ss5, -0.2842063666667719);
        assert_similar(res.ss6, 0.5264567675404539);
        assert_similar(res.ss7, 0.015725643718630555);
        assert_similar(res.sz1, 7.241600464426519);
        assert_similar(res.sz2, -4.643684224593015);
        assert_similar(res.sz3, 1.6686076878687435);
        assert_similar(res.sz11, 5.923151674966536);
        assert_similar(res.sz12, 0.11851053999055416);
        assert_similar(res.sz13, -1.5569425931864267);
        assert_similar(res.sz21, -5.5489812856673435);
        assert_similar(res.sz22, -2.592624873581728);
        assert_similar(res.sz23, 0.16121448720509934);
        assert_similar(res.sz31, 3.273877043602411);
        assert_similar(res.sz32, -0.4464394721650089);
        assert_similar(res.sz33, -2.1469592167364815);
        assert_similar(res.xgh2, 0.0001510256023997251);
        assert_similar(res.xgh3, 0.0003555337415001366);
        assert_similar(res.xgh4, -0.00003239876027006408);
        assert_similar(res.xh2, 0.00011285895673523819);
        assert_similar(res.xh3, -0.0004733943404491607);
        assert_similar(res.xi2, -0.00006414087517640146);
        assert_similar(res.xi3, -0.0005695169725370441);
        assert_similar(res.xl2, -0.0007034864707310533);
        assert_similar(res.xl3, -0.0005240671434151523);
        assert_similar(res.xl4, 0.00013638400715968452);
        assert_similar(res.nm, 0.009971131594572634);
        assert_similar(res.z1, 2.5573881535383824);
        assert_similar(res.z2, 7.311693909959471);
        assert_similar(res.z3, 8.004285429240719);
        assert_similar(res.z11, -1.949045345610987);
        assert_similar(res.z12, 0.9086631598832984);
        assert_similar(res.z13, 6.119118527261723);
        assert_similar(res.z21, 0.6841370517901615);
        assert_similar(res.z22, 1.5988365604014116);
        assert_similar(res.z23, -6.022288391897364);
        assert_similar(res.z31, -1.633514023053113);
        assert_similar(res.z32, 2.3032285656514286);
        assert_similar(res.z33, 3.7885830019843842);
        assert_similar(res.zmol, 3.5674683899705713);
        assert_similar(res.zmos, 3.896090412268542);
    }
}
