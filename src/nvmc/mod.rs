#[doc = r" Register block"]
#[repr(C)]
pub struct RegisterBlock {
    _reserved0: [u8; 1024usize],
    #[doc = "0x400 - Ready flag"]
    pub ready: READY,
    _reserved1: [u8; 256usize],
    #[doc = "0x504 - Configuration register"]
    pub config: CONFIG,
    #[doc = "Deprecated register -  Register for erasing a page in Code area. Equivalent to ERASEPAGE. Register for erasing a page in Code area"]
    pub erasepage: ERASEPAGE_UNION,
    #[doc = "0x50c - Register for erasing all non-volatile user memory"]
    pub eraseall: ERASEALL,
    #[doc = "0x510 - Deprecated register - Register for erasing a page in Code area. Equivalent to ERASEPAGE."]
    pub erasepcr0: ERASEPCR0,
    #[doc = "0x514 - Register for erasing User Information Configuration Registers"]
    pub eraseuicr: ERASEUICR,
    _reserved6: [u8; 40usize],
    #[doc = "0x540 - I-Code cache configuration register."]
    pub icachecnf: ICACHECNF,
    _reserved7: [u8; 4usize],
    #[doc = "0x548 - I-Code cache hit counter."]
    pub ihit: IHIT,
    #[doc = "0x54c - I-Code cache miss counter."]
    pub imiss: IMISS,
}
#[doc = "Deprecated register -  Register for erasing a page in Code area. Equivalent to ERASEPAGE. Register for erasing a page in Code area"]
#[repr(C)]
pub union ERASEPAGE_UNION {
    #[doc = "0x508 - Deprecated register - Register for erasing a page in Code area. Equivalent to ERASEPAGE."]
    pub erasepcr1: ERASEPCR1,
    #[doc = "0x508 - Register for erasing a page in Code area"]
    pub erasepage: ERASEPAGE,
}
#[doc = "Ready flag"]
pub struct READY {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Ready flag"]
pub mod ready;
#[doc = "Configuration register"]
pub struct CONFIG {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Configuration register"]
pub mod config;
#[doc = "Register for erasing a page in Code area"]
pub struct ERASEPAGE {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Register for erasing a page in Code area"]
pub mod erasepage;
#[doc = "Deprecated register - Register for erasing a page in Code area. Equivalent to ERASEPAGE."]
pub struct ERASEPCR1 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Deprecated register - Register for erasing a page in Code area. Equivalent to ERASEPAGE."]
pub mod erasepcr1;
#[doc = "Register for erasing all non-volatile user memory"]
pub struct ERASEALL {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Register for erasing all non-volatile user memory"]
pub mod eraseall;
#[doc = "Deprecated register - Register for erasing a page in Code area. Equivalent to ERASEPAGE."]
pub struct ERASEPCR0 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Deprecated register - Register for erasing a page in Code area. Equivalent to ERASEPAGE."]
pub mod erasepcr0;
#[doc = "Register for erasing User Information Configuration Registers"]
pub struct ERASEUICR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Register for erasing User Information Configuration Registers"]
pub mod eraseuicr;
#[doc = "I-Code cache configuration register."]
pub struct ICACHECNF {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "I-Code cache configuration register."]
pub mod icachecnf;
#[doc = "I-Code cache hit counter."]
pub struct IHIT {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "I-Code cache hit counter."]
pub mod ihit;
#[doc = "I-Code cache miss counter."]
pub struct IMISS {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "I-Code cache miss counter."]
pub mod imiss;
