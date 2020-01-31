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
#[doc = "Shortcut between END event and ACQUIRE task\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum END_ACQUIRE_A {
    #[doc = "0: Disable shortcut"]
    DISABLED = 0,
    #[doc = "1: Enable shortcut"]
    ENABLED = 1,
}
impl From<END_ACQUIRE_A> for bool {
    #[inline(always)]
    fn from(variant: END_ACQUIRE_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `END_ACQUIRE`"]
pub type END_ACQUIRE_R = crate::R<bool, END_ACQUIRE_A>;
impl END_ACQUIRE_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> END_ACQUIRE_A {
        match self.bits {
            false => END_ACQUIRE_A::DISABLED,
            true => END_ACQUIRE_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == END_ACQUIRE_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == END_ACQUIRE_A::ENABLED
    }
}
#[doc = "Write proxy for field `END_ACQUIRE`"]
pub struct END_ACQUIRE_W<'a> {
    w: &'a mut W,
}
impl<'a> END_ACQUIRE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: END_ACQUIRE_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Disable shortcut"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(END_ACQUIRE_A::DISABLED)
    }
    #[doc = "Enable shortcut"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(END_ACQUIRE_A::ENABLED)
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
        self.w.bits = (self.w.bits & !(0x01 << 2)) | (((value as u32) & 0x01) << 2);
        self.w
    }
}
impl R {
    #[doc = "Bit 2 - Shortcut between END event and ACQUIRE task"]
    #[inline(always)]
    pub fn end_acquire(&self) -> END_ACQUIRE_R {
        END_ACQUIRE_R::new(((self.bits >> 2) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 2 - Shortcut between END event and ACQUIRE task"]
    #[inline(always)]
    pub fn end_acquire(&mut self) -> END_ACQUIRE_W {
        END_ACQUIRE_W { w: self }
    }
}
