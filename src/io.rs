use crate::constants::*;
use crate::ext;
use crate::propogation::dpper::*;
use crate::propogation::initl::*;
use crate::propogation::sgp4init;

#[derive(Clone)]
pub struct Satrec {
    pub init: DpperInit,
    pub operationmode: DpperOpsMode,
    pub error: u32,
    pub satnum: String,
    pub epochyr: i64,
    pub epochdays: f64,
    pub ndot: f64,
    pub nddot: f64,
    pub bstar: f64,
    pub inclo: f64,
    pub nodeo: f64,
    pub ecco: f64,
    pub mo: f64,
    pub no: f64,
    pub a: f64,
    pub argpo: f64,
    pub alta: f64,
    pub altp: f64,
    pub jdsatepoch: f64,

    // Near Earth Variables
    pub isimp: i64,
    pub method: InitlMethod,
    pub aycof: f64,
    pub con41: f64,
    pub cc1: f64,
    pub cc4: f64,
    pub cc5: f64,
    pub d2: f64,
    pub d3: f64,
    pub d4: f64,
    pub delmo: f64,
    pub eta: f64,
    pub argpdot: f64,
    pub omgcof: f64,
    pub sinmao: f64,
    pub t: f64,
    pub t2cof: f64,
    pub t3cof: f64,
    pub t4cof: f64,
    pub t5cof: f64,
    pub x1mth2: f64,
    pub x7thm1: f64,
    pub mdot: f64,
    pub nodedot: f64,
    pub xlcof: f64,
    pub xmcof: f64,
    pub nodecf: f64,

    // Deep space variables
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
    pub e3: f64,
    pub ee2: f64,
    pub peo: f64,
    pub pgho: f64,
    pub pho: f64,
    pub pinco: f64,
    pub plo: f64,
    pub se2: f64,
    pub se3: f64,
    pub sgh2: f64,
    pub sgh3: f64,
    pub sgh4: f64,
    pub sh2: f64,
    pub sh3: f64,
    pub si2: f64,
    pub si3: f64,
    pub sl2: f64,
    pub sl3: f64,
    pub sl4: f64,
    pub gsto: f64,
    pub xfact: f64,
    pub xgh2: f64,
    pub xgh3: f64,
    pub xgh4: f64,
    pub xh2: f64,
    pub xh3: f64,
    pub xi2: f64,
    pub xi3: f64,
    pub xl2: f64,
    pub xl3: f64,
    pub xl4: f64,
    pub xlamo: f64,
    pub zmol: f64,
    pub zmos: f64,
    pub atime: f64,
    pub xli: f64,
    pub xni: f64,
}

impl Satrec {
    pub fn zero() -> Satrec {
        Satrec {
            init: DpperInit::Y,
            operationmode: DpperOpsMode::I,
            error: 0,
            satnum: String::new(),
            epochyr: 0,
            epochdays: 0.0,
            ndot: 0.0,
            nddot: 0.0,
            bstar: 0.0,
            inclo: 0.0,
            nodeo: 0.0,
            ecco: 0.0,
            mo: 0.0,
            no: 0.0,
            a: 0.0,
            argpo: 0.0,
            alta: 0.0,
            altp: 0.0,
            jdsatepoch: 0.0,

            // Near Earth Variables
            isimp: 0,
            method: InitlMethod::N,
            aycof: 0.0,
            con41: 0.0,
            cc1: 0.0,
            cc4: 0.0,
            cc5: 0.0,
            d2: 0.0,
            d3: 0.0,
            d4: 0.0,
            delmo: 0.0,
            eta: 0.0,
            argpdot: 0.0,
            omgcof: 0.0,
            sinmao: 0.0,
            t: 0.0,
            t2cof: 0.0,
            t3cof: 0.0,
            t4cof: 0.0,
            t5cof: 0.0,
            x1mth2: 0.0,
            x7thm1: 0.0,
            mdot: 0.0,
            nodedot: 0.0,
            xlcof: 0.0,
            xmcof: 0.0,
            nodecf: 0.0,

            // Deep space variables
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
            e3: 0.0,
            ee2: 0.0,
            peo: 0.0,
            pgho: 0.0,
            pho: 0.0,
            pinco: 0.0,
            plo: 0.0,
            se2: 0.0,
            se3: 0.0,
            sgh2: 0.0,
            sgh3: 0.0,
            sgh4: 0.0,
            sh2: 0.0,
            sh3: 0.0,
            si2: 0.0,
            si3: 0.0,
            sl2: 0.0,
            sl3: 0.0,
            sl4: 0.0,
            gsto: 0.0,
            xfact: 0.0,
            xgh2: 0.0,
            xgh3: 0.0,
            xgh4: 0.0,
            xh2: 0.0,
            xh3: 0.0,
            xi2: 0.0,
            xi3: 0.0,
            xl2: 0.0,
            xl3: 0.0,
            xl4: 0.0,
            xlamo: 0.0,
            zmol: 0.0,
            zmos: 0.0,
            atime: 0.0,
            xli: 0.0,
            xni: 0.0,
        }
    }
}

pub fn twoline2satrec(str1: &str, str2: &str) -> Satrec {
    let mut satrec = parse_satrec(str1, str2);

    let opsmode = crate::propogation::dpper::DpperOpsMode::I;
    let opts = crate::propogation::sgp4init::SGP4InitOptions {
        opsmode: opsmode,
        satn: satrec.satnum.clone(),
        epoch: satrec.jdsatepoch - 2433281.5,
        xbstar: satrec.bstar,
        xecco: satrec.ecco,
        xargpo: satrec.argpo,
        xinclo: satrec.inclo,
        xmo: satrec.mo,
        xno: satrec.no,
        xnodeo: satrec.nodeo,
    };

    sgp4init::sgp4init(&mut satrec, opts);

    return satrec;
}

pub fn parse_satrec(str1: &str, str2: &str) -> Satrec {
    let tumin = tumin();

    const XPDOTP: f64 = 1440.0 / (2.0 * std::f64::consts::PI);

    let satnum = str1[2..7].trim();
    let epochyr = str1[18..20].trim().parse::<i64>().unwrap();
    let epochdays = str1[20..32].trim().parse::<f64>().unwrap();
    let ndot = str1[33..43].trim().parse::<f64>().unwrap() / (XPDOTP * 1440.0);
    let nddot_0 = str1[44..50].trim().parse::<i64>().unwrap();
    let nddot_1 = str1[50..52].to_string();
    let nddot =
        format!(".{}E{}", nddot_0, nddot_1).parse::<f64>().unwrap() / (XPDOTP * 1440.0 * 1440.0);
    let bstar_0 = str1[53..54].to_string();
    let bstar_1 = str1[54..59].trim().parse::<i64>().unwrap();
    let bstar_2 = str1[59..61].to_string();
    let bstar = format!("{}.{}E{}", bstar_0, bstar_1, bstar_2)
        .trim()
        .parse::<f64>()
        .unwrap();
  ;
    let inclo = str2[8..16].trim().parse::<f64>().unwrap() * DEG_2_RAD;
    let nodeo = str2[17..25].trim().parse::<f64>().unwrap() * DEG_2_RAD;
    let ecco = format!(".{}", str2[26..33].trim().to_string())
        .parse::<f64>()
        .unwrap();
    let argpo = str2[34..42].trim().parse::<f64>().unwrap() * DEG_2_RAD;
    let mo = str2[43..51].trim().parse::<f64>().unwrap() * DEG_2_RAD;
    let no = str2[52..63].trim().parse::<f64>().unwrap() / XPDOTP;

    let a = (no * tumin).powf(-2.0 / 3.0);

    let alta = (a * (1.0 + ecco)) - 1.0;
    let altp = (a * (1.0 - ecco)) - 1.0;

    let year;
    if epochyr < 57 {
        year = epochyr + 2000;
    } else {
        year = epochyr + 1900;
    }

    let jd = ext::days2mdhms(year as u64, epochdays);

    let jdsatepoch = ext::jday(
        year as f64,
        jd.month,
        jd.day,
        jd.hour,
        jd.minute,
        jd.second,
        0.0,
    );

    let mut satrec = Satrec::zero();

    satrec.operationmode = DpperOpsMode::I;
    satrec.satnum = satnum.to_string();
    satrec.epochyr = epochyr;
    satrec.epochdays = epochdays;
    satrec.ndot = ndot;
    satrec.nddot = nddot;
    satrec.bstar = bstar;
    satrec.inclo = inclo;
    satrec.nodeo = nodeo;
    satrec.ecco = ecco;
    satrec.argpo = argpo;
    satrec.mo = mo;
    satrec.no = no;
    satrec.a = a;
    satrec.alta = alta;
    satrec.altp = altp;
    satrec.jdsatepoch = jdsatepoch;

    return satrec;
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_parse() {
        let satrec = crate::io::parse_satrec(
            "1 88888U          80275.98708465  .00073094  13844-3  66816-4 0    8",
            "2 88888  72.8435 115.9689 0086731  52.6988 110.5714 16.05824518  105",
        );

        assert_eq!(satrec.error, 0);
        assert_eq!(satrec.satnum, "88888");
        assert_eq!(satrec.epochyr, 80);
        assert_eq!(satrec.epochdays, 275.98708465);
        assert_eq!(satrec.ndot, 2.2148107004387767e-9);
        assert_eq!(satrec.nddot, 2.913090538750181e-13);
        assert_eq!(satrec.bstar, 0.000066816);
        assert_eq!(satrec.inclo, 1.2713589136764896);
        assert_eq!(satrec.nodeo, 2.0240391349160523);
        assert_eq!(satrec.ecco, 0.0086731);
        assert_eq!(satrec.argpo, 0.9197675718499877);
        assert_eq!(satrec.mo, 1.929834988539658);
        assert_eq!(satrec.no, 0.07006731262087737);
        assert_eq!(satrec.a, 1.0405013051291292);
        assert_eq!(satrec.alta, 0.04952567699864474);
        assert_eq!(satrec.altp, 0.03147693325961365);
        assert_eq!(satrec.jdsatepoch, 2444514.48708465);
    }

    #[test]
    fn test_init_tle() {
        let satrec = crate::io::twoline2satrec(
            "1 88888U          80275.98708465  .00073094  13844-3  66816-4 0    8",
            "2 88888  72.8435 115.9689 0086731  52.6988 110.5714 16.05824518  105",
        );

        assert_eq!(satrec.error, 0);
        assert_eq!(satrec.satnum, "88888");
        assert_eq!(satrec.epochyr, 80);
        assert_eq!(satrec.epochdays, 275.98708465);
        assert_eq!(satrec.ndot, 2.2148107004387767e-9);
        assert_eq!(satrec.nddot, 2.913090538750181e-13);
        assert_eq!(satrec.bstar, 0.000066816);
        assert_eq!(satrec.inclo, 1.2713589136764896);
        assert_eq!(satrec.nodeo, 2.0240391349160523);
        assert_eq!(satrec.ecco, 0.0086731);
        assert_eq!(satrec.argpo, 0.9197675718499877);
        assert_eq!(satrec.mo, 1.929834988539658);
        assert_eq!(satrec.no, 0.07010615621239219);
        assert_eq!(satrec.a, 1.0405013051291292);
        assert_eq!(satrec.alta, 0.04952567699864474);
        assert_eq!(satrec.altp, 0.03147693325961365);
        assert_eq!(satrec.jdsatepoch, 2444514.48708465);
        assert_eq!(satrec.isimp, 1);
        assert_eq!(satrec.method, crate::propogation::initl::InitlMethod::N);
        assert_eq!(satrec.aycof, 0.001117407997657797);

        assert_eq!(satrec.con41, -0.7389556198424165);
        assert_eq!(satrec.cc1, 2.3340379369349495e-8);
        assert_eq!(satrec.cc4, 0.00037724513079719584);
        assert_eq!(satrec.cc5, 0.01233625966048993);
        assert_eq!(satrec.d2, 0.0);
        assert_eq!(satrec.d3, 0.0);
        assert_eq!(satrec.d4, 0.0);
        assert_eq!(satrec.delmo, 0.6963031736886937);
        assert_eq!(satrec.eta, 0.32347784078169217);
        assert_eq!(satrec.argpdot, -0.000029718644394179532);
        assert!((satrec.omgcof - 1.6306928260750368e-7).abs() < 1e-15);
        assert_eq!(satrec.sinmao, 0.9362350458581234);
        assert_eq!(satrec.t, 0.0);
        assert_eq!(satrec.t2cof, 3.5010569054024244e-8);
        assert_eq!(satrec.t3cof, 0.0);
        assert_eq!(satrec.t4cof, 0.0);
        assert_eq!(satrec.t5cof, 0.0);
        assert_eq!(satrec.x1mth2, 0.9129852066141388);
        assert_eq!(satrec.x7thm1, -0.3908964462989719);
        assert_eq!(satrec.mdot, 0.07006729343154267);
        assert_eq!(satrec.nodedot, -0.00003096533062994484);
        assert_eq!(satrec.xlcof, 0.0019306451483792333);
        assert_eq!(satrec.xmcof, -0.0000493564796620572);
        assert_eq!(satrec.nodecf, -2.5361112971222384e-12);
        assert_eq!(satrec.irez, 0.0);
        assert_eq!(satrec.d2201, 0.0);
        assert_eq!(satrec.d2211, 0.0);
        assert_eq!(satrec.d3210, 0.0);
        assert_eq!(satrec.d3222, 0.0);
        assert_eq!(satrec.d4410, 0.0);
        assert_eq!(satrec.d4422, 0.0);
        assert_eq!(satrec.d5220, 0.0);
        assert_eq!(satrec.d5232, 0.0);
        assert_eq!(satrec.d5421, 0.0);
        assert_eq!(satrec.d5433, 0.0);
        assert_eq!(satrec.dedt, 0.0);
        assert_eq!(satrec.del1, 0.0);
        assert_eq!(satrec.del2, 0.0);
        assert_eq!(satrec.del3, 0.0);
        assert_eq!(satrec.didt, 0.0);
        assert_eq!(satrec.dmdt, 0.0);
        assert_eq!(satrec.dnodt, 0.0);
        assert_eq!(satrec.domdt, 0.0);
        assert_eq!(satrec.e3, 0.0);
        assert_eq!(satrec.ee2, 0.0);
        assert_eq!(satrec.peo, 0.0);
        assert_eq!(satrec.pgho, 0.0);
        assert_eq!(satrec.pho, 0.0);
        assert_eq!(satrec.pinco, 0.0);
        assert_eq!(satrec.plo, 0.0);
        assert_eq!(satrec.se2, 0.0);
        assert_eq!(satrec.se3, 0.0);
        assert_eq!(satrec.sgh2, 0.0);
        assert_eq!(satrec.sgh3, 0.0);
        assert_eq!(satrec.sgh4, 0.0);
        assert_eq!(satrec.sh2, 0.0);
        assert_eq!(satrec.sh3, 0.0);
        assert_eq!(satrec.si2, 0.0);
        assert_eq!(satrec.si3, 0.0);
        assert_eq!(satrec.sl2, 0.0);
        assert_eq!(satrec.sl3, 0.0);
        assert_eq!(satrec.sl4, 0.0);
        assert_eq!(satrec.gsto, 0.1082901416688955);
        assert_eq!(satrec.xfact, 0.0);
        assert_eq!(satrec.xgh2, 0.0);
        assert_eq!(satrec.xgh3, 0.0);
        assert_eq!(satrec.xgh4, 0.0);
        assert_eq!(satrec.xh2, 0.0);
        assert_eq!(satrec.xh3, 0.0);
        assert_eq!(satrec.xi2, 0.0);
        assert_eq!(satrec.xi3, 0.0);
        assert_eq!(satrec.xl2, 0.0);
        assert_eq!(satrec.xl3, 0.0);
        assert_eq!(satrec.xl4, 0.0);
        assert_eq!(satrec.xlamo, 0.0);
        assert_eq!(satrec.zmol, 0.0);
        assert_eq!(satrec.zmos, 0.0);
        assert_eq!(satrec.atime, 0.0);
        assert_eq!(satrec.xli, 0.0);
        assert_eq!(satrec.xni, 0.0);

        /*


          { error: 0,
        satnum: '88888',
        epochyr: 80,
        epochdays: 275.98708465,
        ndot: 2.2148107004387767e-9,
        nddot: 2.913090538750181e-13,
        bstar: 0.000066816,
        inclo: 1.2713589136764896,
        nodeo: 2.0240391349160523,
        ecco: 0.0086731,
        argpo: 0.9197675718499877,
        mo: 1.929834988539658,
        no: 0.07010615621239219,
        a: 1.0405013051291292,
        alta: 0.04952567699864474,
        altp: 0.03147693325961365,
        jdsatepoch: 2444514.48708465,
        isimp: 1,
        method: 'n',
        aycof: 0.001117407997657797,
        con41: -0.7389556198424165,
        cc1: 2.3340379369349495e-8,
        cc4: 0.00037724513079719584,
        cc5: 0.01233625966048993,
        d2: 0,
        d3: 0,
        d4: 0,
        delmo: 0.6963031736886937,
        eta: 0.32347784078169217,
        argpdot: -0.000029718644394179532,
        omgcof: 1.6306928260750368e-7,
        sinmao: 0.9362350458581234,
        t: 0,
        t2cof: 3.5010569054024244e-8,
        t3cof: 0,
        t4cof: 0,
        t5cof: 0,
        x1mth2: 0.9129852066141388,
        x7thm1: -0.3908964462989719,
        mdot: 0.07006729343154267,
        nodedot: -0.00003096533062994484,
        xlcof: 0.0019306451483792333,
        xmcof: -0.0000493564796620572,
        nodecf: -2.5361112971222384e-12,
        irez: 0,
        d2201: 0,
        d2211: 0,
        d3210: 0,
        d3222: 0,
        d4410: 0,
        d4422: 0,
        d5220: 0,
        d5232: 0,
        d5421: 0,
        d5433: 0,
        dedt: 0,
        del1: 0,
        del2: 0,
        del3: 0,
        didt: 0,
        dmdt: 0,
        dnodt: 0,
        domdt: 0,
        e3: 0,
        ee2: 0,
        peo: 0,
        pgho: 0,
        pho: 0,
        pinco: 0,
        plo: 0,
        se2: 0,
        se3: 0,
        sgh2: 0,
        sgh3: 0,
        sgh4: 0,
        sh2: 0,
        sh3: 0,
        si2: 0,
        si3: 0,
        sl2: 0,
        sl3: 0,
        sl4: 0,
        gsto: 0.1082901416688955,
        xfact: 0,
        xgh2: 0,
        xgh3: 0,
        xgh4: 0,
        xh2: 0,
        xh3: 0,
        xi2: 0,
        xi3: 0,
        xl2: 0,
        xl3: 0,
        xl4: 0,
        xlamo: 0,
        zmol: 0,
        zmos: 0,
        atime: 0,
        xli: 0,
        xni: 0,
        operationmode: 'i',
        init: 'n' }
        */
    }
}
