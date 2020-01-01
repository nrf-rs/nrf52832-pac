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
#[doc = "Shortcut between WRITE event and SUSPEND task\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum WRITE_SUSPEND_A {
    #[doc = "0: Disable shortcut"]
    DISABLED = 0,
    #[doc = "1: Enable shortcut"]
    ENABLED = 1,
}
impl From<WRITE_SUSPEND_A> for bool {
    #[inline(always)]
    fn from(variant: WRITE_SUSPEND_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `WRITE_SUSPEND`"]
pub type WRITE_SUSPEND_R = crate::R<bool, WRITE_SUSPEND_A>;
impl WRITE_SUSPEND_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> WRITE_SUSPEND_A {
        match self.bits {
            false => WRITE_SUSPEND_A::DISABLED,
            true => WRITE_SUSPEND_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == WRITE_SUSPEND_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == WRITE_SUSPEND_A::ENABLED
    }
}
#[doc = "Write proxy for field `WRITE_SUSPEND`"]
pub struct WRITE_SUSPEND_W<'a> {
    w: &'a mut W,
}
impl<'a> WRITE_SUSPEND_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: WRITE_SUSPEND_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Disable shortcut"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(WRITE_SUSPEND_A::DISABLED)
    }
    #[doc = "Enable shortcut"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(WRITE_SUSPEND_A::ENABLED)
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
        self.w.bits = (self.w.bits & !(0x01 << 13)) | (((value as u32) & 0x01) << 13);
        self.w
    }
}
#[doc = "Shortcut between READ event and SUSPEND task\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum READ_SUSPEND_A {
    #[doc = "0: Disable shortcut"]
    DISABLED = 0,
    #[doc = "1: Enable shortcut"]
    ENABLED = 1,
}
impl From<READ_SUSPEND_A> for bool {
    #[inline(always)]
    fn from(variant: READ_SUSPEND_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `READ_SUSPEND`"]
pub type READ_SUSPEND_R = crate::R<bool, READ_SUSPEND_A>;
impl READ_SUSPEND_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> READ_SUSPEND_A {
        match self.bits {
            false => READ_SUSPEND_A::DISABLED,
            true => READ_SUSPEND_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == READ_SUSPEND_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == READ_SUSPEND_A::ENABLED
    }
}
#[doc = "Write proxy for field `READ_SUSPEND`"]
pub struct READ_SUSPEND_W<'a> {
    w: &'a mut W,
}
impl<'a> READ_SUSPEND_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: READ_SUSPEND_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Disable shortcut"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(READ_SUSPEND_A::DISABLED)
    }
    #[doc = "Enable shortcut"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(READ_SUSPEND_A::ENABLED)
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
        self.w.bits = (self.w.bits & !(0x01 << 14)) | (((value as u32) & 0x01) << 14);
        self.w
    }
}
impl R {
    #[doc = "Bit 13 - Shortcut between WRITE event and SUSPEND task"]
    #[inline(always)]
    pub fn write_suspend(&self) -> WRITE_SUSPEND_R {
        WRITE_SUSPEND_R::new(((self.bits >> 13) & 0x01) != 0)
    }
    #[doc = "Bit 14 - Shortcut between READ event and SUSPEND task"]
    #[inline(always)]
    pub fn read_suspend(&self) -> READ_SUSPEND_R {
        READ_SUSPEND_R::new(((self.bits >> 14) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 13 - Shortcut between WRITE event and SUSPEND task"]
    #[inline(always)]
    pub fn write_suspend(&mut self) -> WRITE_SUSPEND_W {
        WRITE_SUSPEND_W { w: self }
    }
    #[doc = "Bit 14 - Shortcut between READ event and SUSPEND task"]
    #[inline(always)]
    pub fn read_suspend(&mut self) -> READ_SUSPEND_W {
        READ_SUSPEND_W { w: self }
    }
}
