#[doc = "Reader of register CONFIG"]
pub type R = crate::R<u32, super::CONFIG>;
#[doc = "Writer for register CONFIG"]
pub type W = crate::W<u32, super::CONFIG>;
#[doc = "Register CONFIG `reset()`'s with value 0"]
impl crate::ResetValue for super::CONFIG {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Hardware flow control\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum HWFC_A {
    #[doc = "0: Disabled"]
    DISABLED = 0,
    #[doc = "1: Enabled"]
    ENABLED = 1,
}
impl From<HWFC_A> for bool {
    #[inline(always)]
    fn from(variant: HWFC_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `HWFC`"]
pub type HWFC_R = crate::R<bool, HWFC_A>;
impl HWFC_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> HWFC_A {
        match self.bits {
            false => HWFC_A::DISABLED,
            true => HWFC_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == HWFC_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == HWFC_A::ENABLED
    }
}
#[doc = "Write proxy for field `HWFC`"]
pub struct HWFC_W<'a> {
    w: &'a mut W,
}
impl<'a> HWFC_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: HWFC_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(HWFC_A::DISABLED)
    }
    #[doc = "Enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(HWFC_A::ENABLED)
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
        self.w.bits = (self.w.bits & !0x01) | ((value as u32) & 0x01);
        self.w
    }
}
#[doc = "Parity\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum PARITY_A {
    #[doc = "0: Exclude parity bit"]
    EXCLUDED = 0,
    #[doc = "7: Include parity bit"]
    INCLUDED = 7,
}
impl From<PARITY_A> for u8 {
    #[inline(always)]
    fn from(variant: PARITY_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `PARITY`"]
pub type PARITY_R = crate::R<u8, PARITY_A>;
impl PARITY_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, PARITY_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(PARITY_A::EXCLUDED),
            7 => Val(PARITY_A::INCLUDED),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `EXCLUDED`"]
    #[inline(always)]
    pub fn is_excluded(&self) -> bool {
        *self == PARITY_A::EXCLUDED
    }
    #[doc = "Checks if the value of the field is `INCLUDED`"]
    #[inline(always)]
    pub fn is_included(&self) -> bool {
        *self == PARITY_A::INCLUDED
    }
}
#[doc = "Write proxy for field `PARITY`"]
pub struct PARITY_W<'a> {
    w: &'a mut W,
}
impl<'a> PARITY_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PARITY_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "Exclude parity bit"]
    #[inline(always)]
    pub fn excluded(self) -> &'a mut W {
        self.variant(PARITY_A::EXCLUDED)
    }
    #[doc = "Include parity bit"]
    #[inline(always)]
    pub fn included(self) -> &'a mut W {
        self.variant(PARITY_A::INCLUDED)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 1)) | (((value as u32) & 0x07) << 1);
        self.w
    }
}
impl R {
    #[doc = "Bit 0 - Hardware flow control"]
    #[inline(always)]
    pub fn hwfc(&self) -> HWFC_R {
        HWFC_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bits 1:3 - Parity"]
    #[inline(always)]
    pub fn parity(&self) -> PARITY_R {
        PARITY_R::new(((self.bits >> 1) & 0x07) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - Hardware flow control"]
    #[inline(always)]
    pub fn hwfc(&mut self) -> HWFC_W {
        HWFC_W { w: self }
    }
    #[doc = "Bits 1:3 - Parity"]
    #[inline(always)]
    pub fn parity(&mut self) -> PARITY_W {
        PARITY_W { w: self }
    }
}
