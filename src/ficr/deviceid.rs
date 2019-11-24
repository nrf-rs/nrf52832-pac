#[doc = "Reader of register DEVICEID[%s]"]
pub type R = crate::R<u32, super::DEVICEID>;
#[doc = "Reader of field `DEVICEID`"]
pub type DEVICEID_R = crate::R<u32, u32>;
impl R {
    #[doc = "Bits 0:31 - 64 bit unique device identifier"]
    #[inline(always)]
    pub fn deviceid(&self) -> DEVICEID_R {
        DEVICEID_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
