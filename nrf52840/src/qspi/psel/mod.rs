#[doc = "Pin select for serial clock SCK"]
pub struct SCK {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Pin select for serial clock SCK"]
pub mod sck;
#[doc = "Pin select for chip select signal CSN."]
pub struct CSN {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Pin select for chip select signal CSN."]
pub mod csn;
#[doc = "Pin select for serial data MOSI/IO0."]
pub struct IO0 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Pin select for serial data MOSI/IO0."]
pub mod io0;
#[doc = "Pin select for serial data MISO/IO1."]
pub struct IO1 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Pin select for serial data MISO/IO1."]
pub mod io1;
#[doc = "Pin select for serial data IO2."]
pub struct IO2 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Pin select for serial data IO2."]
pub mod io2;
#[doc = "Pin select for serial data IO3."]
pub struct IO3 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Pin select for serial data IO3."]
pub mod io3;
