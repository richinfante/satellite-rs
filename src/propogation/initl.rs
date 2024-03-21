use crate::constants::*;
use crate::propogation::dpper::DpperOpsMode;
use crate::propogation::gstime;
use crate::*;
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
    pub ecco: Float,
    pub epoch: Float,
    pub inclo: Float,
    pub no: Float,
}

#[derive(PartialEq, Debug)]
pub struct InitlReturn {
    pub no: Float,
    pub method: InitlMethod,
    pub ainv: Float,
    pub ao: Float,
    pub con41: Float,
    pub con42: Float,
    pub cosio: Float,
    pub cosio2: Float,
    pub eccsq: Float,
    pub omeosq: Float,
    pub posq: Float,
    pub rp: Float,
    pub rteosq: Float,
    pub sinio: Float,
    pub gsto: Float,
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

    // ------------------ un-kozai the mean motion -----------------
    let ak = (XKE / no).powf(X2O3);
    let d1 = (0.75 * J2 * ((3.0 * cosio2) - 1.0)) / (rteosq * omeosq);
    let mut del_prime = d1 / (ak * ak);
    let adel = ak
        * (1.0
            - (del_prime * del_prime)
            - (del_prime * ((1.0 / 3.0) + ((134.0 * del_prime * del_prime) / 81.0))));
    del_prime = d1 / (adel * adel);
    no /= 1.0 + del_prime;

    let ao = (XKE / no).powf(X2O3);
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
    use crate::tests::assert_similar;

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

        assert_similar(result.no, 0.07010615621239219);
        assert_eq!(result.method, InitlMethod::N);
        assert_similar(result.ainv, 0.9614303648645832);
        assert_similar(result.ao, 1.0401169305078577);
        assert_similar(result.con41, -0.7389556198424165);
        assert_similar(result.con42, 0.5649260330706942);
        assert_similar(result.cosio, 0.2949827001467394);
        assert_similar(result.cosio2, 0.08701479338586116);
        assert_similar(result.eccsq, 0.00007522266360999999);
        assert_similar(result.omeosq, 0.99992477733639);
        assert_similar(result.posq, 1.0816804769920354);
        assert_similar(result.rp, 1.03109589235787);
        assert_similar(result.rteosq, 0.9999623879608622);
        assert_similar(result.sinio, 0.9555025937244435);
        assert_similar(result.gsto, 0.1082901416688955);
    }

    #[test]
    fn test_initl_ds() {
        let opts = InitlOptions {
            satn: "11801".to_string(),
            ecco: 0.7318036,
            epoch: 11187.29629787989,
            inclo: 0.8166674822761788,
            no: 0.009971844782555844,
            method: InitlMethod::N,
            opsmode: DpperOpsMode::I,
        };

        let res = initl(opts);
        assert_similar(res.no, 0.009971131594572634);
        assert_eq!(res.method, InitlMethod::N);
        assert_similar(res.ainv, 0.2619650549686258);
        assert_similar(res.ao, 3.8173030373068833);
        assert_similar(res.con41, 0.40625317982989756);
        assert_similar(res.con42, -1.3437552997164959);
        assert_similar(res.cosio, 0.6846539709541596);
        assert_similar(res.cosio2, 0.4687510599432992);
        assert_similar(res.eccsq, 0.53553650897296);
        assert_similar(res.omeosq, 0.46446349102704);
        assert_similar(res.posq, 3.143521535730025);
        assert_similar(res.rp, 1.0237869323147717);
        assert_similar(res.rteosq, 0.6815155838475302);
        assert_similar(res.sinio, 0.7288682597401953);
        assert_similar(res.gsto, 1.265125075734467);
    }
}
