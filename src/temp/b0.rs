#[doc = "Reader of register B0"]
pub type R = crate::R<u32, super::B0>;
#[doc = "Writer for register B0"]
pub type W = crate::W<u32, super::B0>;
#[doc = "Register B0 `reset()`'s with value 0x3fcc"]
impl crate::ResetValue for super::B0 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0x3fcc
    }
}
#[doc = "Reader of field `B0`"]
pub type B0_R = crate::R<u16, u16>;
#[doc = "Write proxy for field `B0`"]
pub struct B0_W<'a> {
    w: &'a mut W,
}
impl<'a> B0_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x3fff) | ((value as u32) & 0x3fff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:13 - y-intercept of 1st piece wise linear function"]
    #[inline(always)]
    pub fn b0(&self) -> B0_R {
        B0_R::new((self.bits & 0x3fff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:13 - y-intercept of 1st piece wise linear function"]
    #[inline(always)]
    pub fn b0(&mut self) -> B0_W {
        B0_W { w: self }
    }
}
