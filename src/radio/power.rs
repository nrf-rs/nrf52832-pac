#[doc = "Reader of register POWER"]
pub type R = crate::R<u32, super::POWER>;
#[doc = "Writer for register POWER"]
pub type W = crate::W<u32, super::POWER>;
#[doc = "Register POWER `reset()`'s with value 0x01"]
impl crate::ResetValue for super::POWER {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0x01
    }
}
#[doc = "Peripheral power control. The peripheral and its registers will be reset to its initial state by switching the peripheral off and then back on again.\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum POWER_A {
    #[doc = "0: Peripheral is powered off"]
    DISABLED = 0,
    #[doc = "1: Peripheral is powered on"]
    ENABLED = 1,
}
impl From<POWER_A> for bool {
    #[inline(always)]
    fn from(variant: POWER_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `POWER`"]
pub type POWER_R = crate::R<bool, POWER_A>;
impl POWER_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> POWER_A {
        match self.bits {
            false => POWER_A::DISABLED,
            true => POWER_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == POWER_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == POWER_A::ENABLED
    }
}
#[doc = "Write proxy for field `POWER`"]
pub struct POWER_W<'a> {
    w: &'a mut W,
}
impl<'a> POWER_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: POWER_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Peripheral is powered off"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(POWER_A::DISABLED)
    }
    #[doc = "Peripheral is powered on"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(POWER_A::ENABLED)
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
    #[doc = "Bit 0 - Peripheral power control. The peripheral and its registers will be reset to its initial state by switching the peripheral off and then back on again."]
    #[inline(always)]
    pub fn power(&self) -> POWER_R {
        POWER_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Peripheral power control. The peripheral and its registers will be reset to its initial state by switching the peripheral off and then back on again."]
    #[inline(always)]
    pub fn power(&mut self) -> POWER_W {
        POWER_W { w: self }
    }
}
