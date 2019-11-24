#[doc = "Reader of register T0"]
pub type R = crate::R<u32, super::T0>;
#[doc = "Writer for register T0"]
pub type W = crate::W<u32, super::T0>;
#[doc = "Register T0 `reset()`'s with value 0xe2"]
impl crate::ResetValue for super::T0 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0xe2
    }
}
#[doc = "Reader of field `T0`"]
pub type T0_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `T0`"]
pub struct T0_W<'a> {
    w: &'a mut W,
}
impl<'a> T0_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xff) | ((value as u32) & 0xff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:7 - End point of 1st piece wise linear function"]
    #[inline(always)]
    pub fn t0(&self) -> T0_R {
        T0_R::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - End point of 1st piece wise linear function"]
    #[inline(always)]
    pub fn t0(&mut self) -> T0_W {
        T0_W { w: self }
    }
}
