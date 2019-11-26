#[doc = "Reader of register CODEPAGESIZE"]
pub type R = crate::R<u32, super::CODEPAGESIZE>;
#[doc = "Reader of field `CODEPAGESIZE`"]
pub type CODEPAGESIZE_R = crate::R<u32, u32>;
impl R {
    #[doc = "Bits 0:31 - Code memory page size"]
    #[inline(always)]
    pub fn codepagesize(&self) -> CODEPAGESIZE_R {
        CODEPAGESIZE_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
