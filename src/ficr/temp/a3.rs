#[doc = "Reader of register A3"]
pub type R = crate::R<u32, super::A3>;
#[doc = "Reader of field `A`"]
pub type A_R = crate::R<u16, u16>;
impl R {
    #[doc = "Bits 0:11 - A (slope definition) register."]
    #[inline(always)]
    pub fn a(&self) -> A_R {
        A_R::new((self.bits & 0x0fff) as u16)
    }
}
