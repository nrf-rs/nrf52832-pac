#[doc = "Reader of register CURRENTLOADCTRL"]
pub type R = crate::R<u32, super::CURRENTLOADCTRL>;
#[doc = "Reader of field `CURRENTLOADCTRL`"]
pub type CURRENTLOADCTRL_R = crate::R<u8, u8>;
impl R {
    #[doc = "Bits 0:5 - Current value driven to the NFC Load Control"]
    #[inline(always)]
    pub fn currentloadctrl(&self) -> CURRENTLOADCTRL_R {
        CURRENTLOADCTRL_R::new((self.bits & 0x3f) as u8)
    }
}
