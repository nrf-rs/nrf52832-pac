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
#[doc = "Shortcut between ENDRX event and STARTRX task\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ENDRX_STARTRX_A {
    #[doc = "0: Disable shortcut"]
    DISABLED = 0,
    #[doc = "1: Enable shortcut"]
    ENABLED = 1,
}
impl From<ENDRX_STARTRX_A> for bool {
    #[inline(always)]
    fn from(variant: ENDRX_STARTRX_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `ENDRX_STARTRX`"]
pub type ENDRX_STARTRX_R = crate::R<bool, ENDRX_STARTRX_A>;
impl ENDRX_STARTRX_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ENDRX_STARTRX_A {
        match self.bits {
            false => ENDRX_STARTRX_A::DISABLED,
            true => ENDRX_STARTRX_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == ENDRX_STARTRX_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == ENDRX_STARTRX_A::ENABLED
    }
}
#[doc = "Write proxy for field `ENDRX_STARTRX`"]
pub struct ENDRX_STARTRX_W<'a> {
    w: &'a mut W,
}
impl<'a> ENDRX_STARTRX_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: ENDRX_STARTRX_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Disable shortcut"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(ENDRX_STARTRX_A::DISABLED)
    }
    #[doc = "Enable shortcut"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(ENDRX_STARTRX_A::ENABLED)
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
        self.w.bits = (self.w.bits & !(0x01 << 5)) | (((value as u32) & 0x01) << 5);
        self.w
    }
}
#[doc = "Shortcut between ENDRX event and STOPRX task\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ENDRX_STOPRX_A {
    #[doc = "0: Disable shortcut"]
    DISABLED = 0,
    #[doc = "1: Enable shortcut"]
    ENABLED = 1,
}
impl From<ENDRX_STOPRX_A> for bool {
    #[inline(always)]
    fn from(variant: ENDRX_STOPRX_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `ENDRX_STOPRX`"]
pub type ENDRX_STOPRX_R = crate::R<bool, ENDRX_STOPRX_A>;
impl ENDRX_STOPRX_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ENDRX_STOPRX_A {
        match self.bits {
            false => ENDRX_STOPRX_A::DISABLED,
            true => ENDRX_STOPRX_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == ENDRX_STOPRX_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == ENDRX_STOPRX_A::ENABLED
    }
}
#[doc = "Write proxy for field `ENDRX_STOPRX`"]
pub struct ENDRX_STOPRX_W<'a> {
    w: &'a mut W,
}
impl<'a> ENDRX_STOPRX_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: ENDRX_STOPRX_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Disable shortcut"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(ENDRX_STOPRX_A::DISABLED)
    }
    #[doc = "Enable shortcut"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(ENDRX_STOPRX_A::ENABLED)
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
        self.w.bits = (self.w.bits & !(0x01 << 6)) | (((value as u32) & 0x01) << 6);
        self.w
    }
}
impl R {
    #[doc = "Bit 5 - Shortcut between ENDRX event and STARTRX task"]
    #[inline(always)]
    pub fn endrx_startrx(&self) -> ENDRX_STARTRX_R {
        ENDRX_STARTRX_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - Shortcut between ENDRX event and STOPRX task"]
    #[inline(always)]
    pub fn endrx_stoprx(&self) -> ENDRX_STOPRX_R {
        ENDRX_STOPRX_R::new(((self.bits >> 6) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 5 - Shortcut between ENDRX event and STARTRX task"]
    #[inline(always)]
    pub fn endrx_startrx(&mut self) -> ENDRX_STARTRX_W {
        ENDRX_STARTRX_W { w: self }
    }
    #[doc = "Bit 6 - Shortcut between ENDRX event and STOPRX task"]
    #[inline(always)]
    pub fn endrx_stoprx(&mut self) -> ENDRX_STOPRX_W {
        ENDRX_STOPRX_W { w: self }
    }
}
