use crate::io::Satrec;
use std::f64::consts::PI;

const TWO_PI : f64 = 2.0 * PI;

pub struct DpperResult {
  pub ep : f64,
  pub inclp : f64,
  pub nodep : f64,
  pub argpp : f64,
  pub mp : f64
}


#[derive(PartialEq, Clone)]
pub enum DpperInit {
  Y, N
}

#[derive(PartialEq, Clone)]
pub enum DpperOpsMode {
  A, I
}

pub struct DpperOptions {
  pub init: DpperInit,
  pub opsmode: DpperOpsMode,
  
  pub inclo: f64,
  pub ep : f64,
  pub inclp : f64,
  pub nodep : f64,
  pub argpp : f64,
  pub mp : f64
}

/* -----------------------------------------------------------------------------
 *
 *                           procedure dpper
 *
 *  this procedure provides deep space long period periodic contributions
 *    to the mean elements.  by design, these periodics are zero at epoch.
 *    this used to be dscom which included initialization, but it's really a
 *    recurring function.
 *
 *  author        : david vallado                  719-573-2600   28 jun 2005
 *
 *  inputs        :
 *    e3          -
 *    ee2         -
 *    peo         -
 *    pgho        -
 *    pho         -
 *    pinco       -
 *    plo         -
 *    se2 , se3 , sgh2, sgh3, sgh4, sh2, sh3, si2, si3, sl2, sl3, sl4 -
 *    t           -
 *    xh2, xh3, xi2, xi3, xl2, xl3, xl4 -
 *    zmol        -
 *    zmos        -
 *    ep          - eccentricity                           0.0 - 1.0
 *    inclo       - inclination - needed for lyddane modification
 *    nodep       - right ascension of ascending node
 *    argpp       - argument of perigee
 *    mp          - mean anomaly
 *
 *  outputs       :
 *    ep          - eccentricity                           0.0 - 1.0
 *    inclp       - inclination
 *    nodep        - right ascension of ascending node
 *    argpp       - argument of perigee
 *    mp          - mean anomaly
 *
 *  locals        :
 *    alfdp       -
 *    betdp       -
 *    cosip  , sinip  , cosop  , sinop  ,
 *    dalf        -
 *    dbet        -
 *    dls         -
 *    f2, f3      -
 *    pe          -
 *    pgh         -
 *    ph          -
 *    pinc        -
 *    pl          -
 *    sel   , ses   , sghl  , sghs  , shl   , shs   , sil   , sinzf , sis   ,
 *    sll   , sls
 *    xls         -
 *    xnoh        -
 *    zf          -
 *    zm          -
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
pub fn dpper(satrec: &Satrec, options: DpperOptions) -> DpperResult {
  let mut e3 = satrec.e3;
  let mut ee2 = satrec.ee2;
  let mut peo = satrec.peo;
  let mut pgho = satrec.pgho;
  let mut pho = satrec.pho;
  let mut pinco = satrec.pinco;
  let mut plo = satrec.plo;
  let mut se2 = satrec.se2;
  let mut se3 = satrec.se3;
  let mut sgh2 = satrec.sgh2;
  let mut sgh3 = satrec.sgh3;
  let mut sgh4 = satrec.sgh4;
  let mut sh2 = satrec.sh2;
  let mut sh3 = satrec.sh3;
  let mut si2 = satrec.si2;
  let mut si3 = satrec.si3;
  let mut sl2 = satrec.sl2;
  let mut sl3 = satrec.sl3;
  let mut sl4 = satrec.sl4;
  let mut t = satrec.t;
  let mut xgh2 = satrec.xgh2;
  let mut xgh3 = satrec.xgh3;
  let mut xgh4 = satrec.xgh4;
  let mut xh2 = satrec.xh2;
  let mut xh3 = satrec.xh3;
  let mut xi2 = satrec.xi2;
  let mut xi3 = satrec.xi3;
  let mut xl2 = satrec.xl2;
  let mut xl3 = satrec.xl3;
  let mut xl4 = satrec.xl4;
  let mut zmol = satrec.zmol;
  let mut zmos = satrec.zmos;

  // Copy satellite attributes into local variables for convenience
  // and symmetry in writing formulae.

  let mut alfdp;
  let mut betdp;
  let mut cosip;
  let mut sinip;
  let mut cosop;
  let mut sinop;
  let mut dalf;
  let mut dbet;
  let mut dls;
  let mut f2;
  let mut f3;
  let mut pe;
  let mut pgh;
  let mut ph;
  let mut pinc;
  let mut pl;
  let mut sinzf;
  let mut xls;
  let mut xnoh;
  let mut zf;
  let mut zm;

  let mut init = options.init;
  let mut opsmode = options.opsmode;
  let mut ep = options.ep;
  let mut inclp = options.inclp;
  let mut nodep = options.nodep;
  let mut argpp = options.argpp;
  let mut mp = options.mp;

  //  ---------------------- constants -----------------------------
  const zns : f64 = 1.19459e-5;
  const zes : f64 = 0.01675;
  const znl : f64 = 1.5835218e-4;
  const zel : f64 = 0.05490;

  //  --------------- calculate time varying periodics -----------
  zm = zmos + (zns * t);

  // be sure that the initial call has time set to zero
  if init == DpperInit::Y {
    zm = zmos;
  }

  zf = zm + (2.0 * zes * zm.sin());
  sinzf = zf.sin();
  f2 = (0.5 * sinzf * sinzf) - 0.25;
  f3 = -0.5 * sinzf * zf.cos();

  let ses = (se2 * f2) + (se3 * f3);
  let sis = (si2 * f2) + (si3 * f3);
  let sls = (sl2 * f2) + (sl3 * f3) + (sl4 * sinzf);
  let sghs = (sgh2 * f2) + (sgh3 * f3) + (sgh4 * sinzf);
  let shs = (sh2 * f2) + (sh3 * f3);

  zm = zmol + (znl * t);

  if init == DpperInit::Y {
    zm = zmol;
  }

  zf = zm + (2.0 * zel * zm.sin());
  sinzf = zf.sin();
  f2 = (0.5 * sinzf * sinzf) - 0.25;
  f3 = -0.5 * sinzf * zf.cos();

  let sel = (ee2 * f2) + (e3 * f3);
  let sil = (xi2 * f2) + (xi3 * f3);
  let sll = (xl2 * f2) + (xl3 * f3) + (xl4 * sinzf);
  let sghl = (xgh2 * f2) + (xgh3 * f3) + (xgh4 * sinzf);
  let shll = (xh2 * f2) + (xh3 * f3);

  pe = ses + sel;
  pinc = sis + sil;
  pl = sls + sll;
  pgh = sghs + sghl;
  ph = shs + shll;

  if init == DpperInit::N {
    pe -= peo;
    pinc -= pinco;
    pl -= plo;
    pgh -= pgho;
    ph -= pho;
    inclp += pinc;
    ep += pe;
    sinip = inclp.sin();
    cosip = inclp.cos();

    /* ----------------- apply periodics directly ------------ */
    // sgp4fix for lyddane choice
    // strn3 used original inclination - this is technically feasible
    // gsfc used perturbed inclination - also technically feasible
    // probably best to readjust the 0.2 limit value and limit discontinuity
    // 0.2 rad = 11.45916 deg
    // use next line for original strn3 approach and original inclination
    // if (inclo >= 0.2)
    // use next line for gsfc version and perturbed inclination
    if inclp >= 0.2 {
      ph /= sinip;
      pgh -= cosip * ph;
      argpp += pgh;
      nodep += ph;
      mp += pl;
    } else {
      //  ---- apply periodics with lyddane modification ----
      sinop = nodep.sin();
      cosop = nodep.cos();
      alfdp = sinip * sinop;
      betdp = sinip * cosop;
      dalf = (ph * cosop) + (pinc * cosip * sinop);
      dbet = (-ph * sinop) + (pinc * cosip * cosop);
      alfdp += dalf;
      betdp += dbet;
      nodep %= TWO_PI;

      //  sgp4fix for afspc written intrinsic functions
      //  nodep used without a trigonometric function ahead
      if nodep < 0.0 && opsmode == DpperOpsMode::A {
        nodep += TWO_PI;
      }
      xls = mp + argpp + (cosip * nodep);
      dls = (pl + pgh) - (pinc * nodep * sinip);
      xls += dls;
      xnoh = nodep;
      nodep = alfdp.atan2(betdp);

      //  sgp4fix for afspc written intrinsic functions
      //  nodep used without a trigonometric function ahead
      if nodep < 0.0 && opsmode == DpperOpsMode::A {
        nodep += TWO_PI;
      }
      if (xnoh - nodep).abs() > std::f64::consts::PI {
        if nodep < xnoh {
          nodep += TWO_PI;
        } else {
          nodep -= TWO_PI;
        }
      }
      mp += pl;
      argpp = xls - mp - (cosip * nodep);
    }
  }

  DpperResult {
    ep,
    inclp,
    nodep,
    argpp,
    mp,
  }
}