#[doc = "Reader of register MAXCNT"]
pub type R = crate::R<u32, super::MAXCNT>;
#[doc = "Writer for register MAXCNT"]
pub type W = crate::W<u32, super::MAXCNT>;
#[doc = "Register MAXCNT `reset()`'s with value 0"]
impl crate::ResetValue for super::MAXCNT {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `MAXCNT`"]
pub type MAXCNT_R = crate::R<u16, u16>;
#[doc = "Write proxy for field `MAXCNT`"]
pub struct MAXCNT_W<'a> {
    w: &'a mut W,
}
impl<'a> MAXCNT_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x3fff) | ((value as u32) & 0x3fff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:13 - Size of RXD and TXD buffers in number of 32 bit words."]
    #[inline(always)]
    pub fn maxcnt(&self) -> MAXCNT_R {
        MAXCNT_R::new((self.bits & 0x3fff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:13 - Size of RXD and TXD buffers in number of 32 bit words."]
    #[inline(always)]
    pub fn maxcnt(&mut self) -> MAXCNT_W {
        MAXCNT_W { w: self }
    }
}
