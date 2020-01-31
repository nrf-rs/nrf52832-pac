#[doc = "Reader of register MODE"]
pub type R = crate::R<u32, super::MODE>;
#[doc = "Writer for register MODE"]
pub type W = crate::W<u32, super::MODE>;
#[doc = "Register MODE `reset()`'s with value 0"]
impl crate::ResetValue for super::MODE {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Speed and power modes\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum SP_A {
    #[doc = "0: Low-power mode"]
    LOW = 0,
    #[doc = "1: Normal mode"]
    NORMAL = 1,
    #[doc = "2: High-speed mode"]
    HIGH = 2,
}
impl From<SP_A> for u8 {
    #[inline(always)]
    fn from(variant: SP_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `SP`"]
pub type SP_R = crate::R<u8, SP_A>;
impl SP_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, SP_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(SP_A::LOW),
            1 => Val(SP_A::NORMAL),
            2 => Val(SP_A::HIGH),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `LOW`"]
    #[inline(always)]
    pub fn is_low(&self) -> bool {
        *self == SP_A::LOW
    }
    #[doc = "Checks if the value of the field is `NORMAL`"]
    #[inline(always)]
    pub fn is_normal(&self) -> bool {
        *self == SP_A::NORMAL
    }
    #[doc = "Checks if the value of the field is `HIGH`"]
    #[inline(always)]
    pub fn is_high(&self) -> bool {
        *self == SP_A::HIGH
    }
}
#[doc = "Write proxy for field `SP`"]
pub struct SP_W<'a> {
    w: &'a mut W,
}
impl<'a> SP_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SP_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "Low-power mode"]
    #[inline(always)]
    pub fn low(self) -> &'a mut W {
        self.variant(SP_A::LOW)
    }
    #[doc = "Normal mode"]
    #[inline(always)]
    pub fn normal(self) -> &'a mut W {
        self.variant(SP_A::NORMAL)
    }
    #[doc = "High-speed mode"]
    #[inline(always)]
    pub fn high(self) -> &'a mut W {
        self.variant(SP_A::HIGH)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x03) | ((value as u32) & 0x03);
        self.w
    }
}
#[doc = "Main operation modes\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum MAIN_A {
    #[doc = "0: Single-ended mode"]
    SE = 0,
    #[doc = "1: Differential mode"]
    DIFF = 1,
}
impl From<MAIN_A> for bool {
    #[inline(always)]
    fn from(variant: MAIN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `MAIN`"]
pub type MAIN_R = crate::R<bool, MAIN_A>;
impl MAIN_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> MAIN_A {
        match self.bits {
            false => MAIN_A::SE,
            true => MAIN_A::DIFF,
        }
    }
    #[doc = "Checks if the value of the field is `SE`"]
    #[inline(always)]
    pub fn is_se(&self) -> bool {
        *self == MAIN_A::SE
    }
    #[doc = "Checks if the value of the field is `DIFF`"]
    #[inline(always)]
    pub fn is_diff(&self) -> bool {
        *self == MAIN_A::DIFF
    }
}
#[doc = "Write proxy for field `MAIN`"]
pub struct MAIN_W<'a> {
    w: &'a mut W,
}
impl<'a> MAIN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: MAIN_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Single-ended mode"]
    #[inline(always)]
    pub fn se(self) -> &'a mut W {
        self.variant(MAIN_A::SE)
    }
    #[doc = "Differential mode"]
    #[inline(always)]
    pub fn diff(self) -> &'a mut W {
        self.variant(MAIN_A::DIFF)
    }
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 8)) | (((value as u32) & 0x01) << 8);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:1 - Speed and power modes"]
    #[inline(always)]
    pub fn sp(&self) -> SP_R {
        SP_R::new((self.bits & 0x03) as u8)
    }
    #[doc = "Bit 8 - Main operation modes"]
    #[inline(always)]
    pub fn main(&self) -> MAIN_R {
        MAIN_R::new(((self.bits >> 8) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bits 0:1 - Speed and power modes"]
    #[inline(always)]
    pub fn sp(&mut self) -> SP_W {
        SP_W { w: self }
    }
    #[doc = "Bit 8 - Main operation modes"]
    #[inline(always)]
    pub fn main(&mut self) -> MAIN_W {
        MAIN_W { w: self }
    }
}
