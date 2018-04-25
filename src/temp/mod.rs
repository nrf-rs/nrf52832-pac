use vcell::VolatileCell;
#[doc = r" Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Start temperature measurement"]
    pub tasks_start: TASKS_START,
    #[doc = "0x04 - Stop temperature measurement"]
    pub tasks_stop: TASKS_STOP,
    _reserved0: [u8; 248usize],
    #[doc = "0x100 - Temperature measurement complete, data ready"]
    pub events_datardy: EVENTS_DATARDY,
    _reserved1: [u8; 512usize],
    #[doc = "0x304 - Enable interrupt"]
    pub intenset: INTENSET,
    #[doc = "0x308 - Disable interrupt"]
    pub intenclr: INTENCLR,
    _reserved2: [u8; 508usize],
    #[doc = "0x508 - Temperature in degC (0.25deg steps)"]
    pub temp: TEMP,
    _reserved3: [u8; 20usize],
    #[doc = "0x520 - Slope of 1st piece wise linear function"]
    pub a0: A0,
    #[doc = "0x524 - Slope of 2nd piece wise linear function"]
    pub a1: A1,
    #[doc = "0x528 - Slope of 3rd piece wise linear function"]
    pub a2: A2,
    #[doc = "0x52c - Slope of 4th piece wise linear function"]
    pub a3: A3,
    #[doc = "0x530 - Slope of 5th piece wise linear function"]
    pub a4: A4,
    #[doc = "0x534 - Slope of 6th piece wise linear function"]
    pub a5: A5,
    _reserved4: [u8; 8usize],
    #[doc = "0x540 - y-intercept of 1st piece wise linear function"]
    pub b0: B0,
    #[doc = "0x544 - y-intercept of 2nd piece wise linear function"]
    pub b1: B1,
    #[doc = "0x548 - y-intercept of 3rd piece wise linear function"]
    pub b2: B2,
    #[doc = "0x54c - y-intercept of 4th piece wise linear function"]
    pub b3: B3,
    #[doc = "0x550 - y-intercept of 5th piece wise linear function"]
    pub b4: B4,
    #[doc = "0x554 - y-intercept of 6th piece wise linear function"]
    pub b5: B5,
    _reserved5: [u8; 8usize],
    #[doc = "0x560 - End point of 1st piece wise linear function"]
    pub t0: T0,
    #[doc = "0x564 - End point of 2nd piece wise linear function"]
    pub t1: T1,
    #[doc = "0x568 - End point of 3rd piece wise linear function"]
    pub t2: T2,
    #[doc = "0x56c - End point of 4th piece wise linear function"]
    pub t3: T3,
    #[doc = "0x570 - End point of 5th piece wise linear function"]
    pub t4: T4,
}
#[doc = "Start temperature measurement"]
pub struct TASKS_START {
    register: VolatileCell<u32>,
}
#[doc = "Start temperature measurement"]
pub mod tasks_start;
#[doc = "Stop temperature measurement"]
pub struct TASKS_STOP {
    register: VolatileCell<u32>,
}
#[doc = "Stop temperature measurement"]
pub mod tasks_stop;
#[doc = "Temperature measurement complete, data ready"]
pub struct EVENTS_DATARDY {
    register: VolatileCell<u32>,
}
#[doc = "Temperature measurement complete, data ready"]
pub mod events_datardy;
#[doc = "Enable interrupt"]
pub struct INTENSET {
    register: VolatileCell<u32>,
}
#[doc = "Enable interrupt"]
pub mod intenset;
#[doc = "Disable interrupt"]
pub struct INTENCLR {
    register: VolatileCell<u32>,
}
#[doc = "Disable interrupt"]
pub mod intenclr;
#[doc = "Temperature in degC (0.25deg steps)"]
pub struct TEMP {
    register: VolatileCell<u32>,
}
#[doc = "Temperature in degC (0.25deg steps)"]
pub mod temp;
#[doc = "Slope of 1st piece wise linear function"]
pub struct A0 {
    register: VolatileCell<u32>,
}
#[doc = "Slope of 1st piece wise linear function"]
pub mod a0;
#[doc = "Slope of 2nd piece wise linear function"]
pub struct A1 {
    register: VolatileCell<u32>,
}
#[doc = "Slope of 2nd piece wise linear function"]
pub mod a1;
#[doc = "Slope of 3rd piece wise linear function"]
pub struct A2 {
    register: VolatileCell<u32>,
}
#[doc = "Slope of 3rd piece wise linear function"]
pub mod a2;
#[doc = "Slope of 4th piece wise linear function"]
pub struct A3 {
    register: VolatileCell<u32>,
}
#[doc = "Slope of 4th piece wise linear function"]
pub mod a3;
#[doc = "Slope of 5th piece wise linear function"]
pub struct A4 {
    register: VolatileCell<u32>,
}
#[doc = "Slope of 5th piece wise linear function"]
pub mod a4;
#[doc = "Slope of 6th piece wise linear function"]
pub struct A5 {
    register: VolatileCell<u32>,
}
#[doc = "Slope of 6th piece wise linear function"]
pub mod a5;
#[doc = "y-intercept of 1st piece wise linear function"]
pub struct B0 {
    register: VolatileCell<u32>,
}
#[doc = "y-intercept of 1st piece wise linear function"]
pub mod b0;
#[doc = "y-intercept of 2nd piece wise linear function"]
pub struct B1 {
    register: VolatileCell<u32>,
}
#[doc = "y-intercept of 2nd piece wise linear function"]
pub mod b1;
#[doc = "y-intercept of 3rd piece wise linear function"]
pub struct B2 {
    register: VolatileCell<u32>,
}
#[doc = "y-intercept of 3rd piece wise linear function"]
pub mod b2;
#[doc = "y-intercept of 4th piece wise linear function"]
pub struct B3 {
    register: VolatileCell<u32>,
}
#[doc = "y-intercept of 4th piece wise linear function"]
pub mod b3;
#[doc = "y-intercept of 5th piece wise linear function"]
pub struct B4 {
    register: VolatileCell<u32>,
}
#[doc = "y-intercept of 5th piece wise linear function"]
pub mod b4;
#[doc = "y-intercept of 6th piece wise linear function"]
pub struct B5 {
    register: VolatileCell<u32>,
}
#[doc = "y-intercept of 6th piece wise linear function"]
pub mod b5;
#[doc = "End point of 1st piece wise linear function"]
pub struct T0 {
    register: VolatileCell<u32>,
}
#[doc = "End point of 1st piece wise linear function"]
pub mod t0;
#[doc = "End point of 2nd piece wise linear function"]
pub struct T1 {
    register: VolatileCell<u32>,
}
#[doc = "End point of 2nd piece wise linear function"]
pub mod t1;
#[doc = "End point of 3rd piece wise linear function"]
pub struct T2 {
    register: VolatileCell<u32>,
}
#[doc = "End point of 3rd piece wise linear function"]
pub mod t2;
#[doc = "End point of 4th piece wise linear function"]
pub struct T3 {
    register: VolatileCell<u32>,
}
#[doc = "End point of 4th piece wise linear function"]
pub mod t3;
#[doc = "End point of 5th piece wise linear function"]
pub struct T4 {
    register: VolatileCell<u32>,
}
#[doc = "End point of 5th piece wise linear function"]
pub mod t4;
