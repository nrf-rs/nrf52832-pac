#[doc = "Reader of register SAMPLE"]
pub type R = crate::R<u32, super::SAMPLE>;
#[doc = "Reader of field `SAMPLE`"]
pub type SAMPLE_R = crate::R<u32, u32>;
impl R {
    #[doc = "Bits 0:31 - Last motion sample"]
    #[inline(always)]
    pub fn sample(&self) -> SAMPLE_R {
        SAMPLE_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
