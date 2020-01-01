#[doc = "Reader of register BITMODE"]
pub type R = crate::R<u32, super::BITMODE>;
#[doc = "Writer for register BITMODE"]
pub type W = crate::W<u32, super::BITMODE>;
#[doc = "Register BITMODE `reset()`'s with value 0"]
impl crate::ResetValue for super::BITMODE {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Timer bit width\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum BITMODE_A {
    #[doc = "0: 16 bit timer bit width"]
    _16BIT = 0,
    #[doc = "1: 8 bit timer bit width"]
    _08BIT = 1,
    #[doc = "2: 24 bit timer bit width"]
    _24BIT = 2,
    #[doc = "3: 32 bit timer bit width"]
    _32BIT = 3,
}
impl From<BITMODE_A> for u8 {
    #[inline(always)]
    fn from(variant: BITMODE_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `BITMODE`"]
pub type BITMODE_R = crate::R<u8, BITMODE_A>;
impl BITMODE_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> BITMODE_A {
        match self.bits {
            0 => BITMODE_A::_16BIT,
            1 => BITMODE_A::_08BIT,
            2 => BITMODE_A::_24BIT,
            3 => BITMODE_A::_32BIT,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `_16BIT`"]
    #[inline(always)]
    pub fn is_16bit(&self) -> bool {
        *self == BITMODE_A::_16BIT
    }
    #[doc = "Checks if the value of the field is `_08BIT`"]
    #[inline(always)]
    pub fn is_08bit(&self) -> bool {
        *self == BITMODE_A::_08BIT
    }
    #[doc = "Checks if the value of the field is `_24BIT`"]
    #[inline(always)]
    pub fn is_24bit(&self) -> bool {
        *self == BITMODE_A::_24BIT
    }
    #[doc = "Checks if the value of the field is `_32BIT`"]
    #[inline(always)]
    pub fn is_32bit(&self) -> bool {
        *self == BITMODE_A::_32BIT
    }
}
#[doc = "Write proxy for field `BITMODE`"]
pub struct BITMODE_W<'a> {
    w: &'a mut W,
}
impl<'a> BITMODE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: BITMODE_A) -> &'a mut W {
        {
            self.bits(variant.into())
        }
    }
    #[doc = "16 bit timer bit width"]
    #[inline(always)]
    pub fn _16bit(self) -> &'a mut W {
        self.variant(BITMODE_A::_16BIT)
    }
    #[doc = "8 bit timer bit width"]
    #[inline(always)]
    pub fn _08bit(self) -> &'a mut W {
        self.variant(BITMODE_A::_08BIT)
    }
    #[doc = "24 bit timer bit width"]
    #[inline(always)]
    pub fn _24bit(self) -> &'a mut W {
        self.variant(BITMODE_A::_24BIT)
    }
    #[doc = "32 bit timer bit width"]
    #[inline(always)]
    pub fn _32bit(self) -> &'a mut W {
        self.variant(BITMODE_A::_32BIT)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x03) | ((value as u32) & 0x03);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:1 - Timer bit width"]
    #[inline(always)]
    pub fn bitmode(&self) -> BITMODE_R {
        BITMODE_R::new((self.bits & 0x03) as u8)
    }
}
impl W {
    #[doc = "Bits 0:1 - Timer bit width"]
    #[inline(always)]
    pub fn bitmode(&mut self) -> BITMODE_W {
        BITMODE_W { w: self }
    }
}
