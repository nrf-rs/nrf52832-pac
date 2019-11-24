#[doc = "Reader of register DAI"]
pub type R = crate::R<u32, super::DAI>;
#[doc = "Reader of field `DAI`"]
pub type DAI_R = crate::R<u8, u8>;
impl R {
    #[doc = "Bits 0:2 - Device address match index"]
    #[inline(always)]
    pub fn dai(&self) -> DAI_R {
        DAI_R::new((self.bits & 0x07) as u8)
    }
}
