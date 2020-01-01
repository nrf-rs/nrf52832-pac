#[doc = "Reader of register DISABLEINDEBUG"]
pub type R = crate::R<u32, super::DISABLEINDEBUG>;
#[doc = "Writer for register DISABLEINDEBUG"]
pub type W = crate::W<u32, super::DISABLEINDEBUG>;
#[doc = "Register DISABLEINDEBUG `reset()`'s with value 0x01"]
impl crate::ResetValue for super::DISABLEINDEBUG {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0x01
    }
}
#[doc = "Disable the protection mechanism for NVM regions while in debug interface mode. This register will only disable the protection mechanism if the device is in debug interface mode.\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DISABLEINDEBUG_A {
    #[doc = "1: Disable in debug"]
    DISABLED = 1,
    #[doc = "0: Enable in debug"]
    ENABLED = 0,
}
impl From<DISABLEINDEBUG_A> for bool {
    #[inline(always)]
    fn from(variant: DISABLEINDEBUG_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `DISABLEINDEBUG`"]
pub type DISABLEINDEBUG_R = crate::R<bool, DISABLEINDEBUG_A>;
impl DISABLEINDEBUG_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DISABLEINDEBUG_A {
        match self.bits {
            true => DISABLEINDEBUG_A::DISABLED,
            false => DISABLEINDEBUG_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == DISABLEINDEBUG_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == DISABLEINDEBUG_A::ENABLED
    }
}
#[doc = "Write proxy for field `DISABLEINDEBUG`"]
pub struct DISABLEINDEBUG_W<'a> {
    w: &'a mut W,
}
impl<'a> DISABLEINDEBUG_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: DISABLEINDEBUG_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Disable in debug"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(DISABLEINDEBUG_A::DISABLED)
    }
    #[doc = "Enable in debug"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(DISABLEINDEBUG_A::ENABLED)
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
    #[doc = "Bit 0 - Disable the protection mechanism for NVM regions while in debug interface mode. This register will only disable the protection mechanism if the device is in debug interface mode."]
    #[inline(always)]
    pub fn disableindebug(&self) -> DISABLEINDEBUG_R {
        DISABLEINDEBUG_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Disable the protection mechanism for NVM regions while in debug interface mode. This register will only disable the protection mechanism if the device is in debug interface mode."]
    #[inline(always)]
    pub fn disableindebug(&mut self) -> DISABLEINDEBUG_W {
        DISABLEINDEBUG_W { w: self }
    }
}
