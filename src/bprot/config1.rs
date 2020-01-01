#[doc = "Reader of register CONFIG1"]
pub type R = crate::R<u32, super::CONFIG1>;
#[doc = "Writer for register CONFIG1"]
pub type W = crate::W<u32, super::CONFIG1>;
#[doc = "Register CONFIG1 `reset()`'s with value 0"]
impl crate::ResetValue for super::CONFIG1 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Enable protection for region 32. Write '0' has no effect.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum REGION32_A {
    #[doc = "0: Protection disabled"]
    DISABLED = 0,
    #[doc = "1: Protection enabled"]
    ENABLED = 1,
}
impl From<REGION32_A> for bool {
    #[inline(always)]
    fn from(variant: REGION32_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `REGION32`"]
pub type REGION32_R = crate::R<bool, REGION32_A>;
impl REGION32_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> REGION32_A {
        match self.bits {
            false => REGION32_A::DISABLED,
            true => REGION32_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == REGION32_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == REGION32_A::ENABLED
    }
}
#[doc = "Write proxy for field `REGION32`"]
pub struct REGION32_W<'a> {
    w: &'a mut W,
}
impl<'a> REGION32_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: REGION32_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Protection disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(REGION32_A::DISABLED)
    }
    #[doc = "Protection enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(REGION32_A::ENABLED)
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
#[doc = "Enable protection for region 33. Write '0' has no effect.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum REGION33_A {
    #[doc = "0: Protection disabled"]
    DISABLED = 0,
    #[doc = "1: Protection enabled"]
    ENABLED = 1,
}
impl From<REGION33_A> for bool {
    #[inline(always)]
    fn from(variant: REGION33_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `REGION33`"]
pub type REGION33_R = crate::R<bool, REGION33_A>;
impl REGION33_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> REGION33_A {
        match self.bits {
            false => REGION33_A::DISABLED,
            true => REGION33_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == REGION33_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == REGION33_A::ENABLED
    }
}
#[doc = "Write proxy for field `REGION33`"]
pub struct REGION33_W<'a> {
    w: &'a mut W,
}
impl<'a> REGION33_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: REGION33_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Protection disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(REGION33_A::DISABLED)
    }
    #[doc = "Protection enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(REGION33_A::ENABLED)
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
#[doc = "Enable protection for region 34. Write '0' has no effect.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum REGION34_A {
    #[doc = "0: Protection disabled"]
    DISABLED = 0,
    #[doc = "1: Protection enabled"]
    ENABLED = 1,
}
impl From<REGION34_A> for bool {
    #[inline(always)]
    fn from(variant: REGION34_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `REGION34`"]
pub type REGION34_R = crate::R<bool, REGION34_A>;
impl REGION34_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> REGION34_A {
        match self.bits {
            false => REGION34_A::DISABLED,
            true => REGION34_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == REGION34_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == REGION34_A::ENABLED
    }
}
#[doc = "Write proxy for field `REGION34`"]
pub struct REGION34_W<'a> {
    w: &'a mut W,
}
impl<'a> REGION34_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: REGION34_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Protection disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(REGION34_A::DISABLED)
    }
    #[doc = "Protection enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(REGION34_A::ENABLED)
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
#[doc = "Enable protection for region 35. Write '0' has no effect.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum REGION35_A {
    #[doc = "0: Protection disabled"]
    DISABLED = 0,
    #[doc = "1: Protection enabled"]
    ENABLED = 1,
}
impl From<REGION35_A> for bool {
    #[inline(always)]
    fn from(variant: REGION35_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `REGION35`"]
pub type REGION35_R = crate::R<bool, REGION35_A>;
impl REGION35_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> REGION35_A {
        match self.bits {
            false => REGION35_A::DISABLED,
            true => REGION35_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == REGION35_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == REGION35_A::ENABLED
    }
}
#[doc = "Write proxy for field `REGION35`"]
pub struct REGION35_W<'a> {
    w: &'a mut W,
}
impl<'a> REGION35_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: REGION35_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Protection disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(REGION35_A::DISABLED)
    }
    #[doc = "Protection enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(REGION35_A::ENABLED)
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
#[doc = "Enable protection for region 36. Write '0' has no effect.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum REGION36_A {
    #[doc = "0: Protection disabled"]
    DISABLED = 0,
    #[doc = "1: Protection enabled"]
    ENABLED = 1,
}
impl From<REGION36_A> for bool {
    #[inline(always)]
    fn from(variant: REGION36_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `REGION36`"]
pub type REGION36_R = crate::R<bool, REGION36_A>;
impl REGION36_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> REGION36_A {
        match self.bits {
            false => REGION36_A::DISABLED,
            true => REGION36_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == REGION36_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == REGION36_A::ENABLED
    }
}
#[doc = "Write proxy for field `REGION36`"]
pub struct REGION36_W<'a> {
    w: &'a mut W,
}
impl<'a> REGION36_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: REGION36_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Protection disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(REGION36_A::DISABLED)
    }
    #[doc = "Protection enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(REGION36_A::ENABLED)
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
#[doc = "Enable protection for region 37. Write '0' has no effect.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum REGION37_A {
    #[doc = "0: Protection disabled"]
    DISABLED = 0,
    #[doc = "1: Protection enabled"]
    ENABLED = 1,
}
impl From<REGION37_A> for bool {
    #[inline(always)]
    fn from(variant: REGION37_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `REGION37`"]
pub type REGION37_R = crate::R<bool, REGION37_A>;
impl REGION37_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> REGION37_A {
        match self.bits {
            false => REGION37_A::DISABLED,
            true => REGION37_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == REGION37_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == REGION37_A::ENABLED
    }
}
#[doc = "Write proxy for field `REGION37`"]
pub struct REGION37_W<'a> {
    w: &'a mut W,
}
impl<'a> REGION37_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: REGION37_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Protection disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(REGION37_A::DISABLED)
    }
    #[doc = "Protection enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(REGION37_A::ENABLED)
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
#[doc = "Enable protection for region 38. Write '0' has no effect.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum REGION38_A {
    #[doc = "0: Protection disabled"]
    DISABLED = 0,
    #[doc = "1: Protection enabled"]
    ENABLED = 1,
}
impl From<REGION38_A> for bool {
    #[inline(always)]
    fn from(variant: REGION38_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `REGION38`"]
pub type REGION38_R = crate::R<bool, REGION38_A>;
impl REGION38_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> REGION38_A {
        match self.bits {
            false => REGION38_A::DISABLED,
            true => REGION38_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == REGION38_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == REGION38_A::ENABLED
    }
}
#[doc = "Write proxy for field `REGION38`"]
pub struct REGION38_W<'a> {
    w: &'a mut W,
}
impl<'a> REGION38_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: REGION38_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Protection disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(REGION38_A::DISABLED)
    }
    #[doc = "Protection enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(REGION38_A::ENABLED)
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
#[doc = "Enable protection for region 39. Write '0' has no effect.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum REGION39_A {
    #[doc = "0: Protection disabled"]
    DISABLED = 0,
    #[doc = "1: Protection enabled"]
    ENABLED = 1,
}
impl From<REGION39_A> for bool {
    #[inline(always)]
    fn from(variant: REGION39_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `REGION39`"]
pub type REGION39_R = crate::R<bool, REGION39_A>;
impl REGION39_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> REGION39_A {
        match self.bits {
            false => REGION39_A::DISABLED,
            true => REGION39_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == REGION39_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == REGION39_A::ENABLED
    }
}
#[doc = "Write proxy for field `REGION39`"]
pub struct REGION39_W<'a> {
    w: &'a mut W,
}
impl<'a> REGION39_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: REGION39_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Protection disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(REGION39_A::DISABLED)
    }
    #[doc = "Protection enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(REGION39_A::ENABLED)
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
#[doc = "Enable protection for region 40. Write '0' has no effect.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum REGION40_A {
    #[doc = "0: Protection disabled"]
    DISABLED = 0,
    #[doc = "1: Protection enabled"]
    ENABLED = 1,
}
impl From<REGION40_A> for bool {
    #[inline(always)]
    fn from(variant: REGION40_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `REGION40`"]
pub type REGION40_R = crate::R<bool, REGION40_A>;
impl REGION40_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> REGION40_A {
        match self.bits {
            false => REGION40_A::DISABLED,
            true => REGION40_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == REGION40_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == REGION40_A::ENABLED
    }
}
#[doc = "Write proxy for field `REGION40`"]
pub struct REGION40_W<'a> {
    w: &'a mut W,
}
impl<'a> REGION40_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: REGION40_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Protection disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(REGION40_A::DISABLED)
    }
    #[doc = "Protection enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(REGION40_A::ENABLED)
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
#[doc = "Enable protection for region 41. Write '0' has no effect.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum REGION41_A {
    #[doc = "0: Protection disabled"]
    DISABLED = 0,
    #[doc = "1: Protection enabled"]
    ENABLED = 1,
}
impl From<REGION41_A> for bool {
    #[inline(always)]
    fn from(variant: REGION41_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `REGION41`"]
pub type REGION41_R = crate::R<bool, REGION41_A>;
impl REGION41_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> REGION41_A {
        match self.bits {
            false => REGION41_A::DISABLED,
            true => REGION41_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == REGION41_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == REGION41_A::ENABLED
    }
}
#[doc = "Write proxy for field `REGION41`"]
pub struct REGION41_W<'a> {
    w: &'a mut W,
}
impl<'a> REGION41_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: REGION41_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Protection disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(REGION41_A::DISABLED)
    }
    #[doc = "Protection enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(REGION41_A::ENABLED)
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
#[doc = "Enable protection for region 42. Write '0' has no effect.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum REGION42_A {
    #[doc = "0: Protection disabled"]
    DISABLED = 0,
    #[doc = "1: Protection enabled"]
    ENABLED = 1,
}
impl From<REGION42_A> for bool {
    #[inline(always)]
    fn from(variant: REGION42_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `REGION42`"]
pub type REGION42_R = crate::R<bool, REGION42_A>;
impl REGION42_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> REGION42_A {
        match self.bits {
            false => REGION42_A::DISABLED,
            true => REGION42_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == REGION42_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == REGION42_A::ENABLED
    }
}
#[doc = "Write proxy for field `REGION42`"]
pub struct REGION42_W<'a> {
    w: &'a mut W,
}
impl<'a> REGION42_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: REGION42_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Protection disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(REGION42_A::DISABLED)
    }
    #[doc = "Protection enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(REGION42_A::ENABLED)
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
#[doc = "Enable protection for region 43. Write '0' has no effect.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum REGION43_A {
    #[doc = "0: Protection disabled"]
    DISABLED = 0,
    #[doc = "1: Protection enabled"]
    ENABLED = 1,
}
impl From<REGION43_A> for bool {
    #[inline(always)]
    fn from(variant: REGION43_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `REGION43`"]
pub type REGION43_R = crate::R<bool, REGION43_A>;
impl REGION43_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> REGION43_A {
        match self.bits {
            false => REGION43_A::DISABLED,
            true => REGION43_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == REGION43_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == REGION43_A::ENABLED
    }
}
#[doc = "Write proxy for field `REGION43`"]
pub struct REGION43_W<'a> {
    w: &'a mut W,
}
impl<'a> REGION43_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: REGION43_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Protection disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(REGION43_A::DISABLED)
    }
    #[doc = "Protection enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(REGION43_A::ENABLED)
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
#[doc = "Enable protection for region 44. Write '0' has no effect.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum REGION44_A {
    #[doc = "0: Protection disabled"]
    DISABLED = 0,
    #[doc = "1: Protection enabled"]
    ENABLED = 1,
}
impl From<REGION44_A> for bool {
    #[inline(always)]
    fn from(variant: REGION44_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `REGION44`"]
pub type REGION44_R = crate::R<bool, REGION44_A>;
impl REGION44_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> REGION44_A {
        match self.bits {
            false => REGION44_A::DISABLED,
            true => REGION44_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == REGION44_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == REGION44_A::ENABLED
    }
}
#[doc = "Write proxy for field `REGION44`"]
pub struct REGION44_W<'a> {
    w: &'a mut W,
}
impl<'a> REGION44_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: REGION44_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Protection disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(REGION44_A::DISABLED)
    }
    #[doc = "Protection enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(REGION44_A::ENABLED)
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
#[doc = "Enable protection for region 45. Write '0' has no effect.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum REGION45_A {
    #[doc = "0: Protection disabled"]
    DISABLED = 0,
    #[doc = "1: Protection enabled"]
    ENABLED = 1,
}
impl From<REGION45_A> for bool {
    #[inline(always)]
    fn from(variant: REGION45_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `REGION45`"]
pub type REGION45_R = crate::R<bool, REGION45_A>;
impl REGION45_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> REGION45_A {
        match self.bits {
            false => REGION45_A::DISABLED,
            true => REGION45_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == REGION45_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == REGION45_A::ENABLED
    }
}
#[doc = "Write proxy for field `REGION45`"]
pub struct REGION45_W<'a> {
    w: &'a mut W,
}
impl<'a> REGION45_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: REGION45_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Protection disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(REGION45_A::DISABLED)
    }
    #[doc = "Protection enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(REGION45_A::ENABLED)
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
#[doc = "Enable protection for region 46. Write '0' has no effect.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum REGION46_A {
    #[doc = "0: Protection disabled"]
    DISABLED = 0,
    #[doc = "1: Protection enabled"]
    ENABLED = 1,
}
impl From<REGION46_A> for bool {
    #[inline(always)]
    fn from(variant: REGION46_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `REGION46`"]
pub type REGION46_R = crate::R<bool, REGION46_A>;
impl REGION46_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> REGION46_A {
        match self.bits {
            false => REGION46_A::DISABLED,
            true => REGION46_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == REGION46_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == REGION46_A::ENABLED
    }
}
#[doc = "Write proxy for field `REGION46`"]
pub struct REGION46_W<'a> {
    w: &'a mut W,
}
impl<'a> REGION46_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: REGION46_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Protection disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(REGION46_A::DISABLED)
    }
    #[doc = "Protection enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(REGION46_A::ENABLED)
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
#[doc = "Enable protection for region 47. Write '0' has no effect.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum REGION47_A {
    #[doc = "0: Protection disabled"]
    DISABLED = 0,
    #[doc = "1: Protection enabled"]
    ENABLED = 1,
}
impl From<REGION47_A> for bool {
    #[inline(always)]
    fn from(variant: REGION47_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `REGION47`"]
pub type REGION47_R = crate::R<bool, REGION47_A>;
impl REGION47_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> REGION47_A {
        match self.bits {
            false => REGION47_A::DISABLED,
            true => REGION47_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == REGION47_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == REGION47_A::ENABLED
    }
}
#[doc = "Write proxy for field `REGION47`"]
pub struct REGION47_W<'a> {
    w: &'a mut W,
}
impl<'a> REGION47_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: REGION47_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Protection disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(REGION47_A::DISABLED)
    }
    #[doc = "Protection enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(REGION47_A::ENABLED)
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
#[doc = "Enable protection for region 48. Write '0' has no effect.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum REGION48_A {
    #[doc = "0: Protection disabled"]
    DISABLED = 0,
    #[doc = "1: Protection enabled"]
    ENABLED = 1,
}
impl From<REGION48_A> for bool {
    #[inline(always)]
    fn from(variant: REGION48_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `REGION48`"]
pub type REGION48_R = crate::R<bool, REGION48_A>;
impl REGION48_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> REGION48_A {
        match self.bits {
            false => REGION48_A::DISABLED,
            true => REGION48_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == REGION48_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == REGION48_A::ENABLED
    }
}
#[doc = "Write proxy for field `REGION48`"]
pub struct REGION48_W<'a> {
    w: &'a mut W,
}
impl<'a> REGION48_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: REGION48_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Protection disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(REGION48_A::DISABLED)
    }
    #[doc = "Protection enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(REGION48_A::ENABLED)
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
#[doc = "Enable protection for region 49. Write '0' has no effect.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum REGION49_A {
    #[doc = "0: Protection disabled"]
    DISABLED = 0,
    #[doc = "1: Protection enabled"]
    ENABLED = 1,
}
impl From<REGION49_A> for bool {
    #[inline(always)]
    fn from(variant: REGION49_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `REGION49`"]
pub type REGION49_R = crate::R<bool, REGION49_A>;
impl REGION49_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> REGION49_A {
        match self.bits {
            false => REGION49_A::DISABLED,
            true => REGION49_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == REGION49_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == REGION49_A::ENABLED
    }
}
#[doc = "Write proxy for field `REGION49`"]
pub struct REGION49_W<'a> {
    w: &'a mut W,
}
impl<'a> REGION49_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: REGION49_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Protection disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(REGION49_A::DISABLED)
    }
    #[doc = "Protection enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(REGION49_A::ENABLED)
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
#[doc = "Enable protection for region 50. Write '0' has no effect.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum REGION50_A {
    #[doc = "0: Protection disabled"]
    DISABLED = 0,
    #[doc = "1: Protection enabled"]
    ENABLED = 1,
}
impl From<REGION50_A> for bool {
    #[inline(always)]
    fn from(variant: REGION50_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `REGION50`"]
pub type REGION50_R = crate::R<bool, REGION50_A>;
impl REGION50_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> REGION50_A {
        match self.bits {
            false => REGION50_A::DISABLED,
            true => REGION50_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == REGION50_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == REGION50_A::ENABLED
    }
}
#[doc = "Write proxy for field `REGION50`"]
pub struct REGION50_W<'a> {
    w: &'a mut W,
}
impl<'a> REGION50_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: REGION50_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Protection disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(REGION50_A::DISABLED)
    }
    #[doc = "Protection enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(REGION50_A::ENABLED)
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
#[doc = "Enable protection for region 51. Write '0' has no effect.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum REGION51_A {
    #[doc = "0: Protection disabled"]
    DISABLED = 0,
    #[doc = "1: Protection enabled"]
    ENABLED = 1,
}
impl From<REGION51_A> for bool {
    #[inline(always)]
    fn from(variant: REGION51_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `REGION51`"]
pub type REGION51_R = crate::R<bool, REGION51_A>;
impl REGION51_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> REGION51_A {
        match self.bits {
            false => REGION51_A::DISABLED,
            true => REGION51_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == REGION51_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == REGION51_A::ENABLED
    }
}
#[doc = "Write proxy for field `REGION51`"]
pub struct REGION51_W<'a> {
    w: &'a mut W,
}
impl<'a> REGION51_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: REGION51_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Protection disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(REGION51_A::DISABLED)
    }
    #[doc = "Protection enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(REGION51_A::ENABLED)
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
#[doc = "Enable protection for region 52. Write '0' has no effect.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum REGION52_A {
    #[doc = "0: Protection disabled"]
    DISABLED = 0,
    #[doc = "1: Protection enabled"]
    ENABLED = 1,
}
impl From<REGION52_A> for bool {
    #[inline(always)]
    fn from(variant: REGION52_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `REGION52`"]
pub type REGION52_R = crate::R<bool, REGION52_A>;
impl REGION52_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> REGION52_A {
        match self.bits {
            false => REGION52_A::DISABLED,
            true => REGION52_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == REGION52_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == REGION52_A::ENABLED
    }
}
#[doc = "Write proxy for field `REGION52`"]
pub struct REGION52_W<'a> {
    w: &'a mut W,
}
impl<'a> REGION52_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: REGION52_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Protection disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(REGION52_A::DISABLED)
    }
    #[doc = "Protection enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(REGION52_A::ENABLED)
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
#[doc = "Enable protection for region 53. Write '0' has no effect.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum REGION53_A {
    #[doc = "0: Protection disabled"]
    DISABLED = 0,
    #[doc = "1: Protection enabled"]
    ENABLED = 1,
}
impl From<REGION53_A> for bool {
    #[inline(always)]
    fn from(variant: REGION53_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `REGION53`"]
pub type REGION53_R = crate::R<bool, REGION53_A>;
impl REGION53_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> REGION53_A {
        match self.bits {
            false => REGION53_A::DISABLED,
            true => REGION53_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == REGION53_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == REGION53_A::ENABLED
    }
}
#[doc = "Write proxy for field `REGION53`"]
pub struct REGION53_W<'a> {
    w: &'a mut W,
}
impl<'a> REGION53_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: REGION53_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Protection disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(REGION53_A::DISABLED)
    }
    #[doc = "Protection enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(REGION53_A::ENABLED)
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
#[doc = "Enable protection for region 54. Write '0' has no effect.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum REGION54_A {
    #[doc = "0: Protection disabled"]
    DISABLED = 0,
    #[doc = "1: Protection enabled"]
    ENABLED = 1,
}
impl From<REGION54_A> for bool {
    #[inline(always)]
    fn from(variant: REGION54_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `REGION54`"]
pub type REGION54_R = crate::R<bool, REGION54_A>;
impl REGION54_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> REGION54_A {
        match self.bits {
            false => REGION54_A::DISABLED,
            true => REGION54_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == REGION54_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == REGION54_A::ENABLED
    }
}
#[doc = "Write proxy for field `REGION54`"]
pub struct REGION54_W<'a> {
    w: &'a mut W,
}
impl<'a> REGION54_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: REGION54_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Protection disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(REGION54_A::DISABLED)
    }
    #[doc = "Protection enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(REGION54_A::ENABLED)
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
#[doc = "Enable protection for region 55. Write '0' has no effect.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum REGION55_A {
    #[doc = "0: Protection disabled"]
    DISABLED = 0,
    #[doc = "1: Protection enabled"]
    ENABLED = 1,
}
impl From<REGION55_A> for bool {
    #[inline(always)]
    fn from(variant: REGION55_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `REGION55`"]
pub type REGION55_R = crate::R<bool, REGION55_A>;
impl REGION55_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> REGION55_A {
        match self.bits {
            false => REGION55_A::DISABLED,
            true => REGION55_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == REGION55_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == REGION55_A::ENABLED
    }
}
#[doc = "Write proxy for field `REGION55`"]
pub struct REGION55_W<'a> {
    w: &'a mut W,
}
impl<'a> REGION55_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: REGION55_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Protection disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(REGION55_A::DISABLED)
    }
    #[doc = "Protection enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(REGION55_A::ENABLED)
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
#[doc = "Enable protection for region 56. Write '0' has no effect.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum REGION56_A {
    #[doc = "0: Protection disabled"]
    DISABLED = 0,
    #[doc = "1: Protection enabled"]
    ENABLED = 1,
}
impl From<REGION56_A> for bool {
    #[inline(always)]
    fn from(variant: REGION56_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `REGION56`"]
pub type REGION56_R = crate::R<bool, REGION56_A>;
impl REGION56_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> REGION56_A {
        match self.bits {
            false => REGION56_A::DISABLED,
            true => REGION56_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == REGION56_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == REGION56_A::ENABLED
    }
}
#[doc = "Write proxy for field `REGION56`"]
pub struct REGION56_W<'a> {
    w: &'a mut W,
}
impl<'a> REGION56_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: REGION56_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Protection disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(REGION56_A::DISABLED)
    }
    #[doc = "Protection enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(REGION56_A::ENABLED)
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
#[doc = "Enable protection for region 57. Write '0' has no effect.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum REGION57_A {
    #[doc = "0: Protection disabled"]
    DISABLED = 0,
    #[doc = "1: Protection enabled"]
    ENABLED = 1,
}
impl From<REGION57_A> for bool {
    #[inline(always)]
    fn from(variant: REGION57_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `REGION57`"]
pub type REGION57_R = crate::R<bool, REGION57_A>;
impl REGION57_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> REGION57_A {
        match self.bits {
            false => REGION57_A::DISABLED,
            true => REGION57_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == REGION57_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == REGION57_A::ENABLED
    }
}
#[doc = "Write proxy for field `REGION57`"]
pub struct REGION57_W<'a> {
    w: &'a mut W,
}
impl<'a> REGION57_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: REGION57_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Protection disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(REGION57_A::DISABLED)
    }
    #[doc = "Protection enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(REGION57_A::ENABLED)
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
#[doc = "Enable protection for region 58. Write '0' has no effect.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum REGION58_A {
    #[doc = "0: Protection disabled"]
    DISABLED = 0,
    #[doc = "1: Protection enabled"]
    ENABLED = 1,
}
impl From<REGION58_A> for bool {
    #[inline(always)]
    fn from(variant: REGION58_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `REGION58`"]
pub type REGION58_R = crate::R<bool, REGION58_A>;
impl REGION58_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> REGION58_A {
        match self.bits {
            false => REGION58_A::DISABLED,
            true => REGION58_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == REGION58_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == REGION58_A::ENABLED
    }
}
#[doc = "Write proxy for field `REGION58`"]
pub struct REGION58_W<'a> {
    w: &'a mut W,
}
impl<'a> REGION58_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: REGION58_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Protection disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(REGION58_A::DISABLED)
    }
    #[doc = "Protection enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(REGION58_A::ENABLED)
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
#[doc = "Enable protection for region 59. Write '0' has no effect.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum REGION59_A {
    #[doc = "0: Protection disabled"]
    DISABLED = 0,
    #[doc = "1: Protection enabled"]
    ENABLED = 1,
}
impl From<REGION59_A> for bool {
    #[inline(always)]
    fn from(variant: REGION59_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `REGION59`"]
pub type REGION59_R = crate::R<bool, REGION59_A>;
impl REGION59_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> REGION59_A {
        match self.bits {
            false => REGION59_A::DISABLED,
            true => REGION59_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == REGION59_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == REGION59_A::ENABLED
    }
}
#[doc = "Write proxy for field `REGION59`"]
pub struct REGION59_W<'a> {
    w: &'a mut W,
}
impl<'a> REGION59_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: REGION59_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Protection disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(REGION59_A::DISABLED)
    }
    #[doc = "Protection enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(REGION59_A::ENABLED)
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
#[doc = "Enable protection for region 60. Write '0' has no effect.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum REGION60_A {
    #[doc = "0: Protection disabled"]
    DISABLED = 0,
    #[doc = "1: Protection enabled"]
    ENABLED = 1,
}
impl From<REGION60_A> for bool {
    #[inline(always)]
    fn from(variant: REGION60_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `REGION60`"]
pub type REGION60_R = crate::R<bool, REGION60_A>;
impl REGION60_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> REGION60_A {
        match self.bits {
            false => REGION60_A::DISABLED,
            true => REGION60_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == REGION60_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == REGION60_A::ENABLED
    }
}
#[doc = "Write proxy for field `REGION60`"]
pub struct REGION60_W<'a> {
    w: &'a mut W,
}
impl<'a> REGION60_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: REGION60_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Protection disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(REGION60_A::DISABLED)
    }
    #[doc = "Protection enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(REGION60_A::ENABLED)
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
#[doc = "Enable protection for region 61. Write '0' has no effect.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum REGION61_A {
    #[doc = "0: Protection disabled"]
    DISABLED = 0,
    #[doc = "1: Protection enabled"]
    ENABLED = 1,
}
impl From<REGION61_A> for bool {
    #[inline(always)]
    fn from(variant: REGION61_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `REGION61`"]
pub type REGION61_R = crate::R<bool, REGION61_A>;
impl REGION61_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> REGION61_A {
        match self.bits {
            false => REGION61_A::DISABLED,
            true => REGION61_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == REGION61_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == REGION61_A::ENABLED
    }
}
#[doc = "Write proxy for field `REGION61`"]
pub struct REGION61_W<'a> {
    w: &'a mut W,
}
impl<'a> REGION61_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: REGION61_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Protection disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(REGION61_A::DISABLED)
    }
    #[doc = "Protection enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(REGION61_A::ENABLED)
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
#[doc = "Enable protection for region 62. Write '0' has no effect.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum REGION62_A {
    #[doc = "0: Protection disabled"]
    DISABLED = 0,
    #[doc = "1: Protection enabled"]
    ENABLED = 1,
}
impl From<REGION62_A> for bool {
    #[inline(always)]
    fn from(variant: REGION62_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `REGION62`"]
pub type REGION62_R = crate::R<bool, REGION62_A>;
impl REGION62_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> REGION62_A {
        match self.bits {
            false => REGION62_A::DISABLED,
            true => REGION62_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == REGION62_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == REGION62_A::ENABLED
    }
}
#[doc = "Write proxy for field `REGION62`"]
pub struct REGION62_W<'a> {
    w: &'a mut W,
}
impl<'a> REGION62_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: REGION62_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Protection disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(REGION62_A::DISABLED)
    }
    #[doc = "Protection enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(REGION62_A::ENABLED)
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
#[doc = "Enable protection for region 63. Write '0' has no effect.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum REGION63_A {
    #[doc = "0: Protection disabled"]
    DISABLED = 0,
    #[doc = "1: Protection enabled"]
    ENABLED = 1,
}
impl From<REGION63_A> for bool {
    #[inline(always)]
    fn from(variant: REGION63_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `REGION63`"]
pub type REGION63_R = crate::R<bool, REGION63_A>;
impl REGION63_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> REGION63_A {
        match self.bits {
            false => REGION63_A::DISABLED,
            true => REGION63_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == REGION63_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == REGION63_A::ENABLED
    }
}
#[doc = "Write proxy for field `REGION63`"]
pub struct REGION63_W<'a> {
    w: &'a mut W,
}
impl<'a> REGION63_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: REGION63_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Protection disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(REGION63_A::DISABLED)
    }
    #[doc = "Protection enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(REGION63_A::ENABLED)
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
    #[doc = "Bit 0 - Enable protection for region 32. Write '0' has no effect."]
    #[inline(always)]
    pub fn region32(&self) -> REGION32_R {
        REGION32_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - Enable protection for region 33. Write '0' has no effect."]
    #[inline(always)]
    pub fn region33(&self) -> REGION33_R {
        REGION33_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - Enable protection for region 34. Write '0' has no effect."]
    #[inline(always)]
    pub fn region34(&self) -> REGION34_R {
        REGION34_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - Enable protection for region 35. Write '0' has no effect."]
    #[inline(always)]
    pub fn region35(&self) -> REGION35_R {
        REGION35_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - Enable protection for region 36. Write '0' has no effect."]
    #[inline(always)]
    pub fn region36(&self) -> REGION36_R {
        REGION36_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - Enable protection for region 37. Write '0' has no effect."]
    #[inline(always)]
    pub fn region37(&self) -> REGION37_R {
        REGION37_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - Enable protection for region 38. Write '0' has no effect."]
    #[inline(always)]
    pub fn region38(&self) -> REGION38_R {
        REGION38_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7 - Enable protection for region 39. Write '0' has no effect."]
    #[inline(always)]
    pub fn region39(&self) -> REGION39_R {
        REGION39_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 8 - Enable protection for region 40. Write '0' has no effect."]
    #[inline(always)]
    pub fn region40(&self) -> REGION40_R {
        REGION40_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 9 - Enable protection for region 41. Write '0' has no effect."]
    #[inline(always)]
    pub fn region41(&self) -> REGION41_R {
        REGION41_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 10 - Enable protection for region 42. Write '0' has no effect."]
    #[inline(always)]
    pub fn region42(&self) -> REGION42_R {
        REGION42_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    #[doc = "Bit 11 - Enable protection for region 43. Write '0' has no effect."]
    #[inline(always)]
    pub fn region43(&self) -> REGION43_R {
        REGION43_R::new(((self.bits >> 11) & 0x01) != 0)
    }
    #[doc = "Bit 12 - Enable protection for region 44. Write '0' has no effect."]
    #[inline(always)]
    pub fn region44(&self) -> REGION44_R {
        REGION44_R::new(((self.bits >> 12) & 0x01) != 0)
    }
    #[doc = "Bit 13 - Enable protection for region 45. Write '0' has no effect."]
    #[inline(always)]
    pub fn region45(&self) -> REGION45_R {
        REGION45_R::new(((self.bits >> 13) & 0x01) != 0)
    }
    #[doc = "Bit 14 - Enable protection for region 46. Write '0' has no effect."]
    #[inline(always)]
    pub fn region46(&self) -> REGION46_R {
        REGION46_R::new(((self.bits >> 14) & 0x01) != 0)
    }
    #[doc = "Bit 15 - Enable protection for region 47. Write '0' has no effect."]
    #[inline(always)]
    pub fn region47(&self) -> REGION47_R {
        REGION47_R::new(((self.bits >> 15) & 0x01) != 0)
    }
    #[doc = "Bit 16 - Enable protection for region 48. Write '0' has no effect."]
    #[inline(always)]
    pub fn region48(&self) -> REGION48_R {
        REGION48_R::new(((self.bits >> 16) & 0x01) != 0)
    }
    #[doc = "Bit 17 - Enable protection for region 49. Write '0' has no effect."]
    #[inline(always)]
    pub fn region49(&self) -> REGION49_R {
        REGION49_R::new(((self.bits >> 17) & 0x01) != 0)
    }
    #[doc = "Bit 18 - Enable protection for region 50. Write '0' has no effect."]
    #[inline(always)]
    pub fn region50(&self) -> REGION50_R {
        REGION50_R::new(((self.bits >> 18) & 0x01) != 0)
    }
    #[doc = "Bit 19 - Enable protection for region 51. Write '0' has no effect."]
    #[inline(always)]
    pub fn region51(&self) -> REGION51_R {
        REGION51_R::new(((self.bits >> 19) & 0x01) != 0)
    }
    #[doc = "Bit 20 - Enable protection for region 52. Write '0' has no effect."]
    #[inline(always)]
    pub fn region52(&self) -> REGION52_R {
        REGION52_R::new(((self.bits >> 20) & 0x01) != 0)
    }
    #[doc = "Bit 21 - Enable protection for region 53. Write '0' has no effect."]
    #[inline(always)]
    pub fn region53(&self) -> REGION53_R {
        REGION53_R::new(((self.bits >> 21) & 0x01) != 0)
    }
    #[doc = "Bit 22 - Enable protection for region 54. Write '0' has no effect."]
    #[inline(always)]
    pub fn region54(&self) -> REGION54_R {
        REGION54_R::new(((self.bits >> 22) & 0x01) != 0)
    }
    #[doc = "Bit 23 - Enable protection for region 55. Write '0' has no effect."]
    #[inline(always)]
    pub fn region55(&self) -> REGION55_R {
        REGION55_R::new(((self.bits >> 23) & 0x01) != 0)
    }
    #[doc = "Bit 24 - Enable protection for region 56. Write '0' has no effect."]
    #[inline(always)]
    pub fn region56(&self) -> REGION56_R {
        REGION56_R::new(((self.bits >> 24) & 0x01) != 0)
    }
    #[doc = "Bit 25 - Enable protection for region 57. Write '0' has no effect."]
    #[inline(always)]
    pub fn region57(&self) -> REGION57_R {
        REGION57_R::new(((self.bits >> 25) & 0x01) != 0)
    }
    #[doc = "Bit 26 - Enable protection for region 58. Write '0' has no effect."]
    #[inline(always)]
    pub fn region58(&self) -> REGION58_R {
        REGION58_R::new(((self.bits >> 26) & 0x01) != 0)
    }
    #[doc = "Bit 27 - Enable protection for region 59. Write '0' has no effect."]
    #[inline(always)]
    pub fn region59(&self) -> REGION59_R {
        REGION59_R::new(((self.bits >> 27) & 0x01) != 0)
    }
    #[doc = "Bit 28 - Enable protection for region 60. Write '0' has no effect."]
    #[inline(always)]
    pub fn region60(&self) -> REGION60_R {
        REGION60_R::new(((self.bits >> 28) & 0x01) != 0)
    }
    #[doc = "Bit 29 - Enable protection for region 61. Write '0' has no effect."]
    #[inline(always)]
    pub fn region61(&self) -> REGION61_R {
        REGION61_R::new(((self.bits >> 29) & 0x01) != 0)
    }
    #[doc = "Bit 30 - Enable protection for region 62. Write '0' has no effect."]
    #[inline(always)]
    pub fn region62(&self) -> REGION62_R {
        REGION62_R::new(((self.bits >> 30) & 0x01) != 0)
    }
    #[doc = "Bit 31 - Enable protection for region 63. Write '0' has no effect."]
    #[inline(always)]
    pub fn region63(&self) -> REGION63_R {
        REGION63_R::new(((self.bits >> 31) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Enable protection for region 32. Write '0' has no effect."]
    #[inline(always)]
    pub fn region32(&mut self) -> REGION32_W {
        REGION32_W { w: self }
    }
    #[doc = "Bit 1 - Enable protection for region 33. Write '0' has no effect."]
    #[inline(always)]
    pub fn region33(&mut self) -> REGION33_W {
        REGION33_W { w: self }
    }
    #[doc = "Bit 2 - Enable protection for region 34. Write '0' has no effect."]
    #[inline(always)]
    pub fn region34(&mut self) -> REGION34_W {
        REGION34_W { w: self }
    }
    #[doc = "Bit 3 - Enable protection for region 35. Write '0' has no effect."]
    #[inline(always)]
    pub fn region35(&mut self) -> REGION35_W {
        REGION35_W { w: self }
    }
    #[doc = "Bit 4 - Enable protection for region 36. Write '0' has no effect."]
    #[inline(always)]
    pub fn region36(&mut self) -> REGION36_W {
        REGION36_W { w: self }
    }
    #[doc = "Bit 5 - Enable protection for region 37. Write '0' has no effect."]
    #[inline(always)]
    pub fn region37(&mut self) -> REGION37_W {
        REGION37_W { w: self }
    }
    #[doc = "Bit 6 - Enable protection for region 38. Write '0' has no effect."]
    #[inline(always)]
    pub fn region38(&mut self) -> REGION38_W {
        REGION38_W { w: self }
    }
    #[doc = "Bit 7 - Enable protection for region 39. Write '0' has no effect."]
    #[inline(always)]
    pub fn region39(&mut self) -> REGION39_W {
        REGION39_W { w: self }
    }
    #[doc = "Bit 8 - Enable protection for region 40. Write '0' has no effect."]
    #[inline(always)]
    pub fn region40(&mut self) -> REGION40_W {
        REGION40_W { w: self }
    }
    #[doc = "Bit 9 - Enable protection for region 41. Write '0' has no effect."]
    #[inline(always)]
    pub fn region41(&mut self) -> REGION41_W {
        REGION41_W { w: self }
    }
    #[doc = "Bit 10 - Enable protection for region 42. Write '0' has no effect."]
    #[inline(always)]
    pub fn region42(&mut self) -> REGION42_W {
        REGION42_W { w: self }
    }
    #[doc = "Bit 11 - Enable protection for region 43. Write '0' has no effect."]
    #[inline(always)]
    pub fn region43(&mut self) -> REGION43_W {
        REGION43_W { w: self }
    }
    #[doc = "Bit 12 - Enable protection for region 44. Write '0' has no effect."]
    #[inline(always)]
    pub fn region44(&mut self) -> REGION44_W {
        REGION44_W { w: self }
    }
    #[doc = "Bit 13 - Enable protection for region 45. Write '0' has no effect."]
    #[inline(always)]
    pub fn region45(&mut self) -> REGION45_W {
        REGION45_W { w: self }
    }
    #[doc = "Bit 14 - Enable protection for region 46. Write '0' has no effect."]
    #[inline(always)]
    pub fn region46(&mut self) -> REGION46_W {
        REGION46_W { w: self }
    }
    #[doc = "Bit 15 - Enable protection for region 47. Write '0' has no effect."]
    #[inline(always)]
    pub fn region47(&mut self) -> REGION47_W {
        REGION47_W { w: self }
    }
    #[doc = "Bit 16 - Enable protection for region 48. Write '0' has no effect."]
    #[inline(always)]
    pub fn region48(&mut self) -> REGION48_W {
        REGION48_W { w: self }
    }
    #[doc = "Bit 17 - Enable protection for region 49. Write '0' has no effect."]
    #[inline(always)]
    pub fn region49(&mut self) -> REGION49_W {
        REGION49_W { w: self }
    }
    #[doc = "Bit 18 - Enable protection for region 50. Write '0' has no effect."]
    #[inline(always)]
    pub fn region50(&mut self) -> REGION50_W {
        REGION50_W { w: self }
    }
    #[doc = "Bit 19 - Enable protection for region 51. Write '0' has no effect."]
    #[inline(always)]
    pub fn region51(&mut self) -> REGION51_W {
        REGION51_W { w: self }
    }
    #[doc = "Bit 20 - Enable protection for region 52. Write '0' has no effect."]
    #[inline(always)]
    pub fn region52(&mut self) -> REGION52_W {
        REGION52_W { w: self }
    }
    #[doc = "Bit 21 - Enable protection for region 53. Write '0' has no effect."]
    #[inline(always)]
    pub fn region53(&mut self) -> REGION53_W {
        REGION53_W { w: self }
    }
    #[doc = "Bit 22 - Enable protection for region 54. Write '0' has no effect."]
    #[inline(always)]
    pub fn region54(&mut self) -> REGION54_W {
        REGION54_W { w: self }
    }
    #[doc = "Bit 23 - Enable protection for region 55. Write '0' has no effect."]
    #[inline(always)]
    pub fn region55(&mut self) -> REGION55_W {
        REGION55_W { w: self }
    }
    #[doc = "Bit 24 - Enable protection for region 56. Write '0' has no effect."]
    #[inline(always)]
    pub fn region56(&mut self) -> REGION56_W {
        REGION56_W { w: self }
    }
    #[doc = "Bit 25 - Enable protection for region 57. Write '0' has no effect."]
    #[inline(always)]
    pub fn region57(&mut self) -> REGION57_W {
        REGION57_W { w: self }
    }
    #[doc = "Bit 26 - Enable protection for region 58. Write '0' has no effect."]
    #[inline(always)]
    pub fn region58(&mut self) -> REGION58_W {
        REGION58_W { w: self }
    }
    #[doc = "Bit 27 - Enable protection for region 59. Write '0' has no effect."]
    #[inline(always)]
    pub fn region59(&mut self) -> REGION59_W {
        REGION59_W { w: self }
    }
    #[doc = "Bit 28 - Enable protection for region 60. Write '0' has no effect."]
    #[inline(always)]
    pub fn region60(&mut self) -> REGION60_W {
        REGION60_W { w: self }
    }
    #[doc = "Bit 29 - Enable protection for region 61. Write '0' has no effect."]
    #[inline(always)]
    pub fn region61(&mut self) -> REGION61_W {
        REGION61_W { w: self }
    }
    #[doc = "Bit 30 - Enable protection for region 62. Write '0' has no effect."]
    #[inline(always)]
    pub fn region62(&mut self) -> REGION62_W {
        REGION62_W { w: self }
    }
    #[doc = "Bit 31 - Enable protection for region 63. Write '0' has no effect."]
    #[inline(always)]
    pub fn region63(&mut self) -> REGION63_W {
        REGION63_W { w: self }
    }
}
