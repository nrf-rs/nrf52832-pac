#[doc = r" Register block"]
#[repr(C)]
pub struct RegisterBlock {
    _reserved0: [u8; 16usize],
    #[doc = "0x10 - Code memory page size"]
    pub codepagesize: CODEPAGESIZE,
    #[doc = "0x14 - Code memory size"]
    pub codesize: CODESIZE,
    _reserved2: [u8; 72usize],
    #[doc = "0x60 - Description collection[n]: Device identifier"]
    pub deviceid: [DEVICEID; 2],
    _reserved3: [u8; 24usize],
    #[doc = "0x80 - Description collection[n]: Encryption root, word n"]
    pub er: [ER; 4],
    #[doc = "0x90 - Description collection[n]: Identity Root, word n"]
    pub ir: [IR; 4],
    #[doc = "0xa0 - Device address type"]
    pub deviceaddrtype: DEVICEADDRTYPE,
    #[doc = "0xa4 - Description collection[n]: Device address n"]
    pub deviceaddr: [DEVICEADDR; 2],
    _reserved7: [u8; 84usize],
    #[doc = "0x100 - Device info"]
    pub info: INFO,
    _reserved8: [u8; 560usize],
    #[doc = "0x350 - Description collection[n]: Production test signature n"]
    pub prodtest: [PRODTEST; 3],
    _reserved9: [u8; 168usize],
    #[doc = "0x404 - Registers storing factory TEMP module linearization coefficients"]
    pub temp: TEMP,
    _reserved10: [u8; 8usize],
    #[doc = "0x450 - Unspecified"]
    pub nfc: NFC,
    _reserved11: [u8; 1952usize],
    #[doc = "0xc00 - NIST800-90B RNG calibration data"]
    pub trng90b: TRNG90B,
}
#[doc = r" Register block"]
#[repr(C)]
pub struct INFO {
    #[doc = "0x00 - Part code"]
    pub part: self::info::PART,
    #[doc = "0x04 - Build code (hardware version and production configuration)"]
    pub variant: self::info::VARIANT,
    #[doc = "0x08 - Package option"]
    pub package: self::info::PACKAGE,
    #[doc = "0x0c - RAM variant"]
    pub ram: self::info::RAM,
    #[doc = "0x10 - Flash variant"]
    pub flash: self::info::FLASH,
    #[doc = "0x14 - Unspecified"]
    pub unused8: [self::info::UNUSED8; 3],
}
#[doc = r" Register block"]
#[doc = "Device info"]
pub mod info;
#[doc = r" Register block"]
#[repr(C)]
pub struct TEMP {
    #[doc = "0x00 - Slope definition A0"]
    pub a0: self::temp::A0,
    #[doc = "0x04 - Slope definition A1"]
    pub a1: self::temp::A1,
    #[doc = "0x08 - Slope definition A2"]
    pub a2: self::temp::A2,
    #[doc = "0x0c - Slope definition A3"]
    pub a3: self::temp::A3,
    #[doc = "0x10 - Slope definition A4"]
    pub a4: self::temp::A4,
    #[doc = "0x14 - Slope definition A5"]
    pub a5: self::temp::A5,
    #[doc = "0x18 - Y-intercept B0"]
    pub b0: self::temp::B0,
    #[doc = "0x1c - Y-intercept B1"]
    pub b1: self::temp::B1,
    #[doc = "0x20 - Y-intercept B2"]
    pub b2: self::temp::B2,
    #[doc = "0x24 - Y-intercept B3"]
    pub b3: self::temp::B3,
    #[doc = "0x28 - Y-intercept B4"]
    pub b4: self::temp::B4,
    #[doc = "0x2c - Y-intercept B5"]
    pub b5: self::temp::B5,
    #[doc = "0x30 - Segment end T0"]
    pub t0: self::temp::T0,
    #[doc = "0x34 - Segment end T1"]
    pub t1: self::temp::T1,
    #[doc = "0x38 - Segment end T2"]
    pub t2: self::temp::T2,
    #[doc = "0x3c - Segment end T3"]
    pub t3: self::temp::T3,
    #[doc = "0x40 - Segment end T4"]
    pub t4: self::temp::T4,
}
#[doc = r" Register block"]
#[doc = "Registers storing factory TEMP module linearization coefficients"]
pub mod temp;
#[doc = r" Register block"]
#[repr(C)]
pub struct NFC {
    #[doc = "0x00 - Default header for NFC tag. Software can read these values to populate NFCID1_3RD_LAST, NFCID1_2ND_LAST and NFCID1_LAST."]
    pub tagheader0: self::nfc::TAGHEADER0,
    #[doc = "0x04 - Default header for NFC tag. Software can read these values to populate NFCID1_3RD_LAST, NFCID1_2ND_LAST and NFCID1_LAST."]
    pub tagheader1: self::nfc::TAGHEADER1,
    #[doc = "0x08 - Default header for NFC tag. Software can read these values to populate NFCID1_3RD_LAST, NFCID1_2ND_LAST and NFCID1_LAST."]
    pub tagheader2: self::nfc::TAGHEADER2,
    #[doc = "0x0c - Default header for NFC tag. Software can read these values to populate NFCID1_3RD_LAST, NFCID1_2ND_LAST and NFCID1_LAST."]
    pub tagheader3: self::nfc::TAGHEADER3,
}
#[doc = r" Register block"]
#[doc = "Unspecified"]
pub mod nfc;
#[doc = r" Register block"]
#[repr(C)]
pub struct TRNG90B {
    #[doc = "0x00 - Amount of bytes for the required entropy bits"]
    pub bytes: self::trng90b::BYTES,
    #[doc = "0x04 - Repetition counter cutoff"]
    pub rccutoff: self::trng90b::RCCUTOFF,
    #[doc = "0x08 - Adaptive proportion cutoff"]
    pub apcutoff: self::trng90b::APCUTOFF,
    #[doc = "0x0c - Amount of bytes for the startup tests"]
    pub startup: self::trng90b::STARTUP,
    #[doc = "0x10 - Sample count for ring oscillator 1"]
    pub rosc1: self::trng90b::ROSC1,
    #[doc = "0x14 - Sample count for ring oscillator 2"]
    pub rosc2: self::trng90b::ROSC2,
    #[doc = "0x18 - Sample count for ring oscillator 3"]
    pub rosc3: self::trng90b::ROSC3,
    #[doc = "0x1c - Sample count for ring oscillator 4"]
    pub rosc4: self::trng90b::ROSC4,
}
#[doc = r" Register block"]
#[doc = "NIST800-90B RNG calibration data"]
pub mod trng90b;
#[doc = "Code memory page size"]
pub struct CODEPAGESIZE {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Code memory page size"]
pub mod codepagesize;
#[doc = "Code memory size"]
pub struct CODESIZE {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Code memory size"]
pub mod codesize;
#[doc = "Description collection[n]: Device identifier"]
pub struct DEVICEID {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Description collection[n]: Device identifier"]
pub mod deviceid;
#[doc = "Description collection[n]: Encryption root, word n"]
pub struct ER {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Description collection[n]: Encryption root, word n"]
pub mod er;
#[doc = "Description collection[n]: Identity Root, word n"]
pub struct IR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Description collection[n]: Identity Root, word n"]
pub mod ir;
#[doc = "Device address type"]
pub struct DEVICEADDRTYPE {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Device address type"]
pub mod deviceaddrtype;
#[doc = "Description collection[n]: Device address n"]
pub struct DEVICEADDR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Description collection[n]: Device address n"]
pub mod deviceaddr;
#[doc = "Description collection[n]: Production test signature n"]
pub struct PRODTEST {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Description collection[n]: Production test signature n"]
pub mod prodtest;
