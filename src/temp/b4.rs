#[doc = "Reader of register B4"]
pub type R = crate::R<u32, super::B4>;
#[doc = "Writer for register B4"]
pub type W = crate::W<u32, super::B4>;
#[doc = "Register B4 `reset()`'s with value 0x6a"]
impl crate::ResetValue for super::B4 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0x6a
    }
}
#[doc = "Reader of field `B4`"]
pub type B4_R = crate::R<u16, u16>;
#[doc = "Write proxy for field `B4`"]
pub struct B4_W<'a> {
    w: &'a mut W,
}
impl<'a> B4_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x3fff) | ((value as u32) & 0x3fff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:13 - y-intercept of 5th piece wise linear function"]
    #[inline(always)]
    pub fn b4(&self) -> B4_R {
        B4_R::new((self.bits & 0x3fff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:13 - y-intercept of 5th piece wise linear function"]
    #[inline(always)]
    pub fn b4(&mut self) -> B4_W {
        B4_W { w: self }
    }
}
