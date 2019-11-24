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
#[doc = "Reader of field `BUFFSIZE`"]
pub type BUFFSIZE_R = crate::R<u16, u16>;
#[doc = "Write proxy for field `BUFFSIZE`"]
pub struct BUFFSIZE_W<'a> {
    w: &'a mut W,
}
impl<'a> BUFFSIZE_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x7fff) | ((value as u32) & 0x7fff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:14 - Length of DMA RAM allocation in number of samples"]
    #[inline(always)]
    pub fn buffsize(&self) -> BUFFSIZE_R {
        BUFFSIZE_R::new((self.bits & 0x7fff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:14 - Length of DMA RAM allocation in number of samples"]
    #[inline(always)]
    pub fn buffsize(&mut self) -> BUFFSIZE_W {
        BUFFSIZE_W { w: self }
    }
}
