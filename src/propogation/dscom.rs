use crate::constants::*;

pub struct DscomOptions {
  pub epoch : f64,
  pub ep : f64,
  pub argpp : f64,
  pub tc : f64,
  pub inclp : f64,
  pub nodep : f64,
  pub np : f64,
}

pub struct DscomResult {
  pub snodm : f64,
  pub cnodm : f64,
  pub sinim : f64,
  pub cosim : f64,
  pub sinomm : f64,
  pub cosomm : f64,
  pub day : f64,
  pub e3 : f64,
  pub ee2 : f64,
  pub em : f64,
  pub emsq : f64,
  pub gam : f64,
  pub peo : f64,
  pub pgho : f64,
  pub pho : f64,
  pub pinco : f64,
  pub plo : f64,
  pub rtemsq : f64,
  pub se2 : f64,
  pub se3 : f64,
  pub sgh2 : f64,
  pub sgh3 : f64,
  pub sgh4 : f64,
  pub sh2 : f64,
  pub sh3 : f64,
  pub si2 : f64,
  pub si3 : f64,
  pub sl2 : f64,
  pub sl3 : f64,
  pub sl4 : f64,
  pub s1 : f64,
  pub s2 : f64,
  pub s3 : f64,
  pub s4 : f64,
  pub s5 : f64,
  pub s6 : f64,
  pub s7 : f64,
  pub ss1 : f64,
  pub ss2 : f64,
  pub ss3 : f64,
  pub ss4 : f64,
  pub ss5 : f64,
  pub ss6 : f64,
  pub ss7 : f64,
  pub sz1 : f64,
  pub sz2 : f64,
  pub sz3 : f64,
  pub sz11 : f64,
  pub sz12 : f64,
  pub sz13 : f64,
  pub sz21 : f64,
  pub sz22 : f64,
  pub sz23 : f64,
  pub sz31 : f64,
  pub sz32 : f64,
  pub sz33 : f64,
  pub xgh2 : f64,
  pub xgh3 : f64,
  pub xgh4 : f64,
  pub xh2 : f64,
  pub xh3 : f64,
  pub xi2 : f64,
  pub xi3 : f64,
  pub xl2 : f64,
  pub xl3 : f64,
  pub xl4 : f64,
  pub nm : f64,
  pub z1 : f64,
  pub z2 : f64,
  pub z3 : f64,
  pub z11 : f64,
  pub z12 : f64,
  pub z13 : f64,
  pub z21 : f64,
  pub z22 : f64,
  pub z23 : f64,
  pub z31 : f64,
  pub z32 : f64,
  pub z33 : f64,
  pub zmol : f64,
  pub zmos : f64
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
  const zes : f64 = 0.01675;
  const zel : f64 = 0.05490;
  const c1ss : f64 = 2.9864797e-6;
  const c1l : f64 = 4.7968065e-7;
  const zsinis : f64 = 0.39785416;
  const zcosis : f64 = 0.91744867;
  const zcosgs : f64 = 0.1945905;
  const zsings : f64 = -0.98088458;

  //  --------------------- local variables ------------------------
  let nm = options.np;
  let em = options.ep;
  let snodm = options.nodep.sin();
  let cnodm = options.nodep.cos();
  let sinomm = options.argpp.sin();
  let cosomm =options.argpp.cos();
  let sinim = options.inclp.sin();
  let cosim =options.inclp.cos();
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
  zcosg = zcosgs;
  zsing = zsings;
  zcosi = zcosis;
  zsini = zsinis;
  zcosh = cnodm;
  zsinh = snodm;
  cc = c1ss;
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

    z11 = (-6.0 * a1 * a5)
      + (emsq * ((-24.0 * x1 * x7) - (6.0 * x3 * x5)));
    z12 = (-6.0 * ((a1 * a6) + (a3 * a5)))
      + (emsq * ((-24.0 * ((x2 * x7) + (x1 * x8))) + (-6.0 * ((x3 * x6) + (x4 * x5)))));

    z13 = (-6.0 * a3 * a6)
      + (emsq * ((-24.0 * x2 * x8) - (6.0 * x4 * x6)));

    z21 = (6.0 * a2 * a5)
      + (emsq * ((24.0 * x1 * x5) - (6.0 * x3 * x7)));
    z22 = (6.0 * ((a4 * a5) + (a2 * a6)))
      + (emsq * ((24.0 * ((x2 * x5) + (x1 * x6))) - (6.0 * ((x4 * x7) + (x3 * x8)))));
    z23 = (6.0 * a4 * a6)
      + (emsq * ((24.0 * x2 * x6) - (6.0 * x4 * x8)));

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
      cc = c1l;
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
  let sl4 = -2.0 * ss3 * (-21.0 - (9.0 * emsq)) * zes;
  let sgh2 = 2.0 * ss4 * sz32;
  let sgh3 = 2.0 * ss4 * (sz33 - sz31);
  let sgh4 = -18.0 * ss4 * zes;
  let sh2 = -2.0 * ss2 * sz22;
  let sh3 = -2.0 * ss2 * (sz23 - sz21);

  //  ------------------------ do lunar terms ----------------------
  let ee2 = 2.0 * s1 * s6;
  let e3 = 2.0 * s1 * s7;
  let xi2 = 2.0 * s2 * z12;
  let xi3 = 2.0 * s2 * (z13 - z11);
  let xl2 = -2.0 * s3 * z2;
  let xl3 = -2.0 * s3 * (z3 - z1);
  let xl4 = -2.0 * s3 * (-21.0 - (9.0 * emsq)) * zel;
  let xgh2 = 2.0 * s4 * z32;
  let xgh3 = 2.0 * s4 * (z33 - z31);
  let xgh4 = -18.0 * s4 * zel;
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