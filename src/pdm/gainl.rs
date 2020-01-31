#[doc = "Reader of register GAINL"]
pub type R = crate::R<u32, super::GAINL>;
#[doc = "Writer for register GAINL"]
pub type W = crate::W<u32, super::GAINL>;
#[doc = "Register GAINL `reset()`'s with value 0x28"]
impl crate::ResetValue for super::GAINL {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0x28
    }
}
#[doc = "Left output gain adjustment, in 0.5 dB steps, around the default module gain (see electrical parameters) 0x00 -20 dB gain adjust 0x01 -19.5 dB gain adjust (...) 0x27 -0.5 dB gain adjust 0x28 0 dB gain adjust 0x29 +0.5 dB gain adjust (...) 0x4F +19.5 dB gain adjust 0x50 +20 dB gain adjust\n\nValue on reset: 40"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum GAINL_A {
    #[doc = "0: -20dB gain adjustment (minimum)"]
    MINGAIN = 0,
    #[doc = "40: 0dB gain adjustment ('2500 RMS' requirement)"]
    DEFAULTGAIN = 40,
    #[doc = "80: +20dB gain adjustment (maximum)"]
    MAXGAIN = 80,
}
impl From<GAINL_A> for u8 {
    #[inline(always)]
    fn from(variant: GAINL_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `GAINL`"]
pub type GAINL_R = crate::R<u8, GAINL_A>;
impl GAINL_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, GAINL_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(GAINL_A::MINGAIN),
            40 => Val(GAINL_A::DEFAULTGAIN),
            80 => Val(GAINL_A::MAXGAIN),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `MINGAIN`"]
    #[inline(always)]
    pub fn is_min_gain(&self) -> bool {
        *self == GAINL_A::MINGAIN
    }
    #[doc = "Checks if the value of the field is `DEFAULTGAIN`"]
    #[inline(always)]
    pub fn is_default_gain(&self) -> bool {
        *self == GAINL_A::DEFAULTGAIN
    }
    #[doc = "Checks if the value of the field is `MAXGAIN`"]
    #[inline(always)]
    pub fn is_max_gain(&self) -> bool {
        *self == GAINL_A::MAXGAIN
    }
}
#[doc = "Write proxy for field `GAINL`"]
pub struct GAINL_W<'a> {
    w: &'a mut W,
}
impl<'a> GAINL_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: GAINL_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "-20dB gain adjustment (minimum)"]
    #[inline(always)]
    pub fn min_gain(self) -> &'a mut W {
        self.variant(GAINL_A::MINGAIN)
    }
    #[doc = "0dB gain adjustment ('2500 RMS' requirement)"]
    #[inline(always)]
    pub fn default_gain(self) -> &'a mut W {
        self.variant(GAINL_A::DEFAULTGAIN)
    }
    #[doc = "+20dB gain adjustment (maximum)"]
    #[inline(always)]
    pub fn max_gain(self) -> &'a mut W {
        self.variant(GAINL_A::MAXGAIN)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x7f) | ((value as u32) & 0x7f);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:6 - Left output gain adjustment, in 0.5 dB steps, around the default module gain (see electrical parameters) 0x00 -20 dB gain adjust 0x01 -19.5 dB gain adjust (...) 0x27 -0.5 dB gain adjust 0x28 0 dB gain adjust 0x29 +0.5 dB gain adjust (...) 0x4F +19.5 dB gain adjust 0x50 +20 dB gain adjust"]
    #[inline(always)]
    pub fn gainl(&self) -> GAINL_R {
        GAINL_R::new((self.bits & 0x7f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:6 - Left output gain adjustment, in 0.5 dB steps, around the default module gain (see electrical parameters) 0x00 -20 dB gain adjust 0x01 -19.5 dB gain adjust (...) 0x27 -0.5 dB gain adjust 0x28 0 dB gain adjust 0x29 +0.5 dB gain adjust (...) 0x4F +19.5 dB gain adjust 0x50 +20 dB gain adjust"]
    #[inline(always)]
    pub fn gainl(&mut self) -> GAINL_W {
        GAINL_W { w: self }
    }
}
