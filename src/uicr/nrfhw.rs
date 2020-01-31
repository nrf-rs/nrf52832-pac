#[doc = "Reader of register NRFHW[%s]"]
pub type R = crate::R<u32, super::NRFHW>;
#[doc = "Writer for register NRFHW[%s]"]
pub type W = crate::W<u32, super::NRFHW>;
#[doc = "Register NRFHW[%s]
`reset()`'s with value 0xffff_ffff"]
impl crate::ResetValue for super::NRFHW {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0xffff_ffff
    }
}
#[doc = "Reader of field `NRFHW`"]
pub type NRFHW_R = crate::R<u32, u32>;
#[doc = "Write proxy for field `NRFHW`"]
pub struct NRFHW_W<'a> {
    w: &'a mut W,
}
impl<'a> NRFHW_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff_ffff) | ((value as u32) & 0xffff_ffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:31 - Reserved for Nordic hardware design"]
    #[inline(always)]
    pub fn nrfhw(&self) -> NRFHW_R {
        NRFHW_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
impl W {
    #[doc = "Bits 0:31 - Reserved for Nordic hardware design"]
    #[inline(always)]
    pub fn nrfhw(&mut self) -> NRFHW_W {
        NRFHW_W { w: self }
    }
}
