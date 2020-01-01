#[doc = "Reader of register SHORTS"]
pub type R = crate::R<u32, super::SHORTS>;
#[doc = "Writer for register SHORTS"]
pub type W = crate::W<u32, super::SHORTS>;
#[doc = "Register SHORTS `reset()`'s with value 0"]
impl crate::ResetValue for super::SHORTS {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Shortcut between VALRDY event and STOP task\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum VALRDY_STOP_A {
    #[doc = "0: Disable shortcut"]
    DISABLED = 0,
    #[doc = "1: Enable shortcut"]
    ENABLED = 1,
}
impl From<VALRDY_STOP_A> for bool {
    #[inline(always)]
    fn from(variant: VALRDY_STOP_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `VALRDY_STOP`"]
pub type VALRDY_STOP_R = crate::R<bool, VALRDY_STOP_A>;
impl VALRDY_STOP_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> VALRDY_STOP_A {
        match self.bits {
            false => VALRDY_STOP_A::DISABLED,
            true => VALRDY_STOP_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == VALRDY_STOP_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == VALRDY_STOP_A::ENABLED
    }
}
#[doc = "Write proxy for field `VALRDY_STOP`"]
pub struct VALRDY_STOP_W<'a> {
    w: &'a mut W,
}
impl<'a> VALRDY_STOP_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: VALRDY_STOP_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Disable shortcut"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(VALRDY_STOP_A::DISABLED)
    }
    #[doc = "Enable shortcut"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(VALRDY_STOP_A::ENABLED)
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
    #[doc = "Bit 0 - Shortcut between VALRDY event and STOP task"]
    #[inline(always)]
    pub fn valrdy_stop(&self) -> VALRDY_STOP_R {
        VALRDY_STOP_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Shortcut between VALRDY event and STOP task"]
    #[inline(always)]
    pub fn valrdy_stop(&mut self) -> VALRDY_STOP_W {
        VALRDY_STOP_W { w: self }
    }
}
