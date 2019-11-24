#[doc = "Reader of register AMOUNT"]
pub type R = crate::R<u32, super::AMOUNT>;
#[doc = "Writer for register AMOUNT"]
pub type W = crate::W<u32, super::AMOUNT>;
#[doc = "Register AMOUNT `reset()`'s with value 0"]
impl crate::ResetValue for super::AMOUNT {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `TXDATABITS`"]
pub type TXDATABITS_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `TXDATABITS`"]
pub struct TXDATABITS_W<'a> {
    w: &'a mut W,
}
impl<'a> TXDATABITS_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x07) | ((value as u32) & 0x07);
        self.w
    }
}
#[doc = "Reader of field `TXDATABYTES`"]
pub type TXDATABYTES_R = crate::R<u16, u16>;
#[doc = "Write proxy for field `TXDATABYTES`"]
pub struct TXDATABYTES_W<'a> {
    w: &'a mut W,
}
impl<'a> TXDATABYTES_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01ff << 3)) | (((value as u32) & 0x01ff) << 3);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:2 - Number of bits in the last or first byte read from RAM that shall be included in the frame (excluding parity bit)."]
    #[inline(always)]
    pub fn txdatabits(&self) -> TXDATABITS_R {
        TXDATABITS_R::new((self.bits & 0x07) as u8)
    }
    #[doc = "Bits 3:11 - Number of complete bytes that shall be included in the frame, excluding CRC, parity and framing"]
    #[inline(always)]
    pub fn txdatabytes(&self) -> TXDATABYTES_R {
        TXDATABYTES_R::new(((self.bits >> 3) & 0x01ff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:2 - Number of bits in the last or first byte read from RAM that shall be included in the frame (excluding parity bit)."]
    #[inline(always)]
    pub fn txdatabits(&mut self) -> TXDATABITS_W {
        TXDATABITS_W { w: self }
    }
    #[doc = "Bits 3:11 - Number of complete bytes that shall be included in the frame, excluding CRC, parity and framing"]
    #[inline(always)]
    pub fn txdatabytes(&mut self) -> TXDATABYTES_W {
        TXDATABYTES_W { w: self }
    }
}
