#[doc = "Reader of register T4"]
pub type R = crate::R<u32, super::T4>;
#[doc = "Writer for register T4"]
pub type W = crate::W<u32, super::T4>;
#[doc = "Register T4 `reset()`'s with value 0x50"]
impl crate::ResetValue for super::T4 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0x50
    }
}
#[doc = "Reader of field `T4`"]
pub type T4_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `T4`"]
pub struct T4_W<'a> {
    w: &'a mut W,
}
impl<'a> T4_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xff) | ((value as u32) & 0xff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:7 - End point of 5th piece wise linear function"]
    #[inline(always)]
    pub fn t4(&self) -> T4_R {
        T4_R::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - End point of 5th piece wise linear function"]
    #[inline(always)]
    pub fn t4(&mut self) -> T4_W {
        T4_W { w: self }
    }
}
