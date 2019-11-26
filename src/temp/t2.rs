#[doc = "Reader of register T2"]
pub type R = crate::R<u32, super::T2>;
#[doc = "Writer for register T2"]
pub type W = crate::W<u32, super::T2>;
#[doc = "Register T2 `reset()`'s with value 0x14"]
impl crate::ResetValue for super::T2 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0x14
    }
}
#[doc = "Reader of field `T2`"]
pub type T2_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `T2`"]
pub struct T2_W<'a> {
    w: &'a mut W,
}
impl<'a> T2_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xff) | ((value as u32) & 0xff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:7 - End point of 3rd piece wise linear function"]
    #[inline(always)]
    pub fn t2(&self) -> T2_R {
        T2_R::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - End point of 3rd piece wise linear function"]
    #[inline(always)]
    pub fn t2(&mut self) -> T2_W {
        T2_W { w: self }
    }
}
