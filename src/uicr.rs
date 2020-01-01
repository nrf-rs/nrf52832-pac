#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Unspecified"]
    pub unused0: UNUSED0,
    #[doc = "0x04 - Unspecified"]
    pub unused1: UNUSED1,
    #[doc = "0x08 - Unspecified"]
    pub unused2: UNUSED2,
    _reserved3: [u8; 4usize],
    #[doc = "0x10 - Unspecified"]
    pub unused3: UNUSED3,
    #[doc = "0x14 - Description collection\\[0\\]: Reserved for Nordic firmware design"]
    pub nrffw: [NRFFW; 15],
    #[doc = "0x50 - Description collection\\[0\\]: Reserved for Nordic hardware design"]
    pub nrfhw: [NRFHW; 12],
    #[doc = "0x80 - Description collection\\[0\\]: Reserved for customer"]
    pub customer: [CUSTOMER; 32],
    _reserved7: [u8; 256usize],
    #[doc = "0x200 - Description collection\\[0\\]: Mapping of the nRESET function (see POWER chapter for details)"]
    pub pselreset: [PSELRESET; 2],
    #[doc = "0x208 - Access Port protection"]
    pub approtect: APPROTECT,
    #[doc = "0x20c - Setting of pins dedicated to NFC functionality: NFC antenna or GPIO"]
    pub nfcpins: NFCPINS,
}
#[doc = "Unspecified\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [unused0](unused0) module"]
pub type UNUSED0 = crate::Reg<u32, _UNUSED0>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _UNUSED0;
#[doc = "`read()` method returns [unused0::R](unused0::R) reader structure"]
impl crate::Readable for UNUSED0 {}
#[doc = "`write(|w| ..)` method takes [unused0::W](unused0::W) writer structure"]
impl crate::Writable for UNUSED0 {}
#[doc = "Unspecified"]
pub mod unused0;
#[doc = "Unspecified\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [unused1](unused1) module"]
pub type UNUSED1 = crate::Reg<u32, _UNUSED1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _UNUSED1;
#[doc = "`read()` method returns [unused1::R](unused1::R) reader structure"]
impl crate::Readable for UNUSED1 {}
#[doc = "`write(|w| ..)` method takes [unused1::W](unused1::W) writer structure"]
impl crate::Writable for UNUSED1 {}
#[doc = "Unspecified"]
pub mod unused1;
#[doc = "Unspecified\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [unused2](unused2) module"]
pub type UNUSED2 = crate::Reg<u32, _UNUSED2>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _UNUSED2;
#[doc = "`read()` method returns [unused2::R](unused2::R) reader structure"]
impl crate::Readable for UNUSED2 {}
#[doc = "`write(|w| ..)` method takes [unused2::W](unused2::W) writer structure"]
impl crate::Writable for UNUSED2 {}
#[doc = "Unspecified"]
pub mod unused2;
#[doc = "Unspecified\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [unused3](unused3) module"]
pub type UNUSED3 = crate::Reg<u32, _UNUSED3>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _UNUSED3;
#[doc = "`read()` method returns [unused3::R](unused3::R) reader structure"]
impl crate::Readable for UNUSED3 {}
#[doc = "`write(|w| ..)` method takes [unused3::W](unused3::W) writer structure"]
impl crate::Writable for UNUSED3 {}
#[doc = "Unspecified"]
pub mod unused3;
#[doc = "Description collection\\[0\\]: Reserved for Nordic firmware design\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [nrffw](nrffw) module"]
pub type NRFFW = crate::Reg<u32, _NRFFW>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _NRFFW;
#[doc = "`read()` method returns [nrffw::R](nrffw::R) reader structure"]
impl crate::Readable for NRFFW {}
#[doc = "`write(|w| ..)` method takes [nrffw::W](nrffw::W) writer structure"]
impl crate::Writable for NRFFW {}
#[doc = "Description collection\\[0\\]: Reserved for Nordic firmware design"]
pub mod nrffw;
#[doc = "Description collection\\[0\\]: Reserved for Nordic hardware design\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [nrfhw](nrfhw) module"]
pub type NRFHW = crate::Reg<u32, _NRFHW>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _NRFHW;
#[doc = "`read()` method returns [nrfhw::R](nrfhw::R) reader structure"]
impl crate::Readable for NRFHW {}
#[doc = "`write(|w| ..)` method takes [nrfhw::W](nrfhw::W) writer structure"]
impl crate::Writable for NRFHW {}
#[doc = "Description collection\\[0\\]: Reserved for Nordic hardware design"]
pub mod nrfhw;
#[doc = "Description collection\\[0\\]: Reserved for customer\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [customer](customer) module"]
pub type CUSTOMER = crate::Reg<u32, _CUSTOMER>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CUSTOMER;
#[doc = "`read()` method returns [customer::R](customer::R) reader structure"]
impl crate::Readable for CUSTOMER {}
#[doc = "`write(|w| ..)` method takes [customer::W](customer::W) writer structure"]
impl crate::Writable for CUSTOMER {}
#[doc = "Description collection\\[0\\]: Reserved for customer"]
pub mod customer;
#[doc = "Description collection\\[0\\]: Mapping of the nRESET function (see POWER chapter for details)\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pselreset](pselreset) module"]
pub type PSELRESET = crate::Reg<u32, _PSELRESET>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PSELRESET;
#[doc = "`read()` method returns [pselreset::R](pselreset::R) reader structure"]
impl crate::Readable for PSELRESET {}
#[doc = "`write(|w| ..)` method takes [pselreset::W](pselreset::W) writer structure"]
impl crate::Writable for PSELRESET {}
#[doc = "Description collection\\[0\\]: Mapping of the nRESET function (see POWER chapter for details)"]
pub mod pselreset;
#[doc = "Access Port protection\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [approtect](approtect) module"]
pub type APPROTECT = crate::Reg<u32, _APPROTECT>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _APPROTECT;
#[doc = "`read()` method returns [approtect::R](approtect::R) reader structure"]
impl crate::Readable for APPROTECT {}
#[doc = "`write(|w| ..)` method takes [approtect::W](approtect::W) writer structure"]
impl crate::Writable for APPROTECT {}
#[doc = "Access Port protection"]
pub mod approtect;
#[doc = "Setting of pins dedicated to NFC functionality: NFC antenna or GPIO\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [nfcpins](nfcpins) module"]
pub type NFCPINS = crate::Reg<u32, _NFCPINS>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _NFCPINS;
#[doc = "`read()` method returns [nfcpins::R](nfcpins::R) reader structure"]
impl crate::Readable for NFCPINS {}
#[doc = "`write(|w| ..)` method takes [nfcpins::W](nfcpins::W) writer structure"]
impl crate::Writable for NFCPINS {}
#[doc = "Setting of pins dedicated to NFC functionality: NFC antenna or GPIO"]
pub mod nfcpins;
