#[doc = "Reader of register NRFFW[%s]"]
pub type R = crate::R<u32, super::NRFFW>;
#[doc = "Writer for register NRFFW[%s]"]
pub type W = crate::W<u32, super::NRFFW>;
#[doc = "Register NRFFW[%s]
`reset()`'s with value 0xffff_ffff"]
impl crate::ResetValue for super::NRFFW {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0xffff_ffff
    }
}
#[doc = "Reader of field `NRFFW`"]
pub type NRFFW_R = crate::R<u32, u32>;
#[doc = "Write proxy for field `NRFFW`"]
pub struct NRFFW_W<'a> {
    w: &'a mut W,
}
impl<'a> NRFFW_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff_ffff) | ((value as u32) & 0xffff_ffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:31 - Reserved for Nordic firmware design"]
    #[inline(always)]
    pub fn nrffw(&self) -> NRFFW_R {
        NRFFW_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
impl W {
    #[doc = "Bits 0:31 - Reserved for Nordic firmware design"]
    #[inline(always)]
    pub fn nrffw(&mut self) -> NRFFW_W {
        NRFFW_W { w: self }
    }
}
