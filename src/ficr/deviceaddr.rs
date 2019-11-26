#[doc = "Reader of register DEVICEADDR[%s]"]
pub type R = crate::R<u32, super::DEVICEADDR>;
#[doc = "Reader of field `DEVICEADDR`"]
pub type DEVICEADDR_R = crate::R<u32, u32>;
impl R {
    #[doc = "Bits 0:31 - 48 bit device address"]
    #[inline(always)]
    pub fn deviceaddr(&self) -> DEVICEADDR_R {
        DEVICEADDR_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
