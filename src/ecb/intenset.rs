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
#[doc = "Write '1' to Enable interrupt for ENDECB event\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ENDECB_A {
    #[doc = "0: Read: Disabled"]
    DISABLED = 0,
    #[doc = "1: Read: Enabled"]
    ENABLED = 1,
}
impl From<ENDECB_A> for bool {
    #[inline(always)]
    fn from(variant: ENDECB_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `ENDECB`"]
pub type ENDECB_R = crate::R<bool, ENDECB_A>;
impl ENDECB_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ENDECB_A {
        match self.bits {
            false => ENDECB_A::DISABLED,
            true => ENDECB_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == ENDECB_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == ENDECB_A::ENABLED
    }
}
#[doc = "Write '1' to Enable interrupt for ENDECB event\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ENDECB_AW {
    #[doc = "1: Enable"]
    SET = 1,
}
impl From<ENDECB_AW> for bool {
    #[inline(always)]
    fn from(variant: ENDECB_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Write proxy for field `ENDECB`"]
pub struct ENDECB_W<'a> {
    w: &'a mut W,
}
impl<'a> ENDECB_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: ENDECB_AW) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Enable"]
    #[inline(always)]
    pub fn set(self) -> &'a mut W {
        self.variant(ENDECB_AW::SET)
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
#[doc = "Write '1' to Enable interrupt for ERRORECB event\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ERRORECB_A {
    #[doc = "0: Read: Disabled"]
    DISABLED = 0,
    #[doc = "1: Read: Enabled"]
    ENABLED = 1,
}
impl From<ERRORECB_A> for bool {
    #[inline(always)]
    fn from(variant: ERRORECB_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `ERRORECB`"]
pub type ERRORECB_R = crate::R<bool, ERRORECB_A>;
impl ERRORECB_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ERRORECB_A {
        match self.bits {
            false => ERRORECB_A::DISABLED,
            true => ERRORECB_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == ERRORECB_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == ERRORECB_A::ENABLED
    }
}
#[doc = "Write '1' to Enable interrupt for ERRORECB event\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ERRORECB_AW {
    #[doc = "1: Enable"]
    SET = 1,
}
impl From<ERRORECB_AW> for bool {
    #[inline(always)]
    fn from(variant: ERRORECB_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Write proxy for field `ERRORECB`"]
pub struct ERRORECB_W<'a> {
    w: &'a mut W,
}
impl<'a> ERRORECB_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: ERRORECB_AW) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Enable"]
    #[inline(always)]
    pub fn set(self) -> &'a mut W {
        self.variant(ERRORECB_AW::SET)
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
    #[doc = "Bit 0 - Write '1' to Enable interrupt for ENDECB event"]
    #[inline(always)]
    pub fn endecb(&self) -> ENDECB_R {
        ENDECB_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - Write '1' to Enable interrupt for ERRORECB event"]
    #[inline(always)]
    pub fn errorecb(&self) -> ERRORECB_R {
        ERRORECB_R::new(((self.bits >> 1) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Write '1' to Enable interrupt for ENDECB event"]
    #[inline(always)]
    pub fn endecb(&mut self) -> ENDECB_W {
        ENDECB_W { w: self }
    }
    #[doc = "Bit 1 - Write '1' to Enable interrupt for ERRORECB event"]
    #[inline(always)]
    pub fn errorecb(&mut self) -> ERRORECB_W {
        ERRORECB_W { w: self }
    }
}
