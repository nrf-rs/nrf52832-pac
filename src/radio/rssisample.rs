#[doc = "Reader of register RSSISAMPLE"]
pub type R = crate::R<u32, super::RSSISAMPLE>;
#[doc = "Reader of field `RSSISAMPLE`"]
pub type RSSISAMPLE_R = crate::R<u8, u8>;
impl R {
    #[doc = "Bits 0:6 - RSSI sample"]
    #[inline(always)]
    pub fn rssisample(&self) -> RSSISAMPLE_R {
        RSSISAMPLE_R::new((self.bits & 0x7f) as u8)
    }
}
