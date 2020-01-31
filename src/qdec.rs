#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Task starting the quadrature decoder"]
    pub tasks_start: TASKS_START,
    #[doc = "0x04 - Task stopping the quadrature decoder"]
    pub tasks_stop: TASKS_STOP,
    #[doc = "0x08 - Read and clear ACC and ACCDBL"]
    pub tasks_readclracc: TASKS_READCLRACC,
    #[doc = "0x0c - Read and clear ACC"]
    pub tasks_rdclracc: TASKS_RDCLRACC,
    #[doc = "0x10 - Read and clear ACCDBL"]
    pub tasks_rdclrdbl: TASKS_RDCLRDBL,
    _reserved5: [u8; 236usize],
    #[doc = "0x100 - Event being generated for every new sample value written to the SAMPLE register"]
    pub events_samplerdy: EVENTS_SAMPLERDY,
    #[doc = "0x104 - Non-null report ready"]
    pub events_reportrdy: EVENTS_REPORTRDY,
    #[doc = "0x108 - ACC or ACCDBL register overflow"]
    pub events_accof: EVENTS_ACCOF,
    #[doc = "0x10c - Double displacement(s) detected"]
    pub events_dblrdy: EVENTS_DBLRDY,
    #[doc = "0x110 - QDEC has been stopped"]
    pub events_stopped: EVENTS_STOPPED,
    _reserved10: [u8; 236usize],
    #[doc = "0x200 - Shortcut register"]
    pub shorts: SHORTS,
    _reserved11: [u8; 256usize],
    #[doc = "0x304 - Enable interrupt"]
    pub intenset: INTENSET,
    #[doc = "0x308 - Disable interrupt"]
    pub intenclr: INTENCLR,
    _reserved13: [u8; 500usize],
    #[doc = "0x500 - Enable the quadrature decoder"]
    pub enable: ENABLE,
    #[doc = "0x504 - LED output pin polarity"]
    pub ledpol: LEDPOL,
    #[doc = "0x508 - Sample period"]
    pub sampleper: SAMPLEPER,
    #[doc = "0x50c - Motion sample value"]
    pub sample: SAMPLE,
    #[doc = "0x510 - Number of samples to be taken before REPORTRDY and DBLRDY events can be generated"]
    pub reportper: REPORTPER,
    #[doc = "0x514 - Register accumulating the valid transitions"]
    pub acc: ACC,
    #[doc = "0x518 - Snapshot of the ACC register, updated by the READCLRACC or RDCLRACC task"]
    pub accread: ACCREAD,
    #[doc = "0x51c - Unspecified"]
    pub psel: PSEL,
    #[doc = "0x528 - Enable input debounce filters"]
    pub dbfen: DBFEN,
    _reserved22: [u8; 20usize],
    #[doc = "0x540 - Time period the LED is switched ON prior to sampling"]
    pub ledpre: LEDPRE,
    #[doc = "0x544 - Register accumulating the number of detected double transitions"]
    pub accdbl: ACCDBL,
    #[doc = "0x548 - Snapshot of the ACCDBL, updated by the READCLRACC or RDCLRDBL task"]
    pub accdblread: ACCDBLREAD,
}
#[doc = r"Register block"]
#[repr(C)]
pub struct PSEL {
    #[doc = "0x00 - Pin select for LED signal"]
    pub led: self::psel::LED,
    #[doc = "0x04 - Pin select for A signal"]
    pub a: self::psel::A,
    #[doc = "0x08 - Pin select for B signal"]
    pub b: self::psel::B,
}
#[doc = r"Register block"]
#[doc = "Unspecified"]
pub mod psel;
#[doc = "Task starting the quadrature decoder\n\nThis register you can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tasks_start](tasks_start) module"]
pub type TASKS_START = crate::Reg<u32, _TASKS_START>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TASKS_START;
#[doc = "`write(|w| ..)` method takes [tasks_start::W](tasks_start::W) writer structure"]
impl crate::Writable for TASKS_START {}
#[doc = "Task starting the quadrature decoder"]
pub mod tasks_start;
#[doc = "Task stopping the quadrature decoder\n\nThis register you can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tasks_stop](tasks_stop) module"]
pub type TASKS_STOP = crate::Reg<u32, _TASKS_STOP>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TASKS_STOP;
#[doc = "`write(|w| ..)` method takes [tasks_stop::W](tasks_stop::W) writer structure"]
impl crate::Writable for TASKS_STOP {}
#[doc = "Task stopping the quadrature decoder"]
pub mod tasks_stop;
#[doc = "Read and clear ACC and ACCDBL\n\nThis register you can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tasks_readclracc](tasks_readclracc) module"]
pub type TASKS_READCLRACC = crate::Reg<u32, _TASKS_READCLRACC>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TASKS_READCLRACC;
#[doc = "`write(|w| ..)` method takes [tasks_readclracc::W](tasks_readclracc::W) writer structure"]
impl crate::Writable for TASKS_READCLRACC {}
#[doc = "Read and clear ACC and ACCDBL"]
pub mod tasks_readclracc;
#[doc = "Read and clear ACC\n\nThis register you can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tasks_rdclracc](tasks_rdclracc) module"]
pub type TASKS_RDCLRACC = crate::Reg<u32, _TASKS_RDCLRACC>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TASKS_RDCLRACC;
#[doc = "`write(|w| ..)` method takes [tasks_rdclracc::W](tasks_rdclracc::W) writer structure"]
impl crate::Writable for TASKS_RDCLRACC {}
#[doc = "Read and clear ACC"]
pub mod tasks_rdclracc;
#[doc = "Read and clear ACCDBL\n\nThis register you can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tasks_rdclrdbl](tasks_rdclrdbl) module"]
pub type TASKS_RDCLRDBL = crate::Reg<u32, _TASKS_RDCLRDBL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TASKS_RDCLRDBL;
#[doc = "`write(|w| ..)` method takes [tasks_rdclrdbl::W](tasks_rdclrdbl::W) writer structure"]
impl crate::Writable for TASKS_RDCLRDBL {}
#[doc = "Read and clear ACCDBL"]
pub mod tasks_rdclrdbl;
#[doc = "Event being generated for every new sample value written to the SAMPLE register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [events_samplerdy](events_samplerdy) module"]
pub type EVENTS_SAMPLERDY = crate::Reg<u32, _EVENTS_SAMPLERDY>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _EVENTS_SAMPLERDY;
#[doc = "`read()` method returns [events_samplerdy::R](events_samplerdy::R) reader structure"]
impl crate::Readable for EVENTS_SAMPLERDY {}
#[doc = "`write(|w| ..)` method takes [events_samplerdy::W](events_samplerdy::W) writer structure"]
impl crate::Writable for EVENTS_SAMPLERDY {}
#[doc = "Event being generated for every new sample value written to the SAMPLE register"]
pub mod events_samplerdy;
#[doc = "Non-null report ready\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [events_reportrdy](events_reportrdy) module"]
pub type EVENTS_REPORTRDY = crate::Reg<u32, _EVENTS_REPORTRDY>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _EVENTS_REPORTRDY;
#[doc = "`read()` method returns [events_reportrdy::R](events_reportrdy::R) reader structure"]
impl crate::Readable for EVENTS_REPORTRDY {}
#[doc = "`write(|w| ..)` method takes [events_reportrdy::W](events_reportrdy::W) writer structure"]
impl crate::Writable for EVENTS_REPORTRDY {}
#[doc = "Non-null report ready"]
pub mod events_reportrdy;
#[doc = "ACC or ACCDBL register overflow\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [events_accof](events_accof) module"]
pub type EVENTS_ACCOF = crate::Reg<u32, _EVENTS_ACCOF>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _EVENTS_ACCOF;
#[doc = "`read()` method returns [events_accof::R](events_accof::R) reader structure"]
impl crate::Readable for EVENTS_ACCOF {}
#[doc = "`write(|w| ..)` method takes [events_accof::W](events_accof::W) writer structure"]
impl crate::Writable for EVENTS_ACCOF {}
#[doc = "ACC or ACCDBL register overflow"]
pub mod events_accof;
#[doc = "Double displacement(s) detected\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [events_dblrdy](events_dblrdy) module"]
pub type EVENTS_DBLRDY = crate::Reg<u32, _EVENTS_DBLRDY>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _EVENTS_DBLRDY;
#[doc = "`read()` method returns [events_dblrdy::R](events_dblrdy::R) reader structure"]
impl crate::Readable for EVENTS_DBLRDY {}
#[doc = "`write(|w| ..)` method takes [events_dblrdy::W](events_dblrdy::W) writer structure"]
impl crate::Writable for EVENTS_DBLRDY {}
#[doc = "Double displacement(s) detected"]
pub mod events_dblrdy;
#[doc = "QDEC has been stopped\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [events_stopped](events_stopped) module"]
pub type EVENTS_STOPPED = crate::Reg<u32, _EVENTS_STOPPED>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _EVENTS_STOPPED;
#[doc = "`read()` method returns [events_stopped::R](events_stopped::R) reader structure"]
impl crate::Readable for EVENTS_STOPPED {}
#[doc = "`write(|w| ..)` method takes [events_stopped::W](events_stopped::W) writer structure"]
impl crate::Writable for EVENTS_STOPPED {}
#[doc = "QDEC has been stopped"]
pub mod events_stopped;
#[doc = "Shortcut register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [shorts](shorts) module"]
pub type SHORTS = crate::Reg<u32, _SHORTS>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SHORTS;
#[doc = "`read()` method returns [shorts::R](shorts::R) reader structure"]
impl crate::Readable for SHORTS {}
#[doc = "`write(|w| ..)` method takes [shorts::W](shorts::W) writer structure"]
impl crate::Writable for SHORTS {}
#[doc = "Shortcut register"]
pub mod shorts;
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
#[doc = "Enable the quadrature decoder\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [enable](enable) module"]
pub type ENABLE = crate::Reg<u32, _ENABLE>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _ENABLE;
#[doc = "`read()` method returns [enable::R](enable::R) reader structure"]
impl crate::Readable for ENABLE {}
#[doc = "`write(|w| ..)` method takes [enable::W](enable::W) writer structure"]
impl crate::Writable for ENABLE {}
#[doc = "Enable the quadrature decoder"]
pub mod enable;
#[doc = "LED output pin polarity\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ledpol](ledpol) module"]
pub type LEDPOL = crate::Reg<u32, _LEDPOL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _LEDPOL;
#[doc = "`read()` method returns [ledpol::R](ledpol::R) reader structure"]
impl crate::Readable for LEDPOL {}
#[doc = "`write(|w| ..)` method takes [ledpol::W](ledpol::W) writer structure"]
impl crate::Writable for LEDPOL {}
#[doc = "LED output pin polarity"]
pub mod ledpol;
#[doc = "Sample period\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sampleper](sampleper) module"]
pub type SAMPLEPER = crate::Reg<u32, _SAMPLEPER>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SAMPLEPER;
#[doc = "`read()` method returns [sampleper::R](sampleper::R) reader structure"]
impl crate::Readable for SAMPLEPER {}
#[doc = "`write(|w| ..)` method takes [sampleper::W](sampleper::W) writer structure"]
impl crate::Writable for SAMPLEPER {}
#[doc = "Sample period"]
pub mod sampleper;
#[doc = "Motion sample value\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sample](sample) module"]
pub type SAMPLE = crate::Reg<u32, _SAMPLE>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SAMPLE;
#[doc = "`read()` method returns [sample::R](sample::R) reader structure"]
impl crate::Readable for SAMPLE {}
#[doc = "Motion sample value"]
pub mod sample;
#[doc = "Number of samples to be taken before REPORTRDY and DBLRDY events can be generated\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [reportper](reportper) module"]
pub type REPORTPER = crate::Reg<u32, _REPORTPER>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _REPORTPER;
#[doc = "`read()` method returns [reportper::R](reportper::R) reader structure"]
impl crate::Readable for REPORTPER {}
#[doc = "`write(|w| ..)` method takes [reportper::W](reportper::W) writer structure"]
impl crate::Writable for REPORTPER {}
#[doc = "Number of samples to be taken before REPORTRDY and DBLRDY events can be generated"]
pub mod reportper;
#[doc = "Register accumulating the valid transitions\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [acc](acc) module"]
pub type ACC = crate::Reg<u32, _ACC>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _ACC;
#[doc = "`read()` method returns [acc::R](acc::R) reader structure"]
impl crate::Readable for ACC {}
#[doc = "Register accumulating the valid transitions"]
pub mod acc;
#[doc = "Snapshot of the ACC register, updated by the READCLRACC or RDCLRACC task\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [accread](accread) module"]
pub type ACCREAD = crate::Reg<u32, _ACCREAD>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _ACCREAD;
#[doc = "`read()` method returns [accread::R](accread::R) reader structure"]
impl crate::Readable for ACCREAD {}
#[doc = "Snapshot of the ACC register, updated by the READCLRACC or RDCLRACC task"]
pub mod accread;
#[doc = "Enable input debounce filters\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dbfen](dbfen) module"]
pub type DBFEN = crate::Reg<u32, _DBFEN>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DBFEN;
#[doc = "`read()` method returns [dbfen::R](dbfen::R) reader structure"]
impl crate::Readable for DBFEN {}
#[doc = "`write(|w| ..)` method takes [dbfen::W](dbfen::W) writer structure"]
impl crate::Writable for DBFEN {}
#[doc = "Enable input debounce filters"]
pub mod dbfen;
#[doc = "Time period the LED is switched ON prior to sampling\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ledpre](ledpre) module"]
pub type LEDPRE = crate::Reg<u32, _LEDPRE>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _LEDPRE;
#[doc = "`read()` method returns [ledpre::R](ledpre::R) reader structure"]
impl crate::Readable for LEDPRE {}
#[doc = "`write(|w| ..)` method takes [ledpre::W](ledpre::W) writer structure"]
impl crate::Writable for LEDPRE {}
#[doc = "Time period the LED is switched ON prior to sampling"]
pub mod ledpre;
#[doc = "Register accumulating the number of detected double transitions\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [accdbl](accdbl) module"]
pub type ACCDBL = crate::Reg<u32, _ACCDBL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _ACCDBL;
#[doc = "`read()` method returns [accdbl::R](accdbl::R) reader structure"]
impl crate::Readable for ACCDBL {}
#[doc = "Register accumulating the number of detected double transitions"]
pub mod accdbl;
#[doc = "Snapshot of the ACCDBL, updated by the READCLRACC or RDCLRDBL task\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [accdblread](accdblread) module"]
pub type ACCDBLREAD = crate::Reg<u32, _ACCDBLREAD>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _ACCDBLREAD;
#[doc = "`read()` method returns [accdblread::R](accdblread::R) reader structure"]
impl crate::Readable for ACCDBLREAD {}
#[doc = "Snapshot of the ACCDBL, updated by the READCLRACC or RDCLRDBL task"]
pub mod accdblread;
