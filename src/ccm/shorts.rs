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
#[doc = "Shortcut between ENDKSGEN event and CRYPT task\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ENDKSGEN_CRYPT_A {
    #[doc = "0: Disable shortcut"]
    DISABLED = 0,
    #[doc = "1: Enable shortcut"]
    ENABLED = 1,
}
impl From<ENDKSGEN_CRYPT_A> for bool {
    #[inline(always)]
    fn from(variant: ENDKSGEN_CRYPT_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `ENDKSGEN_CRYPT`"]
pub type ENDKSGEN_CRYPT_R = crate::R<bool, ENDKSGEN_CRYPT_A>;
impl ENDKSGEN_CRYPT_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ENDKSGEN_CRYPT_A {
        match self.bits {
            false => ENDKSGEN_CRYPT_A::DISABLED,
            true => ENDKSGEN_CRYPT_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == ENDKSGEN_CRYPT_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == ENDKSGEN_CRYPT_A::ENABLED
    }
}
#[doc = "Write proxy for field `ENDKSGEN_CRYPT`"]
pub struct ENDKSGEN_CRYPT_W<'a> {
    w: &'a mut W,
}
impl<'a> ENDKSGEN_CRYPT_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: ENDKSGEN_CRYPT_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Disable shortcut"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(ENDKSGEN_CRYPT_A::DISABLED)
    }
    #[doc = "Enable shortcut"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(ENDKSGEN_CRYPT_A::ENABLED)
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
    #[doc = "Bit 0 - Shortcut between ENDKSGEN event and CRYPT task"]
    #[inline(always)]
    pub fn endksgen_crypt(&self) -> ENDKSGEN_CRYPT_R {
        ENDKSGEN_CRYPT_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Shortcut between ENDKSGEN event and CRYPT task"]
    #[inline(always)]
    pub fn endksgen_crypt(&mut self) -> ENDKSGEN_CRYPT_W {
        ENDKSGEN_CRYPT_W { w: self }
    }
}
