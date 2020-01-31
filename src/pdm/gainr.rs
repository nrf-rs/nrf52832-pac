#[doc = "Reader of register GAINR"]
pub type R = crate::R<u32, super::GAINR>;
#[doc = "Writer for register GAINR"]
pub type W = crate::W<u32, super::GAINR>;
#[doc = "Register GAINR `reset()`'s with value 0x28"]
impl crate::ResetValue for super::GAINR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0x28
    }
}
#[doc = "Right output gain adjustment, in 0.5 dB steps, around the default module gain (see electrical parameters)\n\nValue on reset: 40"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum GAINR_A {
    #[doc = "0: -20dB gain adjustment (minimum)"]
    MINGAIN = 0,
    #[doc = "40: 0dB gain adjustment ('2500 RMS' requirement)"]
    DEFAULTGAIN = 40,
    #[doc = "80: +20dB gain adjustment (maximum)"]
    MAXGAIN = 80,
}
impl From<GAINR_A> for u8 {
    #[inline(always)]
    fn from(variant: GAINR_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `GAINR`"]
pub type GAINR_R = crate::R<u8, GAINR_A>;
impl GAINR_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, GAINR_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(GAINR_A::MINGAIN),
            40 => Val(GAINR_A::DEFAULTGAIN),
            80 => Val(GAINR_A::MAXGAIN),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `MINGAIN`"]
    #[inline(always)]
    pub fn is_min_gain(&self) -> bool {
        *self == GAINR_A::MINGAIN
    }
    #[doc = "Checks if the value of the field is `DEFAULTGAIN`"]
    #[inline(always)]
    pub fn is_default_gain(&self) -> bool {
        *self == GAINR_A::DEFAULTGAIN
    }
    #[doc = "Checks if the value of the field is `MAXGAIN`"]
    #[inline(always)]
    pub fn is_max_gain(&self) -> bool {
        *self == GAINR_A::MAXGAIN
    }
}
#[doc = "Write proxy for field `GAINR`"]
pub struct GAINR_W<'a> {
    w: &'a mut W,
}
impl<'a> GAINR_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: GAINR_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "-20dB gain adjustment (minimum)"]
    #[inline(always)]
    pub fn min_gain(self) -> &'a mut W {
        self.variant(GAINR_A::MINGAIN)
    }
    #[doc = "0dB gain adjustment ('2500 RMS' requirement)"]
    #[inline(always)]
    pub fn default_gain(self) -> &'a mut W {
        self.variant(GAINR_A::DEFAULTGAIN)
    }
    #[doc = "+20dB gain adjustment (maximum)"]
    #[inline(always)]
    pub fn max_gain(self) -> &'a mut W {
        self.variant(GAINR_A::MAXGAIN)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xff) | ((value as u32) & 0xff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:7 - Right output gain adjustment, in 0.5 dB steps, around the default module gain (see electrical parameters)"]
    #[inline(always)]
    pub fn gainr(&self) -> GAINR_R {
        GAINR_R::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - Right output gain adjustment, in 0.5 dB steps, around the default module gain (see electrical parameters)"]
    #[inline(always)]
    pub fn gainr(&mut self) -> GAINR_W {
        GAINR_W { w: self }
    }
}
