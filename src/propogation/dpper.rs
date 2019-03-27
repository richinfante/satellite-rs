use crate::io::Satrec;
use std::f64::consts::PI;

const TWO_PI: f64 = 2.0 * PI;

pub struct DpperResult {
    pub ep: f64,
    pub inclp: f64,
    pub nodep: f64,
    pub argpp: f64,
    pub mp: f64,
}

#[derive(PartialEq, Clone)]
pub enum DpperInit {
    Y,
    N,
}

#[derive(PartialEq, Clone)]
pub enum DpperOpsMode {
    A,
    I,
    UNDEFINED,
}

pub struct DpperOptions {
    pub init: DpperInit,
    pub opsmode: DpperOpsMode,

    pub inclo: f64,
    pub ep: f64,
    pub inclp: f64,
    pub nodep: f64,
    pub argpp: f64,
    pub mp: f64,
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
    let e3 = satrec.e3;
    let ee2 = satrec.ee2;
    let peo = satrec.peo;
    let pgho = satrec.pgho;
    let pho = satrec.pho;
    let pinco = satrec.pinco;
    let plo = satrec.plo;
    let se2 = satrec.se2;
    let se3 = satrec.se3;
    let sgh2 = satrec.sgh2;
    let sgh3 = satrec.sgh3;
    let sgh4 = satrec.sgh4;
    let sh2 = satrec.sh2;
    let sh3 = satrec.sh3;
    let si2 = satrec.si2;
    let si3 = satrec.si3;
    let sl2 = satrec.sl2;
    let sl3 = satrec.sl3;
    let sl4 = satrec.sl4;
    let t = satrec.t;
    let xgh2 = satrec.xgh2;
    let xgh3 = satrec.xgh3;
    let xgh4 = satrec.xgh4;
    let xh2 = satrec.xh2;
    let xh3 = satrec.xh3;
    let xi2 = satrec.xi2;
    let xi3 = satrec.xi3;
    let xl2 = satrec.xl2;
    let xl3 = satrec.xl3;
    let xl4 = satrec.xl4;
    let zmol = satrec.zmol;
    let zmos = satrec.zmos;

    // Copy satellite attributes into local variables for convenience
    // and symmetry in writing formulae.

    let mut alfdp;
    let mut betdp;
    let cosip;
    let sinip;
    let cosop;
    let sinop;
    let dalf;
    let dbet;
    let dls;
    let mut f2;
    let mut f3;
    let mut pe;
    let mut pgh;
    let mut ph;
    let mut pinc;
    let mut pl;
    let mut sinzf;
    let mut xls;
    let xnoh;
    let mut zf;
    let mut zm;

    let init = options.init;
    let opsmode = options.opsmode;
    let mut ep = options.ep;
    let mut inclp = options.inclp;
    let mut nodep = options.nodep;
    let mut argpp = options.argpp;
    let mut mp = options.mp;

    //  ---------------------- constants -----------------------------
    const ZNS: f64 = 1.19459e-5;
    const ZES: f64 = 0.01675;
    const ZNL: f64 = 1.5835218e-4;
    const ZEL: f64 = 0.05490;

    //  --------------- calculate time varying periodics -----------
    zm = zmos + (ZNS * t);

    // be sure that the initial call has time set to zero
    if init == DpperInit::Y {
        zm = zmos;
    }

    zf = zm + (2.0 * ZES * zm.sin());
    sinzf = zf.sin();
    f2 = (0.5 * sinzf * sinzf) - 0.25;
    f3 = -0.5 * sinzf * zf.cos();

    let ses = (se2 * f2) + (se3 * f3);
    let sis = (si2 * f2) + (si3 * f3);
    let sls = (sl2 * f2) + (sl3 * f3) + (sl4 * sinzf);
    let sghs = (sgh2 * f2) + (sgh3 * f3) + (sgh4 * sinzf);
    let shs = (sh2 * f2) + (sh3 * f3);

    zm = zmol + (ZNL * t);

    if init == DpperInit::Y {
        zm = zmol;
    }

    zf = zm + (2.0 * ZEL * zm.sin());
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

#[cfg(test)]
mod test {
    use crate::io::Satrec;
    use crate::propogation::dpper::*;
    use crate::propogation::initl::*;
    use crate::tests::assert_similar;

    #[test]
    fn test_ds() {
        let satrec = Satrec {
            name: None,
            error: 0,
            satnum: "11801".to_string(),
            epochyr: 80,
            epochdays: 230.29629788,
            ndot: 4.3363644592306274e-8,
            nddot: 0.0,
            bstar: 0.014311,
            inclo: 0.8166674822761788,
            nodeo: 4.021856443150141,
            ecco: 0.7318036,
            argpo: 0.8285461931652521,
            mo: 0.1817184457298936,
            no: 0.009971131594572634,
            a: 3.817121025708788,
            alta: 5.610503933958173,
            altp: 0.023738117459404462,
            jdsatepoch: 2444468.79629788,
            isimp: 1,
            method: InitlMethod::D,
            aycof: 0.0008523715456365274,
            con41: 0.40625317982989756,
            cc1: 0.000002456092742119515,
            cc4: 0.0001111232746219671,
            cc5: 0.01667847920486393,
            d2: 0.0,
            d3: 0.0,
            d4: 0.0,
            delmo: 7.753547406313254,
            eta: 0.9956413449218073,
            argpdot: 0.0000034607723715772176,
            omgcof: 7.166665721514533e-18,
            sinmao: 0.1807199902443773,
            t: 0.0,
            t2cof: 0.0000036841391131792723,
            t3cof: 0.0,
            t4cof: 0.0,
            t5cof: 0.0,
            x1mth2: 0.5312489400567009,
            x7thm1: 2.281257419603094,
            mdot: 0.009971844927637492,
            nodedot: -0.0000035284989537160483,
            xlcof: 0.0016249664763650373,
            xmcof: -5.859015475500135e-13,
            nodecf: -1.4081043481329519e-11,
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
            dedt: 0.0,
            del1: 0.0,
            del2: 0.0,
            del3: 0.0,
            didt: 0.0,
            dmdt: 0.0,
            dnodt: 0.0,
            domdt: 0.0,
            e3: -0.00007499513323066247,
            ee2: 0.0003984687913511968,
            peo: 0.0,
            pgho: 0.0,
            pho: 0.0,
            pinco: 0.0,
            plo: 0.0,
            se2: -0.0023592253136306925,
            se3: -0.00007047176334622737,
            sgh2: -0.00018225669552040974,
            sgh3: -0.0022130294594592042,
            sgh4: -0.00006154293820943018,
            sh2: -0.0011394073362677001,
            sh3: 0.002509518064658255,
            si2: -0.00005208303756792119,
            si3: 0.0032873534354906702,
            sl2: 0.0027816840121748848,
            sl3: 0.0033383632815548628,
            sl4: 0.00025906770677081676,
            gsto: 1.265125075734467,
            xfact: 0.0,
            xgh2: 0.0001510256023997251,
            xgh3: 0.0003555337415001366,
            xgh4: -0.00003239876027006408,
            xh2: 0.00011285895673523819,
            xh3: -0.0004733943404491607,
            xi2: -0.00006414087517640146,
            xi3: -0.0005695169725370441,
            xl2: -0.0007034864707310533,
            xl3: -0.0005240671434151523,
            xl4: 0.00013638400715968452,
            xlamo: 0.0,
            zmol: 3.5674683899705713,
            zmos: 3.896090412268542,
            atime: 0.0,
            xli: 0.0,
            xni: 0.0,
            operationmode: DpperOpsMode::I,
            init: DpperInit::Y,
        };

        let opts = DpperOptions {
            inclo: 0.8166674822761788,
            init: DpperInit::Y,
            ep: 0.7318036,
            inclp: 0.8166674822761788,
            nodep: 4.021856443150141,
            argpp: 0.8285461931652521,
            mp: 0.1817184457298936,
            opsmode: DpperOpsMode::I,
        };

        let res = dpper(&satrec, opts);

        assert_eq!(res.ep, 0.7318036);
        assert_eq!(res.inclp, 0.8166674822761788);
        assert_eq!(res.nodep, 4.021856443150141);
        assert_eq!(res.argpp, 0.8285461931652521);
        assert_eq!(res.mp, 0.1817184457298936);
    }

    #[test]
    fn test_no_init() {
        let satrec = Satrec {
            name: None,
            error: 0,
            satnum: "11801".to_string(),
            epochyr: 80,
            epochdays: 230.29629788,
            ndot: 4.3363644592306274e-8,
            nddot: 0.0,
            bstar: 0.014311,
            inclo: 0.8166674822761788,
            nodeo: 4.021856443150141,
            ecco: 0.7318036,
            argpo: 0.8285461931652521,
            mo: 0.1817184457298936,
            no: 0.009971131594572634,
            a: 3.817121025708788,
            alta: 5.610503933958173,
            altp: 0.023738117459404462,
            jdsatepoch: 2444468.79629788,
            isimp: 1,
            method: InitlMethod::D,
            aycof: 0.0008518061660986528,
            con41: 0.40836674874013124,
            cc1: 0.000002456092742119515,
            cc4: 0.0001111232746219671,
            cc5: 0.01667847920486393,
            d2: 0.0,
            d3: 0.0,
            d4: 0.0,
            delmo: 7.753547406313254,
            eta: 0.9956413449218073,
            argpdot: 0.0000034607723715772176,
            omgcof: 7.166665721514533e-18,
            sinmao: 0.1807199902443773,
            t: 0.0,
            t2cof: 0.0000036841391131792723,
            t3cof: 0.0,
            t4cof: 0.0,
            t5cof: 0.0,
            x1mth2: 0.5305444170866229,
            x7thm1: 2.2861890803936395,
            mdot: 0.009971844927637492,
            nodedot: -0.0000035284989537160483,
            xlcof: 0.0016240429516640184,
            xmcof: -5.859015475500135e-13,
            nodecf: -1.4081043481329519e-11,
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
            e3: -0.00007499513323066247,
            ee2: 0.0003984687913511968,
            peo: 0.0,
            pgho: 0.0,
            pho: 0.0,
            pinco: 0.0,
            plo: 0.0,
            se2: -0.0023592253136306925,
            se3: -0.00007047176334622737,
            sgh2: -0.00018225669552040974,
            sgh3: -0.0022130294594592042,
            sgh4: -0.00006154293820943018,
            sh2: -0.0011394073362677001,
            sh3: 0.002509518064658255,
            si2: -0.00005208303756792119,
            si3: 0.0032873534354906702,
            sl2: 0.0027816840121748848,
            sl3: 0.0033383632815548628,
            sl4: 0.00025906770677081676,
            gsto: 1.265125075734467,
            xfact: 0.0,
            xgh2: 0.0001510256023997251,
            xgh3: 0.0003555337415001366,
            xgh4: -0.00003239876027006408,
            xh2: 0.00011285895673523819,
            xh3: -0.0004733943404491607,
            xi2: -0.00006414087517640146,
            xi3: -0.0005695169725370441,
            xl2: -0.0007034864707310533,
            xl3: -0.0005240671434151523,
            xl4: 0.00013638400715968452,
            xlamo: 0.0,
            zmol: 3.5674683899705713,
            zmos: 3.896090412268542,
            atime: 0.0,
            xli: 0.0,
            xni: 0.0,
            operationmode: DpperOpsMode::I,
            init: DpperInit::N,
        };

        let opts = DpperOptions {
            inclo: 0.8166674822761788,
            init: DpperInit::N,
            ep: 0.7318036,
            inclp: 0.8166674822761788,
            nodep: 4.021856443150141,
            argpp: 0.8285461931652521,
            mp: 0.18171844572989393,
            opsmode: DpperOpsMode::UNDEFINED, // TODO: satellite-js has undefined here.
        };

        let res = dpper(&satrec, opts);

        assert_eq!(res.ep, 0.7318253048776667);
        assert_eq!(res.inclp, 0.8159616103005034);
        assert_eq!(res.nodep, 4.02112614477864);
        assert_eq!(res.argpp, 0.829566024830811);
        assert_similar(res.mp, 0.1808079384635203);
    }
}
