#[doc = "Reader of register START"]
pub type R = crate::R<u32, super::START>;
#[doc = "Reader of field `START`"]
pub type START_R = crate::R<u32, u32>;
impl R {
    #[doc = "Bits 0:31 - Reserved for future use"]
    #[inline(always)]
    pub fn start(&self) -> START_R {
        START_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
