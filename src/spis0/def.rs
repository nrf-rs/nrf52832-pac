#[doc = "Reader of register DEF"]
pub type R = crate::R<u32, super::DEF>;
#[doc = "Writer for register DEF"]
pub type W = crate::W<u32, super::DEF>;
#[doc = "Register DEF `reset()`'s with value 0"]
impl crate::ResetValue for super::DEF {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `DEF`"]
pub type DEF_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `DEF`"]
pub struct DEF_W<'a> {
    w: &'a mut W,
}
impl<'a> DEF_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xff) | ((value as u32) & 0xff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:7 - Default character. Character clocked out in case of an ignored transaction."]
    #[inline(always)]
    pub fn def(&self) -> DEF_R {
        DEF_R::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - Default character. Character clocked out in case of an ignored transaction."]
    #[inline(always)]
    pub fn def(&mut self) -> DEF_W {
        DEF_W { w: self }
    }
}
