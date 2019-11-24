#[doc = "Reader of register AMOUNT"]
pub type R = crate::R<u32, super::AMOUNT>;
#[doc = "Reader of field `AMOUNT`"]
pub type AMOUNT_R = crate::R<u16, u16>;
impl R {
    #[doc = "Bits 0:14 - Number of buffer words transferred since last START. This register can be read after an END or STOPPED event."]
    #[inline(always)]
    pub fn amount(&self) -> AMOUNT_R {
        AMOUNT_R::new((self.bits & 0x7fff) as u16)
    }
}
