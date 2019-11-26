#[doc = "Reader of register ER[%s]"]
pub type R = crate::R<u32, super::ER>;
#[doc = "Reader of field `ER`"]
pub type ER_R = crate::R<u32, u32>;
impl R {
    #[doc = "Bits 0:31 - Encryption Root, word n"]
    #[inline(always)]
    pub fn er(&self) -> ER_R {
        ER_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
