#[doc = "Reader of register RATIO"]
pub type R = crate::R<u32, super::RATIO>;
#[doc = "Writer for register RATIO"]
pub type W = crate::W<u32, super::RATIO>;
#[doc = "Register RATIO `reset()`'s with value 0x06"]
impl crate::ResetValue for super::RATIO {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0x06
    }
}
#[doc = "MCK / LRCK ratio.\n\nValue on reset: 6"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum RATIO_A {
    #[doc = "0: LRCK = MCK / 32"]
    _32X = 0,
    #[doc = "1: LRCK = MCK / 48"]
    _48X = 1,
    #[doc = "2: LRCK = MCK / 64"]
    _64X = 2,
    #[doc = "3: LRCK = MCK / 96"]
    _96X = 3,
    #[doc = "4: LRCK = MCK / 128"]
    _128X = 4,
    #[doc = "5: LRCK = MCK / 192"]
    _192X = 5,
    #[doc = "6: LRCK = MCK / 256"]
    _256X = 6,
    #[doc = "7: LRCK = MCK / 384"]
    _384X = 7,
    #[doc = "8: LRCK = MCK / 512"]
    _512X = 8,
}
impl From<RATIO_A> for u8 {
    #[inline(always)]
    fn from(variant: RATIO_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `RATIO`"]
pub type RATIO_R = crate::R<u8, RATIO_A>;
impl RATIO_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, RATIO_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(RATIO_A::_32X),
            1 => Val(RATIO_A::_48X),
            2 => Val(RATIO_A::_64X),
            3 => Val(RATIO_A::_96X),
            4 => Val(RATIO_A::_128X),
            5 => Val(RATIO_A::_192X),
            6 => Val(RATIO_A::_256X),
            7 => Val(RATIO_A::_384X),
            8 => Val(RATIO_A::_512X),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `_32X`"]
    #[inline(always)]
    pub fn is_32x(&self) -> bool {
        *self == RATIO_A::_32X
    }
    #[doc = "Checks if the value of the field is `_48X`"]
    #[inline(always)]
    pub fn is_48x(&self) -> bool {
        *self == RATIO_A::_48X
    }
    #[doc = "Checks if the value of the field is `_64X`"]
    #[inline(always)]
    pub fn is_64x(&self) -> bool {
        *self == RATIO_A::_64X
    }
    #[doc = "Checks if the value of the field is `_96X`"]
    #[inline(always)]
    pub fn is_96x(&self) -> bool {
        *self == RATIO_A::_96X
    }
    #[doc = "Checks if the value of the field is `_128X`"]
    #[inline(always)]
    pub fn is_128x(&self) -> bool {
        *self == RATIO_A::_128X
    }
    #[doc = "Checks if the value of the field is `_192X`"]
    #[inline(always)]
    pub fn is_192x(&self) -> bool {
        *self == RATIO_A::_192X
    }
    #[doc = "Checks if the value of the field is `_256X`"]
    #[inline(always)]
    pub fn is_256x(&self) -> bool {
        *self == RATIO_A::_256X
    }
    #[doc = "Checks if the value of the field is `_384X`"]
    #[inline(always)]
    pub fn is_384x(&self) -> bool {
        *self == RATIO_A::_384X
    }
    #[doc = "Checks if the value of the field is `_512X`"]
    #[inline(always)]
    pub fn is_512x(&self) -> bool {
        *self == RATIO_A::_512X
    }
}
#[doc = "Write proxy for field `RATIO`"]
pub struct RATIO_W<'a> {
    w: &'a mut W,
}
impl<'a> RATIO_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: RATIO_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "LRCK = MCK / 32"]
    #[inline(always)]
    pub fn _32x(self) -> &'a mut W {
        self.variant(RATIO_A::_32X)
    }
    #[doc = "LRCK = MCK / 48"]
    #[inline(always)]
    pub fn _48x(self) -> &'a mut W {
        self.variant(RATIO_A::_48X)
    }
    #[doc = "LRCK = MCK / 64"]
    #[inline(always)]
    pub fn _64x(self) -> &'a mut W {
        self.variant(RATIO_A::_64X)
    }
    #[doc = "LRCK = MCK / 96"]
    #[inline(always)]
    pub fn _96x(self) -> &'a mut W {
        self.variant(RATIO_A::_96X)
    }
    #[doc = "LRCK = MCK / 128"]
    #[inline(always)]
    pub fn _128x(self) -> &'a mut W {
        self.variant(RATIO_A::_128X)
    }
    #[doc = "LRCK = MCK / 192"]
    #[inline(always)]
    pub fn _192x(self) -> &'a mut W {
        self.variant(RATIO_A::_192X)
    }
    #[doc = "LRCK = MCK / 256"]
    #[inline(always)]
    pub fn _256x(self) -> &'a mut W {
        self.variant(RATIO_A::_256X)
    }
    #[doc = "LRCK = MCK / 384"]
    #[inline(always)]
    pub fn _384x(self) -> &'a mut W {
        self.variant(RATIO_A::_384X)
    }
    #[doc = "LRCK = MCK / 512"]
    #[inline(always)]
    pub fn _512x(self) -> &'a mut W {
        self.variant(RATIO_A::_512X)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x0f) | ((value as u32) & 0x0f);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:3 - MCK / LRCK ratio."]
    #[inline(always)]
    pub fn ratio(&self) -> RATIO_R {
        RATIO_R::new((self.bits & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:3 - MCK / LRCK ratio."]
    #[inline(always)]
    pub fn ratio(&mut self) -> RATIO_W {
        RATIO_W { w: self }
    }
}
