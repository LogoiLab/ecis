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

pub struct Mount {
    id: u32,
    name: String
}

pub struct Package {
    id: u32,
    pvalue: String
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
    cvalues: Option<Vec<(i32, Unit)>>,
    mount_style: Option<Mount>,
    package: Option<Package>
}
