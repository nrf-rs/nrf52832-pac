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
#[doc = "Shortcut between CTS event and STARTRX task\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CTS_STARTRX_A {
    #[doc = "0: Disable shortcut"]
    DISABLED = 0,
    #[doc = "1: Enable shortcut"]
    ENABLED = 1,
}
impl From<CTS_STARTRX_A> for bool {
    #[inline(always)]
    fn from(variant: CTS_STARTRX_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `CTS_STARTRX`"]
pub type CTS_STARTRX_R = crate::R<bool, CTS_STARTRX_A>;
impl CTS_STARTRX_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CTS_STARTRX_A {
        match self.bits {
            false => CTS_STARTRX_A::DISABLED,
            true => CTS_STARTRX_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == CTS_STARTRX_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == CTS_STARTRX_A::ENABLED
    }
}
#[doc = "Write proxy for field `CTS_STARTRX`"]
pub struct CTS_STARTRX_W<'a> {
    w: &'a mut W,
}
impl<'a> CTS_STARTRX_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CTS_STARTRX_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Disable shortcut"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(CTS_STARTRX_A::DISABLED)
    }
    #[doc = "Enable shortcut"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(CTS_STARTRX_A::ENABLED)
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
        self.w.bits = (self.w.bits & !(0x01 << 3)) | (((value as u32) & 0x01) << 3);
        self.w
    }
}
#[doc = "Shortcut between NCTS event and STOPRX task\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum NCTS_STOPRX_A {
    #[doc = "0: Disable shortcut"]
    DISABLED = 0,
    #[doc = "1: Enable shortcut"]
    ENABLED = 1,
}
impl From<NCTS_STOPRX_A> for bool {
    #[inline(always)]
    fn from(variant: NCTS_STOPRX_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `NCTS_STOPRX`"]
pub type NCTS_STOPRX_R = crate::R<bool, NCTS_STOPRX_A>;
impl NCTS_STOPRX_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> NCTS_STOPRX_A {
        match self.bits {
            false => NCTS_STOPRX_A::DISABLED,
            true => NCTS_STOPRX_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == NCTS_STOPRX_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == NCTS_STOPRX_A::ENABLED
    }
}
#[doc = "Write proxy for field `NCTS_STOPRX`"]
pub struct NCTS_STOPRX_W<'a> {
    w: &'a mut W,
}
impl<'a> NCTS_STOPRX_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: NCTS_STOPRX_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Disable shortcut"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(NCTS_STOPRX_A::DISABLED)
    }
    #[doc = "Enable shortcut"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(NCTS_STOPRX_A::ENABLED)
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
        self.w.bits = (self.w.bits & !(0x01 << 4)) | (((value as u32) & 0x01) << 4);
        self.w
    }
}
impl R {
    #[doc = "Bit 3 - Shortcut between CTS event and STARTRX task"]
    #[inline(always)]
    pub fn cts_startrx(&self) -> CTS_STARTRX_R {
        CTS_STARTRX_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - Shortcut between NCTS event and STOPRX task"]
    #[inline(always)]
    pub fn ncts_stoprx(&self) -> NCTS_STOPRX_R {
        NCTS_STOPRX_R::new(((self.bits >> 4) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 3 - Shortcut between CTS event and STARTRX task"]
    #[inline(always)]
    pub fn cts_startrx(&mut self) -> CTS_STARTRX_W {
        CTS_STARTRX_W { w: self }
    }
    #[doc = "Bit 4 - Shortcut between NCTS event and STOPRX task"]
    #[inline(always)]
    pub fn ncts_stoprx(&mut self) -> NCTS_STOPRX_W {
        NCTS_STOPRX_W { w: self }
    }
}
