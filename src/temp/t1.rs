#[doc = "Reader of register T1"]
pub type R = crate::R<u32, super::T1>;
#[doc = "Writer for register T1"]
pub type W = crate::W<u32, super::T1>;
#[doc = "Register T1 `reset()`'s with value 0"]
impl crate::ResetValue for super::T1 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `T1`"]
pub type T1_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `T1`"]
pub struct T1_W<'a> {
    w: &'a mut W,
}
impl<'a> T1_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xff) | ((value as u32) & 0xff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:7 - End point of 2nd piece wise linear function"]
    #[inline(always)]
    pub fn t1(&self) -> T1_R {
        T1_R::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - End point of 2nd piece wise linear function"]
    #[inline(always)]
    pub fn t1(&mut self) -> T1_W {
        T1_W { w: self }
    }
}
