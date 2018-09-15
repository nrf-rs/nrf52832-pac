#[doc = r" Register block"]
#[repr(C)]
pub struct RegisterBlock {
    _reserved0: [u8; 16usize],
    #[doc = "0x10 - Code memory page size in bytes."]
    pub codepagesize: CODEPAGESIZE,
    #[doc = "0x14 - Code memory size in pages."]
    pub codesize: CODESIZE,
    _reserved2: [u8; 16usize],
    #[doc = "0x28 - Length of code region 0 in bytes."]
    pub clenr0: CLENR0,
    #[doc = "0x2c - Pre-programmed factory code present."]
    pub ppfc: PPFC,
    _reserved4: [u8; 4usize],
    #[doc = "0x34 - Number of individualy controllable RAM blocks."]
    pub numramblock: NUMRAMBLOCK,
    #[doc = "Deprecated array of size of RAM block in bytes. This name is kept for backward compatinility purposes. Use SIZERAMBLOCKS instead. Size of RAM blocks in bytes."]
    pub sizeramblock: SIZERAMBLOCK_UNION,
    _reserved6: [u8; 20usize],
    #[doc = "0x5c - Configuration identifier."]
    pub configid: CONFIGID,
    #[doc = "0x60 - Device identifier."]
    pub deviceid: [DEVICEID; 2],
    _reserved8: [u8; 24usize],
    #[doc = "0x80 - Encryption root."]
    pub er: [ER; 4],
    #[doc = "0x90 - Identity root."]
    pub ir: [IR; 4],
    #[doc = "0xa0 - Device address type."]
    pub deviceaddrtype: DEVICEADDRTYPE,
    #[doc = "0xa4 - Device address."]
    pub deviceaddr: [DEVICEADDR; 2],
    #[doc = "0xac - Radio calibration override enable."]
    pub overrideen: OVERRIDEEN,
    #[doc = "0xb0 - Override values for the OVERRIDEn registers in RADIO for NRF_1Mbit mode."]
    pub nrf_1mbit: [NRF_1MBIT; 5],
    _reserved14: [u8; 40usize],
    #[doc = "0xec - Override values for the OVERRIDEn registers in RADIO for BLE_1Mbit mode."]
    pub ble_1mbit: [BLE_1MBIT; 5],
}
#[doc = "Deprecated array of size of RAM block in bytes. This name is kept for backward compatinility purposes. Use SIZERAMBLOCKS instead. Size of RAM blocks in bytes."]
#[repr(C)]
pub union SIZERAMBLOCK_UNION {
    #[doc = "0x38 - Deprecated array of size of RAM block in bytes. This name is kept for backward compatinility purposes. Use SIZERAMBLOCKS instead."]
    pub sizeramblock: [SIZERAMBLOCK; 4],
    #[doc = "0x38 - Size of RAM blocks in bytes."]
    pub sizeramblocks: SIZERAMBLOCKS,
}
#[doc = "Code memory page size in bytes."]
pub struct CODEPAGESIZE {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Code memory page size in bytes."]
pub mod codepagesize;
#[doc = "Code memory size in pages."]
pub struct CODESIZE {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Code memory size in pages."]
pub mod codesize;
#[doc = "Length of code region 0 in bytes."]
pub struct CLENR0 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Length of code region 0 in bytes."]
pub mod clenr0;
#[doc = "Pre-programmed factory code present."]
pub struct PPFC {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Pre-programmed factory code present."]
pub mod ppfc;
#[doc = "Number of individualy controllable RAM blocks."]
pub struct NUMRAMBLOCK {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Number of individualy controllable RAM blocks."]
pub mod numramblock;
#[doc = "Size of RAM blocks in bytes."]
pub struct SIZERAMBLOCKS {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Size of RAM blocks in bytes."]
pub mod sizeramblocks;
#[doc = "Deprecated array of size of RAM block in bytes. This name is kept for backward compatinility purposes. Use SIZERAMBLOCKS instead."]
pub struct SIZERAMBLOCK {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Deprecated array of size of RAM block in bytes. This name is kept for backward compatinility purposes. Use SIZERAMBLOCKS instead."]
pub mod sizeramblock;
#[doc = "Configuration identifier."]
pub struct CONFIGID {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Configuration identifier."]
pub mod configid;
#[doc = "Device identifier."]
pub struct DEVICEID {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Device identifier."]
pub mod deviceid;
#[doc = "Encryption root."]
pub struct ER {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Encryption root."]
pub mod er;
#[doc = "Identity root."]
pub struct IR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Identity root."]
pub mod ir;
#[doc = "Device address type."]
pub struct DEVICEADDRTYPE {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Device address type."]
pub mod deviceaddrtype;
#[doc = "Device address."]
pub struct DEVICEADDR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Device address."]
pub mod deviceaddr;
#[doc = "Radio calibration override enable."]
pub struct OVERRIDEEN {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Radio calibration override enable."]
pub mod overrideen;
#[doc = "Override values for the OVERRIDEn registers in RADIO for NRF_1Mbit mode."]
pub struct NRF_1MBIT {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Override values for the OVERRIDEn registers in RADIO for NRF_1Mbit mode."]
pub mod nrf_1mbit;
#[doc = "Override values for the OVERRIDEn registers in RADIO for BLE_1Mbit mode."]
pub struct BLE_1MBIT {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Override values for the OVERRIDEn registers in RADIO for BLE_1Mbit mode."]
pub mod ble_1mbit;
