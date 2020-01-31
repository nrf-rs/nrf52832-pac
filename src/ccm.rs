#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Start generation of key-stream. This operation will stop by itself when completed."]
    pub tasks_ksgen: TASKS_KSGEN,
    #[doc = "0x04 - Start encryption/decryption. This operation will stop by itself when completed."]
    pub tasks_crypt: TASKS_CRYPT,
    #[doc = "0x08 - Stop encryption/decryption"]
    pub tasks_stop: TASKS_STOP,
    _reserved3: [u8; 244usize],
    #[doc = "0x100 - Key-stream generation complete"]
    pub events_endksgen: EVENTS_ENDKSGEN,
    #[doc = "0x104 - Encrypt/decrypt complete"]
    pub events_endcrypt: EVENTS_ENDCRYPT,
    #[doc = "0x108 - CCM error event"]
    pub events_error: EVENTS_ERROR,
    _reserved6: [u8; 244usize],
    #[doc = "0x200 - Shortcut register"]
    pub shorts: SHORTS,
    _reserved7: [u8; 256usize],
    #[doc = "0x304 - Enable interrupt"]
    pub intenset: INTENSET,
    #[doc = "0x308 - Disable interrupt"]
    pub intenclr: INTENCLR,
    _reserved9: [u8; 244usize],
    #[doc = "0x400 - MIC check result"]
    pub micstatus: MICSTATUS,
    _reserved10: [u8; 252usize],
    #[doc = "0x500 - Enable"]
    pub enable: ENABLE,
    #[doc = "0x504 - Operation mode"]
    pub mode: MODE,
    #[doc = "0x508 - Pointer to data structure holding AES key and NONCE vector"]
    pub cnfptr: CNFPTR,
    #[doc = "0x50c - Input pointer"]
    pub inptr: INPTR,
    #[doc = "0x510 - Output pointer"]
    pub outptr: OUTPTR,
    #[doc = "0x514 - Pointer to data area used for temporary storage"]
    pub scratchptr: SCRATCHPTR,
}
#[doc = "Start generation of key-stream. This operation will stop by itself when completed.\n\nThis register you can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tasks_ksgen](tasks_ksgen) module"]
pub type TASKS_KSGEN = crate::Reg<u32, _TASKS_KSGEN>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TASKS_KSGEN;
#[doc = "`write(|w| ..)` method takes [tasks_ksgen::W](tasks_ksgen::W) writer structure"]
impl crate::Writable for TASKS_KSGEN {}
#[doc = "Start generation of key-stream. This operation will stop by itself when completed."]
pub mod tasks_ksgen;
#[doc = "Start encryption/decryption. This operation will stop by itself when completed.\n\nThis register you can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tasks_crypt](tasks_crypt) module"]
pub type TASKS_CRYPT = crate::Reg<u32, _TASKS_CRYPT>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TASKS_CRYPT;
#[doc = "`write(|w| ..)` method takes [tasks_crypt::W](tasks_crypt::W) writer structure"]
impl crate::Writable for TASKS_CRYPT {}
#[doc = "Start encryption/decryption. This operation will stop by itself when completed."]
pub mod tasks_crypt;
#[doc = "Stop encryption/decryption\n\nThis register you can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tasks_stop](tasks_stop) module"]
pub type TASKS_STOP = crate::Reg<u32, _TASKS_STOP>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TASKS_STOP;
#[doc = "`write(|w| ..)` method takes [tasks_stop::W](tasks_stop::W) writer structure"]
impl crate::Writable for TASKS_STOP {}
#[doc = "Stop encryption/decryption"]
pub mod tasks_stop;
#[doc = "Key-stream generation complete\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [events_endksgen](events_endksgen) module"]
pub type EVENTS_ENDKSGEN = crate::Reg<u32, _EVENTS_ENDKSGEN>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _EVENTS_ENDKSGEN;
#[doc = "`read()` method returns [events_endksgen::R](events_endksgen::R) reader structure"]
impl crate::Readable for EVENTS_ENDKSGEN {}
#[doc = "`write(|w| ..)` method takes [events_endksgen::W](events_endksgen::W) writer structure"]
impl crate::Writable for EVENTS_ENDKSGEN {}
#[doc = "Key-stream generation complete"]
pub mod events_endksgen;
#[doc = "Encrypt/decrypt complete\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [events_endcrypt](events_endcrypt) module"]
pub type EVENTS_ENDCRYPT = crate::Reg<u32, _EVENTS_ENDCRYPT>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _EVENTS_ENDCRYPT;
#[doc = "`read()` method returns [events_endcrypt::R](events_endcrypt::R) reader structure"]
impl crate::Readable for EVENTS_ENDCRYPT {}
#[doc = "`write(|w| ..)` method takes [events_endcrypt::W](events_endcrypt::W) writer structure"]
impl crate::Writable for EVENTS_ENDCRYPT {}
#[doc = "Encrypt/decrypt complete"]
pub mod events_endcrypt;
#[doc = "CCM error event\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [events_error](events_error) module"]
pub type EVENTS_ERROR = crate::Reg<u32, _EVENTS_ERROR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _EVENTS_ERROR;
#[doc = "`read()` method returns [events_error::R](events_error::R) reader structure"]
impl crate::Readable for EVENTS_ERROR {}
#[doc = "`write(|w| ..)` method takes [events_error::W](events_error::W) writer structure"]
impl crate::Writable for EVENTS_ERROR {}
#[doc = "CCM error event"]
pub mod events_error;
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
#[doc = "MIC check result\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [micstatus](micstatus) module"]
pub type MICSTATUS = crate::Reg<u32, _MICSTATUS>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _MICSTATUS;
#[doc = "`read()` method returns [micstatus::R](micstatus::R) reader structure"]
impl crate::Readable for MICSTATUS {}
#[doc = "MIC check result"]
pub mod micstatus;
#[doc = "Enable\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [enable](enable) module"]
pub type ENABLE = crate::Reg<u32, _ENABLE>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _ENABLE;
#[doc = "`read()` method returns [enable::R](enable::R) reader structure"]
impl crate::Readable for ENABLE {}
#[doc = "`write(|w| ..)` method takes [enable::W](enable::W) writer structure"]
impl crate::Writable for ENABLE {}
#[doc = "Enable"]
pub mod enable;
#[doc = "Operation mode\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mode](mode) module"]
pub type MODE = crate::Reg<u32, _MODE>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _MODE;
#[doc = "`read()` method returns [mode::R](mode::R) reader structure"]
impl crate::Readable for MODE {}
#[doc = "`write(|w| ..)` method takes [mode::W](mode::W) writer structure"]
impl crate::Writable for MODE {}
#[doc = "Operation mode"]
pub mod mode;
#[doc = "Pointer to data structure holding AES key and NONCE vector\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cnfptr](cnfptr) module"]
pub type CNFPTR = crate::Reg<u32, _CNFPTR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CNFPTR;
#[doc = "`read()` method returns [cnfptr::R](cnfptr::R) reader structure"]
impl crate::Readable for CNFPTR {}
#[doc = "`write(|w| ..)` method takes [cnfptr::W](cnfptr::W) writer structure"]
impl crate::Writable for CNFPTR {}
#[doc = "Pointer to data structure holding AES key and NONCE vector"]
pub mod cnfptr;
#[doc = "Input pointer\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [inptr](inptr) module"]
pub type INPTR = crate::Reg<u32, _INPTR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _INPTR;
#[doc = "`read()` method returns [inptr::R](inptr::R) reader structure"]
impl crate::Readable for INPTR {}
#[doc = "`write(|w| ..)` method takes [inptr::W](inptr::W) writer structure"]
impl crate::Writable for INPTR {}
#[doc = "Input pointer"]
pub mod inptr;
#[doc = "Output pointer\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [outptr](outptr) module"]
pub type OUTPTR = crate::Reg<u32, _OUTPTR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _OUTPTR;
#[doc = "`read()` method returns [outptr::R](outptr::R) reader structure"]
impl crate::Readable for OUTPTR {}
#[doc = "`write(|w| ..)` method takes [outptr::W](outptr::W) writer structure"]
impl crate::Writable for OUTPTR {}
#[doc = "Output pointer"]
pub mod outptr;
#[doc = "Pointer to data area used for temporary storage\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [scratchptr](scratchptr) module"]
pub type SCRATCHPTR = crate::Reg<u32, _SCRATCHPTR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SCRATCHPTR;
#[doc = "`read()` method returns [scratchptr::R](scratchptr::R) reader structure"]
impl crate::Readable for SCRATCHPTR {}
#[doc = "`write(|w| ..)` method takes [scratchptr::W](scratchptr::W) writer structure"]
impl crate::Writable for SCRATCHPTR {}
#[doc = "Pointer to data area used for temporary storage"]
pub mod scratchptr;
