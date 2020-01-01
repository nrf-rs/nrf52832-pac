#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    _reserved0: [u8; 120usize],
    #[doc = "0x78 - Enable constant latency mode"]
    pub tasks_constlat: TASKS_CONSTLAT,
    #[doc = "0x7c - Enable low power mode (variable latency)"]
    pub tasks_lowpwr: TASKS_LOWPWR,
    _reserved2: [u8; 136usize],
    #[doc = "0x108 - Power failure warning"]
    pub events_pofwarn: EVENTS_POFWARN,
    _reserved3: [u8; 8usize],
    #[doc = "0x114 - CPU entered WFI/WFE sleep"]
    pub events_sleepenter: EVENTS_SLEEPENTER,
    #[doc = "0x118 - CPU exited WFI/WFE sleep"]
    pub events_sleepexit: EVENTS_SLEEPEXIT,
    _reserved5: [u8; 488usize],
    #[doc = "0x304 - Enable interrupt"]
    pub intenset: INTENSET,
    #[doc = "0x308 - Disable interrupt"]
    pub intenclr: INTENCLR,
    _reserved7: [u8; 244usize],
    #[doc = "0x400 - Reset reason"]
    pub resetreas: RESETREAS,
    _reserved8: [u8; 36usize],
    #[doc = "0x428 - Deprecated register - RAM status register"]
    pub ramstatus: RAMSTATUS,
    _reserved9: [u8; 212usize],
    #[doc = "0x500 - System OFF register"]
    pub systemoff: SYSTEMOFF,
    _reserved10: [u8; 12usize],
    #[doc = "0x510 - Power failure comparator configuration"]
    pub pofcon: POFCON,
    _reserved11: [u8; 8usize],
    #[doc = "0x51c - General purpose retention register"]
    pub gpregret: GPREGRET,
    #[doc = "0x520 - General purpose retention register"]
    pub gpregret2: GPREGRET2,
    #[doc = "0x524 - Deprecated register - RAM on/off register (this register is retained)"]
    pub ramon: RAMON,
    _reserved14: [u8; 44usize],
    #[doc = "0x554 - Deprecated register - RAM on/off register (this register is retained)"]
    pub ramonb: RAMONB,
    _reserved15: [u8; 32usize],
    #[doc = "0x578 - DC/DC enable register"]
    pub dcdcen: DCDCEN,
    _reserved16: [u8; 900usize],
    #[doc = "0x900 - Unspecified"]
    pub ram0: RAM,
    _reserved17: [u8; 4usize],
    #[doc = "0x910 - Unspecified"]
    pub ram1: RAM,
    _reserved18: [u8; 4usize],
    #[doc = "0x920 - Unspecified"]
    pub ram2: RAM,
    _reserved19: [u8; 4usize],
    #[doc = "0x930 - Unspecified"]
    pub ram3: RAM,
    _reserved20: [u8; 4usize],
    #[doc = "0x940 - Unspecified"]
    pub ram4: RAM,
    _reserved21: [u8; 4usize],
    #[doc = "0x950 - Unspecified"]
    pub ram5: RAM,
    _reserved22: [u8; 4usize],
    #[doc = "0x960 - Unspecified"]
    pub ram6: RAM,
    _reserved23: [u8; 4usize],
    #[doc = "0x970 - Unspecified"]
    pub ram7: RAM,
}
#[doc = r"Register block"]
#[repr(C)]
pub struct RAM {
    #[doc = "0x00 - Description cluster\\[0\\]: RAM0 power control register"]
    pub power: self::ram::POWER,
    #[doc = "0x04 - Description cluster\\[0\\]: RAM0 power control set register"]
    pub powerset: self::ram::POWERSET,
    #[doc = "0x08 - Description cluster\\[0\\]: RAM0 power control clear register"]
    pub powerclr: self::ram::POWERCLR,
}
#[doc = r"Register block"]
#[doc = "Unspecified"]
pub mod ram;
#[doc = "Enable constant latency mode\n\nThis register you can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tasks_constlat](tasks_constlat) module"]
pub type TASKS_CONSTLAT = crate::Reg<u32, _TASKS_CONSTLAT>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TASKS_CONSTLAT;
#[doc = "`write(|w| ..)` method takes [tasks_constlat::W](tasks_constlat::W) writer structure"]
impl crate::Writable for TASKS_CONSTLAT {}
#[doc = "Enable constant latency mode"]
pub mod tasks_constlat;
#[doc = "Enable low power mode (variable latency)\n\nThis register you can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tasks_lowpwr](tasks_lowpwr) module"]
pub type TASKS_LOWPWR = crate::Reg<u32, _TASKS_LOWPWR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TASKS_LOWPWR;
#[doc = "`write(|w| ..)` method takes [tasks_lowpwr::W](tasks_lowpwr::W) writer structure"]
impl crate::Writable for TASKS_LOWPWR {}
#[doc = "Enable low power mode (variable latency)"]
pub mod tasks_lowpwr;
#[doc = "Power failure warning\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [events_pofwarn](events_pofwarn) module"]
pub type EVENTS_POFWARN = crate::Reg<u32, _EVENTS_POFWARN>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _EVENTS_POFWARN;
#[doc = "`read()` method returns [events_pofwarn::R](events_pofwarn::R) reader structure"]
impl crate::Readable for EVENTS_POFWARN {}
#[doc = "`write(|w| ..)` method takes [events_pofwarn::W](events_pofwarn::W) writer structure"]
impl crate::Writable for EVENTS_POFWARN {}
#[doc = "Power failure warning"]
pub mod events_pofwarn;
#[doc = "CPU entered WFI/WFE sleep\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [events_sleepenter](events_sleepenter) module"]
pub type EVENTS_SLEEPENTER = crate::Reg<u32, _EVENTS_SLEEPENTER>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _EVENTS_SLEEPENTER;
#[doc = "`read()` method returns [events_sleepenter::R](events_sleepenter::R) reader structure"]
impl crate::Readable for EVENTS_SLEEPENTER {}
#[doc = "`write(|w| ..)` method takes [events_sleepenter::W](events_sleepenter::W) writer structure"]
impl crate::Writable for EVENTS_SLEEPENTER {}
#[doc = "CPU entered WFI/WFE sleep"]
pub mod events_sleepenter;
#[doc = "CPU exited WFI/WFE sleep\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [events_sleepexit](events_sleepexit) module"]
pub type EVENTS_SLEEPEXIT = crate::Reg<u32, _EVENTS_SLEEPEXIT>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _EVENTS_SLEEPEXIT;
#[doc = "`read()` method returns [events_sleepexit::R](events_sleepexit::R) reader structure"]
impl crate::Readable for EVENTS_SLEEPEXIT {}
#[doc = "`write(|w| ..)` method takes [events_sleepexit::W](events_sleepexit::W) writer structure"]
impl crate::Writable for EVENTS_SLEEPEXIT {}
#[doc = "CPU exited WFI/WFE sleep"]
pub mod events_sleepexit;
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
#[doc = "Reset reason\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [resetreas](resetreas) module"]
pub type RESETREAS = crate::Reg<u32, _RESETREAS>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RESETREAS;
#[doc = "`read()` method returns [resetreas::R](resetreas::R) reader structure"]
impl crate::Readable for RESETREAS {}
#[doc = "`write(|w| ..)` method takes [resetreas::W](resetreas::W) writer structure"]
impl crate::Writable for RESETREAS {}
#[doc = "Reset reason"]
pub mod resetreas;
#[doc = "Deprecated register - RAM status register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ramstatus](ramstatus) module"]
pub type RAMSTATUS = crate::Reg<u32, _RAMSTATUS>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RAMSTATUS;
#[doc = "`read()` method returns [ramstatus::R](ramstatus::R) reader structure"]
impl crate::Readable for RAMSTATUS {}
#[doc = "Deprecated register - RAM status register"]
pub mod ramstatus;
#[doc = "System OFF register\n\nThis register you can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [systemoff](systemoff) module"]
pub type SYSTEMOFF = crate::Reg<u32, _SYSTEMOFF>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SYSTEMOFF;
#[doc = "`write(|w| ..)` method takes [systemoff::W](systemoff::W) writer structure"]
impl crate::Writable for SYSTEMOFF {}
#[doc = "System OFF register"]
pub mod systemoff;
#[doc = "Power failure comparator configuration\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pofcon](pofcon) module"]
pub type POFCON = crate::Reg<u32, _POFCON>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _POFCON;
#[doc = "`read()` method returns [pofcon::R](pofcon::R) reader structure"]
impl crate::Readable for POFCON {}
#[doc = "`write(|w| ..)` method takes [pofcon::W](pofcon::W) writer structure"]
impl crate::Writable for POFCON {}
#[doc = "Power failure comparator configuration"]
pub mod pofcon;
#[doc = "General purpose retention register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [gpregret](gpregret) module"]
pub type GPREGRET = crate::Reg<u32, _GPREGRET>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _GPREGRET;
#[doc = "`read()` method returns [gpregret::R](gpregret::R) reader structure"]
impl crate::Readable for GPREGRET {}
#[doc = "`write(|w| ..)` method takes [gpregret::W](gpregret::W) writer structure"]
impl crate::Writable for GPREGRET {}
#[doc = "General purpose retention register"]
pub mod gpregret;
#[doc = "General purpose retention register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [gpregret2](gpregret2) module"]
pub type GPREGRET2 = crate::Reg<u32, _GPREGRET2>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _GPREGRET2;
#[doc = "`read()` method returns [gpregret2::R](gpregret2::R) reader structure"]
impl crate::Readable for GPREGRET2 {}
#[doc = "`write(|w| ..)` method takes [gpregret2::W](gpregret2::W) writer structure"]
impl crate::Writable for GPREGRET2 {}
#[doc = "General purpose retention register"]
pub mod gpregret2;
#[doc = "Deprecated register - RAM on/off register (this register is retained)\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ramon](ramon) module"]
pub type RAMON = crate::Reg<u32, _RAMON>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RAMON;
#[doc = "`read()` method returns [ramon::R](ramon::R) reader structure"]
impl crate::Readable for RAMON {}
#[doc = "`write(|w| ..)` method takes [ramon::W](ramon::W) writer structure"]
impl crate::Writable for RAMON {}
#[doc = "Deprecated register - RAM on/off register (this register is retained)"]
pub mod ramon;
#[doc = "Deprecated register - RAM on/off register (this register is retained)\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ramonb](ramonb) module"]
pub type RAMONB = crate::Reg<u32, _RAMONB>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RAMONB;
#[doc = "`read()` method returns [ramonb::R](ramonb::R) reader structure"]
impl crate::Readable for RAMONB {}
#[doc = "`write(|w| ..)` method takes [ramonb::W](ramonb::W) writer structure"]
impl crate::Writable for RAMONB {}
#[doc = "Deprecated register - RAM on/off register (this register is retained)"]
pub mod ramonb;
#[doc = "DC/DC enable register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dcdcen](dcdcen) module"]
pub type DCDCEN = crate::Reg<u32, _DCDCEN>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DCDCEN;
#[doc = "`read()` method returns [dcdcen::R](dcdcen::R) reader structure"]
impl crate::Readable for DCDCEN {}
#[doc = "`write(|w| ..)` method takes [dcdcen::W](dcdcen::W) writer structure"]
impl crate::Writable for DCDCEN {}
#[doc = "DC/DC enable register"]
pub mod dcdcen;
