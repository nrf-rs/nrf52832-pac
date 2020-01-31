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
#[doc = "Write '1' to Enable interrupt for RXPTRUPD event\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RXPTRUPD_A {
    #[doc = "0: Read: Disabled"]
    DISABLED = 0,
    #[doc = "1: Read: Enabled"]
    ENABLED = 1,
}
impl From<RXPTRUPD_A> for bool {
    #[inline(always)]
    fn from(variant: RXPTRUPD_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `RXPTRUPD`"]
pub type RXPTRUPD_R = crate::R<bool, RXPTRUPD_A>;
impl RXPTRUPD_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RXPTRUPD_A {
        match self.bits {
            false => RXPTRUPD_A::DISABLED,
            true => RXPTRUPD_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == RXPTRUPD_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == RXPTRUPD_A::ENABLED
    }
}
#[doc = "Write '1' to Enable interrupt for RXPTRUPD event\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RXPTRUPD_AW {
    #[doc = "1: Enable"]
    SET = 1,
}
impl From<RXPTRUPD_AW> for bool {
    #[inline(always)]
    fn from(variant: RXPTRUPD_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Write proxy for field `RXPTRUPD`"]
pub struct RXPTRUPD_W<'a> {
    w: &'a mut W,
}
impl<'a> RXPTRUPD_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: RXPTRUPD_AW) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Enable"]
    #[inline(always)]
    pub fn set(self) -> &'a mut W {
        self.variant(RXPTRUPD_AW::SET)
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
#[doc = "Write '1' to Enable interrupt for STOPPED event\n\nValue on reset: 0"]
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
#[doc = "Write '1' to Enable interrupt for STOPPED event\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum STOPPED_AW {
    #[doc = "1: Enable"]
    SET = 1,
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
    #[doc = "Enable"]
    #[inline(always)]
    pub fn set(self) -> &'a mut W {
        self.variant(STOPPED_AW::SET)
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
#[doc = "Write '1' to Enable interrupt for TXPTRUPD event\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TXPTRUPD_A {
    #[doc = "0: Read: Disabled"]
    DISABLED = 0,
    #[doc = "1: Read: Enabled"]
    ENABLED = 1,
}
impl From<TXPTRUPD_A> for bool {
    #[inline(always)]
    fn from(variant: TXPTRUPD_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `TXPTRUPD`"]
pub type TXPTRUPD_R = crate::R<bool, TXPTRUPD_A>;
impl TXPTRUPD_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TXPTRUPD_A {
        match self.bits {
            false => TXPTRUPD_A::DISABLED,
            true => TXPTRUPD_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == TXPTRUPD_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == TXPTRUPD_A::ENABLED
    }
}
#[doc = "Write '1' to Enable interrupt for TXPTRUPD event\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TXPTRUPD_AW {
    #[doc = "1: Enable"]
    SET = 1,
}
impl From<TXPTRUPD_AW> for bool {
    #[inline(always)]
    fn from(variant: TXPTRUPD_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Write proxy for field `TXPTRUPD`"]
pub struct TXPTRUPD_W<'a> {
    w: &'a mut W,
}
impl<'a> TXPTRUPD_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: TXPTRUPD_AW) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Enable"]
    #[inline(always)]
    pub fn set(self) -> &'a mut W {
        self.variant(TXPTRUPD_AW::SET)
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
impl R {
    #[doc = "Bit 1 - Write '1' to Enable interrupt for RXPTRUPD event"]
    #[inline(always)]
    pub fn rxptrupd(&self) -> RXPTRUPD_R {
        RXPTRUPD_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - Write '1' to Enable interrupt for STOPPED event"]
    #[inline(always)]
    pub fn stopped(&self) -> STOPPED_R {
        STOPPED_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 5 - Write '1' to Enable interrupt for TXPTRUPD event"]
    #[inline(always)]
    pub fn txptrupd(&self) -> TXPTRUPD_R {
        TXPTRUPD_R::new(((self.bits >> 5) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 1 - Write '1' to Enable interrupt for RXPTRUPD event"]
    #[inline(always)]
    pub fn rxptrupd(&mut self) -> RXPTRUPD_W {
        RXPTRUPD_W { w: self }
    }
    #[doc = "Bit 2 - Write '1' to Enable interrupt for STOPPED event"]
    #[inline(always)]
    pub fn stopped(&mut self) -> STOPPED_W {
        STOPPED_W { w: self }
    }
    #[doc = "Bit 5 - Write '1' to Enable interrupt for TXPTRUPD event"]
    #[inline(always)]
    pub fn txptrupd(&mut self) -> TXPTRUPD_W {
        TXPTRUPD_W { w: self }
    }
}
