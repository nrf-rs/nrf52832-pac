#[doc = "Reader of register LEDPRE"]
pub type R = crate::R<u32, super::LEDPRE>;
#[doc = "Writer for register LEDPRE"]
pub type W = crate::W<u32, super::LEDPRE>;
#[doc = "Register LEDPRE `reset()`'s with value 0x10"]
impl crate::ResetValue for super::LEDPRE {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0x10
    }
}
#[doc = "Reader of field `LEDPRE`"]
pub type LEDPRE_R = crate::R<u16, u16>;
#[doc = "Write proxy for field `LEDPRE`"]
pub struct LEDPRE_W<'a> {
    w: &'a mut W,
}
impl<'a> LEDPRE_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x01ff) | ((value as u32) & 0x01ff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:8 - Period in us the LED is switched on prior to sampling"]
    #[inline(always)]
    pub fn ledpre(&self) -> LEDPRE_R {
        LEDPRE_R::new((self.bits & 0x01ff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:8 - Period in us the LED is switched on prior to sampling"]
    #[inline(always)]
    pub fn ledpre(&mut self) -> LEDPRE_W {
        LEDPRE_W { w: self }
    }
}
