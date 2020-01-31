#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Unused."]
    pub unused: UNUSED,
}
#[doc = "Unused.\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [unused](unused) module"]
pub type UNUSED = crate::Reg<u32, _UNUSED>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _UNUSED;
#[doc = "`read()` method returns [unused::R](unused::R) reader structure"]
impl crate::Readable for UNUSED {}
#[doc = "Unused."]
pub mod unused;
