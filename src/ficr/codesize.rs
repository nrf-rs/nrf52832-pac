#[doc = "Reader of register CODESIZE"]
pub type R = crate::R<u32, super::CODESIZE>;
#[doc = "Reader of field `CODESIZE`"]
pub type CODESIZE_R = crate::R<u32, u32>;
impl R {
    #[doc = "Bits 0:31 - Code memory size in number of pages"]
    #[inline(always)]
    pub fn codesize(&self) -> CODESIZE_R {
        CODESIZE_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
