#[doc = "Reader of register MAXLEN"]
pub type R = crate::R<u32, super::MAXLEN>;
#[doc = "Writer for register MAXLEN"]
pub type W = crate::W<u32, super::MAXLEN>;
#[doc = "Register MAXLEN `reset()`'s with value 0"]
impl crate::ResetValue for super::MAXLEN {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `MAXLEN`"]
pub type MAXLEN_R = crate::R<u16, u16>;
#[doc = "Write proxy for field `MAXLEN`"]
pub struct MAXLEN_W<'a> {
    w: &'a mut W,
}
impl<'a> MAXLEN_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x01ff) | ((value as u32) & 0x01ff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:8 - Size of allocated for TXD and RXD data storage buffer in Data RAM"]
    #[inline(always)]
    pub fn maxlen(&self) -> MAXLEN_R {
        MAXLEN_R::new((self.bits & 0x01ff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:8 - Size of allocated for TXD and RXD data storage buffer in Data RAM"]
    #[inline(always)]
    pub fn maxlen(&mut self) -> MAXLEN_W {
        MAXLEN_W { w: self }
    }
}
