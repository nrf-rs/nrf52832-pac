#[doc = "Reader of register COUNTER"]
pub type R = crate::R<u32, super::COUNTER>;
#[doc = "Reader of field `COUNTER`"]
pub type COUNTER_R = crate::R<u32, u32>;
impl R {
    #[doc = "Bits 0:23 - Counter value"]
    #[inline(always)]
    pub fn counter(&self) -> COUNTER_R {
        COUNTER_R::new((self.bits & 0x00ff_ffff) as u32)
    }
}
