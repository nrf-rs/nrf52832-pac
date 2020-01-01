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
#[doc = "Shortcut between BB event and SUSPEND task\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum BB_SUSPEND_A {
    #[doc = "0: Disable shortcut"]
    DISABLED = 0,
    #[doc = "1: Enable shortcut"]
    ENABLED = 1,
}
impl From<BB_SUSPEND_A> for bool {
    #[inline(always)]
    fn from(variant: BB_SUSPEND_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `BB_SUSPEND`"]
pub type BB_SUSPEND_R = crate::R<bool, BB_SUSPEND_A>;
impl BB_SUSPEND_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> BB_SUSPEND_A {
        match self.bits {
            false => BB_SUSPEND_A::DISABLED,
            true => BB_SUSPEND_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == BB_SUSPEND_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == BB_SUSPEND_A::ENABLED
    }
}
#[doc = "Write proxy for field `BB_SUSPEND`"]
pub struct BB_SUSPEND_W<'a> {
    w: &'a mut W,
}
impl<'a> BB_SUSPEND_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: BB_SUSPEND_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Disable shortcut"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(BB_SUSPEND_A::DISABLED)
    }
    #[doc = "Enable shortcut"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(BB_SUSPEND_A::ENABLED)
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
#[doc = "Shortcut between BB event and STOP task\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum BB_STOP_A {
    #[doc = "0: Disable shortcut"]
    DISABLED = 0,
    #[doc = "1: Enable shortcut"]
    ENABLED = 1,
}
impl From<BB_STOP_A> for bool {
    #[inline(always)]
    fn from(variant: BB_STOP_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `BB_STOP`"]
pub type BB_STOP_R = crate::R<bool, BB_STOP_A>;
impl BB_STOP_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> BB_STOP_A {
        match self.bits {
            false => BB_STOP_A::DISABLED,
            true => BB_STOP_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == BB_STOP_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == BB_STOP_A::ENABLED
    }
}
#[doc = "Write proxy for field `BB_STOP`"]
pub struct BB_STOP_W<'a> {
    w: &'a mut W,
}
impl<'a> BB_STOP_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: BB_STOP_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Disable shortcut"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(BB_STOP_A::DISABLED)
    }
    #[doc = "Enable shortcut"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(BB_STOP_A::ENABLED)
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
        self.w.bits = (self.w.bits & !(0x01 << 1)) | (((value as u32) & 0x01) << 1);
        self.w
    }
}
impl R {
    #[doc = "Bit 0 - Shortcut between BB event and SUSPEND task"]
    #[inline(always)]
    pub fn bb_suspend(&self) -> BB_SUSPEND_R {
        BB_SUSPEND_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - Shortcut between BB event and STOP task"]
    #[inline(always)]
    pub fn bb_stop(&self) -> BB_STOP_R {
        BB_STOP_R::new(((self.bits >> 1) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Shortcut between BB event and SUSPEND task"]
    #[inline(always)]
    pub fn bb_suspend(&mut self) -> BB_SUSPEND_W {
        BB_SUSPEND_W { w: self }
    }
    #[doc = "Bit 1 - Shortcut between BB event and STOP task"]
    #[inline(always)]
    pub fn bb_stop(&mut self) -> BB_STOP_W {
        BB_STOP_W { w: self }
    }
}
