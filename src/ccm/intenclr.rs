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
#[doc = "Write '1' to Disable interrupt for ENDKSGEN event\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ENDKSGEN_A {
    #[doc = "0: Read: Disabled"]
    DISABLED = 0,
    #[doc = "1: Read: Enabled"]
    ENABLED = 1,
}
impl From<ENDKSGEN_A> for bool {
    #[inline(always)]
    fn from(variant: ENDKSGEN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `ENDKSGEN`"]
pub type ENDKSGEN_R = crate::R<bool, ENDKSGEN_A>;
impl ENDKSGEN_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ENDKSGEN_A {
        match self.bits {
            false => ENDKSGEN_A::DISABLED,
            true => ENDKSGEN_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == ENDKSGEN_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == ENDKSGEN_A::ENABLED
    }
}
#[doc = "Write '1' to Disable interrupt for ENDKSGEN event\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ENDKSGEN_AW {
    #[doc = "1: Disable"]
    CLEAR = 1,
}
impl From<ENDKSGEN_AW> for bool {
    #[inline(always)]
    fn from(variant: ENDKSGEN_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Write proxy for field `ENDKSGEN`"]
pub struct ENDKSGEN_W<'a> {
    w: &'a mut W,
}
impl<'a> ENDKSGEN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: ENDKSGEN_AW) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Disable"]
    #[inline(always)]
    pub fn clear(self) -> &'a mut W {
        self.variant(ENDKSGEN_AW::CLEAR)
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
#[doc = "Write '1' to Disable interrupt for ENDCRYPT event\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ENDCRYPT_A {
    #[doc = "0: Read: Disabled"]
    DISABLED = 0,
    #[doc = "1: Read: Enabled"]
    ENABLED = 1,
}
impl From<ENDCRYPT_A> for bool {
    #[inline(always)]
    fn from(variant: ENDCRYPT_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `ENDCRYPT`"]
pub type ENDCRYPT_R = crate::R<bool, ENDCRYPT_A>;
impl ENDCRYPT_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ENDCRYPT_A {
        match self.bits {
            false => ENDCRYPT_A::DISABLED,
            true => ENDCRYPT_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == ENDCRYPT_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == ENDCRYPT_A::ENABLED
    }
}
#[doc = "Write '1' to Disable interrupt for ENDCRYPT event\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ENDCRYPT_AW {
    #[doc = "1: Disable"]
    CLEAR = 1,
}
impl From<ENDCRYPT_AW> for bool {
    #[inline(always)]
    fn from(variant: ENDCRYPT_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Write proxy for field `ENDCRYPT`"]
pub struct ENDCRYPT_W<'a> {
    w: &'a mut W,
}
impl<'a> ENDCRYPT_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: ENDCRYPT_AW) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Disable"]
    #[inline(always)]
    pub fn clear(self) -> &'a mut W {
        self.variant(ENDCRYPT_AW::CLEAR)
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
#[doc = "Write '1' to Disable interrupt for ERROR event\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ERROR_A {
    #[doc = "0: Read: Disabled"]
    DISABLED = 0,
    #[doc = "1: Read: Enabled"]
    ENABLED = 1,
}
impl From<ERROR_A> for bool {
    #[inline(always)]
    fn from(variant: ERROR_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `ERROR`"]
pub type ERROR_R = crate::R<bool, ERROR_A>;
impl ERROR_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ERROR_A {
        match self.bits {
            false => ERROR_A::DISABLED,
            true => ERROR_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == ERROR_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == ERROR_A::ENABLED
    }
}
#[doc = "Write '1' to Disable interrupt for ERROR event\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ERROR_AW {
    #[doc = "1: Disable"]
    CLEAR = 1,
}
impl From<ERROR_AW> for bool {
    #[inline(always)]
    fn from(variant: ERROR_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Write proxy for field `ERROR`"]
pub struct ERROR_W<'a> {
    w: &'a mut W,
}
impl<'a> ERROR_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: ERROR_AW) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Disable"]
    #[inline(always)]
    pub fn clear(self) -> &'a mut W {
        self.variant(ERROR_AW::CLEAR)
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
    #[doc = "Bit 0 - Write '1' to Disable interrupt for ENDKSGEN event"]
    #[inline(always)]
    pub fn endksgen(&self) -> ENDKSGEN_R {
        ENDKSGEN_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - Write '1' to Disable interrupt for ENDCRYPT event"]
    #[inline(always)]
    pub fn endcrypt(&self) -> ENDCRYPT_R {
        ENDCRYPT_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - Write '1' to Disable interrupt for ERROR event"]
    #[inline(always)]
    pub fn error(&self) -> ERROR_R {
        ERROR_R::new(((self.bits >> 2) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Write '1' to Disable interrupt for ENDKSGEN event"]
    #[inline(always)]
    pub fn endksgen(&mut self) -> ENDKSGEN_W {
        ENDKSGEN_W { w: self }
    }
    #[doc = "Bit 1 - Write '1' to Disable interrupt for ENDCRYPT event"]
    #[inline(always)]
    pub fn endcrypt(&mut self) -> ENDCRYPT_W {
        ENDCRYPT_W { w: self }
    }
    #[doc = "Bit 2 - Write '1' to Disable interrupt for ERROR event"]
    #[inline(always)]
    pub fn error(&mut self) -> ERROR_W {
        ERROR_W { w: self }
    }
}
