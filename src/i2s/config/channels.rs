#[doc = "Reader of register CHANNELS"]
pub type R = crate::R<u32, super::CHANNELS>;
#[doc = "Writer for register CHANNELS"]
pub type W = crate::W<u32, super::CHANNELS>;
#[doc = "Register CHANNELS `reset()`'s with value 0"]
impl crate::ResetValue for super::CHANNELS {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Enable channels.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum CHANNELS_A {
    #[doc = "0: Stereo."]
    STEREO = 0,
    #[doc = "1: Left only."]
    LEFT = 1,
    #[doc = "2: Right only."]
    RIGHT = 2,
}
impl From<CHANNELS_A> for u8 {
    #[inline(always)]
    fn from(variant: CHANNELS_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `CHANNELS`"]
pub type CHANNELS_R = crate::R<u8, CHANNELS_A>;
impl CHANNELS_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, CHANNELS_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(CHANNELS_A::STEREO),
            1 => Val(CHANNELS_A::LEFT),
            2 => Val(CHANNELS_A::RIGHT),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `STEREO`"]
    #[inline(always)]
    pub fn is_stereo(&self) -> bool {
        *self == CHANNELS_A::STEREO
    }
    #[doc = "Checks if the value of the field is `LEFT`"]
    #[inline(always)]
    pub fn is_left(&self) -> bool {
        *self == CHANNELS_A::LEFT
    }
    #[doc = "Checks if the value of the field is `RIGHT`"]
    #[inline(always)]
    pub fn is_right(&self) -> bool {
        *self == CHANNELS_A::RIGHT
    }
}
#[doc = "Write proxy for field `CHANNELS`"]
pub struct CHANNELS_W<'a> {
    w: &'a mut W,
}
impl<'a> CHANNELS_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CHANNELS_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "Stereo."]
    #[inline(always)]
    pub fn stereo(self) -> &'a mut W {
        self.variant(CHANNELS_A::STEREO)
    }
    #[doc = "Left only."]
    #[inline(always)]
    pub fn left(self) -> &'a mut W {
        self.variant(CHANNELS_A::LEFT)
    }
    #[doc = "Right only."]
    #[inline(always)]
    pub fn right(self) -> &'a mut W {
        self.variant(CHANNELS_A::RIGHT)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x03) | ((value as u32) & 0x03);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:1 - Enable channels."]
    #[inline(always)]
    pub fn channels(&self) -> CHANNELS_R {
        CHANNELS_R::new((self.bits & 0x03) as u8)
    }
}
impl W {
    #[doc = "Bits 0:1 - Enable channels."]
    #[inline(always)]
    pub fn channels(&mut self) -> CHANNELS_W {
        CHANNELS_W { w: self }
    }
}
