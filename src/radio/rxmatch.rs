#[doc = "Reader of register RXMATCH"]
pub type R = crate::R<u32, super::RXMATCH>;
#[doc = "Reader of field `RXMATCH`"]
pub type RXMATCH_R = crate::R<u8, u8>;
impl R {
    #[doc = "Bits 0:2 - Received address"]
    #[inline(always)]
    pub fn rxmatch(&self) -> RXMATCH_R {
        RXMATCH_R::new((self.bits & 0x07) as u8)
    }
}
