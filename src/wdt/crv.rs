#[doc = "Reader of register CRV"]
pub type R = crate::R<u32, super::CRV>;
#[doc = "Writer for register CRV"]
pub type W = crate::W<u32, super::CRV>;
#[doc = "Register CRV `reset()`'s with value 0xffff_ffff"]
impl crate::ResetValue for super::CRV {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0xffff_ffff
    }
}
#[doc = "Reader of field `CRV`"]
pub type CRV_R = crate::R<u32, u32>;
#[doc = "Write proxy for field `CRV`"]
pub struct CRV_W<'a> {
    w: &'a mut W,
}
impl<'a> CRV_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff_ffff) | ((value as u32) & 0xffff_ffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:31 - Counter reload value in number of cycles of the 32.768 kHz clock"]
    #[inline(always)]
    pub fn crv(&self) -> CRV_R {
        CRV_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
impl W {
    #[doc = "Bits 0:31 - Counter reload value in number of cycles of the 32.768 kHz clock"]
    #[inline(always)]
    pub fn crv(&mut self) -> CRV_W {
        CRV_W { w: self }
    }
}
