#[doc = "Pin select for RTS signal"]
pub struct RTS {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Pin select for RTS signal"]
pub mod rts;
#[doc = "Pin select for TXD signal"]
pub struct TXD {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Pin select for TXD signal"]
pub mod txd;
#[doc = "Pin select for CTS signal"]
pub struct CTS {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Pin select for CTS signal"]
pub mod cts;
#[doc = "Pin select for RXD signal"]
pub struct RXD {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Pin select for RXD signal"]
pub mod rxd;
