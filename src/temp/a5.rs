#[doc = "Reader of register A5"]
pub type R = crate::R<u32, super::A5>;
#[doc = "Writer for register A5"]
pub type W = crate::W<u32, super::A5>;
#[doc = "Register A5 `reset()`'s with value 0x037b"]
impl crate::ResetValue for super::A5 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0x037b
    }
}
#[doc = "Reader of field `A5`"]
pub type A5_R = crate::R<u16, u16>;
#[doc = "Write proxy for field `A5`"]
pub struct A5_W<'a> {
    w: &'a mut W,
}
impl<'a> A5_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x0fff) | ((value as u32) & 0x0fff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:11 - Slope of 6th piece wise linear function"]
    #[inline(always)]
    pub fn a5(&self) -> A5_R {
        A5_R::new((self.bits & 0x0fff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:11 - Slope of 6th piece wise linear function"]
    #[inline(always)]
    pub fn a5(&mut self) -> A5_W {
        A5_W { w: self }
    }
}
