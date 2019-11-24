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
pub type MAXCNT_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `MAXCNT`"]
pub struct MAXCNT_W<'a> {
    w: &'a mut W,
}
impl<'a> MAXCNT_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xff) | ((value as u32) & 0xff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:7 - Maximum number of bytes in transmit buffer"]
    #[inline(always)]
    pub fn maxcnt(&self) -> MAXCNT_R {
        MAXCNT_R::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - Maximum number of bytes in transmit buffer"]
    #[inline(always)]
    pub fn maxcnt(&mut self) -> MAXCNT_W {
        MAXCNT_W { w: self }
    }
}
