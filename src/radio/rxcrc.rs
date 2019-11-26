#[doc = "Reader of register RXCRC"]
pub type R = crate::R<u32, super::RXCRC>;
#[doc = "Reader of field `RXCRC`"]
pub type RXCRC_R = crate::R<u32, u32>;
impl R {
    #[doc = "Bits 0:23 - CRC field of previously received packet"]
    #[inline(always)]
    pub fn rxcrc(&self) -> RXCRC_R {
        RXCRC_R::new((self.bits & 0x00ff_ffff) as u32)
    }
}
