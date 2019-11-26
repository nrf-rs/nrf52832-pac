#[doc = "Reader of register CTIV"]
pub type R = crate::R<u32, super::CTIV>;
#[doc = "Writer for register CTIV"]
pub type W = crate::W<u32, super::CTIV>;
#[doc = "Register CTIV `reset()`'s with value 0"]
impl crate::ResetValue for super::CTIV {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `CTIV`"]
pub type CTIV_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `CTIV`"]
pub struct CTIV_W<'a> {
    w: &'a mut W,
}
impl<'a> CTIV_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x7f) | ((value as u32) & 0x7f);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:6 - Calibration timer interval in multiple of 0.25 seconds. Range: 0.25 seconds to 31.75 seconds."]
    #[inline(always)]
    pub fn ctiv(&self) -> CTIV_R {
        CTIV_R::new((self.bits & 0x7f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:6 - Calibration timer interval in multiple of 0.25 seconds. Range: 0.25 seconds to 31.75 seconds."]
    #[inline(always)]
    pub fn ctiv(&mut self) -> CTIV_W {
        CTIV_W { w: self }
    }
}
