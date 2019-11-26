#[doc = "Reader of register A2"]
pub type R = crate::R<u32, super::A2>;
#[doc = "Writer for register A2"]
pub type W = crate::W<u32, super::A2>;
#[doc = "Register A2 `reset()`'s with value 0x035d"]
impl crate::ResetValue for super::A2 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0x035d
    }
}
#[doc = "Reader of field `A2`"]
pub type A2_R = crate::R<u16, u16>;
#[doc = "Write proxy for field `A2`"]
pub struct A2_W<'a> {
    w: &'a mut W,
}
impl<'a> A2_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x0fff) | ((value as u32) & 0x0fff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:11 - Slope of 3rd piece wise linear function"]
    #[inline(always)]
    pub fn a2(&self) -> A2_R {
        A2_R::new((self.bits & 0x0fff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:11 - Slope of 3rd piece wise linear function"]
    #[inline(always)]
    pub fn a2(&mut self) -> A2_W {
        A2_W { w: self }
    }
}
