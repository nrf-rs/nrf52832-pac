#[doc = "Reader of register RESOLUTION"]
pub type R = crate::R<u32, super::RESOLUTION>;
#[doc = "Writer for register RESOLUTION"]
pub type W = crate::W<u32, super::RESOLUTION>;
#[doc = "Register RESOLUTION `reset()`'s with value 0x01"]
impl crate::ResetValue for super::RESOLUTION {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0x01
    }
}
#[doc = "Set the resolution\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum VAL_A {
    #[doc = "0: 8 bit"]
    _8BIT = 0,
    #[doc = "1: 10 bit"]
    _10BIT = 1,
    #[doc = "2: 12 bit"]
    _12BIT = 2,
    #[doc = "3: 14 bit"]
    _14BIT = 3,
}
impl From<VAL_A> for u8 {
    #[inline(always)]
    fn from(variant: VAL_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `VAL`"]
pub type VAL_R = crate::R<u8, VAL_A>;
impl VAL_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, VAL_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(VAL_A::_8BIT),
            1 => Val(VAL_A::_10BIT),
            2 => Val(VAL_A::_12BIT),
            3 => Val(VAL_A::_14BIT),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `_8BIT`"]
    #[inline(always)]
    pub fn is_8bit(&self) -> bool {
        *self == VAL_A::_8BIT
    }
    #[doc = "Checks if the value of the field is `_10BIT`"]
    #[inline(always)]
    pub fn is_10bit(&self) -> bool {
        *self == VAL_A::_10BIT
    }
    #[doc = "Checks if the value of the field is `_12BIT`"]
    #[inline(always)]
    pub fn is_12bit(&self) -> bool {
        *self == VAL_A::_12BIT
    }
    #[doc = "Checks if the value of the field is `_14BIT`"]
    #[inline(always)]
    pub fn is_14bit(&self) -> bool {
        *self == VAL_A::_14BIT
    }
}
#[doc = "Write proxy for field `VAL`"]
pub struct VAL_W<'a> {
    w: &'a mut W,
}
impl<'a> VAL_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: VAL_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "8 bit"]
    #[inline(always)]
    pub fn _8bit(self) -> &'a mut W {
        self.variant(VAL_A::_8BIT)
    }
    #[doc = "10 bit"]
    #[inline(always)]
    pub fn _10bit(self) -> &'a mut W {
        self.variant(VAL_A::_10BIT)
    }
    #[doc = "12 bit"]
    #[inline(always)]
    pub fn _12bit(self) -> &'a mut W {
        self.variant(VAL_A::_12BIT)
    }
    #[doc = "14 bit"]
    #[inline(always)]
    pub fn _14bit(self) -> &'a mut W {
        self.variant(VAL_A::_14BIT)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x07) | ((value as u32) & 0x07);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:2 - Set the resolution"]
    #[inline(always)]
    pub fn val(&self) -> VAL_R {
        VAL_R::new((self.bits & 0x07) as u8)
    }
}
impl W {
    #[doc = "Bits 0:2 - Set the resolution"]
    #[inline(always)]
    pub fn val(&mut self) -> VAL_W {
        VAL_W { w: self }
    }
}
