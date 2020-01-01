#[doc = "Reader of register SWIDTH"]
pub type R = crate::R<u32, super::SWIDTH>;
#[doc = "Writer for register SWIDTH"]
pub type W = crate::W<u32, super::SWIDTH>;
#[doc = "Register SWIDTH `reset()`'s with value 0x01"]
impl crate::ResetValue for super::SWIDTH {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0x01
    }
}
#[doc = "Sample width.\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum SWIDTH_A {
    #[doc = "0: 8 bit."]
    _8BIT = 0,
    #[doc = "1: 16 bit."]
    _16BIT = 1,
    #[doc = "2: 24 bit."]
    _24BIT = 2,
}
impl From<SWIDTH_A> for u8 {
    #[inline(always)]
    fn from(variant: SWIDTH_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `SWIDTH`"]
pub type SWIDTH_R = crate::R<u8, SWIDTH_A>;
impl SWIDTH_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, SWIDTH_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(SWIDTH_A::_8BIT),
            1 => Val(SWIDTH_A::_16BIT),
            2 => Val(SWIDTH_A::_24BIT),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `_8BIT`"]
    #[inline(always)]
    pub fn is_8bit(&self) -> bool {
        *self == SWIDTH_A::_8BIT
    }
    #[doc = "Checks if the value of the field is `_16BIT`"]
    #[inline(always)]
    pub fn is_16bit(&self) -> bool {
        *self == SWIDTH_A::_16BIT
    }
    #[doc = "Checks if the value of the field is `_24BIT`"]
    #[inline(always)]
    pub fn is_24bit(&self) -> bool {
        *self == SWIDTH_A::_24BIT
    }
}
#[doc = "Write proxy for field `SWIDTH`"]
pub struct SWIDTH_W<'a> {
    w: &'a mut W,
}
impl<'a> SWIDTH_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SWIDTH_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "8 bit."]
    #[inline(always)]
    pub fn _8bit(self) -> &'a mut W {
        self.variant(SWIDTH_A::_8BIT)
    }
    #[doc = "16 bit."]
    #[inline(always)]
    pub fn _16bit(self) -> &'a mut W {
        self.variant(SWIDTH_A::_16BIT)
    }
    #[doc = "24 bit."]
    #[inline(always)]
    pub fn _24bit(self) -> &'a mut W {
        self.variant(SWIDTH_A::_24BIT)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x03) | ((value as u32) & 0x03);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:1 - Sample width."]
    #[inline(always)]
    pub fn swidth(&self) -> SWIDTH_R {
        SWIDTH_R::new((self.bits & 0x03) as u8)
    }
}
impl W {
    #[doc = "Bits 0:1 - Sample width."]
    #[inline(always)]
    pub fn swidth(&mut self) -> SWIDTH_W {
        SWIDTH_W { w: self }
    }
}
