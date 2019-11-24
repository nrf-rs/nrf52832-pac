#[doc = "Reader of register B1"]
pub type R = crate::R<u32, super::B1>;
#[doc = "Writer for register B1"]
pub type W = crate::W<u32, super::B1>;
#[doc = "Register B1 `reset()`'s with value 0x3f98"]
impl crate::ResetValue for super::B1 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0x3f98
    }
}
#[doc = "Reader of field `B1`"]
pub type B1_R = crate::R<u16, u16>;
#[doc = "Write proxy for field `B1`"]
pub struct B1_W<'a> {
    w: &'a mut W,
}
impl<'a> B1_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x3fff) | ((value as u32) & 0x3fff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:13 - y-intercept of 2nd piece wise linear function"]
    #[inline(always)]
    pub fn b1(&self) -> B1_R {
        B1_R::new((self.bits & 0x3fff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:13 - y-intercept of 2nd piece wise linear function"]
    #[inline(always)]
    pub fn b1(&mut self) -> B1_W {
        B1_W { w: self }
    }
}
