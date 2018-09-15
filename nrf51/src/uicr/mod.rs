#[doc = r" Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Length of code region 0."]
    pub clenr0: CLENR0,
    #[doc = "0x04 - Readback protection configuration."]
    pub rbpconf: RBPCONF,
    #[doc = "0x08 - Reset value for CLOCK XTALFREQ register."]
    pub xtalfreq: XTALFREQ,
    _reserved3: [u8; 4usize],
    #[doc = "0x10 - Firmware ID."]
    pub fwid: FWID,
    #[doc = "Reserved for Nordic firmware design. Bootloader start address."]
    pub nrffw: NRFFW_UNION,
    #[doc = "0x50 - Reserved for Nordic hardware design."]
    pub nrfhw: [NRFHW; 12],
    #[doc = "0x80 - Reserved for customer."]
    pub customer: [CUSTOMER; 32],
}
#[doc = "Reserved for Nordic firmware design. Bootloader start address."]
#[repr(C)]
pub union NRFFW_UNION {
    #[doc = "0x14 - Reserved for Nordic firmware design."]
    pub nrffw: [NRFFW; 15],
    #[doc = "0x14 - Bootloader start address."]
    pub bootloaderaddr: BOOTLOADERADDR,
}
#[doc = "Length of code region 0."]
pub struct CLENR0 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Length of code region 0."]
pub mod clenr0;
#[doc = "Readback protection configuration."]
pub struct RBPCONF {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Readback protection configuration."]
pub mod rbpconf;
#[doc = "Reset value for CLOCK XTALFREQ register."]
pub struct XTALFREQ {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Reset value for CLOCK XTALFREQ register."]
pub mod xtalfreq;
#[doc = "Firmware ID."]
pub struct FWID {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Firmware ID."]
pub mod fwid;
#[doc = "Bootloader start address."]
pub struct BOOTLOADERADDR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Bootloader start address."]
pub mod bootloaderaddr;
#[doc = "Reserved for Nordic firmware design."]
pub struct NRFFW {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Reserved for Nordic firmware design."]
pub mod nrffw;
#[doc = "Reserved for Nordic hardware design."]
pub struct NRFHW {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Reserved for Nordic hardware design."]
pub mod nrfhw;
#[doc = "Reserved for customer."]
pub struct CUSTOMER {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Reserved for customer."]
pub mod customer;
