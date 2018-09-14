#[doc = "Sample delay for input serial data on MISO"]
pub struct RXDELAY {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Sample delay for input serial data on MISO"]
pub mod rxdelay;
#[doc = "Minimum duration between edge of CSN and edge of SCK and minimum duration CSN must stay high between transactions"]
pub struct CSNDUR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Minimum duration between edge of CSN and edge of SCK and minimum duration CSN must stay high between transactions"]
pub mod csndur;
