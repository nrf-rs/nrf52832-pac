#[doc = "Reader of register CONFIG2"]
pub type R = crate::R<u32, super::CONFIG2>;
#[doc = "Writer for register CONFIG2"]
pub type W = crate::W<u32, super::CONFIG2>;
#[doc = "Register CONFIG2 `reset()`'s with value 0"]
impl crate::ResetValue for super::CONFIG2 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Enable protection for region 64. Write '0' has no effect.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum REGION64_A {
    #[doc = "0: Protection disabled"]
    DISABLED = 0,
    #[doc = "1: Protection enabled"]
    ENABLED = 1,
}
impl From<REGION64_A> for bool {
    #[inline(always)]
    fn from(variant: REGION64_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `REGION64`"]
pub type REGION64_R = crate::R<bool, REGION64_A>;
impl REGION64_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> REGION64_A {
        match self.bits {
            false => REGION64_A::DISABLED,
            true => REGION64_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == REGION64_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == REGION64_A::ENABLED
    }
}
#[doc = "Write proxy for field `REGION64`"]
pub struct REGION64_W<'a> {
    w: &'a mut W,
}
impl<'a> REGION64_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: REGION64_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Protection disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(REGION64_A::DISABLED)
    }
    #[doc = "Protection enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(REGION64_A::ENABLED)
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
#[doc = "Enable protection for region 65. Write '0' has no effect.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum REGION65_A {
    #[doc = "0: Protection disabled"]
    DISABLED = 0,
    #[doc = "1: Protection enabled"]
    ENABLED = 1,
}
impl From<REGION65_A> for bool {
    #[inline(always)]
    fn from(variant: REGION65_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `REGION65`"]
pub type REGION65_R = crate::R<bool, REGION65_A>;
impl REGION65_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> REGION65_A {
        match self.bits {
            false => REGION65_A::DISABLED,
            true => REGION65_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == REGION65_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == REGION65_A::ENABLED
    }
}
#[doc = "Write proxy for field `REGION65`"]
pub struct REGION65_W<'a> {
    w: &'a mut W,
}
impl<'a> REGION65_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: REGION65_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Protection disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(REGION65_A::DISABLED)
    }
    #[doc = "Protection enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(REGION65_A::ENABLED)
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
#[doc = "Enable protection for region 66. Write '0' has no effect.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum REGION66_A {
    #[doc = "0: Protection disabled"]
    DISABLED = 0,
    #[doc = "1: Protection enabled"]
    ENABLED = 1,
}
impl From<REGION66_A> for bool {
    #[inline(always)]
    fn from(variant: REGION66_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `REGION66`"]
pub type REGION66_R = crate::R<bool, REGION66_A>;
impl REGION66_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> REGION66_A {
        match self.bits {
            false => REGION66_A::DISABLED,
            true => REGION66_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == REGION66_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == REGION66_A::ENABLED
    }
}
#[doc = "Write proxy for field `REGION66`"]
pub struct REGION66_W<'a> {
    w: &'a mut W,
}
impl<'a> REGION66_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: REGION66_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Protection disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(REGION66_A::DISABLED)
    }
    #[doc = "Protection enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(REGION66_A::ENABLED)
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
#[doc = "Enable protection for region 67. Write '0' has no effect.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum REGION67_A {
    #[doc = "0: Protection disabled"]
    DISABLED = 0,
    #[doc = "1: Protection enabled"]
    ENABLED = 1,
}
impl From<REGION67_A> for bool {
    #[inline(always)]
    fn from(variant: REGION67_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `REGION67`"]
pub type REGION67_R = crate::R<bool, REGION67_A>;
impl REGION67_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> REGION67_A {
        match self.bits {
            false => REGION67_A::DISABLED,
            true => REGION67_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == REGION67_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == REGION67_A::ENABLED
    }
}
#[doc = "Write proxy for field `REGION67`"]
pub struct REGION67_W<'a> {
    w: &'a mut W,
}
impl<'a> REGION67_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: REGION67_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Protection disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(REGION67_A::DISABLED)
    }
    #[doc = "Protection enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(REGION67_A::ENABLED)
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
#[doc = "Enable protection for region 68. Write '0' has no effect.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum REGION68_A {
    #[doc = "0: Protection disabled"]
    DISABLED = 0,
    #[doc = "1: Protection enabled"]
    ENABLED = 1,
}
impl From<REGION68_A> for bool {
    #[inline(always)]
    fn from(variant: REGION68_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `REGION68`"]
pub type REGION68_R = crate::R<bool, REGION68_A>;
impl REGION68_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> REGION68_A {
        match self.bits {
            false => REGION68_A::DISABLED,
            true => REGION68_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == REGION68_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == REGION68_A::ENABLED
    }
}
#[doc = "Write proxy for field `REGION68`"]
pub struct REGION68_W<'a> {
    w: &'a mut W,
}
impl<'a> REGION68_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: REGION68_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Protection disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(REGION68_A::DISABLED)
    }
    #[doc = "Protection enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(REGION68_A::ENABLED)
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
#[doc = "Enable protection for region 69. Write '0' has no effect.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum REGION69_A {
    #[doc = "0: Protection disabled"]
    DISABLED = 0,
    #[doc = "1: Protection enabled"]
    ENABLED = 1,
}
impl From<REGION69_A> for bool {
    #[inline(always)]
    fn from(variant: REGION69_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `REGION69`"]
pub type REGION69_R = crate::R<bool, REGION69_A>;
impl REGION69_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> REGION69_A {
        match self.bits {
            false => REGION69_A::DISABLED,
            true => REGION69_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == REGION69_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == REGION69_A::ENABLED
    }
}
#[doc = "Write proxy for field `REGION69`"]
pub struct REGION69_W<'a> {
    w: &'a mut W,
}
impl<'a> REGION69_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: REGION69_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Protection disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(REGION69_A::DISABLED)
    }
    #[doc = "Protection enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(REGION69_A::ENABLED)
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
#[doc = "Enable protection for region 70. Write '0' has no effect.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum REGION70_A {
    #[doc = "0: Protection disabled"]
    DISABLED = 0,
    #[doc = "1: Protection enabled"]
    ENABLED = 1,
}
impl From<REGION70_A> for bool {
    #[inline(always)]
    fn from(variant: REGION70_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `REGION70`"]
pub type REGION70_R = crate::R<bool, REGION70_A>;
impl REGION70_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> REGION70_A {
        match self.bits {
            false => REGION70_A::DISABLED,
            true => REGION70_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == REGION70_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == REGION70_A::ENABLED
    }
}
#[doc = "Write proxy for field `REGION70`"]
pub struct REGION70_W<'a> {
    w: &'a mut W,
}
impl<'a> REGION70_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: REGION70_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Protection disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(REGION70_A::DISABLED)
    }
    #[doc = "Protection enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(REGION70_A::ENABLED)
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
        self.w.bits = (self.w.bits & !(0x01 << 6)) | (((value as u32) & 0x01) << 6);
        self.w
    }
}
#[doc = "Enable protection for region 71. Write '0' has no effect.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum REGION71_A {
    #[doc = "0: Protection disabled"]
    DISABLED = 0,
    #[doc = "1: Protection enabled"]
    ENABLED = 1,
}
impl From<REGION71_A> for bool {
    #[inline(always)]
    fn from(variant: REGION71_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `REGION71`"]
pub type REGION71_R = crate::R<bool, REGION71_A>;
impl REGION71_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> REGION71_A {
        match self.bits {
            false => REGION71_A::DISABLED,
            true => REGION71_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == REGION71_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == REGION71_A::ENABLED
    }
}
#[doc = "Write proxy for field `REGION71`"]
pub struct REGION71_W<'a> {
    w: &'a mut W,
}
impl<'a> REGION71_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: REGION71_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Protection disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(REGION71_A::DISABLED)
    }
    #[doc = "Protection enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(REGION71_A::ENABLED)
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
        self.w.bits = (self.w.bits & !(0x01 << 7)) | (((value as u32) & 0x01) << 7);
        self.w
    }
}
#[doc = "Enable protection for region 72. Write '0' has no effect.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum REGION72_A {
    #[doc = "0: Protection disabled"]
    DISABLED = 0,
    #[doc = "1: Protection enabled"]
    ENABLED = 1,
}
impl From<REGION72_A> for bool {
    #[inline(always)]
    fn from(variant: REGION72_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `REGION72`"]
pub type REGION72_R = crate::R<bool, REGION72_A>;
impl REGION72_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> REGION72_A {
        match self.bits {
            false => REGION72_A::DISABLED,
            true => REGION72_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == REGION72_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == REGION72_A::ENABLED
    }
}
#[doc = "Write proxy for field `REGION72`"]
pub struct REGION72_W<'a> {
    w: &'a mut W,
}
impl<'a> REGION72_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: REGION72_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Protection disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(REGION72_A::DISABLED)
    }
    #[doc = "Protection enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(REGION72_A::ENABLED)
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
        self.w.bits = (self.w.bits & !(0x01 << 8)) | (((value as u32) & 0x01) << 8);
        self.w
    }
}
#[doc = "Enable protection for region 73. Write '0' has no effect.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum REGION73_A {
    #[doc = "0: Protection disabled"]
    DISABLED = 0,
    #[doc = "1: Protection enabled"]
    ENABLED = 1,
}
impl From<REGION73_A> for bool {
    #[inline(always)]
    fn from(variant: REGION73_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `REGION73`"]
pub type REGION73_R = crate::R<bool, REGION73_A>;
impl REGION73_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> REGION73_A {
        match self.bits {
            false => REGION73_A::DISABLED,
            true => REGION73_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == REGION73_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == REGION73_A::ENABLED
    }
}
#[doc = "Write proxy for field `REGION73`"]
pub struct REGION73_W<'a> {
    w: &'a mut W,
}
impl<'a> REGION73_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: REGION73_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Protection disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(REGION73_A::DISABLED)
    }
    #[doc = "Protection enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(REGION73_A::ENABLED)
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
        self.w.bits = (self.w.bits & !(0x01 << 9)) | (((value as u32) & 0x01) << 9);
        self.w
    }
}
#[doc = "Enable protection for region 74. Write '0' has no effect.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum REGION74_A {
    #[doc = "0: Protection disabled"]
    DISABLED = 0,
    #[doc = "1: Protection enabled"]
    ENABLED = 1,
}
impl From<REGION74_A> for bool {
    #[inline(always)]
    fn from(variant: REGION74_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `REGION74`"]
pub type REGION74_R = crate::R<bool, REGION74_A>;
impl REGION74_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> REGION74_A {
        match self.bits {
            false => REGION74_A::DISABLED,
            true => REGION74_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == REGION74_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == REGION74_A::ENABLED
    }
}
#[doc = "Write proxy for field `REGION74`"]
pub struct REGION74_W<'a> {
    w: &'a mut W,
}
impl<'a> REGION74_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: REGION74_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Protection disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(REGION74_A::DISABLED)
    }
    #[doc = "Protection enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(REGION74_A::ENABLED)
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
        self.w.bits = (self.w.bits & !(0x01 << 10)) | (((value as u32) & 0x01) << 10);
        self.w
    }
}
#[doc = "Enable protection for region 75. Write '0' has no effect.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum REGION75_A {
    #[doc = "0: Protection disabled"]
    DISABLED = 0,
    #[doc = "1: Protection enabled"]
    ENABLED = 1,
}
impl From<REGION75_A> for bool {
    #[inline(always)]
    fn from(variant: REGION75_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `REGION75`"]
pub type REGION75_R = crate::R<bool, REGION75_A>;
impl REGION75_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> REGION75_A {
        match self.bits {
            false => REGION75_A::DISABLED,
            true => REGION75_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == REGION75_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == REGION75_A::ENABLED
    }
}
#[doc = "Write proxy for field `REGION75`"]
pub struct REGION75_W<'a> {
    w: &'a mut W,
}
impl<'a> REGION75_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: REGION75_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Protection disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(REGION75_A::DISABLED)
    }
    #[doc = "Protection enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(REGION75_A::ENABLED)
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
        self.w.bits = (self.w.bits & !(0x01 << 11)) | (((value as u32) & 0x01) << 11);
        self.w
    }
}
#[doc = "Enable protection for region 76. Write '0' has no effect.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum REGION76_A {
    #[doc = "0: Protection disabled"]
    DISABLED = 0,
    #[doc = "1: Protection enabled"]
    ENABLED = 1,
}
impl From<REGION76_A> for bool {
    #[inline(always)]
    fn from(variant: REGION76_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `REGION76`"]
pub type REGION76_R = crate::R<bool, REGION76_A>;
impl REGION76_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> REGION76_A {
        match self.bits {
            false => REGION76_A::DISABLED,
            true => REGION76_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == REGION76_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == REGION76_A::ENABLED
    }
}
#[doc = "Write proxy for field `REGION76`"]
pub struct REGION76_W<'a> {
    w: &'a mut W,
}
impl<'a> REGION76_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: REGION76_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Protection disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(REGION76_A::DISABLED)
    }
    #[doc = "Protection enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(REGION76_A::ENABLED)
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
        self.w.bits = (self.w.bits & !(0x01 << 12)) | (((value as u32) & 0x01) << 12);
        self.w
    }
}
#[doc = "Enable protection for region 77. Write '0' has no effect.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum REGION77_A {
    #[doc = "0: Protection disabled"]
    DISABLED = 0,
    #[doc = "1: Protection enabled"]
    ENABLED = 1,
}
impl From<REGION77_A> for bool {
    #[inline(always)]
    fn from(variant: REGION77_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `REGION77`"]
pub type REGION77_R = crate::R<bool, REGION77_A>;
impl REGION77_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> REGION77_A {
        match self.bits {
            false => REGION77_A::DISABLED,
            true => REGION77_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == REGION77_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == REGION77_A::ENABLED
    }
}
#[doc = "Write proxy for field `REGION77`"]
pub struct REGION77_W<'a> {
    w: &'a mut W,
}
impl<'a> REGION77_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: REGION77_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Protection disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(REGION77_A::DISABLED)
    }
    #[doc = "Protection enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(REGION77_A::ENABLED)
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
#[doc = "Enable protection for region 78. Write '0' has no effect.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum REGION78_A {
    #[doc = "0: Protection disabled"]
    DISABLED = 0,
    #[doc = "1: Protection enabled"]
    ENABLED = 1,
}
impl From<REGION78_A> for bool {
    #[inline(always)]
    fn from(variant: REGION78_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `REGION78`"]
pub type REGION78_R = crate::R<bool, REGION78_A>;
impl REGION78_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> REGION78_A {
        match self.bits {
            false => REGION78_A::DISABLED,
            true => REGION78_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == REGION78_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == REGION78_A::ENABLED
    }
}
#[doc = "Write proxy for field `REGION78`"]
pub struct REGION78_W<'a> {
    w: &'a mut W,
}
impl<'a> REGION78_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: REGION78_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Protection disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(REGION78_A::DISABLED)
    }
    #[doc = "Protection enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(REGION78_A::ENABLED)
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
#[doc = "Enable protection for region 79. Write '0' has no effect.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum REGION79_A {
    #[doc = "0: Protection disabled"]
    DISABLED = 0,
    #[doc = "1: Protection enabled"]
    ENABLED = 1,
}
impl From<REGION79_A> for bool {
    #[inline(always)]
    fn from(variant: REGION79_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `REGION79`"]
pub type REGION79_R = crate::R<bool, REGION79_A>;
impl REGION79_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> REGION79_A {
        match self.bits {
            false => REGION79_A::DISABLED,
            true => REGION79_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == REGION79_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == REGION79_A::ENABLED
    }
}
#[doc = "Write proxy for field `REGION79`"]
pub struct REGION79_W<'a> {
    w: &'a mut W,
}
impl<'a> REGION79_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: REGION79_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Protection disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(REGION79_A::DISABLED)
    }
    #[doc = "Protection enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(REGION79_A::ENABLED)
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
        self.w.bits = (self.w.bits & !(0x01 << 15)) | (((value as u32) & 0x01) << 15);
        self.w
    }
}
#[doc = "Enable protection for region 80. Write '0' has no effect.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum REGION80_A {
    #[doc = "0: Protection disabled"]
    DISABLED = 0,
    #[doc = "1: Protection enabled"]
    ENABLED = 1,
}
impl From<REGION80_A> for bool {
    #[inline(always)]
    fn from(variant: REGION80_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `REGION80`"]
pub type REGION80_R = crate::R<bool, REGION80_A>;
impl REGION80_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> REGION80_A {
        match self.bits {
            false => REGION80_A::DISABLED,
            true => REGION80_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == REGION80_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == REGION80_A::ENABLED
    }
}
#[doc = "Write proxy for field `REGION80`"]
pub struct REGION80_W<'a> {
    w: &'a mut W,
}
impl<'a> REGION80_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: REGION80_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Protection disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(REGION80_A::DISABLED)
    }
    #[doc = "Protection enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(REGION80_A::ENABLED)
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
        self.w.bits = (self.w.bits & !(0x01 << 16)) | (((value as u32) & 0x01) << 16);
        self.w
    }
}
#[doc = "Enable protection for region 81. Write '0' has no effect.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum REGION81_A {
    #[doc = "0: Protection disabled"]
    DISABLED = 0,
    #[doc = "1: Protection enabled"]
    ENABLED = 1,
}
impl From<REGION81_A> for bool {
    #[inline(always)]
    fn from(variant: REGION81_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `REGION81`"]
pub type REGION81_R = crate::R<bool, REGION81_A>;
impl REGION81_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> REGION81_A {
        match self.bits {
            false => REGION81_A::DISABLED,
            true => REGION81_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == REGION81_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == REGION81_A::ENABLED
    }
}
#[doc = "Write proxy for field `REGION81`"]
pub struct REGION81_W<'a> {
    w: &'a mut W,
}
impl<'a> REGION81_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: REGION81_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Protection disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(REGION81_A::DISABLED)
    }
    #[doc = "Protection enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(REGION81_A::ENABLED)
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
        self.w.bits = (self.w.bits & !(0x01 << 17)) | (((value as u32) & 0x01) << 17);
        self.w
    }
}
#[doc = "Enable protection for region 82. Write '0' has no effect.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum REGION82_A {
    #[doc = "0: Protection disabled"]
    DISABLED = 0,
    #[doc = "1: Protection enabled"]
    ENABLED = 1,
}
impl From<REGION82_A> for bool {
    #[inline(always)]
    fn from(variant: REGION82_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `REGION82`"]
pub type REGION82_R = crate::R<bool, REGION82_A>;
impl REGION82_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> REGION82_A {
        match self.bits {
            false => REGION82_A::DISABLED,
            true => REGION82_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == REGION82_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == REGION82_A::ENABLED
    }
}
#[doc = "Write proxy for field `REGION82`"]
pub struct REGION82_W<'a> {
    w: &'a mut W,
}
impl<'a> REGION82_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: REGION82_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Protection disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(REGION82_A::DISABLED)
    }
    #[doc = "Protection enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(REGION82_A::ENABLED)
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
        self.w.bits = (self.w.bits & !(0x01 << 18)) | (((value as u32) & 0x01) << 18);
        self.w
    }
}
#[doc = "Enable protection for region 83. Write '0' has no effect.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum REGION83_A {
    #[doc = "0: Protection disabled"]
    DISABLED = 0,
    #[doc = "1: Protection enabled"]
    ENABLED = 1,
}
impl From<REGION83_A> for bool {
    #[inline(always)]
    fn from(variant: REGION83_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `REGION83`"]
pub type REGION83_R = crate::R<bool, REGION83_A>;
impl REGION83_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> REGION83_A {
        match self.bits {
            false => REGION83_A::DISABLED,
            true => REGION83_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == REGION83_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == REGION83_A::ENABLED
    }
}
#[doc = "Write proxy for field `REGION83`"]
pub struct REGION83_W<'a> {
    w: &'a mut W,
}
impl<'a> REGION83_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: REGION83_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Protection disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(REGION83_A::DISABLED)
    }
    #[doc = "Protection enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(REGION83_A::ENABLED)
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
        self.w.bits = (self.w.bits & !(0x01 << 19)) | (((value as u32) & 0x01) << 19);
        self.w
    }
}
#[doc = "Enable protection for region 84. Write '0' has no effect.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum REGION84_A {
    #[doc = "0: Protection disabled"]
    DISABLED = 0,
    #[doc = "1: Protection enabled"]
    ENABLED = 1,
}
impl From<REGION84_A> for bool {
    #[inline(always)]
    fn from(variant: REGION84_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `REGION84`"]
pub type REGION84_R = crate::R<bool, REGION84_A>;
impl REGION84_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> REGION84_A {
        match self.bits {
            false => REGION84_A::DISABLED,
            true => REGION84_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == REGION84_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == REGION84_A::ENABLED
    }
}
#[doc = "Write proxy for field `REGION84`"]
pub struct REGION84_W<'a> {
    w: &'a mut W,
}
impl<'a> REGION84_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: REGION84_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Protection disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(REGION84_A::DISABLED)
    }
    #[doc = "Protection enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(REGION84_A::ENABLED)
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
        self.w.bits = (self.w.bits & !(0x01 << 20)) | (((value as u32) & 0x01) << 20);
        self.w
    }
}
#[doc = "Enable protection for region 85. Write '0' has no effect.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum REGION85_A {
    #[doc = "0: Protection disabled"]
    DISABLED = 0,
    #[doc = "1: Protection enabled"]
    ENABLED = 1,
}
impl From<REGION85_A> for bool {
    #[inline(always)]
    fn from(variant: REGION85_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `REGION85`"]
pub type REGION85_R = crate::R<bool, REGION85_A>;
impl REGION85_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> REGION85_A {
        match self.bits {
            false => REGION85_A::DISABLED,
            true => REGION85_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == REGION85_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == REGION85_A::ENABLED
    }
}
#[doc = "Write proxy for field `REGION85`"]
pub struct REGION85_W<'a> {
    w: &'a mut W,
}
impl<'a> REGION85_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: REGION85_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Protection disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(REGION85_A::DISABLED)
    }
    #[doc = "Protection enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(REGION85_A::ENABLED)
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
        self.w.bits = (self.w.bits & !(0x01 << 21)) | (((value as u32) & 0x01) << 21);
        self.w
    }
}
#[doc = "Enable protection for region 86. Write '0' has no effect.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum REGION86_A {
    #[doc = "0: Protection disabled"]
    DISABLED = 0,
    #[doc = "1: Protection enabled"]
    ENABLED = 1,
}
impl From<REGION86_A> for bool {
    #[inline(always)]
    fn from(variant: REGION86_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `REGION86`"]
pub type REGION86_R = crate::R<bool, REGION86_A>;
impl REGION86_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> REGION86_A {
        match self.bits {
            false => REGION86_A::DISABLED,
            true => REGION86_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == REGION86_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == REGION86_A::ENABLED
    }
}
#[doc = "Write proxy for field `REGION86`"]
pub struct REGION86_W<'a> {
    w: &'a mut W,
}
impl<'a> REGION86_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: REGION86_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Protection disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(REGION86_A::DISABLED)
    }
    #[doc = "Protection enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(REGION86_A::ENABLED)
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
        self.w.bits = (self.w.bits & !(0x01 << 22)) | (((value as u32) & 0x01) << 22);
        self.w
    }
}
#[doc = "Enable protection for region 87. Write '0' has no effect.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum REGION87_A {
    #[doc = "0: Protection disabled"]
    DISABLED = 0,
    #[doc = "1: Protection enabled"]
    ENABLED = 1,
}
impl From<REGION87_A> for bool {
    #[inline(always)]
    fn from(variant: REGION87_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `REGION87`"]
pub type REGION87_R = crate::R<bool, REGION87_A>;
impl REGION87_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> REGION87_A {
        match self.bits {
            false => REGION87_A::DISABLED,
            true => REGION87_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == REGION87_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == REGION87_A::ENABLED
    }
}
#[doc = "Write proxy for field `REGION87`"]
pub struct REGION87_W<'a> {
    w: &'a mut W,
}
impl<'a> REGION87_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: REGION87_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Protection disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(REGION87_A::DISABLED)
    }
    #[doc = "Protection enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(REGION87_A::ENABLED)
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
        self.w.bits = (self.w.bits & !(0x01 << 23)) | (((value as u32) & 0x01) << 23);
        self.w
    }
}
#[doc = "Enable protection for region 88. Write '0' has no effect.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum REGION88_A {
    #[doc = "0: Protection disabled"]
    DISABLED = 0,
    #[doc = "1: Protection enabled"]
    ENABLED = 1,
}
impl From<REGION88_A> for bool {
    #[inline(always)]
    fn from(variant: REGION88_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `REGION88`"]
pub type REGION88_R = crate::R<bool, REGION88_A>;
impl REGION88_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> REGION88_A {
        match self.bits {
            false => REGION88_A::DISABLED,
            true => REGION88_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == REGION88_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == REGION88_A::ENABLED
    }
}
#[doc = "Write proxy for field `REGION88`"]
pub struct REGION88_W<'a> {
    w: &'a mut W,
}
impl<'a> REGION88_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: REGION88_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Protection disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(REGION88_A::DISABLED)
    }
    #[doc = "Protection enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(REGION88_A::ENABLED)
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
        self.w.bits = (self.w.bits & !(0x01 << 24)) | (((value as u32) & 0x01) << 24);
        self.w
    }
}
#[doc = "Enable protection for region 89. Write '0' has no effect.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum REGION89_A {
    #[doc = "0: Protection disabled"]
    DISABLED = 0,
    #[doc = "1: Protection enabled"]
    ENABLED = 1,
}
impl From<REGION89_A> for bool {
    #[inline(always)]
    fn from(variant: REGION89_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `REGION89`"]
pub type REGION89_R = crate::R<bool, REGION89_A>;
impl REGION89_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> REGION89_A {
        match self.bits {
            false => REGION89_A::DISABLED,
            true => REGION89_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == REGION89_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == REGION89_A::ENABLED
    }
}
#[doc = "Write proxy for field `REGION89`"]
pub struct REGION89_W<'a> {
    w: &'a mut W,
}
impl<'a> REGION89_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: REGION89_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Protection disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(REGION89_A::DISABLED)
    }
    #[doc = "Protection enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(REGION89_A::ENABLED)
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
        self.w.bits = (self.w.bits & !(0x01 << 25)) | (((value as u32) & 0x01) << 25);
        self.w
    }
}
#[doc = "Enable protection for region 90. Write '0' has no effect.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum REGION90_A {
    #[doc = "0: Protection disabled"]
    DISABLED = 0,
    #[doc = "1: Protection enabled"]
    ENABLED = 1,
}
impl From<REGION90_A> for bool {
    #[inline(always)]
    fn from(variant: REGION90_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `REGION90`"]
pub type REGION90_R = crate::R<bool, REGION90_A>;
impl REGION90_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> REGION90_A {
        match self.bits {
            false => REGION90_A::DISABLED,
            true => REGION90_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == REGION90_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == REGION90_A::ENABLED
    }
}
#[doc = "Write proxy for field `REGION90`"]
pub struct REGION90_W<'a> {
    w: &'a mut W,
}
impl<'a> REGION90_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: REGION90_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Protection disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(REGION90_A::DISABLED)
    }
    #[doc = "Protection enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(REGION90_A::ENABLED)
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
        self.w.bits = (self.w.bits & !(0x01 << 26)) | (((value as u32) & 0x01) << 26);
        self.w
    }
}
#[doc = "Enable protection for region 91. Write '0' has no effect.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum REGION91_A {
    #[doc = "0: Protection disabled"]
    DISABLED = 0,
    #[doc = "1: Protection enabled"]
    ENABLED = 1,
}
impl From<REGION91_A> for bool {
    #[inline(always)]
    fn from(variant: REGION91_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `REGION91`"]
pub type REGION91_R = crate::R<bool, REGION91_A>;
impl REGION91_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> REGION91_A {
        match self.bits {
            false => REGION91_A::DISABLED,
            true => REGION91_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == REGION91_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == REGION91_A::ENABLED
    }
}
#[doc = "Write proxy for field `REGION91`"]
pub struct REGION91_W<'a> {
    w: &'a mut W,
}
impl<'a> REGION91_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: REGION91_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Protection disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(REGION91_A::DISABLED)
    }
    #[doc = "Protection enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(REGION91_A::ENABLED)
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
        self.w.bits = (self.w.bits & !(0x01 << 27)) | (((value as u32) & 0x01) << 27);
        self.w
    }
}
#[doc = "Enable protection for region 92. Write '0' has no effect.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum REGION92_A {
    #[doc = "0: Protection disabled"]
    DISABLED = 0,
    #[doc = "1: Protection enabled"]
    ENABLED = 1,
}
impl From<REGION92_A> for bool {
    #[inline(always)]
    fn from(variant: REGION92_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `REGION92`"]
pub type REGION92_R = crate::R<bool, REGION92_A>;
impl REGION92_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> REGION92_A {
        match self.bits {
            false => REGION92_A::DISABLED,
            true => REGION92_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == REGION92_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == REGION92_A::ENABLED
    }
}
#[doc = "Write proxy for field `REGION92`"]
pub struct REGION92_W<'a> {
    w: &'a mut W,
}
impl<'a> REGION92_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: REGION92_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Protection disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(REGION92_A::DISABLED)
    }
    #[doc = "Protection enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(REGION92_A::ENABLED)
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
        self.w.bits = (self.w.bits & !(0x01 << 28)) | (((value as u32) & 0x01) << 28);
        self.w
    }
}
#[doc = "Enable protection for region 93. Write '0' has no effect.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum REGION93_A {
    #[doc = "0: Protection disabled"]
    DISABLED = 0,
    #[doc = "1: Protection enabled"]
    ENABLED = 1,
}
impl From<REGION93_A> for bool {
    #[inline(always)]
    fn from(variant: REGION93_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `REGION93`"]
pub type REGION93_R = crate::R<bool, REGION93_A>;
impl REGION93_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> REGION93_A {
        match self.bits {
            false => REGION93_A::DISABLED,
            true => REGION93_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == REGION93_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == REGION93_A::ENABLED
    }
}
#[doc = "Write proxy for field `REGION93`"]
pub struct REGION93_W<'a> {
    w: &'a mut W,
}
impl<'a> REGION93_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: REGION93_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Protection disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(REGION93_A::DISABLED)
    }
    #[doc = "Protection enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(REGION93_A::ENABLED)
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
        self.w.bits = (self.w.bits & !(0x01 << 29)) | (((value as u32) & 0x01) << 29);
        self.w
    }
}
#[doc = "Enable protection for region 94. Write '0' has no effect.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum REGION94_A {
    #[doc = "0: Protection disabled"]
    DISABLED = 0,
    #[doc = "1: Protection enabled"]
    ENABLED = 1,
}
impl From<REGION94_A> for bool {
    #[inline(always)]
    fn from(variant: REGION94_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `REGION94`"]
pub type REGION94_R = crate::R<bool, REGION94_A>;
impl REGION94_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> REGION94_A {
        match self.bits {
            false => REGION94_A::DISABLED,
            true => REGION94_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == REGION94_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == REGION94_A::ENABLED
    }
}
#[doc = "Write proxy for field `REGION94`"]
pub struct REGION94_W<'a> {
    w: &'a mut W,
}
impl<'a> REGION94_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: REGION94_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Protection disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(REGION94_A::DISABLED)
    }
    #[doc = "Protection enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(REGION94_A::ENABLED)
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
        self.w.bits = (self.w.bits & !(0x01 << 30)) | (((value as u32) & 0x01) << 30);
        self.w
    }
}
#[doc = "Enable protection for region 95. Write '0' has no effect.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum REGION95_A {
    #[doc = "0: Protection disabled"]
    DISABLED = 0,
    #[doc = "1: Protection enabled"]
    ENABLED = 1,
}
impl From<REGION95_A> for bool {
    #[inline(always)]
    fn from(variant: REGION95_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `REGION95`"]
pub type REGION95_R = crate::R<bool, REGION95_A>;
impl REGION95_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> REGION95_A {
        match self.bits {
            false => REGION95_A::DISABLED,
            true => REGION95_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == REGION95_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == REGION95_A::ENABLED
    }
}
#[doc = "Write proxy for field `REGION95`"]
pub struct REGION95_W<'a> {
    w: &'a mut W,
}
impl<'a> REGION95_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: REGION95_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Protection disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(REGION95_A::DISABLED)
    }
    #[doc = "Protection enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(REGION95_A::ENABLED)
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
        self.w.bits = (self.w.bits & !(0x01 << 31)) | (((value as u32) & 0x01) << 31);
        self.w
    }
}
impl R {
    #[doc = "Bit 0 - Enable protection for region 64. Write '0' has no effect."]
    #[inline(always)]
    pub fn region64(&self) -> REGION64_R {
        REGION64_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - Enable protection for region 65. Write '0' has no effect."]
    #[inline(always)]
    pub fn region65(&self) -> REGION65_R {
        REGION65_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - Enable protection for region 66. Write '0' has no effect."]
    #[inline(always)]
    pub fn region66(&self) -> REGION66_R {
        REGION66_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - Enable protection for region 67. Write '0' has no effect."]
    #[inline(always)]
    pub fn region67(&self) -> REGION67_R {
        REGION67_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - Enable protection for region 68. Write '0' has no effect."]
    #[inline(always)]
    pub fn region68(&self) -> REGION68_R {
        REGION68_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - Enable protection for region 69. Write '0' has no effect."]
    #[inline(always)]
    pub fn region69(&self) -> REGION69_R {
        REGION69_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - Enable protection for region 70. Write '0' has no effect."]
    #[inline(always)]
    pub fn region70(&self) -> REGION70_R {
        REGION70_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7 - Enable protection for region 71. Write '0' has no effect."]
    #[inline(always)]
    pub fn region71(&self) -> REGION71_R {
        REGION71_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 8 - Enable protection for region 72. Write '0' has no effect."]
    #[inline(always)]
    pub fn region72(&self) -> REGION72_R {
        REGION72_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 9 - Enable protection for region 73. Write '0' has no effect."]
    #[inline(always)]
    pub fn region73(&self) -> REGION73_R {
        REGION73_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 10 - Enable protection for region 74. Write '0' has no effect."]
    #[inline(always)]
    pub fn region74(&self) -> REGION74_R {
        REGION74_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    #[doc = "Bit 11 - Enable protection for region 75. Write '0' has no effect."]
    #[inline(always)]
    pub fn region75(&self) -> REGION75_R {
        REGION75_R::new(((self.bits >> 11) & 0x01) != 0)
    }
    #[doc = "Bit 12 - Enable protection for region 76. Write '0' has no effect."]
    #[inline(always)]
    pub fn region76(&self) -> REGION76_R {
        REGION76_R::new(((self.bits >> 12) & 0x01) != 0)
    }
    #[doc = "Bit 13 - Enable protection for region 77. Write '0' has no effect."]
    #[inline(always)]
    pub fn region77(&self) -> REGION77_R {
        REGION77_R::new(((self.bits >> 13) & 0x01) != 0)
    }
    #[doc = "Bit 14 - Enable protection for region 78. Write '0' has no effect."]
    #[inline(always)]
    pub fn region78(&self) -> REGION78_R {
        REGION78_R::new(((self.bits >> 14) & 0x01) != 0)
    }
    #[doc = "Bit 15 - Enable protection for region 79. Write '0' has no effect."]
    #[inline(always)]
    pub fn region79(&self) -> REGION79_R {
        REGION79_R::new(((self.bits >> 15) & 0x01) != 0)
    }
    #[doc = "Bit 16 - Enable protection for region 80. Write '0' has no effect."]
    #[inline(always)]
    pub fn region80(&self) -> REGION80_R {
        REGION80_R::new(((self.bits >> 16) & 0x01) != 0)
    }
    #[doc = "Bit 17 - Enable protection for region 81. Write '0' has no effect."]
    #[inline(always)]
    pub fn region81(&self) -> REGION81_R {
        REGION81_R::new(((self.bits >> 17) & 0x01) != 0)
    }
    #[doc = "Bit 18 - Enable protection for region 82. Write '0' has no effect."]
    #[inline(always)]
    pub fn region82(&self) -> REGION82_R {
        REGION82_R::new(((self.bits >> 18) & 0x01) != 0)
    }
    #[doc = "Bit 19 - Enable protection for region 83. Write '0' has no effect."]
    #[inline(always)]
    pub fn region83(&self) -> REGION83_R {
        REGION83_R::new(((self.bits >> 19) & 0x01) != 0)
    }
    #[doc = "Bit 20 - Enable protection for region 84. Write '0' has no effect."]
    #[inline(always)]
    pub fn region84(&self) -> REGION84_R {
        REGION84_R::new(((self.bits >> 20) & 0x01) != 0)
    }
    #[doc = "Bit 21 - Enable protection for region 85. Write '0' has no effect."]
    #[inline(always)]
    pub fn region85(&self) -> REGION85_R {
        REGION85_R::new(((self.bits >> 21) & 0x01) != 0)
    }
    #[doc = "Bit 22 - Enable protection for region 86. Write '0' has no effect."]
    #[inline(always)]
    pub fn region86(&self) -> REGION86_R {
        REGION86_R::new(((self.bits >> 22) & 0x01) != 0)
    }
    #[doc = "Bit 23 - Enable protection for region 87. Write '0' has no effect."]
    #[inline(always)]
    pub fn region87(&self) -> REGION87_R {
        REGION87_R::new(((self.bits >> 23) & 0x01) != 0)
    }
    #[doc = "Bit 24 - Enable protection for region 88. Write '0' has no effect."]
    #[inline(always)]
    pub fn region88(&self) -> REGION88_R {
        REGION88_R::new(((self.bits >> 24) & 0x01) != 0)
    }
    #[doc = "Bit 25 - Enable protection for region 89. Write '0' has no effect."]
    #[inline(always)]
    pub fn region89(&self) -> REGION89_R {
        REGION89_R::new(((self.bits >> 25) & 0x01) != 0)
    }
    #[doc = "Bit 26 - Enable protection for region 90. Write '0' has no effect."]
    #[inline(always)]
    pub fn region90(&self) -> REGION90_R {
        REGION90_R::new(((self.bits >> 26) & 0x01) != 0)
    }
    #[doc = "Bit 27 - Enable protection for region 91. Write '0' has no effect."]
    #[inline(always)]
    pub fn region91(&self) -> REGION91_R {
        REGION91_R::new(((self.bits >> 27) & 0x01) != 0)
    }
    #[doc = "Bit 28 - Enable protection for region 92. Write '0' has no effect."]
    #[inline(always)]
    pub fn region92(&self) -> REGION92_R {
        REGION92_R::new(((self.bits >> 28) & 0x01) != 0)
    }
    #[doc = "Bit 29 - Enable protection for region 93. Write '0' has no effect."]
    #[inline(always)]
    pub fn region93(&self) -> REGION93_R {
        REGION93_R::new(((self.bits >> 29) & 0x01) != 0)
    }
    #[doc = "Bit 30 - Enable protection for region 94. Write '0' has no effect."]
    #[inline(always)]
    pub fn region94(&self) -> REGION94_R {
        REGION94_R::new(((self.bits >> 30) & 0x01) != 0)
    }
    #[doc = "Bit 31 - Enable protection for region 95. Write '0' has no effect."]
    #[inline(always)]
    pub fn region95(&self) -> REGION95_R {
        REGION95_R::new(((self.bits >> 31) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Enable protection for region 64. Write '0' has no effect."]
    #[inline(always)]
    pub fn region64(&mut self) -> REGION64_W {
        REGION64_W { w: self }
    }
    #[doc = "Bit 1 - Enable protection for region 65. Write '0' has no effect."]
    #[inline(always)]
    pub fn region65(&mut self) -> REGION65_W {
        REGION65_W { w: self }
    }
    #[doc = "Bit 2 - Enable protection for region 66. Write '0' has no effect."]
    #[inline(always)]
    pub fn region66(&mut self) -> REGION66_W {
        REGION66_W { w: self }
    }
    #[doc = "Bit 3 - Enable protection for region 67. Write '0' has no effect."]
    #[inline(always)]
    pub fn region67(&mut self) -> REGION67_W {
        REGION67_W { w: self }
    }
    #[doc = "Bit 4 - Enable protection for region 68. Write '0' has no effect."]
    #[inline(always)]
    pub fn region68(&mut self) -> REGION68_W {
        REGION68_W { w: self }
    }
    #[doc = "Bit 5 - Enable protection for region 69. Write '0' has no effect."]
    #[inline(always)]
    pub fn region69(&mut self) -> REGION69_W {
        REGION69_W { w: self }
    }
    #[doc = "Bit 6 - Enable protection for region 70. Write '0' has no effect."]
    #[inline(always)]
    pub fn region70(&mut self) -> REGION70_W {
        REGION70_W { w: self }
    }
    #[doc = "Bit 7 - Enable protection for region 71. Write '0' has no effect."]
    #[inline(always)]
    pub fn region71(&mut self) -> REGION71_W {
        REGION71_W { w: self }
    }
    #[doc = "Bit 8 - Enable protection for region 72. Write '0' has no effect."]
    #[inline(always)]
    pub fn region72(&mut self) -> REGION72_W {
        REGION72_W { w: self }
    }
    #[doc = "Bit 9 - Enable protection for region 73. Write '0' has no effect."]
    #[inline(always)]
    pub fn region73(&mut self) -> REGION73_W {
        REGION73_W { w: self }
    }
    #[doc = "Bit 10 - Enable protection for region 74. Write '0' has no effect."]
    #[inline(always)]
    pub fn region74(&mut self) -> REGION74_W {
        REGION74_W { w: self }
    }
    #[doc = "Bit 11 - Enable protection for region 75. Write '0' has no effect."]
    #[inline(always)]
    pub fn region75(&mut self) -> REGION75_W {
        REGION75_W { w: self }
    }
    #[doc = "Bit 12 - Enable protection for region 76. Write '0' has no effect."]
    #[inline(always)]
    pub fn region76(&mut self) -> REGION76_W {
        REGION76_W { w: self }
    }
    #[doc = "Bit 13 - Enable protection for region 77. Write '0' has no effect."]
    #[inline(always)]
    pub fn region77(&mut self) -> REGION77_W {
        REGION77_W { w: self }
    }
    #[doc = "Bit 14 - Enable protection for region 78. Write '0' has no effect."]
    #[inline(always)]
    pub fn region78(&mut self) -> REGION78_W {
        REGION78_W { w: self }
    }
    #[doc = "Bit 15 - Enable protection for region 79. Write '0' has no effect."]
    #[inline(always)]
    pub fn region79(&mut self) -> REGION79_W {
        REGION79_W { w: self }
    }
    #[doc = "Bit 16 - Enable protection for region 80. Write '0' has no effect."]
    #[inline(always)]
    pub fn region80(&mut self) -> REGION80_W {
        REGION80_W { w: self }
    }
    #[doc = "Bit 17 - Enable protection for region 81. Write '0' has no effect."]
    #[inline(always)]
    pub fn region81(&mut self) -> REGION81_W {
        REGION81_W { w: self }
    }
    #[doc = "Bit 18 - Enable protection for region 82. Write '0' has no effect."]
    #[inline(always)]
    pub fn region82(&mut self) -> REGION82_W {
        REGION82_W { w: self }
    }
    #[doc = "Bit 19 - Enable protection for region 83. Write '0' has no effect."]
    #[inline(always)]
    pub fn region83(&mut self) -> REGION83_W {
        REGION83_W { w: self }
    }
    #[doc = "Bit 20 - Enable protection for region 84. Write '0' has no effect."]
    #[inline(always)]
    pub fn region84(&mut self) -> REGION84_W {
        REGION84_W { w: self }
    }
    #[doc = "Bit 21 - Enable protection for region 85. Write '0' has no effect."]
    #[inline(always)]
    pub fn region85(&mut self) -> REGION85_W {
        REGION85_W { w: self }
    }
    #[doc = "Bit 22 - Enable protection for region 86. Write '0' has no effect."]
    #[inline(always)]
    pub fn region86(&mut self) -> REGION86_W {
        REGION86_W { w: self }
    }
    #[doc = "Bit 23 - Enable protection for region 87. Write '0' has no effect."]
    #[inline(always)]
    pub fn region87(&mut self) -> REGION87_W {
        REGION87_W { w: self }
    }
    #[doc = "Bit 24 - Enable protection for region 88. Write '0' has no effect."]
    #[inline(always)]
    pub fn region88(&mut self) -> REGION88_W {
        REGION88_W { w: self }
    }
    #[doc = "Bit 25 - Enable protection for region 89. Write '0' has no effect."]
    #[inline(always)]
    pub fn region89(&mut self) -> REGION89_W {
        REGION89_W { w: self }
    }
    #[doc = "Bit 26 - Enable protection for region 90. Write '0' has no effect."]
    #[inline(always)]
    pub fn region90(&mut self) -> REGION90_W {
        REGION90_W { w: self }
    }
    #[doc = "Bit 27 - Enable protection for region 91. Write '0' has no effect."]
    #[inline(always)]
    pub fn region91(&mut self) -> REGION91_W {
        REGION91_W { w: self }
    }
    #[doc = "Bit 28 - Enable protection for region 92. Write '0' has no effect."]
    #[inline(always)]
    pub fn region92(&mut self) -> REGION92_W {
        REGION92_W { w: self }
    }
    #[doc = "Bit 29 - Enable protection for region 93. Write '0' has no effect."]
    #[inline(always)]
    pub fn region93(&mut self) -> REGION93_W {
        REGION93_W { w: self }
    }
    #[doc = "Bit 30 - Enable protection for region 94. Write '0' has no effect."]
    #[inline(always)]
    pub fn region94(&mut self) -> REGION94_W {
        REGION94_W { w: self }
    }
    #[doc = "Bit 31 - Enable protection for region 95. Write '0' has no effect."]
    #[inline(always)]
    pub fn region95(&mut self) -> REGION95_W {
        REGION95_W { w: self }
    }
}
