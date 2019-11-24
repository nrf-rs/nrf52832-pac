#[doc = "Reader of register A3"]
pub type R = crate::R<u32, super::A3>;
#[doc = "Writer for register A3"]
pub type W = crate::W<u32, super::A3>;
#[doc = "Register A3 `reset()`'s with value 0x0400"]
impl crate::ResetValue for super::A3 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0x0400
    }
}
#[doc = "Reader of field `A3`"]
pub type A3_R = crate::R<u16, u16>;
#[doc = "Write proxy for field `A3`"]
pub struct A3_W<'a> {
    w: &'a mut W,
}
impl<'a> A3_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x0fff) | ((value as u32) & 0x0fff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:11 - Slope of 4th piece wise linear function"]
    #[inline(always)]
    pub fn a3(&self) -> A3_R {
        A3_R::new((self.bits & 0x0fff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:11 - Slope of 4th piece wise linear function"]
    #[inline(always)]
    pub fn a3(&mut self) -> A3_W {
        A3_W { w: self }
    }
}
