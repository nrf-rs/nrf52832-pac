#[doc = "Reader of register INTENCLR"]
pub type R = crate::R<u32, super::INTENCLR>;
#[doc = "Writer for register INTENCLR"]
pub type W = crate::W<u32, super::INTENCLR>;
#[doc = "Register INTENCLR `reset()`'s with value 0"]
impl crate::ResetValue for super::INTENCLR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Write '1' to Disable interrupt for SAMPLERDY event\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SAMPLERDY_A {
    #[doc = "0: Read: Disabled"]
    DISABLED = 0,
    #[doc = "1: Read: Enabled"]
    ENABLED = 1,
}
impl From<SAMPLERDY_A> for bool {
    #[inline(always)]
    fn from(variant: SAMPLERDY_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `SAMPLERDY`"]
pub type SAMPLERDY_R = crate::R<bool, SAMPLERDY_A>;
impl SAMPLERDY_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SAMPLERDY_A {
        match self.bits {
            false => SAMPLERDY_A::DISABLED,
            true => SAMPLERDY_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == SAMPLERDY_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == SAMPLERDY_A::ENABLED
    }
}
#[doc = "Write '1' to Disable interrupt for SAMPLERDY event\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SAMPLERDY_AW {
    #[doc = "1: Disable"]
    CLEAR = 1,
}
impl From<SAMPLERDY_AW> for bool {
    #[inline(always)]
    fn from(variant: SAMPLERDY_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Write proxy for field `SAMPLERDY`"]
pub struct SAMPLERDY_W<'a> {
    w: &'a mut W,
}
impl<'a> SAMPLERDY_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SAMPLERDY_AW) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Disable"]
    #[inline(always)]
    pub fn clear(self) -> &'a mut W {
        self.variant(SAMPLERDY_AW::CLEAR)
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
#[doc = "Write '1' to Disable interrupt for REPORTRDY event\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum REPORTRDY_A {
    #[doc = "0: Read: Disabled"]
    DISABLED = 0,
    #[doc = "1: Read: Enabled"]
    ENABLED = 1,
}
impl From<REPORTRDY_A> for bool {
    #[inline(always)]
    fn from(variant: REPORTRDY_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `REPORTRDY`"]
pub type REPORTRDY_R = crate::R<bool, REPORTRDY_A>;
impl REPORTRDY_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> REPORTRDY_A {
        match self.bits {
            false => REPORTRDY_A::DISABLED,
            true => REPORTRDY_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == REPORTRDY_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == REPORTRDY_A::ENABLED
    }
}
#[doc = "Write '1' to Disable interrupt for REPORTRDY event\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum REPORTRDY_AW {
    #[doc = "1: Disable"]
    CLEAR = 1,
}
impl From<REPORTRDY_AW> for bool {
    #[inline(always)]
    fn from(variant: REPORTRDY_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Write proxy for field `REPORTRDY`"]
pub struct REPORTRDY_W<'a> {
    w: &'a mut W,
}
impl<'a> REPORTRDY_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: REPORTRDY_AW) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Disable"]
    #[inline(always)]
    pub fn clear(self) -> &'a mut W {
        self.variant(REPORTRDY_AW::CLEAR)
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
#[doc = "Write '1' to Disable interrupt for ACCOF event\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ACCOF_A {
    #[doc = "0: Read: Disabled"]
    DISABLED = 0,
    #[doc = "1: Read: Enabled"]
    ENABLED = 1,
}
impl From<ACCOF_A> for bool {
    #[inline(always)]
    fn from(variant: ACCOF_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `ACCOF`"]
pub type ACCOF_R = crate::R<bool, ACCOF_A>;
impl ACCOF_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ACCOF_A {
        match self.bits {
            false => ACCOF_A::DISABLED,
            true => ACCOF_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == ACCOF_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == ACCOF_A::ENABLED
    }
}
#[doc = "Write '1' to Disable interrupt for ACCOF event\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ACCOF_AW {
    #[doc = "1: Disable"]
    CLEAR = 1,
}
impl From<ACCOF_AW> for bool {
    #[inline(always)]
    fn from(variant: ACCOF_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Write proxy for field `ACCOF`"]
pub struct ACCOF_W<'a> {
    w: &'a mut W,
}
impl<'a> ACCOF_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: ACCOF_AW) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Disable"]
    #[inline(always)]
    pub fn clear(self) -> &'a mut W {
        self.variant(ACCOF_AW::CLEAR)
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
#[doc = "Write '1' to Disable interrupt for DBLRDY event\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DBLRDY_A {
    #[doc = "0: Read: Disabled"]
    DISABLED = 0,
    #[doc = "1: Read: Enabled"]
    ENABLED = 1,
}
impl From<DBLRDY_A> for bool {
    #[inline(always)]
    fn from(variant: DBLRDY_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `DBLRDY`"]
pub type DBLRDY_R = crate::R<bool, DBLRDY_A>;
impl DBLRDY_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DBLRDY_A {
        match self.bits {
            false => DBLRDY_A::DISABLED,
            true => DBLRDY_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == DBLRDY_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == DBLRDY_A::ENABLED
    }
}
#[doc = "Write '1' to Disable interrupt for DBLRDY event\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DBLRDY_AW {
    #[doc = "1: Disable"]
    CLEAR = 1,
}
impl From<DBLRDY_AW> for bool {
    #[inline(always)]
    fn from(variant: DBLRDY_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Write proxy for field `DBLRDY`"]
pub struct DBLRDY_W<'a> {
    w: &'a mut W,
}
impl<'a> DBLRDY_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: DBLRDY_AW) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Disable"]
    #[inline(always)]
    pub fn clear(self) -> &'a mut W {
        self.variant(DBLRDY_AW::CLEAR)
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
#[doc = "Write '1' to Disable interrupt for STOPPED event\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum STOPPED_A {
    #[doc = "0: Read: Disabled"]
    DISABLED = 0,
    #[doc = "1: Read: Enabled"]
    ENABLED = 1,
}
impl From<STOPPED_A> for bool {
    #[inline(always)]
    fn from(variant: STOPPED_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `STOPPED`"]
pub type STOPPED_R = crate::R<bool, STOPPED_A>;
impl STOPPED_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> STOPPED_A {
        match self.bits {
            false => STOPPED_A::DISABLED,
            true => STOPPED_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == STOPPED_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == STOPPED_A::ENABLED
    }
}
#[doc = "Write '1' to Disable interrupt for STOPPED event\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum STOPPED_AW {
    #[doc = "1: Disable"]
    CLEAR = 1,
}
impl From<STOPPED_AW> for bool {
    #[inline(always)]
    fn from(variant: STOPPED_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Write proxy for field `STOPPED`"]
pub struct STOPPED_W<'a> {
    w: &'a mut W,
}
impl<'a> STOPPED_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: STOPPED_AW) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Disable"]
    #[inline(always)]
    pub fn clear(self) -> &'a mut W {
        self.variant(STOPPED_AW::CLEAR)
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
    #[doc = "Bit 0 - Write '1' to Disable interrupt for SAMPLERDY event"]
    #[inline(always)]
    pub fn samplerdy(&self) -> SAMPLERDY_R {
        SAMPLERDY_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - Write '1' to Disable interrupt for REPORTRDY event"]
    #[inline(always)]
    pub fn reportrdy(&self) -> REPORTRDY_R {
        REPORTRDY_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - Write '1' to Disable interrupt for ACCOF event"]
    #[inline(always)]
    pub fn accof(&self) -> ACCOF_R {
        ACCOF_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - Write '1' to Disable interrupt for DBLRDY event"]
    #[inline(always)]
    pub fn dblrdy(&self) -> DBLRDY_R {
        DBLRDY_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - Write '1' to Disable interrupt for STOPPED event"]
    #[inline(always)]
    pub fn stopped(&self) -> STOPPED_R {
        STOPPED_R::new(((self.bits >> 4) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Write '1' to Disable interrupt for SAMPLERDY event"]
    #[inline(always)]
    pub fn samplerdy(&mut self) -> SAMPLERDY_W {
        SAMPLERDY_W { w: self }
    }
    #[doc = "Bit 1 - Write '1' to Disable interrupt for REPORTRDY event"]
    #[inline(always)]
    pub fn reportrdy(&mut self) -> REPORTRDY_W {
        REPORTRDY_W { w: self }
    }
    #[doc = "Bit 2 - Write '1' to Disable interrupt for ACCOF event"]
    #[inline(always)]
    pub fn accof(&mut self) -> ACCOF_W {
        ACCOF_W { w: self }
    }
    #[doc = "Bit 3 - Write '1' to Disable interrupt for DBLRDY event"]
    #[inline(always)]
    pub fn dblrdy(&mut self) -> DBLRDY_W {
        DBLRDY_W { w: self }
    }
    #[doc = "Bit 4 - Write '1' to Disable interrupt for STOPPED event"]
    #[inline(always)]
    pub fn stopped(&mut self) -> STOPPED_W {
        STOPPED_W { w: self }
    }
}
