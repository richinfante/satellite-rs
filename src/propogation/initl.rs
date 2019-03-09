use crate::constants::*;
use crate::propogation::dpper::DpperOpsMode;
use crate::propogation::gstime;
/*-----------------------------------------------------------------------------
*
*                           procedure initl
*
*  this procedure initializes the sgp4 propagator. all the initialization is
*    consolidated here instead of having multiple loops inside other routines.
*
*  author        : david vallado                  719-573-2600   28 jun 2005
*
*  inputs        :
*    ecco        - eccentricity                           0.0 - 1.0
*    epoch       - epoch time in days from jan 0, 1950. 0 hr
*    inclo       - inclination of satellite
*    no          - mean motion of satellite
*    satn        - satellite number
*
*  outputs       :
*    ainv        - 1.0 / a
*    ao          - semi major axis
*    con41       -
*    con42       - 1.0 - 5.0 cos(i)
*    cosio       - cosine of inclination
*    cosio2      - cosio squared
*    eccsq       - eccentricity squared
*    method      - flag for deep space                    'd', 'n'
*    omeosq      - 1.0 - ecco * ecco
*    posq        - semi-parameter squared
*    rp          - radius of perigee
*    rteosq      - square root of (1.0 - ecco*ecco)
*    sinio       - sine of inclination
*    gsto        - gst at time of observation               rad
*    no          - mean motion of satellite
*
*  locals        :
*    ak          -
*    d1          -
*    del         -
*    adel        -
*    po          -
*
*  coupling      :
*    getgravconst
*    gstime      - find greenwich sidereal time from the julian date
*
*  references    :
*    hoots, roehrich, norad spacetrack report #3 1980
*    hoots, norad spacetrack report #6 1986
*    hoots, schumacher and glover 2004
*    vallado, crawford, hujsak, kelso  2006
----------------------------------------------------------------------------*/

pub struct InitlOptions {
    pub opsmode: DpperOpsMode,
    pub method: InitlMethod,

    pub satn: String,
    pub ecco: f64,
    pub epoch: f64,
    pub inclo: f64,
    pub no: f64,
}

#[derive(PartialEq, Debug)]
pub struct InitlReturn {
    pub no: f64,
    pub method: InitlMethod,
    pub ainv: f64,
    pub ao: f64,
    pub con41: f64,
    pub con42: f64,
    pub cosio: f64,
    pub cosio2: f64,
    pub eccsq: f64,
    pub omeosq: f64,
    pub posq: f64,
    pub rp: f64,
    pub rteosq: f64,
    pub sinio: f64,
    pub gsto: f64,
}

#[derive(PartialEq, Clone, Debug)]
pub enum InitlMethod {
    N,
    D,
}

pub fn initl(options: InitlOptions) -> InitlReturn {
    let InitlOptions {
        ecco,
        epoch,
        inclo,
        opsmode,
        ..
    } = options;

    let InitlOptions { mut no, .. } = options;

    // sgp4fix use old way of finding gst
    // ----------------------- earth constants ---------------------
    // sgp4fix identify constants and allow alternate values

    // ------------- calculate auxillary epoch quantities ----------
    let eccsq = ecco * ecco;
    let omeosq = 1.0 - eccsq;
    let rteosq = omeosq.sqrt();
    let cosio = inclo.cos();
    let cosio2 = cosio * cosio;

    // assert_eq!(eccsq, 0.00007522266360999999);
    // assert_eq!(omeosq, 0.99992477733639);
    // assert_eq!(rteosq, 0.9999623879608622);
    // assert_eq!(cosio, 0.2949827001467394);
    // assert_eq!(cosio2, 0.08701479338586116);
    // assert_eq!(X2O3, 0.6666666666666666);

    // ------------------ un-kozai the mean motion -----------------
    let ak = (xke() / no).powf(X2O3);
    let d1 = (0.75 * J2 * ((3.0 * cosio2) - 1.0)) / (rteosq * omeosq);
    let mut delPrime = d1 / (ak * ak);
    let adel = ak
        * (1.0
            - (delPrime * delPrime)
            - (delPrime * ((1.0 / 3.0) + ((134.0 * delPrime * delPrime) / 81.0))));
    delPrime = d1 / (adel * adel);
    no /= 1.0 + delPrime;

    let ao = (xke() / no).powf(X2O3);
    let sinio = inclo.sin();
    let po = ao * omeosq;
    let con42 = 1.0 - (5.0 * cosio2);
    let con41 = -con42 - cosio2 - cosio2;
    let ainv = 1.0 / ao;
    let posq = po * po;
    let rp = ao * (1.0 - ecco);
    let method = InitlMethod::N;

    //  sgp4fix modern approach to finding sidereal time
    let mut gsto;
    if opsmode == DpperOpsMode::A {
        //  sgp4fix use old way of finding gst
        //  count integer number of days from 0 jan 1970
        let ts70 = epoch - 7305.0;
        let ds70 = (ts70 + 1.0e-8).floor();
        let tfrac = ts70 - ds70;

        //  find greenwich location at epoch
        let c1 = 1.72027916940703639e-2;
        let thgr70 = 1.7321343856509374;
        let fk5r = 5.07551419432269442e-15;
        let c1p2p = c1 + TWO_PI;
        gsto = (thgr70 + (c1 * ds70) + (c1p2p * tfrac) + (ts70 * ts70 * fk5r)) % TWO_PI;
        if gsto < 0.0 {
            gsto += TWO_PI;
        }
    } else {
        gsto = gstime::gstime(epoch + 2433281.5);
    }

    InitlReturn {
        no,

        method,

        ainv,
        ao,
        con41,
        con42,
        cosio,

        cosio2,
        eccsq,
        omeosq,
        posq,

        rp,
        rteosq,
        sinio,
        gsto,
    }
}

#[cfg(test)]
mod test {
    use crate::propogation::dpper::*;
    use crate::propogation::initl::*;

    #[test]
    fn test_initl() {
        let opts = InitlOptions {
            satn: "88888".to_string(),
            ecco: 0.0086731,
            epoch: 11232.987084649969,
            inclo: 1.2713589136764896,
            no: 0.07006731262087737,
            method: InitlMethod::N,
            opsmode: DpperOpsMode::I,
        };

        let result = initl(opts);

        assert_eq!(result.no, 0.07010615621239219);
        assert_eq!(result.method, InitlMethod::N);
        assert_eq!(result.ainv, 0.9614303648645832);
        assert_eq!(result.ao, 1.0401169305078577);
        assert_eq!(result.con41, -0.7389556198424165);
        assert_eq!(result.con42, 0.5649260330706942);
        assert_eq!(result.cosio, 0.2949827001467394);
        assert_eq!(result.cosio2, 0.08701479338586116);
        assert_eq!(result.eccsq, 0.00007522266360999999);
        assert_eq!(result.omeosq, 0.99992477733639);
        assert_eq!(result.posq, 1.0816804769920354);
        assert_eq!(result.rp, 1.03109589235787);
        assert_eq!(result.rteosq, 0.9999623879608622);
        assert_eq!(result.sinio, 0.9555025937244435);
        assert_eq!(result.gsto, 0.1082901416688955);
    }
}
