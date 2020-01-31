#[doc = "Reader of register TXEN"]
pub type R = crate::R<u32, super::TXEN>;
#[doc = "Writer for register TXEN"]
pub type W = crate::W<u32, super::TXEN>;
#[doc = "Register TXEN `reset()`'s with value 0x01"]
impl crate::ResetValue for super::TXEN {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0x01
    }
}
#[doc = "Transmission (TX) enable.\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TXEN_A {
    #[doc = "0: Transmission disabled and now data will be read from the RXD.TXD address."]
    DISABLED = 0,
    #[doc = "1: Transmission enabled."]
    ENABLED = 1,
}
impl From<TXEN_A> for bool {
    #[inline(always)]
    fn from(variant: TXEN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `TXEN`"]
pub type TXEN_R = crate::R<bool, TXEN_A>;
impl TXEN_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TXEN_A {
        match self.bits {
            false => TXEN_A::DISABLED,
            true => TXEN_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == TXEN_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == TXEN_A::ENABLED
    }
}
#[doc = "Write proxy for field `TXEN`"]
pub struct TXEN_W<'a> {
    w: &'a mut W,
}
impl<'a> TXEN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: TXEN_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Transmission disabled and now data will be read from the RXD.TXD address."]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(TXEN_A::DISABLED)
    }
    #[doc = "Transmission enabled."]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(TXEN_A::ENABLED)
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
    #[doc = "Bit 0 - Transmission (TX) enable."]
    #[inline(always)]
    pub fn txen(&self) -> TXEN_R {
        TXEN_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Transmission (TX) enable."]
    #[inline(always)]
    pub fn txen(&mut self) -> TXEN_W {
        TXEN_W { w: self }
    }
}
