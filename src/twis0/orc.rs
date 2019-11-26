#[doc = "Reader of register ORC"]
pub type R = crate::R<u32, super::ORC>;
#[doc = "Writer for register ORC"]
pub type W = crate::W<u32, super::ORC>;
#[doc = "Register ORC `reset()`'s with value 0"]
impl crate::ResetValue for super::ORC {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `ORC`"]
pub type ORC_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `ORC`"]
pub struct ORC_W<'a> {
    w: &'a mut W,
}
impl<'a> ORC_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xff) | ((value as u32) & 0xff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:7 - Over-read character. Character sent out in case of an over-read of the transmit buffer."]
    #[inline(always)]
    pub fn orc(&self) -> ORC_R {
        ORC_R::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - Over-read character. Character sent out in case of an over-read of the transmit buffer."]
    #[inline(always)]
    pub fn orc(&mut self) -> ORC_W {
        ORC_W { w: self }
    }
}
