#[doc = "Reader of register TXADDRESS"]
pub type R = crate::R<u32, super::TXADDRESS>;
#[doc = "Writer for register TXADDRESS"]
pub type W = crate::W<u32, super::TXADDRESS>;
#[doc = "Register TXADDRESS `reset()`'s with value 0"]
impl crate::ResetValue for super::TXADDRESS {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `TXADDRESS`"]
pub type TXADDRESS_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `TXADDRESS`"]
pub struct TXADDRESS_W<'a> {
    w: &'a mut W,
}
impl<'a> TXADDRESS_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x07) | ((value as u32) & 0x07);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:2 - Transmit address select"]
    #[inline(always)]
    pub fn txaddress(&self) -> TXADDRESS_R {
        TXADDRESS_R::new((self.bits & 0x07) as u8)
    }
}
impl W {
    #[doc = "Bits 0:2 - Transmit address select"]
    #[inline(always)]
    pub fn txaddress(&mut self) -> TXADDRESS_W {
        TXADDRESS_W { w: self }
    }
}
