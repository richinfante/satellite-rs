use crate::constants::*;
use crate::propogation::dpper::*;
use crate::propogation::initl::*;
use crate::propogation::sgp4::*;
use crate::propogation::sgp4init;
use crate::*;
#[derive(Clone)]
pub struct Satrec {
    pub name: Option<String>,
    pub init: DpperInit,
    pub operationmode: DpperOpsMode,
    pub error: u32,
    pub satnum: String,
    pub epochyr: i64,
    pub epochdays: Float,
    pub ndot: Float,
    pub nddot: Float,
    pub bstar: Float,
    pub inclo: Float,
    pub nodeo: Float,
    pub ecco: Float,
    pub mo: Float,
    pub no: Float,
    pub a: Float,
    pub argpo: Float,
    pub alta: Float,
    pub altp: Float,
    pub jdsatepoch: Float,

    // Near Earth Variables
    pub isimp: i64,
    pub method: InitlMethod,
    pub aycof: Float,
    pub con41: Float,
    pub cc1: Float,
    pub cc4: Float,
    pub cc5: Float,
    pub d2: Float,
    pub d3: Float,
    pub d4: Float,
    pub delmo: Float,
    pub eta: Float,
    pub argpdot: Float,
    pub omgcof: Float,
    pub sinmao: Float,
    pub t: Float,
    pub t2cof: Float,
    pub t3cof: Float,
    pub t4cof: Float,
    pub t5cof: Float,
    pub x1mth2: Float,
    pub x7thm1: Float,
    pub mdot: Float,
    pub nodedot: Float,
    pub xlcof: Float,
    pub xmcof: Float,
    pub nodecf: Float,

    // Deep space variables
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
    pub e3: Float,
    pub ee2: Float,
    pub peo: Float,
    pub pgho: Float,
    pub pho: Float,
    pub pinco: Float,
    pub plo: Float,
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
    pub gsto: Float,
    pub xfact: Float,
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
    pub xlamo: Float,
    pub zmol: Float,
    pub zmos: Float,
    pub atime: Float,
    pub xli: Float,
    pub xni: Float,
}

impl Satrec {
    pub fn zero() -> Satrec {
        Satrec {
            name: None,
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

pub fn parse_multiple(string: &str) -> (Vec<Satrec>, Vec<SatrecParseError>) {
    let lines = string
        .split("\n")
        .into_iter()
        .filter(|el| return el.trim() != "")
        .collect::<Vec<&str>>();
    let mut recs: Vec<Satrec> = vec![];
    let mut errors: Vec<SatrecParseError> = vec![];
    let mut i = 0;
    while i < lines.len() {
        // If line 1 is not equal to a line operator, try next.
        if lines[i].trim().len() != 25 && i + 2 < lines.len() {
            match twoline2satrec(lines[i + 1], lines[i + 2]) {
                Ok(mut rec) => {
                    if lines[i].bytes().collect::<Vec<u8>>()[0] == '0' as u8 {
                        rec.name = Some(lines[i][2..].trim().to_string());
                    } else {
                        rec.name = Some(lines[i].trim().to_string());
                    }

                    recs.push(rec);
                }
                Err(err) => errors.push(SatrecParseError::SatrecMultiError(i, Box::new(err))),
            }
            i += 3;
        } else if i + 1 < lines.len() {
            match twoline2satrec(lines[i], lines[i + 1]) {
                Ok(rec) => recs.push(rec),
                Err(err) => errors.push(SatrecParseError::SatrecMultiError(i, Box::new(err))),
            }
            i += 2;
        } else {
            errors.push(SatrecParseError::SatrecMultiError(
                i,
                Box::new(SatrecParseError::InvalidTLEBadLineCount),
            ));
            i += 1;
        }
    }

    (recs, errors)
}

pub fn parse(string: &str) -> Result<Satrec, SatrecParseError> {
    let lines = string
        .split("\n")
        .into_iter()
        .filter(|el| return el.trim() != "")
        .collect::<Vec<&str>>();

    // If there are three lines, parse as 3LE.
    if lines.len() == 3 {
        // First, perform check on the first character of each line. should be line numbers.
        if lines[1].bytes().collect::<Vec<u8>>()[0] != '1' as u8
            || lines[2].bytes().collect::<Vec<u8>>()[0] != '2' as u8
        {
            return Err(SatrecParseError::InvalidTLELineCheckFailed);
        }

        // Now, parse the satrec.
        let satrec = twoline2satrec(lines[1], lines[2]);

        // Because this is a 3le, we have a name. Extract/save it.
        match satrec {
            Ok(mut satrec) => {
                if lines[0].bytes().collect::<Vec<u8>>()[0] == '0' as u8 {
                    satrec.name = Some(lines[0][2..].to_string());
                } else {
                    satrec.name = Some(lines[0].to_string());
                }

                return Ok(satrec);
            }
            Err(err) => return Err(err),
        }
    } else if lines.len() == 2 {
        // First, perform check on the first character of each line. should be line numbers.
        if lines[0].bytes().collect::<Vec<u8>>()[0] != '1' as u8
            || lines[1].bytes().collect::<Vec<u8>>()[0] != '2' as u8
        {
            return Err(SatrecParseError::InvalidTLELineCheckFailed);
        }

        return twoline2satrec(lines[0], lines[1]);
    } else {
        return Err(SatrecParseError::InvalidTLEBadLineCount);
    }
}

pub fn twoline2satrec(str1: &str, str2: &str) -> Result<Satrec, SatrecParseError> {
    let mut satrec = parse_satrec(str1, str2)?;

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

    match sgp4init::sgp4init(&mut satrec, opts) {
        Ok(_) => {} // Ignore initial sgp4 positioning.
        Err(err) => return Err(SatrecParseError::Sgp4InitError(err)),
    };

    return Ok(satrec);
}

#[derive(Debug, PartialEq)]
pub enum SatrecParseError {
    FloatParseError(&'static str, usize, usize, String),
    IntParseError(&'static str, usize, usize, String),
    CompoundError(&'static str, String),
    Sgp4InitError(SGP4Error),
    InvalidTLELineCheckFailed,
    InvalidTLEBadLineCount,
    SatrecMultiError(usize, Box<SatrecParseError>),
}

fn parse_float(
    line: &str,
    name: &'static str,
    low: usize,
    high: usize,
) -> Result<Float, SatrecParseError> {
    match line[low..high].trim().parse::<Float>() {
        Ok(res) => Ok(res),
        Err(_) => {
            return Err(SatrecParseError::IntParseError(
                name,
                low,
                high,
                line[low..high].to_string(),
            ))
        }
    }
}

fn parse_int(
    line: &str,
    name: &'static str,
    low: usize,
    high: usize,
) -> Result<i64, SatrecParseError> {
    match line[low..high].trim().parse::<i64>() {
        Ok(res) => Ok(res),
        Err(_) => {
            return Err(SatrecParseError::IntParseError(
                name,
                low,
                high,
                line[low..high].to_string(),
            ))
        }
    }
}

pub fn parse_satrec(str1: &str, str2: &str) -> Result<Satrec, SatrecParseError> {
    // Parse sat num
    let satnum = str1[2..7].trim();

    // Parse epoch
    let epochyr = parse_int(str1, "epochyr", 18, 20)?;
    let epochdays = parse_float(str1, "epochdays", 20, 32)?;

    // Parse ndot
    let ndot = parse_float(str1, "ndot", 33, 43)? / (XPDOTP * 1440.0);

    // Parse nndot
    let nddot_0 = parse_int(str1, "nndot frac", 44, 50)?;
    let nddot_1 = str1[50..52].to_string();
    let nndot_str = format!(".{}E{}", nddot_0, nddot_1);

    let nddot = match nndot_str.parse::<Float>() {
        Ok(res) => res / (XPDOTP * 1440.0 * 1440.0),
        Err(_) => return Err(SatrecParseError::CompoundError("nndot", nndot_str)),
    };

    // Parse bstar
    let bstar_0 = str1[53..54].to_string();
    let bstar_1 = parse_int(str1, "bstar frac", 54, 59)?;
    let bstar_2 = str1[59..61].to_string();
    let bstar_str = format!("{}.{}E{}", bstar_0, bstar_1, bstar_2);

    let bstar = match bstar_str.trim().parse::<Float>() {
        Ok(res) => res,
        Err(_) => return Err(SatrecParseError::CompoundError("bstr", bstar_str)),
    };

    let inclo = parse_float(str2, "inclo", 8, 16)? * DEG_2_RAD;
    let nodeo = parse_float(str2, "nodeo", 17, 25)? * DEG_2_RAD;

    // Parse ecco
    let ecco_str = format!(".{}", str2[26..33].trim().to_string());
    let ecco = match ecco_str.trim().parse::<Float>() {
        Ok(res) => res,
        Err(_) => return Err(SatrecParseError::CompoundError("ecco", ecco_str)),
    };

    let argpo = parse_float(str2, "argpo", 34, 42)? * DEG_2_RAD;
    let mo = parse_float(str2, "mo", 43, 51)? * DEG_2_RAD;
    let no = parse_float(str2, "no", 52, 63)? / XPDOTP;

    let a = (no * TUMIN).powf(-2.0 / 3.0);

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
        year as Float,
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

    return Ok(satrec);
}

#[cfg(test)]
mod tests {
    use crate::tests::*;
    #[allow(unused_imports)]
    use crate::*;

    #[test]
    fn test_parse_full() {
        let element = r###"
0 ISS (ZARYA)
1 25544U 98067A   19085.83761025  .00001292  00000-0  28282-4 0  9995
2 25544  51.6446  50.5941 0002332 117.0184 328.1109 15.52438493162461
"###;
        let sat = crate::io::parse(element).unwrap();
        assert_eq!(sat.name, Some("ISS (ZARYA)".to_string()));
    }

    #[test]
    fn test_parse_part() {
        let element = r###"1 25544U 98067A   19085.83761025  .00001292  00000-0  28282-4 0  9995
2 25544  51.6446  50.5941 0002332 117.0184 328.1109 15.52438493162461"###;
        let sat = crate::io::parse(element).unwrap();
        assert_eq!(sat.name, None);
    }

    #[test]
    fn test_parse() {
        let satrec = crate::io::parse_satrec(
            "1 88888U          80275.98708465  .00073094  13844-3  66816-4 0    8",
            "2 88888  72.8435 115.9689 0086731  52.6988 110.5714 16.05824518  105",
        )
        .unwrap();

        assert_eq!(satrec.error, 0);
        assert_eq!(satrec.satnum, "88888");
        assert_eq!(satrec.epochyr, 80);
        assert_similar(satrec.epochdays, 275.98708465);
        assert_similar(satrec.ndot, 2.2148107004387767e-9);
        assert_similar(satrec.nddot, 2.913090538750181e-13);
        assert_similar(satrec.bstar, 0.000066816);
        assert_similar(satrec.inclo, 1.2713589136764896);
        assert_similar(satrec.nodeo, 2.0240391349160523);
        assert_similar(satrec.ecco, 0.0086731);
        assert_similar(satrec.argpo, 0.9197675718499877);
        assert_similar(satrec.mo, 1.929834988539658);
        assert_similar(satrec.no, 0.07006731262087737);
        assert_similar(satrec.a, 1.0405013051291292);
        assert_similar(satrec.alta, 0.04952567699864474);
        assert_similar(satrec.altp, 0.03147693325961365);
        assert_similar(satrec.jdsatepoch, 2444514.48708465);
    }

    //     #[test]
    //     fn test_parse_multi() {
    //         let satrec = &crate::io::parse_multiple(
    // r###"1 88888U          80275.98708465  .00073094  13844-3  66816-4 0    8
    // 2 88888  72.8435 115.9689 0086731  52.6988 110.5714 16.05824518  105"###
    //         ).unwrap()[0];

    //         assert_eq!(satrec.error, 0);
    //         assert_eq!(satrec.satnum, "88888");
    //         assert_eq!(satrec.epochyr, 80);
    //         assert_eq!(satrec.epochdays, 275.98708465);
    //         assert_eq!(satrec.ndot, 2.2148107004387767e-9);
    //         assert_eq!(satrec.nddot, 2.913090538750181e-13);
    //         assert_eq!(satrec.bstar, 0.000066816);
    //         assert_eq!(satrec.inclo, 1.2713589136764896);
    //         assert_eq!(satrec.nodeo, 2.0240391349160523);
    //         assert_eq!(satrec.ecco, 0.0086731);
    //         assert_eq!(satrec.argpo, 0.9197675718499877);
    //         assert_eq!(satrec.mo, 1.929834988539658);
    //         assert_eq!(satrec.no, 0.07006731262087737);
    //         assert_eq!(satrec.a, 1.0405013051291292);
    //         assert_eq!(satrec.alta, 0.04952567699864474);
    //         assert_eq!(satrec.altp, 0.03147693325961365);
    //         assert_eq!(satrec.jdsatepoch, 2444514.48708465);
    //     }

    #[test]
    fn test_init_tle() {
        let satrec = crate::io::twoline2satrec(
            "1 88888U          80275.98708465  .00073094  13844-3  66816-4 0    8",
            "2 88888  72.8435 115.9689 0086731  52.6988 110.5714 16.05824518  105",
        )
        .unwrap();

        assert_eq!(satrec.error, 0);
        assert_eq!(satrec.satnum, "88888");
        assert_eq!(satrec.epochyr, 80);
        assert_eq!(satrec.epochdays, 275.98708465);
        assert_eq!(satrec.ndot, 2.2148107004387767e-9);
        assert_eq!(satrec.nddot, 2.913090538750181e-13);
        assert_similar(satrec.bstar, 0.000066816);
        assert_similar(satrec.inclo, 1.2713589136764896);
        assert_similar(satrec.nodeo, 2.0240391349160523);
        assert_eq!(satrec.ecco, 0.0086731);
        assert_eq!(satrec.argpo, 0.9197675718499877);
        assert_similar(satrec.mo, 1.929834988539658);
        assert_similar(satrec.no, 0.07010615621239219);
        assert_similar(satrec.a, 1.0405013051291292);
        assert_similar(satrec.alta, 0.04952567699864474);
        assert_similar(satrec.altp, 0.03147693325961365);
        assert_eq!(satrec.jdsatepoch, 2444514.48708465);
        assert_eq!(satrec.isimp, 1);
        assert_eq!(satrec.method, crate::propogation::initl::InitlMethod::N);
        assert_similar(satrec.aycof, 0.00111740799765779);

        assert_similar(satrec.con41, -0.7389556198424165);
        assert_similar(satrec.cc1, 2.3340379369349495e-8);
        assert_similar(satrec.cc4, 0.00037724513079719584);
        assert_similar(satrec.cc5, 0.01233625966048993);
        assert_eq!(satrec.d2, 0.0);
        assert_eq!(satrec.d3, 0.0);
        assert_eq!(satrec.d4, 0.0);
        assert_similar(satrec.delmo, 0.6963031736886937);
        assert_similar(satrec.eta, 0.32347784078169217);
        assert_similar(satrec.argpdot, -0.000029718644394179532);
        assert!((satrec.omgcof - 1.6306928260750368e-7).abs() < 1e-6);
        assert_similar(satrec.sinmao, 0.9362350458581234);
        assert_eq!(satrec.t, 0.0);
        assert_similar(satrec.t2cof, 3.5010569054024244e-8);
        assert_eq!(satrec.t3cof, 0.0);
        assert_eq!(satrec.t4cof, 0.0);
        assert_eq!(satrec.t5cof, 0.0);
        assert_similar(satrec.x1mth2, 0.9129852066141388);
        assert_similar(satrec.x7thm1, -0.3908964462989719);
        assert_similar(satrec.mdot, 0.07006729343154267);
        assert_similar(satrec.nodedot, -0.00003096533062994484);
        assert_similar(satrec.xlcof, 0.0019306451483792333);
        assert_similar(satrec.xmcof, -0.0000493564796620572);
        assert_similar(satrec.nodecf, -2.5361112971222384e-12);
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
        assert_similar(satrec.gsto, 0.1082901416688955);
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
    }
}
