#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Start temperature measurement"]
    pub tasks_start: TASKS_START,
    #[doc = "0x04 - Stop temperature measurement"]
    pub tasks_stop: TASKS_STOP,
    _reserved2: [u8; 248usize],
    #[doc = "0x100 - Temperature measurement complete, data ready"]
    pub events_datardy: EVENTS_DATARDY,
    _reserved3: [u8; 512usize],
    #[doc = "0x304 - Enable interrupt"]
    pub intenset: INTENSET,
    #[doc = "0x308 - Disable interrupt"]
    pub intenclr: INTENCLR,
    _reserved5: [u8; 508usize],
    #[doc = "0x508 - Temperature in degC (0.25deg steps)"]
    pub temp: TEMP,
    _reserved6: [u8; 20usize],
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
    _reserved12: [u8; 8usize],
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
    _reserved18: [u8; 8usize],
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
#[doc = "Start temperature measurement\n\nThis register you can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tasks_start](tasks_start) module"]
pub type TASKS_START = crate::Reg<u32, _TASKS_START>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TASKS_START;
#[doc = "`write(|w| ..)` method takes [tasks_start::W](tasks_start::W) writer structure"]
impl crate::Writable for TASKS_START {}
#[doc = "Start temperature measurement"]
pub mod tasks_start;
#[doc = "Stop temperature measurement\n\nThis register you can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tasks_stop](tasks_stop) module"]
pub type TASKS_STOP = crate::Reg<u32, _TASKS_STOP>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TASKS_STOP;
#[doc = "`write(|w| ..)` method takes [tasks_stop::W](tasks_stop::W) writer structure"]
impl crate::Writable for TASKS_STOP {}
#[doc = "Stop temperature measurement"]
pub mod tasks_stop;
#[doc = "Temperature measurement complete, data ready\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [events_datardy](events_datardy) module"]
pub type EVENTS_DATARDY = crate::Reg<u32, _EVENTS_DATARDY>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _EVENTS_DATARDY;
#[doc = "`read()` method returns [events_datardy::R](events_datardy::R) reader structure"]
impl crate::Readable for EVENTS_DATARDY {}
#[doc = "`write(|w| ..)` method takes [events_datardy::W](events_datardy::W) writer structure"]
impl crate::Writable for EVENTS_DATARDY {}
#[doc = "Temperature measurement complete, data ready"]
pub mod events_datardy;
#[doc = "Enable interrupt\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [intenset](intenset) module"]
pub type INTENSET = crate::Reg<u32, _INTENSET>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _INTENSET;
#[doc = "`read()` method returns [intenset::R](intenset::R) reader structure"]
impl crate::Readable for INTENSET {}
#[doc = "`write(|w| ..)` method takes [intenset::W](intenset::W) writer structure"]
impl crate::Writable for INTENSET {}
#[doc = "Enable interrupt"]
pub mod intenset;
#[doc = "Disable interrupt\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [intenclr](intenclr) module"]
pub type INTENCLR = crate::Reg<u32, _INTENCLR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _INTENCLR;
#[doc = "`read()` method returns [intenclr::R](intenclr::R) reader structure"]
impl crate::Readable for INTENCLR {}
#[doc = "`write(|w| ..)` method takes [intenclr::W](intenclr::W) writer structure"]
impl crate::Writable for INTENCLR {}
#[doc = "Disable interrupt"]
pub mod intenclr;
#[doc = "Temperature in degC (0.25deg steps)\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [temp](temp) module"]
pub type TEMP = crate::Reg<u32, _TEMP>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TEMP;
#[doc = "`read()` method returns [temp::R](temp::R) reader structure"]
impl crate::Readable for TEMP {}
#[doc = "Temperature in degC (0.25deg steps)"]
pub mod temp;
#[doc = "Slope of 1st piece wise linear function\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [a0](a0) module"]
pub type A0 = crate::Reg<u32, _A0>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _A0;
#[doc = "`read()` method returns [a0::R](a0::R) reader structure"]
impl crate::Readable for A0 {}
#[doc = "`write(|w| ..)` method takes [a0::W](a0::W) writer structure"]
impl crate::Writable for A0 {}
#[doc = "Slope of 1st piece wise linear function"]
pub mod a0;
#[doc = "Slope of 2nd piece wise linear function\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [a1](a1) module"]
pub type A1 = crate::Reg<u32, _A1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _A1;
#[doc = "`read()` method returns [a1::R](a1::R) reader structure"]
impl crate::Readable for A1 {}
#[doc = "`write(|w| ..)` method takes [a1::W](a1::W) writer structure"]
impl crate::Writable for A1 {}
#[doc = "Slope of 2nd piece wise linear function"]
pub mod a1;
#[doc = "Slope of 3rd piece wise linear function\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [a2](a2) module"]
pub type A2 = crate::Reg<u32, _A2>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _A2;
#[doc = "`read()` method returns [a2::R](a2::R) reader structure"]
impl crate::Readable for A2 {}
#[doc = "`write(|w| ..)` method takes [a2::W](a2::W) writer structure"]
impl crate::Writable for A2 {}
#[doc = "Slope of 3rd piece wise linear function"]
pub mod a2;
#[doc = "Slope of 4th piece wise linear function\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [a3](a3) module"]
pub type A3 = crate::Reg<u32, _A3>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _A3;
#[doc = "`read()` method returns [a3::R](a3::R) reader structure"]
impl crate::Readable for A3 {}
#[doc = "`write(|w| ..)` method takes [a3::W](a3::W) writer structure"]
impl crate::Writable for A3 {}
#[doc = "Slope of 4th piece wise linear function"]
pub mod a3;
#[doc = "Slope of 5th piece wise linear function\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [a4](a4) module"]
pub type A4 = crate::Reg<u32, _A4>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _A4;
#[doc = "`read()` method returns [a4::R](a4::R) reader structure"]
impl crate::Readable for A4 {}
#[doc = "`write(|w| ..)` method takes [a4::W](a4::W) writer structure"]
impl crate::Writable for A4 {}
#[doc = "Slope of 5th piece wise linear function"]
pub mod a4;
#[doc = "Slope of 6th piece wise linear function\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [a5](a5) module"]
pub type A5 = crate::Reg<u32, _A5>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _A5;
#[doc = "`read()` method returns [a5::R](a5::R) reader structure"]
impl crate::Readable for A5 {}
#[doc = "`write(|w| ..)` method takes [a5::W](a5::W) writer structure"]
impl crate::Writable for A5 {}
#[doc = "Slope of 6th piece wise linear function"]
pub mod a5;
#[doc = "y-intercept of 1st piece wise linear function\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [b0](b0) module"]
pub type B0 = crate::Reg<u32, _B0>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _B0;
#[doc = "`read()` method returns [b0::R](b0::R) reader structure"]
impl crate::Readable for B0 {}
#[doc = "`write(|w| ..)` method takes [b0::W](b0::W) writer structure"]
impl crate::Writable for B0 {}
#[doc = "y-intercept of 1st piece wise linear function"]
pub mod b0;
#[doc = "y-intercept of 2nd piece wise linear function\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [b1](b1) module"]
pub type B1 = crate::Reg<u32, _B1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _B1;
#[doc = "`read()` method returns [b1::R](b1::R) reader structure"]
impl crate::Readable for B1 {}
#[doc = "`write(|w| ..)` method takes [b1::W](b1::W) writer structure"]
impl crate::Writable for B1 {}
#[doc = "y-intercept of 2nd piece wise linear function"]
pub mod b1;
#[doc = "y-intercept of 3rd piece wise linear function\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [b2](b2) module"]
pub type B2 = crate::Reg<u32, _B2>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _B2;
#[doc = "`read()` method returns [b2::R](b2::R) reader structure"]
impl crate::Readable for B2 {}
#[doc = "`write(|w| ..)` method takes [b2::W](b2::W) writer structure"]
impl crate::Writable for B2 {}
#[doc = "y-intercept of 3rd piece wise linear function"]
pub mod b2;
#[doc = "y-intercept of 4th piece wise linear function\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [b3](b3) module"]
pub type B3 = crate::Reg<u32, _B3>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _B3;
#[doc = "`read()` method returns [b3::R](b3::R) reader structure"]
impl crate::Readable for B3 {}
#[doc = "`write(|w| ..)` method takes [b3::W](b3::W) writer structure"]
impl crate::Writable for B3 {}
#[doc = "y-intercept of 4th piece wise linear function"]
pub mod b3;
#[doc = "y-intercept of 5th piece wise linear function\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [b4](b4) module"]
pub type B4 = crate::Reg<u32, _B4>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _B4;
#[doc = "`read()` method returns [b4::R](b4::R) reader structure"]
impl crate::Readable for B4 {}
#[doc = "`write(|w| ..)` method takes [b4::W](b4::W) writer structure"]
impl crate::Writable for B4 {}
#[doc = "y-intercept of 5th piece wise linear function"]
pub mod b4;
#[doc = "y-intercept of 6th piece wise linear function\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [b5](b5) module"]
pub type B5 = crate::Reg<u32, _B5>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _B5;
#[doc = "`read()` method returns [b5::R](b5::R) reader structure"]
impl crate::Readable for B5 {}
#[doc = "`write(|w| ..)` method takes [b5::W](b5::W) writer structure"]
impl crate::Writable for B5 {}
#[doc = "y-intercept of 6th piece wise linear function"]
pub mod b5;
#[doc = "End point of 1st piece wise linear function\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [t0](t0) module"]
pub type T0 = crate::Reg<u32, _T0>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _T0;
#[doc = "`read()` method returns [t0::R](t0::R) reader structure"]
impl crate::Readable for T0 {}
#[doc = "`write(|w| ..)` method takes [t0::W](t0::W) writer structure"]
impl crate::Writable for T0 {}
#[doc = "End point of 1st piece wise linear function"]
pub mod t0;
#[doc = "End point of 2nd piece wise linear function\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [t1](t1) module"]
pub type T1 = crate::Reg<u32, _T1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _T1;
#[doc = "`read()` method returns [t1::R](t1::R) reader structure"]
impl crate::Readable for T1 {}
#[doc = "`write(|w| ..)` method takes [t1::W](t1::W) writer structure"]
impl crate::Writable for T1 {}
#[doc = "End point of 2nd piece wise linear function"]
pub mod t1;
#[doc = "End point of 3rd piece wise linear function\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [t2](t2) module"]
pub type T2 = crate::Reg<u32, _T2>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _T2;
#[doc = "`read()` method returns [t2::R](t2::R) reader structure"]
impl crate::Readable for T2 {}
#[doc = "`write(|w| ..)` method takes [t2::W](t2::W) writer structure"]
impl crate::Writable for T2 {}
#[doc = "End point of 3rd piece wise linear function"]
pub mod t2;
#[doc = "End point of 4th piece wise linear function\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [t3](t3) module"]
pub type T3 = crate::Reg<u32, _T3>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _T3;
#[doc = "`read()` method returns [t3::R](t3::R) reader structure"]
impl crate::Readable for T3 {}
#[doc = "`write(|w| ..)` method takes [t3::W](t3::W) writer structure"]
impl crate::Writable for T3 {}
#[doc = "End point of 4th piece wise linear function"]
pub mod t3;
#[doc = "End point of 5th piece wise linear function\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [t4](t4) module"]
pub type T4 = crate::Reg<u32, _T4>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _T4;
#[doc = "`read()` method returns [t4::R](t4::R) reader structure"]
impl crate::Readable for T4 {}
#[doc = "`write(|w| ..)` method takes [t4::W](t4::W) writer structure"]
impl crate::Writable for T4 {}
#[doc = "End point of 5th piece wise linear function"]
pub mod t4;
