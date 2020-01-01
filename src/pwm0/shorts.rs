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
#[doc = "Shortcut between SEQEND\\[0\\]
event and STOP task\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SEQEND0_STOP_A {
    #[doc = "0: Disable shortcut"]
    DISABLED = 0,
    #[doc = "1: Enable shortcut"]
    ENABLED = 1,
}
impl From<SEQEND0_STOP_A> for bool {
    #[inline(always)]
    fn from(variant: SEQEND0_STOP_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `SEQEND0_STOP`"]
pub type SEQEND0_STOP_R = crate::R<bool, SEQEND0_STOP_A>;
impl SEQEND0_STOP_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SEQEND0_STOP_A {
        match self.bits {
            false => SEQEND0_STOP_A::DISABLED,
            true => SEQEND0_STOP_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == SEQEND0_STOP_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == SEQEND0_STOP_A::ENABLED
    }
}
#[doc = "Write proxy for field `SEQEND0_STOP`"]
pub struct SEQEND0_STOP_W<'a> {
    w: &'a mut W,
}
impl<'a> SEQEND0_STOP_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SEQEND0_STOP_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Disable shortcut"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(SEQEND0_STOP_A::DISABLED)
    }
    #[doc = "Enable shortcut"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(SEQEND0_STOP_A::ENABLED)
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
#[doc = "Shortcut between SEQEND\\[1\\]
event and STOP task\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SEQEND1_STOP_A {
    #[doc = "0: Disable shortcut"]
    DISABLED = 0,
    #[doc = "1: Enable shortcut"]
    ENABLED = 1,
}
impl From<SEQEND1_STOP_A> for bool {
    #[inline(always)]
    fn from(variant: SEQEND1_STOP_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `SEQEND1_STOP`"]
pub type SEQEND1_STOP_R = crate::R<bool, SEQEND1_STOP_A>;
impl SEQEND1_STOP_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SEQEND1_STOP_A {
        match self.bits {
            false => SEQEND1_STOP_A::DISABLED,
            true => SEQEND1_STOP_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == SEQEND1_STOP_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == SEQEND1_STOP_A::ENABLED
    }
}
#[doc = "Write proxy for field `SEQEND1_STOP`"]
pub struct SEQEND1_STOP_W<'a> {
    w: &'a mut W,
}
impl<'a> SEQEND1_STOP_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SEQEND1_STOP_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Disable shortcut"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(SEQEND1_STOP_A::DISABLED)
    }
    #[doc = "Enable shortcut"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(SEQEND1_STOP_A::ENABLED)
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
#[doc = "Shortcut between LOOPSDONE event and SEQSTART\\[0\\]
task\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum LOOPSDONE_SEQSTART0_A {
    #[doc = "0: Disable shortcut"]
    DISABLED = 0,
    #[doc = "1: Enable shortcut"]
    ENABLED = 1,
}
impl From<LOOPSDONE_SEQSTART0_A> for bool {
    #[inline(always)]
    fn from(variant: LOOPSDONE_SEQSTART0_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `LOOPSDONE_SEQSTART0`"]
pub type LOOPSDONE_SEQSTART0_R = crate::R<bool, LOOPSDONE_SEQSTART0_A>;
impl LOOPSDONE_SEQSTART0_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> LOOPSDONE_SEQSTART0_A {
        match self.bits {
            false => LOOPSDONE_SEQSTART0_A::DISABLED,
            true => LOOPSDONE_SEQSTART0_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == LOOPSDONE_SEQSTART0_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == LOOPSDONE_SEQSTART0_A::ENABLED
    }
}
#[doc = "Write proxy for field `LOOPSDONE_SEQSTART0`"]
pub struct LOOPSDONE_SEQSTART0_W<'a> {
    w: &'a mut W,
}
impl<'a> LOOPSDONE_SEQSTART0_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: LOOPSDONE_SEQSTART0_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Disable shortcut"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(LOOPSDONE_SEQSTART0_A::DISABLED)
    }
    #[doc = "Enable shortcut"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(LOOPSDONE_SEQSTART0_A::ENABLED)
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
#[doc = "Shortcut between LOOPSDONE event and SEQSTART\\[1\\]
task\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum LOOPSDONE_SEQSTART1_A {
    #[doc = "0: Disable shortcut"]
    DISABLED = 0,
    #[doc = "1: Enable shortcut"]
    ENABLED = 1,
}
impl From<LOOPSDONE_SEQSTART1_A> for bool {
    #[inline(always)]
    fn from(variant: LOOPSDONE_SEQSTART1_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `LOOPSDONE_SEQSTART1`"]
pub type LOOPSDONE_SEQSTART1_R = crate::R<bool, LOOPSDONE_SEQSTART1_A>;
impl LOOPSDONE_SEQSTART1_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> LOOPSDONE_SEQSTART1_A {
        match self.bits {
            false => LOOPSDONE_SEQSTART1_A::DISABLED,
            true => LOOPSDONE_SEQSTART1_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == LOOPSDONE_SEQSTART1_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == LOOPSDONE_SEQSTART1_A::ENABLED
    }
}
#[doc = "Write proxy for field `LOOPSDONE_SEQSTART1`"]
pub struct LOOPSDONE_SEQSTART1_W<'a> {
    w: &'a mut W,
}
impl<'a> LOOPSDONE_SEQSTART1_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: LOOPSDONE_SEQSTART1_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Disable shortcut"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(LOOPSDONE_SEQSTART1_A::DISABLED)
    }
    #[doc = "Enable shortcut"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(LOOPSDONE_SEQSTART1_A::ENABLED)
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
#[doc = "Shortcut between LOOPSDONE event and STOP task\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum LOOPSDONE_STOP_A {
    #[doc = "0: Disable shortcut"]
    DISABLED = 0,
    #[doc = "1: Enable shortcut"]
    ENABLED = 1,
}
impl From<LOOPSDONE_STOP_A> for bool {
    #[inline(always)]
    fn from(variant: LOOPSDONE_STOP_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `LOOPSDONE_STOP`"]
pub type LOOPSDONE_STOP_R = crate::R<bool, LOOPSDONE_STOP_A>;
impl LOOPSDONE_STOP_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> LOOPSDONE_STOP_A {
        match self.bits {
            false => LOOPSDONE_STOP_A::DISABLED,
            true => LOOPSDONE_STOP_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == LOOPSDONE_STOP_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == LOOPSDONE_STOP_A::ENABLED
    }
}
#[doc = "Write proxy for field `LOOPSDONE_STOP`"]
pub struct LOOPSDONE_STOP_W<'a> {
    w: &'a mut W,
}
impl<'a> LOOPSDONE_STOP_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: LOOPSDONE_STOP_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Disable shortcut"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(LOOPSDONE_STOP_A::DISABLED)
    }
    #[doc = "Enable shortcut"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(LOOPSDONE_STOP_A::ENABLED)
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
    #[doc = "Bit 0 - Shortcut between SEQEND\\[0\\]
event and STOP task"]
    #[inline(always)]
    pub fn seqend0_stop(&self) -> SEQEND0_STOP_R {
        SEQEND0_STOP_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - Shortcut between SEQEND\\[1\\]
event and STOP task"]
    #[inline(always)]
    pub fn seqend1_stop(&self) -> SEQEND1_STOP_R {
        SEQEND1_STOP_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - Shortcut between LOOPSDONE event and SEQSTART\\[0\\]
task"]
    #[inline(always)]
    pub fn loopsdone_seqstart0(&self) -> LOOPSDONE_SEQSTART0_R {
        LOOPSDONE_SEQSTART0_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - Shortcut between LOOPSDONE event and SEQSTART\\[1\\]
task"]
    #[inline(always)]
    pub fn loopsdone_seqstart1(&self) -> LOOPSDONE_SEQSTART1_R {
        LOOPSDONE_SEQSTART1_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - Shortcut between LOOPSDONE event and STOP task"]
    #[inline(always)]
    pub fn loopsdone_stop(&self) -> LOOPSDONE_STOP_R {
        LOOPSDONE_STOP_R::new(((self.bits >> 4) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Shortcut between SEQEND\\[0\\]
event and STOP task"]
    #[inline(always)]
    pub fn seqend0_stop(&mut self) -> SEQEND0_STOP_W {
        SEQEND0_STOP_W { w: self }
    }
    #[doc = "Bit 1 - Shortcut between SEQEND\\[1\\]
event and STOP task"]
    #[inline(always)]
    pub fn seqend1_stop(&mut self) -> SEQEND1_STOP_W {
        SEQEND1_STOP_W { w: self }
    }
    #[doc = "Bit 2 - Shortcut between LOOPSDONE event and SEQSTART\\[0\\]
task"]
    #[inline(always)]
    pub fn loopsdone_seqstart0(&mut self) -> LOOPSDONE_SEQSTART0_W {
        LOOPSDONE_SEQSTART0_W { w: self }
    }
    #[doc = "Bit 3 - Shortcut between LOOPSDONE event and SEQSTART\\[1\\]
task"]
    #[inline(always)]
    pub fn loopsdone_seqstart1(&mut self) -> LOOPSDONE_SEQSTART1_W {
        LOOPSDONE_SEQSTART1_W { w: self }
    }
    #[doc = "Bit 4 - Shortcut between LOOPSDONE event and STOP task"]
    #[inline(always)]
    pub fn loopsdone_stop(&mut self) -> LOOPSDONE_STOP_W {
        LOOPSDONE_STOP_W { w: self }
    }
}
