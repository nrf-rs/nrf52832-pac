#[doc = "Amount of bytes for the required entropy bits"]
pub struct BYTES {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Amount of bytes for the required entropy bits"]
pub mod bytes;
#[doc = "Repetition counter cutoff"]
pub struct RCCUTOFF {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Repetition counter cutoff"]
pub mod rccutoff;
#[doc = "Adaptive proportion cutoff"]
pub struct APCUTOFF {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Adaptive proportion cutoff"]
pub mod apcutoff;
#[doc = "Amount of bytes for the startup tests"]
pub struct STARTUP {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Amount of bytes for the startup tests"]
pub mod startup;
#[doc = "Sample count for ring oscillator 1"]
pub struct ROSC1 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Sample count for ring oscillator 1"]
pub mod rosc1;
#[doc = "Sample count for ring oscillator 2"]
pub struct ROSC2 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Sample count for ring oscillator 2"]
pub mod rosc2;
#[doc = "Sample count for ring oscillator 3"]
pub struct ROSC3 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Sample count for ring oscillator 3"]
pub mod rosc3;
#[doc = "Sample count for ring oscillator 4"]
pub struct ROSC4 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Sample count for ring oscillator 4"]
pub mod rosc4;
