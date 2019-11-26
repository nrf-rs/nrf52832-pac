#[doc = "Reader of register COUNTERTOP"]
pub type R = crate::R<u32, super::COUNTERTOP>;
#[doc = "Writer for register COUNTERTOP"]
pub type W = crate::W<u32, super::COUNTERTOP>;
#[doc = "Register COUNTERTOP `reset()`'s with value 0x03ff"]
impl crate::ResetValue for super::COUNTERTOP {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0x03ff
    }
}
#[doc = "Reader of field `COUNTERTOP`"]
pub type COUNTERTOP_R = crate::R<u16, u16>;
#[doc = "Write proxy for field `COUNTERTOP`"]
pub struct COUNTERTOP_W<'a> {
    w: &'a mut W,
}
impl<'a> COUNTERTOP_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x7fff) | ((value as u32) & 0x7fff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:14 - Value up to which the pulse generator counter counts. This register is ignored when DECODER.MODE=WaveForm and only values from RAM will be used."]
    #[inline(always)]
    pub fn countertop(&self) -> COUNTERTOP_R {
        COUNTERTOP_R::new((self.bits & 0x7fff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:14 - Value up to which the pulse generator counter counts. This register is ignored when DECODER.MODE=WaveForm and only values from RAM will be used."]
    #[inline(always)]
    pub fn countertop(&mut self) -> COUNTERTOP_W {
        COUNTERTOP_W { w: self }
    }
}
