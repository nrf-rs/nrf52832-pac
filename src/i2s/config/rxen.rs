#[doc = "Reader of register RXEN"]
pub type R = crate::R<u32, super::RXEN>;
#[doc = "Writer for register RXEN"]
pub type W = crate::W<u32, super::RXEN>;
#[doc = "Register RXEN `reset()`'s with value 0"]
impl crate::ResetValue for super::RXEN {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reception (RX) enable.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RXEN_A {
    #[doc = "0: Reception disabled and now data will be written to the RXD.PTR address."]
    DISABLED = 0,
    #[doc = "1: Reception enabled."]
    ENABLED = 1,
}
impl From<RXEN_A> for bool {
    #[inline(always)]
    fn from(variant: RXEN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `RXEN`"]
pub type RXEN_R = crate::R<bool, RXEN_A>;
impl RXEN_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RXEN_A {
        match self.bits {
            false => RXEN_A::DISABLED,
            true => RXEN_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == RXEN_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == RXEN_A::ENABLED
    }
}
#[doc = "Write proxy for field `RXEN`"]
pub struct RXEN_W<'a> {
    w: &'a mut W,
}
impl<'a> RXEN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: RXEN_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Reception disabled and now data will be written to the RXD.PTR address."]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(RXEN_A::DISABLED)
    }
    #[doc = "Reception enabled."]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(RXEN_A::ENABLED)
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
    #[doc = "Bit 0 - Reception (RX) enable."]
    #[inline(always)]
    pub fn rxen(&self) -> RXEN_R {
        RXEN_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Reception (RX) enable."]
    #[inline(always)]
    pub fn rxen(&mut self) -> RXEN_W {
        RXEN_W { w: self }
    }
}
