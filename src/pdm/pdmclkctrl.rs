#[doc = "Reader of register PDMCLKCTRL"]
pub type R = crate::R<u32, super::PDMCLKCTRL>;
#[doc = "Writer for register PDMCLKCTRL"]
pub type W = crate::W<u32, super::PDMCLKCTRL>;
#[doc = "Register PDMCLKCTRL `reset()`'s with value 0x0840_0000"]
impl crate::ResetValue for super::PDMCLKCTRL {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0x0840_0000
    }
}
#[doc = "PDM_CLK frequency\n\nValue on reset: 138412032"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u32)]
pub enum FREQ_A {
    #[doc = "134217728: PDM_CLK = 32 MHz / 32 = 1.000 MHz"]
    _1000K = 134217728,
    #[doc = "138412032: PDM_CLK = 32 MHz / 31 = 1.032 MHz"]
    DEFAULT = 138412032,
    #[doc = "142606336: PDM_CLK = 32 MHz / 30 = 1.067 MHz"]
    _1067K = 142606336,
}
impl From<FREQ_A> for u32 {
    #[inline(always)]
    fn from(variant: FREQ_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `FREQ`"]
pub type FREQ_R = crate::R<u32, FREQ_A>;
impl FREQ_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u32, FREQ_A> {
        use crate::Variant::*;
        match self.bits {
            134217728 => Val(FREQ_A::_1000K),
            138412032 => Val(FREQ_A::DEFAULT),
            142606336 => Val(FREQ_A::_1067K),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `_1000K`"]
    #[inline(always)]
    pub fn is_1000k(&self) -> bool {
        *self == FREQ_A::_1000K
    }
    #[doc = "Checks if the value of the field is `DEFAULT`"]
    #[inline(always)]
    pub fn is_default(&self) -> bool {
        *self == FREQ_A::DEFAULT
    }
    #[doc = "Checks if the value of the field is `_1067K`"]
    #[inline(always)]
    pub fn is_1067k(&self) -> bool {
        *self == FREQ_A::_1067K
    }
}
#[doc = "Write proxy for field `FREQ`"]
pub struct FREQ_W<'a> {
    w: &'a mut W,
}
impl<'a> FREQ_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: FREQ_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "PDM_CLK = 32 MHz / 32 = 1.000 MHz"]
    #[inline(always)]
    pub fn _1000k(self) -> &'a mut W {
        self.variant(FREQ_A::_1000K)
    }
    #[doc = "PDM_CLK = 32 MHz / 31 = 1.032 MHz"]
    #[inline(always)]
    pub fn default(self) -> &'a mut W {
        self.variant(FREQ_A::DEFAULT)
    }
    #[doc = "PDM_CLK = 32 MHz / 30 = 1.067 MHz"]
    #[inline(always)]
    pub fn _1067k(self) -> &'a mut W {
        self.variant(FREQ_A::_1067K)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff_ffff) | ((value as u32) & 0xffff_ffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:31 - PDM_CLK frequency"]
    #[inline(always)]
    pub fn freq(&self) -> FREQ_R {
        FREQ_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
impl W {
    #[doc = "Bits 0:31 - PDM_CLK frequency"]
    #[inline(always)]
    pub fn freq(&mut self) -> FREQ_W {
        FREQ_W { w: self }
    }
}
