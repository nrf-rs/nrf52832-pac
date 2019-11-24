#[doc = "Reader of register AMOUNT"]
pub type R = crate::R<u32, super::AMOUNT>;
#[doc = "Reader of field `RXDATABITS`"]
pub type RXDATABITS_R = crate::R<u8, u8>;
#[doc = "Reader of field `RXDATABYTES`"]
pub type RXDATABYTES_R = crate::R<u16, u16>;
impl R {
    #[doc = "Bits 0:2 - Number of bits in the last byte in the frame, if less than 8 (including CRC, but excluding parity and SoF/EoF framing)."]
    #[inline(always)]
    pub fn rxdatabits(&self) -> RXDATABITS_R {
        RXDATABITS_R::new((self.bits & 0x07) as u8)
    }
    #[doc = "Bits 3:11 - Number of complete bytes received in the frame (including CRC, but excluding parity and SoF/EoF framing)"]
    #[inline(always)]
    pub fn rxdatabytes(&self) -> RXDATABYTES_R {
        RXDATABYTES_R::new(((self.bits >> 3) & 0x01ff) as u16)
    }
}
