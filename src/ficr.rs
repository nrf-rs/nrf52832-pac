#[doc = r" Register block"]
#[repr(C)]
pub struct RegisterBlock {
    _reserved0: [u8; 16usize],
    #[doc = "0x10 - Code memory page size"]
    pub codepagesize: CODEPAGESIZE,
    #[doc = "0x14 - Code memory size"]
    pub codesize: CODESIZE,
    _reserved1: [u8; 72usize],
    #[doc = "0x60 - Description collection\\[0\\]: Device identifier"]
    pub deviceid: [DEVICEID; 2],
    _reserved2: [u8; 24usize],
    #[doc = "0x80 - Description collection\\[0\\]: Encryption Root, word 0"]
    pub er: [ER; 4],
    #[doc = "0x90 - Description collection\\[0\\]: Identity Root, word 0"]
    pub ir: [IR; 4],
    #[doc = "0xa0 - Device address type"]
    pub deviceaddrtype: DEVICEADDRTYPE,
    #[doc = "0xa4 - Description collection\\[0\\]: Device address 0"]
    pub deviceaddr: [DEVICEADDR; 2],
    _reserved3: [u8; 84usize],
    #[doc = "0x100 - Device info"]
    pub info: INFO,
    _reserved4: [u8; 740usize],
    #[doc = "0x404 - Registers storing factory TEMP module linearization coefficients"]
    pub temp: TEMP,
    _reserved5: [u8; 8usize],
    #[doc = "0x450 - Unspecified"]
    pub nfc: NFC,
}
#[doc = r" Register block"]
#[repr(C)]
pub struct INFO {
    #[doc = "0x00 - Part code"]
    pub part: self::info::PART,
    #[doc = "0x04 - Part Variant, Hardware version and Production configuration"]
    pub variant: self::info::VARIANT,
    #[doc = "0x08 - Package option"]
    pub package: self::info::PACKAGE,
    #[doc = "0x0c - RAM variant"]
    pub ram: self::info::RAM,
    #[doc = "0x10 - Flash variant"]
    pub flash: self::info::FLASH,
    #[doc = "0x14 - Description collection\\[0\\]: Unspecified"]
    pub unused0: [self::info::UNUSED0; 3],
}
#[doc = r" Register block"]
#[doc = "Device info"]
pub mod info;
#[doc = r" Register block"]
#[repr(C)]
pub struct TEMP {
    #[doc = "0x00 - Slope definition A0."]
    pub a0: self::temp::A0,
    #[doc = "0x04 - Slope definition A1."]
    pub a1: self::temp::A1,
    #[doc = "0x08 - Slope definition A2."]
    pub a2: self::temp::A2,
    #[doc = "0x0c - Slope definition A3."]
    pub a3: self::temp::A3,
    #[doc = "0x10 - Slope definition A4."]
    pub a4: self::temp::A4,
    #[doc = "0x14 - Slope definition A5."]
    pub a5: self::temp::A5,
    #[doc = "0x18 - y-intercept B0."]
    pub b0: self::temp::B0,
    #[doc = "0x1c - y-intercept B1."]
    pub b1: self::temp::B1,
    #[doc = "0x20 - y-intercept B2."]
    pub b2: self::temp::B2,
    #[doc = "0x24 - y-intercept B3."]
    pub b3: self::temp::B3,
    #[doc = "0x28 - y-intercept B4."]
    pub b4: self::temp::B4,
    #[doc = "0x2c - y-intercept B5."]
    pub b5: self::temp::B5,
    #[doc = "0x30 - Segment end T0."]
    pub t0: self::temp::T0,
    #[doc = "0x34 - Segment end T1."]
    pub t1: self::temp::T1,
    #[doc = "0x38 - Segment end T2."]
    pub t2: self::temp::T2,
    #[doc = "0x3c - Segment end T3."]
    pub t3: self::temp::T3,
    #[doc = "0x40 - Segment end T4."]
    pub t4: self::temp::T4,
}
#[doc = r" Register block"]
#[doc = "Registers storing factory TEMP module linearization coefficients"]
pub mod temp;
#[doc = r" Register block"]
#[repr(C)]
pub struct NFC {
    #[doc = "0x00 - Default header for NFC Tag. Software can read these values to populate NFCID1_3RD_LAST, NFCID1_2ND_LAST and NFCID1_LAST."]
    pub tagheader0: self::nfc::TAGHEADER0,
    #[doc = "0x04 - Default header for NFC Tag. Software can read these values to populate NFCID1_3RD_LAST, NFCID1_2ND_LAST and NFCID1_LAST."]
    pub tagheader1: self::nfc::TAGHEADER1,
    #[doc = "0x08 - Default header for NFC Tag. Software can read these values to populate NFCID1_3RD_LAST, NFCID1_2ND_LAST and NFCID1_LAST."]
    pub tagheader2: self::nfc::TAGHEADER2,
    #[doc = "0x0c - Default header for NFC Tag. Software can read these values to populate NFCID1_3RD_LAST, NFCID1_2ND_LAST and NFCID1_LAST."]
    pub tagheader3: self::nfc::TAGHEADER3,
}
#[doc = r" Register block"]
#[doc = "Unspecified"]
pub mod nfc;
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
#[doc = "Description collection\\[0\\]: Device identifier"]
pub struct DEVICEID {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Description collection\\[0\\]: Device identifier"]
pub mod deviceid;
#[doc = "Description collection\\[0\\]: Encryption Root, word 0"]
pub struct ER {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Description collection\\[0\\]: Encryption Root, word 0"]
pub mod er;
#[doc = "Description collection\\[0\\]: Identity Root, word 0"]
pub struct IR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Description collection\\[0\\]: Identity Root, word 0"]
pub mod ir;
#[doc = "Device address type"]
pub struct DEVICEADDRTYPE {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Device address type"]
pub mod deviceaddrtype;
#[doc = "Description collection\\[0\\]: Device address 0"]
pub struct DEVICEADDR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Description collection\\[0\\]: Device address 0"]
pub mod deviceaddr;
