pub struct Unit {
    base: Base,
    decimation: Decimation
}

pub enum Base {
    VAC,
    VDC,
    AMPHERE,
    HENRY,
    NEWTON,
    HERTZ,
    JOULE,
    CANDELA,
    LUMEN,
    LUX,
    FARAD,
    OHM,
    WATT,
    TESLA,
    SIEVERT,
    BECQUEREL,
    GRAY,
    COULOMB,
    WEBER,
    DECIBEL,
    SIEMENS,
    KATAL,
    PASCAL,
    BYTE,
    BIT,
    SECONDS
}

pub enum Decimation {
    EXA,
    PETA,
    TERA,
    GIGA,
    MEGA,
    KILO,
    HECTO,
    DEKA,
    BASE,
    DECI,
    CENTI,
    MILLI,
    MICRO,
    NANO,
    PICO,
    FEMTO

}

pub struct Component {
    id: u64,
    manufacturer: Option<super::manufacturer::Manufacturer>,
    purchase_date: Option<u64>,
    part_number: String,
    category: Option<super::category::Category>,
    sub_category: Option<super::category::SubCategory>,
    description: Option<String>,
    datasheet: Option<String>,
    values: Option<Vec<(i32, Unit)>>,
}
