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
#[doc = "Shortcut between READY event and SAMPLE task\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum READY_SAMPLE_A {
    #[doc = "0: Disable shortcut"]
    DISABLED = 0,
    #[doc = "1: Enable shortcut"]
    ENABLED = 1,
}
impl From<READY_SAMPLE_A> for bool {
    #[inline(always)]
    fn from(variant: READY_SAMPLE_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `READY_SAMPLE`"]
pub type READY_SAMPLE_R = crate::R<bool, READY_SAMPLE_A>;
impl READY_SAMPLE_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> READY_SAMPLE_A {
        match self.bits {
            false => READY_SAMPLE_A::DISABLED,
            true => READY_SAMPLE_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == READY_SAMPLE_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == READY_SAMPLE_A::ENABLED
    }
}
#[doc = "Write proxy for field `READY_SAMPLE`"]
pub struct READY_SAMPLE_W<'a> {
    w: &'a mut W,
}
impl<'a> READY_SAMPLE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: READY_SAMPLE_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Disable shortcut"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(READY_SAMPLE_A::DISABLED)
    }
    #[doc = "Enable shortcut"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(READY_SAMPLE_A::ENABLED)
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
#[doc = "Shortcut between READY event and STOP task\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum READY_STOP_A {
    #[doc = "0: Disable shortcut"]
    DISABLED = 0,
    #[doc = "1: Enable shortcut"]
    ENABLED = 1,
}
impl From<READY_STOP_A> for bool {
    #[inline(always)]
    fn from(variant: READY_STOP_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `READY_STOP`"]
pub type READY_STOP_R = crate::R<bool, READY_STOP_A>;
impl READY_STOP_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> READY_STOP_A {
        match self.bits {
            false => READY_STOP_A::DISABLED,
            true => READY_STOP_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == READY_STOP_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == READY_STOP_A::ENABLED
    }
}
#[doc = "Write proxy for field `READY_STOP`"]
pub struct READY_STOP_W<'a> {
    w: &'a mut W,
}
impl<'a> READY_STOP_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: READY_STOP_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Disable shortcut"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(READY_STOP_A::DISABLED)
    }
    #[doc = "Enable shortcut"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(READY_STOP_A::ENABLED)
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
#[doc = "Shortcut between DOWN event and STOP task\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DOWN_STOP_A {
    #[doc = "0: Disable shortcut"]
    DISABLED = 0,
    #[doc = "1: Enable shortcut"]
    ENABLED = 1,
}
impl From<DOWN_STOP_A> for bool {
    #[inline(always)]
    fn from(variant: DOWN_STOP_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `DOWN_STOP`"]
pub type DOWN_STOP_R = crate::R<bool, DOWN_STOP_A>;
impl DOWN_STOP_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DOWN_STOP_A {
        match self.bits {
            false => DOWN_STOP_A::DISABLED,
            true => DOWN_STOP_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == DOWN_STOP_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == DOWN_STOP_A::ENABLED
    }
}
#[doc = "Write proxy for field `DOWN_STOP`"]
pub struct DOWN_STOP_W<'a> {
    w: &'a mut W,
}
impl<'a> DOWN_STOP_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: DOWN_STOP_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Disable shortcut"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(DOWN_STOP_A::DISABLED)
    }
    #[doc = "Enable shortcut"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(DOWN_STOP_A::ENABLED)
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
#[doc = "Shortcut between UP event and STOP task\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum UP_STOP_A {
    #[doc = "0: Disable shortcut"]
    DISABLED = 0,
    #[doc = "1: Enable shortcut"]
    ENABLED = 1,
}
impl From<UP_STOP_A> for bool {
    #[inline(always)]
    fn from(variant: UP_STOP_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `UP_STOP`"]
pub type UP_STOP_R = crate::R<bool, UP_STOP_A>;
impl UP_STOP_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> UP_STOP_A {
        match self.bits {
            false => UP_STOP_A::DISABLED,
            true => UP_STOP_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == UP_STOP_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == UP_STOP_A::ENABLED
    }
}
#[doc = "Write proxy for field `UP_STOP`"]
pub struct UP_STOP_W<'a> {
    w: &'a mut W,
}
impl<'a> UP_STOP_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: UP_STOP_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Disable shortcut"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(UP_STOP_A::DISABLED)
    }
    #[doc = "Enable shortcut"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(UP_STOP_A::ENABLED)
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
#[doc = "Shortcut between CROSS event and STOP task\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CROSS_STOP_A {
    #[doc = "0: Disable shortcut"]
    DISABLED = 0,
    #[doc = "1: Enable shortcut"]
    ENABLED = 1,
}
impl From<CROSS_STOP_A> for bool {
    #[inline(always)]
    fn from(variant: CROSS_STOP_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `CROSS_STOP`"]
pub type CROSS_STOP_R = crate::R<bool, CROSS_STOP_A>;
impl CROSS_STOP_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CROSS_STOP_A {
        match self.bits {
            false => CROSS_STOP_A::DISABLED,
            true => CROSS_STOP_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == CROSS_STOP_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == CROSS_STOP_A::ENABLED
    }
}
#[doc = "Write proxy for field `CROSS_STOP`"]
pub struct CROSS_STOP_W<'a> {
    w: &'a mut W,
}
impl<'a> CROSS_STOP_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CROSS_STOP_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Disable shortcut"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(CROSS_STOP_A::DISABLED)
    }
    #[doc = "Enable shortcut"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(CROSS_STOP_A::ENABLED)
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
    #[doc = "Bit 0 - Shortcut between READY event and SAMPLE task"]
    #[inline(always)]
    pub fn ready_sample(&self) -> READY_SAMPLE_R {
        READY_SAMPLE_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - Shortcut between READY event and STOP task"]
    #[inline(always)]
    pub fn ready_stop(&self) -> READY_STOP_R {
        READY_STOP_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - Shortcut between DOWN event and STOP task"]
    #[inline(always)]
    pub fn down_stop(&self) -> DOWN_STOP_R {
        DOWN_STOP_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - Shortcut between UP event and STOP task"]
    #[inline(always)]
    pub fn up_stop(&self) -> UP_STOP_R {
        UP_STOP_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - Shortcut between CROSS event and STOP task"]
    #[inline(always)]
    pub fn cross_stop(&self) -> CROSS_STOP_R {
        CROSS_STOP_R::new(((self.bits >> 4) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Shortcut between READY event and SAMPLE task"]
    #[inline(always)]
    pub fn ready_sample(&mut self) -> READY_SAMPLE_W {
        READY_SAMPLE_W { w: self }
    }
    #[doc = "Bit 1 - Shortcut between READY event and STOP task"]
    #[inline(always)]
    pub fn ready_stop(&mut self) -> READY_STOP_W {
        READY_STOP_W { w: self }
    }
    #[doc = "Bit 2 - Shortcut between DOWN event and STOP task"]
    #[inline(always)]
    pub fn down_stop(&mut self) -> DOWN_STOP_W {
        DOWN_STOP_W { w: self }
    }
    #[doc = "Bit 3 - Shortcut between UP event and STOP task"]
    #[inline(always)]
    pub fn up_stop(&mut self) -> UP_STOP_W {
        UP_STOP_W { w: self }
    }
    #[doc = "Bit 4 - Shortcut between CROSS event and STOP task"]
    #[inline(always)]
    pub fn cross_stop(&mut self) -> CROSS_STOP_W {
        CROSS_STOP_W { w: self }
    }
}
