#[doc = "Reader of register OVERSAMPLE"]
pub type R = crate::R<u32, super::OVERSAMPLE>;
#[doc = "Writer for register OVERSAMPLE"]
pub type W = crate::W<u32, super::OVERSAMPLE>;
#[doc = "Register OVERSAMPLE `reset()`'s with value 0"]
impl crate::ResetValue for super::OVERSAMPLE {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Oversample control\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum OVERSAMPLE_A {
    #[doc = "0: Bypass oversampling"]
    BYPASS = 0,
    #[doc = "1: Oversample 2x"]
    OVER2X = 1,
    #[doc = "2: Oversample 4x"]
    OVER4X = 2,
    #[doc = "3: Oversample 8x"]
    OVER8X = 3,
    #[doc = "4: Oversample 16x"]
    OVER16X = 4,
    #[doc = "5: Oversample 32x"]
    OVER32X = 5,
    #[doc = "6: Oversample 64x"]
    OVER64X = 6,
    #[doc = "7: Oversample 128x"]
    OVER128X = 7,
    #[doc = "8: Oversample 256x"]
    OVER256X = 8,
}
impl From<OVERSAMPLE_A> for u8 {
    #[inline(always)]
    fn from(variant: OVERSAMPLE_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `OVERSAMPLE`"]
pub type OVERSAMPLE_R = crate::R<u8, OVERSAMPLE_A>;
impl OVERSAMPLE_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, OVERSAMPLE_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(OVERSAMPLE_A::BYPASS),
            1 => Val(OVERSAMPLE_A::OVER2X),
            2 => Val(OVERSAMPLE_A::OVER4X),
            3 => Val(OVERSAMPLE_A::OVER8X),
            4 => Val(OVERSAMPLE_A::OVER16X),
            5 => Val(OVERSAMPLE_A::OVER32X),
            6 => Val(OVERSAMPLE_A::OVER64X),
            7 => Val(OVERSAMPLE_A::OVER128X),
            8 => Val(OVERSAMPLE_A::OVER256X),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `BYPASS`"]
    #[inline(always)]
    pub fn is_bypass(&self) -> bool {
        *self == OVERSAMPLE_A::BYPASS
    }
    #[doc = "Checks if the value of the field is `OVER2X`"]
    #[inline(always)]
    pub fn is_over2x(&self) -> bool {
        *self == OVERSAMPLE_A::OVER2X
    }
    #[doc = "Checks if the value of the field is `OVER4X`"]
    #[inline(always)]
    pub fn is_over4x(&self) -> bool {
        *self == OVERSAMPLE_A::OVER4X
    }
    #[doc = "Checks if the value of the field is `OVER8X`"]
    #[inline(always)]
    pub fn is_over8x(&self) -> bool {
        *self == OVERSAMPLE_A::OVER8X
    }
    #[doc = "Checks if the value of the field is `OVER16X`"]
    #[inline(always)]
    pub fn is_over16x(&self) -> bool {
        *self == OVERSAMPLE_A::OVER16X
    }
    #[doc = "Checks if the value of the field is `OVER32X`"]
    #[inline(always)]
    pub fn is_over32x(&self) -> bool {
        *self == OVERSAMPLE_A::OVER32X
    }
    #[doc = "Checks if the value of the field is `OVER64X`"]
    #[inline(always)]
    pub fn is_over64x(&self) -> bool {
        *self == OVERSAMPLE_A::OVER64X
    }
    #[doc = "Checks if the value of the field is `OVER128X`"]
    #[inline(always)]
    pub fn is_over128x(&self) -> bool {
        *self == OVERSAMPLE_A::OVER128X
    }
    #[doc = "Checks if the value of the field is `OVER256X`"]
    #[inline(always)]
    pub fn is_over256x(&self) -> bool {
        *self == OVERSAMPLE_A::OVER256X
    }
}
#[doc = "Write proxy for field `OVERSAMPLE`"]
pub struct OVERSAMPLE_W<'a> {
    w: &'a mut W,
}
impl<'a> OVERSAMPLE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: OVERSAMPLE_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "Bypass oversampling"]
    #[inline(always)]
    pub fn bypass(self) -> &'a mut W {
        self.variant(OVERSAMPLE_A::BYPASS)
    }
    #[doc = "Oversample 2x"]
    #[inline(always)]
    pub fn over2x(self) -> &'a mut W {
        self.variant(OVERSAMPLE_A::OVER2X)
    }
    #[doc = "Oversample 4x"]
    #[inline(always)]
    pub fn over4x(self) -> &'a mut W {
        self.variant(OVERSAMPLE_A::OVER4X)
    }
    #[doc = "Oversample 8x"]
    #[inline(always)]
    pub fn over8x(self) -> &'a mut W {
        self.variant(OVERSAMPLE_A::OVER8X)
    }
    #[doc = "Oversample 16x"]
    #[inline(always)]
    pub fn over16x(self) -> &'a mut W {
        self.variant(OVERSAMPLE_A::OVER16X)
    }
    #[doc = "Oversample 32x"]
    #[inline(always)]
    pub fn over32x(self) -> &'a mut W {
        self.variant(OVERSAMPLE_A::OVER32X)
    }
    #[doc = "Oversample 64x"]
    #[inline(always)]
    pub fn over64x(self) -> &'a mut W {
        self.variant(OVERSAMPLE_A::OVER64X)
    }
    #[doc = "Oversample 128x"]
    #[inline(always)]
    pub fn over128x(self) -> &'a mut W {
        self.variant(OVERSAMPLE_A::OVER128X)
    }
    #[doc = "Oversample 256x"]
    #[inline(always)]
    pub fn over256x(self) -> &'a mut W {
        self.variant(OVERSAMPLE_A::OVER256X)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x0f) | ((value as u32) & 0x0f);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:3 - Oversample control"]
    #[inline(always)]
    pub fn oversample(&self) -> OVERSAMPLE_R {
        OVERSAMPLE_R::new((self.bits & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:3 - Oversample control"]
    #[inline(always)]
    pub fn oversample(&mut self) -> OVERSAMPLE_W {
        OVERSAMPLE_W { w: self }
    }
}
