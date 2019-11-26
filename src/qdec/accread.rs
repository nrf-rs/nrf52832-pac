#[doc = "Reader of register ACCREAD"]
pub type R = crate::R<u32, super::ACCREAD>;
#[doc = "Reader of field `ACCREAD`"]
pub type ACCREAD_R = crate::R<u32, u32>;
impl R {
    #[doc = "Bits 0:31 - Snapshot of the ACC register."]
    #[inline(always)]
    pub fn accread(&self) -> ACCREAD_R {
        ACCREAD_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
