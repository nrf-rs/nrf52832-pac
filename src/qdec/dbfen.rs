#[doc = "Reader of register DBFEN"]
pub type R = crate::R<u32, super::DBFEN>;
#[doc = "Writer for register DBFEN"]
pub type W = crate::W<u32, super::DBFEN>;
#[doc = "Register DBFEN `reset()`'s with value 0"]
impl crate::ResetValue for super::DBFEN {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Enable input debounce filters\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DBFEN_A {
    #[doc = "0: Debounce input filters disabled"]
    DISABLED = 0,
    #[doc = "1: Debounce input filters enabled"]
    ENABLED = 1,
}
impl From<DBFEN_A> for bool {
    #[inline(always)]
    fn from(variant: DBFEN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `DBFEN`"]
pub type DBFEN_R = crate::R<bool, DBFEN_A>;
impl DBFEN_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DBFEN_A {
        match self.bits {
            false => DBFEN_A::DISABLED,
            true => DBFEN_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == DBFEN_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == DBFEN_A::ENABLED
    }
}
#[doc = "Write proxy for field `DBFEN`"]
pub struct DBFEN_W<'a> {
    w: &'a mut W,
}
impl<'a> DBFEN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: DBFEN_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Debounce input filters disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(DBFEN_A::DISABLED)
    }
    #[doc = "Debounce input filters enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(DBFEN_A::ENABLED)
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
    #[doc = "Bit 0 - Enable input debounce filters"]
    #[inline(always)]
    pub fn dbfen(&self) -> DBFEN_R {
        DBFEN_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Enable input debounce filters"]
    #[inline(always)]
    pub fn dbfen(&mut self) -> DBFEN_W {
        DBFEN_W { w: self }
    }
}
