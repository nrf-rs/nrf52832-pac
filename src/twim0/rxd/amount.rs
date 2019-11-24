#[doc = "Reader of register AMOUNT"]
pub type R = crate::R<u32, super::AMOUNT>;
#[doc = "Reader of field `AMOUNT`"]
pub type AMOUNT_R = crate::R<u8, u8>;
impl R {
    #[doc = "Bits 0:7 - Number of bytes transferred in the last transaction. In case of NACK error, includes the NACK'ed byte."]
    #[inline(always)]
    pub fn amount(&self) -> AMOUNT_R {
        AMOUNT_R::new((self.bits & 0xff) as u8)
    }
}
