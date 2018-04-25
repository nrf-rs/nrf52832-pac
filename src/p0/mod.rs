use vcell::VolatileCell;
#[doc = r" Register block"]
#[repr(C)]
pub struct RegisterBlock {
    _reserved0: [u8; 1284usize],
    #[doc = "0x504 - Write GPIO port"]
    pub out: OUT,
    #[doc = "0x508 - Set individual bits in GPIO port"]
    pub outset: OUTSET,
    #[doc = "0x50c - Clear individual bits in GPIO port"]
    pub outclr: OUTCLR,
    #[doc = "0x510 - Read GPIO port"]
    pub in_: IN,
    #[doc = "0x514 - Direction of GPIO pins"]
    pub dir: DIR,
    #[doc = "0x518 - DIR set register"]
    pub dirset: DIRSET,
    #[doc = "0x51c - DIR clear register"]
    pub dirclr: DIRCLR,
    #[doc = "0x520 - Latch register indicating what GPIO pins that have met the criteria set in the PIN_CNF[n].SENSE registers"]
    pub latch: LATCH,
    #[doc = "0x524 - Select between default DETECT signal behaviour and LDETECT mode"]
    pub detectmode: DETECTMODE,
    _reserved1: [u8; 472usize],
    #[doc = "0x700 - Description collection[0]: Configuration of GPIO pins"]
    pub pin_cnf: [PIN_CNF; 32],
}
#[doc = "Write GPIO port"]
pub struct OUT {
    register: VolatileCell<u32>,
}
#[doc = "Write GPIO port"]
pub mod out;
#[doc = "Set individual bits in GPIO port"]
pub struct OUTSET {
    register: VolatileCell<u32>,
}
#[doc = "Set individual bits in GPIO port"]
pub mod outset;
#[doc = "Clear individual bits in GPIO port"]
pub struct OUTCLR {
    register: VolatileCell<u32>,
}
#[doc = "Clear individual bits in GPIO port"]
pub mod outclr;
#[doc = "Read GPIO port"]
pub struct IN {
    register: VolatileCell<u32>,
}
#[doc = "Read GPIO port"]
pub mod in_;
#[doc = "Direction of GPIO pins"]
pub struct DIR {
    register: VolatileCell<u32>,
}
#[doc = "Direction of GPIO pins"]
pub mod dir;
#[doc = "DIR set register"]
pub struct DIRSET {
    register: VolatileCell<u32>,
}
#[doc = "DIR set register"]
pub mod dirset;
#[doc = "DIR clear register"]
pub struct DIRCLR {
    register: VolatileCell<u32>,
}
#[doc = "DIR clear register"]
pub mod dirclr;
#[doc = "Latch register indicating what GPIO pins that have met the criteria set in the PIN_CNF[n].SENSE registers"]
pub struct LATCH {
    register: VolatileCell<u32>,
}
#[doc = "Latch register indicating what GPIO pins that have met the criteria set in the PIN_CNF[n].SENSE registers"]
pub mod latch;
#[doc = "Select between default DETECT signal behaviour and LDETECT mode"]
pub struct DETECTMODE {
    register: VolatileCell<u32>,
}
#[doc = "Select between default DETECT signal behaviour and LDETECT mode"]
pub mod detectmode;
#[doc = "Description collection[0]: Configuration of GPIO pins"]
pub struct PIN_CNF {
    register: VolatileCell<u32>,
}
#[doc = "Description collection[0]: Configuration of GPIO pins"]
pub mod pin_cnf;
