#[doc = "Reader of register A1"]
pub type R = crate::R<u32, super::A1>;
#[doc = "Writer for register A1"]
pub type W = crate::W<u32, super::A1>;
#[doc = "Register A1 `reset()`'s with value 0x0343"]
impl crate::ResetValue for super::A1 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0x0343
    }
}
#[doc = "Reader of field `A1`"]
pub type A1_R = crate::R<u16, u16>;
#[doc = "Write proxy for field `A1`"]
pub struct A1_W<'a> {
    w: &'a mut W,
}
impl<'a> A1_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x0fff) | ((value as u32) & 0x0fff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:11 - Slope of 2nd piece wise linear function"]
    #[inline(always)]
    pub fn a1(&self) -> A1_R {
        A1_R::new((self.bits & 0x0fff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:11 - Slope of 2nd piece wise linear function"]
    #[inline(always)]
    pub fn a1(&mut self) -> A1_W {
        A1_W { w: self }
    }
}
