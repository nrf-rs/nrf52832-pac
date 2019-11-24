#[doc = "Reader of register B5"]
pub type R = crate::R<u32, super::B5>;
#[doc = "Writer for register B5"]
pub type W = crate::W<u32, super::B5>;
#[doc = "Register B5 `reset()`'s with value 0x3dd0"]
impl crate::ResetValue for super::B5 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0x3dd0
    }
}
#[doc = "Reader of field `B5`"]
pub type B5_R = crate::R<u16, u16>;
#[doc = "Write proxy for field `B5`"]
pub struct B5_W<'a> {
    w: &'a mut W,
}
impl<'a> B5_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x3fff) | ((value as u32) & 0x3fff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:13 - y-intercept of 6th piece wise linear function"]
    #[inline(always)]
    pub fn b5(&self) -> B5_R {
        B5_R::new((self.bits & 0x3fff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:13 - y-intercept of 6th piece wise linear function"]
    #[inline(always)]
    pub fn b5(&mut self) -> B5_W {
        B5_W { w: self }
    }
}
