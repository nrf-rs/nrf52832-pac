#[doc = "Reader of register B4"]
pub type R = crate::R<u32, super::B4>;
#[doc = "Reader of field `B`"]
pub type B_R = crate::R<u16, u16>;
impl R {
    #[doc = "Bits 0:13 - B (y-intercept)"]
    #[inline(always)]
    pub fn b(&self) -> B_R {
        B_R::new((self.bits & 0x3fff) as u16)
    }
}
