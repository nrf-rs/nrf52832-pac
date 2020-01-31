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
#[doc = "Bias correction\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DERCEN_A {
    #[doc = "0: Disabled"]
    DISABLED = 0,
    #[doc = "1: Enabled"]
    ENABLED = 1,
}
impl From<DERCEN_A> for bool {
    #[inline(always)]
    fn from(variant: DERCEN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `DERCEN`"]
pub type DERCEN_R = crate::R<bool, DERCEN_A>;
impl DERCEN_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DERCEN_A {
        match self.bits {
            false => DERCEN_A::DISABLED,
            true => DERCEN_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == DERCEN_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == DERCEN_A::ENABLED
    }
}
#[doc = "Write proxy for field `DERCEN`"]
pub struct DERCEN_W<'a> {
    w: &'a mut W,
}
impl<'a> DERCEN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: DERCEN_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(DERCEN_A::DISABLED)
    }
    #[doc = "Enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(DERCEN_A::ENABLED)
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
    #[doc = "Bit 0 - Bias correction"]
    #[inline(always)]
    pub fn dercen(&self) -> DERCEN_R {
        DERCEN_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Bias correction"]
    #[inline(always)]
    pub fn dercen(&mut self) -> DERCEN_W {
        DERCEN_W { w: self }
    }
}
