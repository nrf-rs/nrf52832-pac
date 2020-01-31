#[doc = "Reader of register MCKEN"]
pub type R = crate::R<u32, super::MCKEN>;
#[doc = "Writer for register MCKEN"]
pub type W = crate::W<u32, super::MCKEN>;
#[doc = "Register MCKEN `reset()`'s with value 0x01"]
impl crate::ResetValue for super::MCKEN {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0x01
    }
}
#[doc = "Master clock generator enable.\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum MCKEN_A {
    #[doc = "0: Master clock generator disabled and PSEL.MCK not connected(available as GPIO)."]
    DISABLED = 0,
    #[doc = "1: Master clock generator running and MCK output on PSEL.MCK."]
    ENABLED = 1,
}
impl From<MCKEN_A> for bool {
    #[inline(always)]
    fn from(variant: MCKEN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `MCKEN`"]
pub type MCKEN_R = crate::R<bool, MCKEN_A>;
impl MCKEN_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> MCKEN_A {
        match self.bits {
            false => MCKEN_A::DISABLED,
            true => MCKEN_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == MCKEN_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == MCKEN_A::ENABLED
    }
}
#[doc = "Write proxy for field `MCKEN`"]
pub struct MCKEN_W<'a> {
    w: &'a mut W,
}
impl<'a> MCKEN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: MCKEN_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Master clock generator disabled and PSEL.MCK not connected(available as GPIO)."]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(MCKEN_A::DISABLED)
    }
    #[doc = "Master clock generator running and MCK output on PSEL.MCK."]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(MCKEN_A::ENABLED)
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
impl R {
    #[doc = "Bit 0 - Master clock generator enable."]
    #[inline(always)]
    pub fn mcken(&self) -> MCKEN_R {
        MCKEN_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Master clock generator enable."]
    #[inline(always)]
    pub fn mcken(&mut self) -> MCKEN_W {
        MCKEN_W { w: self }
    }
}
