#[doc = r" Register block"]
#[repr(C)]
pub struct RegisterBlock {
    _reserved0: [u8; 1024usize],
    #[doc = "0x400 - Ready flag."]
    pub ready: READY,
    _reserved1: [u8; 256usize],
    #[doc = "0x504 - Configuration register."]
    pub config: CONFIG,
    #[doc = "Register for erasing a non-protected non-volatile memory page."]
    pub erasepage: ERASEPAGE_UNION,
    #[doc = "0x50c - Register for erasing all non-volatile user memory."]
    pub eraseall: ERASEALL,
    #[doc = "0x510 - Register for erasing a protected non-volatile memory page."]
    pub erasepcr0: ERASEPCR0,
    #[doc = "0x514 - Register for start erasing User Information Congfiguration Registers."]
    pub eraseuicr: ERASEUICR,
}
#[doc = "Register for erasing a non-protected non-volatile memory page."]
#[repr(C)]
pub union ERASEPAGE_UNION {
    #[doc = "0x508 - Register for erasing a non-protected non-volatile memory page."]
    pub erasepcr1: ERASEPCR1,
    #[doc = "0x508 - Register for erasing a non-protected non-volatile memory page."]
    pub erasepage: ERASEPAGE,
}
#[doc = "Ready flag."]
pub struct READY {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Ready flag."]
pub mod ready;
#[doc = "Configuration register."]
pub struct CONFIG {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Configuration register."]
pub mod config;
#[doc = "Register for erasing a non-protected non-volatile memory page."]
pub struct ERASEPAGE {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Register for erasing a non-protected non-volatile memory page."]
pub mod erasepage;
#[doc = "Register for erasing a non-protected non-volatile memory page."]
pub struct ERASEPCR1 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Register for erasing a non-protected non-volatile memory page."]
pub mod erasepcr1;
#[doc = "Register for erasing all non-volatile user memory."]
pub struct ERASEALL {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Register for erasing all non-volatile user memory."]
pub mod eraseall;
#[doc = "Register for erasing a protected non-volatile memory page."]
pub struct ERASEPCR0 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Register for erasing a protected non-volatile memory page."]
pub mod erasepcr0;
#[doc = "Register for start erasing User Information Congfiguration Registers."]
pub struct ERASEUICR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Register for start erasing User Information Congfiguration Registers."]
pub mod eraseuicr;
