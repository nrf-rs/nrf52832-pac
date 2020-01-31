#[doc = "Reader of register INTENSET"]
pub type R = crate::R<u32, super::INTENSET>;
#[doc = "Writer for register INTENSET"]
pub type W = crate::W<u32, super::INTENSET>;
#[doc = "Register INTENSET `reset()`'s with value 0"]
impl crate::ResetValue for super::INTENSET {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Write '1' to Enable interrupt for END event\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum END_A {
    #[doc = "0: Read: Disabled"]
    DISABLED = 0,
    #[doc = "1: Read: Enabled"]
    ENABLED = 1,
}
impl From<END_A> for bool {
    #[inline(always)]
    fn from(variant: END_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `END`"]
pub type END_R = crate::R<bool, END_A>;
impl END_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> END_A {
        match self.bits {
            false => END_A::DISABLED,
            true => END_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == END_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == END_A::ENABLED
    }
}
#[doc = "Write '1' to Enable interrupt for END event\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum END_AW {
    #[doc = "1: Enable"]
    SET = 1,
}
impl From<END_AW> for bool {
    #[inline(always)]
    fn from(variant: END_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Write proxy for field `END`"]
pub struct END_W<'a> {
    w: &'a mut W,
}
impl<'a> END_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: END_AW) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Enable"]
    #[inline(always)]
    pub fn set(self) -> &'a mut W {
        self.variant(END_AW::SET)
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
#[doc = "Write '1' to Enable interrupt for RESOLVED event\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RESOLVED_A {
    #[doc = "0: Read: Disabled"]
    DISABLED = 0,
    #[doc = "1: Read: Enabled"]
    ENABLED = 1,
}
impl From<RESOLVED_A> for bool {
    #[inline(always)]
    fn from(variant: RESOLVED_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `RESOLVED`"]
pub type RESOLVED_R = crate::R<bool, RESOLVED_A>;
impl RESOLVED_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RESOLVED_A {
        match self.bits {
            false => RESOLVED_A::DISABLED,
            true => RESOLVED_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == RESOLVED_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == RESOLVED_A::ENABLED
    }
}
#[doc = "Write '1' to Enable interrupt for RESOLVED event\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RESOLVED_AW {
    #[doc = "1: Enable"]
    SET = 1,
}
impl From<RESOLVED_AW> for bool {
    #[inline(always)]
    fn from(variant: RESOLVED_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Write proxy for field `RESOLVED`"]
pub struct RESOLVED_W<'a> {
    w: &'a mut W,
}
impl<'a> RESOLVED_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: RESOLVED_AW) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Enable"]
    #[inline(always)]
    pub fn set(self) -> &'a mut W {
        self.variant(RESOLVED_AW::SET)
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
#[doc = "Write '1' to Enable interrupt for NOTRESOLVED event\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum NOTRESOLVED_A {
    #[doc = "0: Read: Disabled"]
    DISABLED = 0,
    #[doc = "1: Read: Enabled"]
    ENABLED = 1,
}
impl From<NOTRESOLVED_A> for bool {
    #[inline(always)]
    fn from(variant: NOTRESOLVED_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `NOTRESOLVED`"]
pub type NOTRESOLVED_R = crate::R<bool, NOTRESOLVED_A>;
impl NOTRESOLVED_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> NOTRESOLVED_A {
        match self.bits {
            false => NOTRESOLVED_A::DISABLED,
            true => NOTRESOLVED_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == NOTRESOLVED_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == NOTRESOLVED_A::ENABLED
    }
}
#[doc = "Write '1' to Enable interrupt for NOTRESOLVED event\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum NOTRESOLVED_AW {
    #[doc = "1: Enable"]
    SET = 1,
}
impl From<NOTRESOLVED_AW> for bool {
    #[inline(always)]
    fn from(variant: NOTRESOLVED_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Write proxy for field `NOTRESOLVED`"]
pub struct NOTRESOLVED_W<'a> {
    w: &'a mut W,
}
impl<'a> NOTRESOLVED_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: NOTRESOLVED_AW) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Enable"]
    #[inline(always)]
    pub fn set(self) -> &'a mut W {
        self.variant(NOTRESOLVED_AW::SET)
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
    #[doc = "Bit 0 - Write '1' to Enable interrupt for END event"]
    #[inline(always)]
    pub fn end(&self) -> END_R {
        END_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - Write '1' to Enable interrupt for RESOLVED event"]
    #[inline(always)]
    pub fn resolved(&self) -> RESOLVED_R {
        RESOLVED_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - Write '1' to Enable interrupt for NOTRESOLVED event"]
    #[inline(always)]
    pub fn notresolved(&self) -> NOTRESOLVED_R {
        NOTRESOLVED_R::new(((self.bits >> 2) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Write '1' to Enable interrupt for END event"]
    #[inline(always)]
    pub fn end(&mut self) -> END_W {
        END_W { w: self }
    }
    #[doc = "Bit 1 - Write '1' to Enable interrupt for RESOLVED event"]
    #[inline(always)]
    pub fn resolved(&mut self) -> RESOLVED_W {
        RESOLVED_W { w: self }
    }
    #[doc = "Bit 2 - Write '1' to Enable interrupt for NOTRESOLVED event"]
    #[inline(always)]
    pub fn notresolved(&mut self) -> NOTRESOLVED_W {
        NOTRESOLVED_W { w: self }
    }
}
