#[doc = "Reader of register FRAMEDELAYMIN"]
pub type R = crate::R<u32, super::FRAMEDELAYMIN>;
#[doc = "Writer for register FRAMEDELAYMIN"]
pub type W = crate::W<u32, super::FRAMEDELAYMIN>;
#[doc = "Register FRAMEDELAYMIN `reset()`'s with value 0x0480"]
impl crate::ResetValue for super::FRAMEDELAYMIN {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0x0480
    }
}
#[doc = "Reader of field `FRAMEDELAYMIN`"]
pub type FRAMEDELAYMIN_R = crate::R<u16, u16>;
#[doc = "Write proxy for field `FRAMEDELAYMIN`"]
pub struct FRAMEDELAYMIN_W<'a> {
    w: &'a mut W,
}
impl<'a> FRAMEDELAYMIN_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff) | ((value as u32) & 0xffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:15 - Minimum frame delay in number of 13.56 MHz clocks"]
    #[inline(always)]
    pub fn framedelaymin(&self) -> FRAMEDELAYMIN_R {
        FRAMEDELAYMIN_R::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - Minimum frame delay in number of 13.56 MHz clocks"]
    #[inline(always)]
    pub fn framedelaymin(&mut self) -> FRAMEDELAYMIN_W {
        FRAMEDELAYMIN_W { w: self }
    }
}
