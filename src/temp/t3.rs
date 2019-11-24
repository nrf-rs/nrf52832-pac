#[doc = "Reader of register T3"]
pub type R = crate::R<u32, super::T3>;
#[doc = "Writer for register T3"]
pub type W = crate::W<u32, super::T3>;
#[doc = "Register T3 `reset()`'s with value 0x19"]
impl crate::ResetValue for super::T3 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0x19
    }
}
#[doc = "Reader of field `T3`"]
pub type T3_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `T3`"]
pub struct T3_W<'a> {
    w: &'a mut W,
}
impl<'a> T3_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xff) | ((value as u32) & 0xff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:7 - End point of 4th piece wise linear function"]
    #[inline(always)]
    pub fn t3(&self) -> T3_R {
        T3_R::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - End point of 4th piece wise linear function"]
    #[inline(always)]
    pub fn t3(&mut self) -> T3_W {
        T3_W { w: self }
    }
}
