#[doc = r" Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Unspecified"]
    pub unused0: UNUSED0,
    #[doc = "0x04 - Unspecified"]
    pub unused1: UNUSED1,
    #[doc = "0x08 - Unspecified"]
    pub unused2: UNUSED2,
    _reserved3: [u8; 4usize],
    #[doc = "0x10 - Unspecified"]
    pub unused3: UNUSED3,
    #[doc = "0x14 - Description collection[0]: Reserved for Nordic firmware design"]
    pub nrffw: [NRFFW; 15],
    #[doc = "0x50 - Description collection[0]: Reserved for Nordic hardware design"]
    pub nrfhw: [NRFHW; 12],
    #[doc = "0x80 - Description collection[0]: Reserved for customer"]
    pub customer: [CUSTOMER; 32],
    _reserved7: [u8; 256usize],
    #[doc = "0x200 - Description collection[0]: Mapping of the nRESET function (see POWER chapter for details)"]
    pub pselreset: [PSELRESET; 2],
    #[doc = "0x208 - Access Port protection"]
    pub approtect: APPROTECT,
    #[doc = "0x20c - Setting of pins dedicated to NFC functionality: NFC antenna or GPIO"]
    pub nfcpins: NFCPINS,
}
#[doc = "Unspecified"]
pub struct UNUSED0 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Unspecified"]
pub mod unused0;
#[doc = "Unspecified"]
pub struct UNUSED1 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Unspecified"]
pub mod unused1;
#[doc = "Unspecified"]
pub struct UNUSED2 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Unspecified"]
pub mod unused2;
#[doc = "Unspecified"]
pub struct UNUSED3 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Unspecified"]
pub mod unused3;
#[doc = "Description collection[0]: Reserved for Nordic firmware design"]
pub struct NRFFW {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Description collection[0]: Reserved for Nordic firmware design"]
pub mod nrffw;
#[doc = "Description collection[0]: Reserved for Nordic hardware design"]
pub struct NRFHW {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Description collection[0]: Reserved for Nordic hardware design"]
pub mod nrfhw;
#[doc = "Description collection[0]: Reserved for customer"]
pub struct CUSTOMER {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Description collection[0]: Reserved for customer"]
pub mod customer;
#[doc = "Description collection[0]: Mapping of the nRESET function (see POWER chapter for details)"]
pub struct PSELRESET {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Description collection[0]: Mapping of the nRESET function (see POWER chapter for details)"]
pub mod pselreset;
#[doc = "Access Port protection"]
pub struct APPROTECT {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Access Port protection"]
pub mod approtect;
#[doc = "Setting of pins dedicated to NFC functionality: NFC antenna or GPIO"]
pub struct NFCPINS {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Setting of pins dedicated to NFC functionality: NFC antenna or GPIO"]
pub mod nfcpins;
