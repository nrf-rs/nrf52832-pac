#[doc = "Reader of register B2"]
pub type R = crate::R<u32, super::B2>;
#[doc = "Writer for register B2"]
pub type W = crate::W<u32, super::B2>;
#[doc = "Register B2 `reset()`'s with value 0x3f98"]
impl crate::ResetValue for super::B2 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0x3f98
    }
}
#[doc = "Reader of field `B2`"]
pub type B2_R = crate::R<u16, u16>;
#[doc = "Write proxy for field `B2`"]
pub struct B2_W<'a> {
    w: &'a mut W,
}
impl<'a> B2_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x3fff) | ((value as u32) & 0x3fff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:13 - y-intercept of 3rd piece wise linear function"]
    #[inline(always)]
    pub fn b2(&self) -> B2_R {
        B2_R::new((self.bits & 0x3fff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:13 - y-intercept of 3rd piece wise linear function"]
    #[inline(always)]
    pub fn b2(&mut self) -> B2_W {
        B2_W { w: self }
    }
}
