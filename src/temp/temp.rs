#[doc = "Reader of register TEMP"]
pub type R = crate::R<u32, super::TEMP>;
#[doc = "Reader of field `TEMP`"]
pub type TEMP_R = crate::R<u32, u32>;
impl R {
    #[doc = "Bits 0:31 - Temperature in degC (0.25deg steps)"]
    #[inline(always)]
    pub fn temp(&self) -> TEMP_R {
        TEMP_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
