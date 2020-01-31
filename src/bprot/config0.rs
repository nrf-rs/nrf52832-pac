#[doc = "Reader of register CONFIG0"]
pub type R = crate::R<u32, super::CONFIG0>;
#[doc = "Writer for register CONFIG0"]
pub type W = crate::W<u32, super::CONFIG0>;
#[doc = "Register CONFIG0 `reset()`'s with value 0"]
impl crate::ResetValue for super::CONFIG0 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Enable protection for region 0. Write '0' has no effect.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum REGION0_A {
    #[doc = "0: Protection disabled"]
    DISABLED = 0,
    #[doc = "1: Protection enable"]
    ENABLED = 1,
}
impl From<REGION0_A> for bool {
    #[inline(always)]
    fn from(variant: REGION0_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `REGION0`"]
pub type REGION0_R = crate::R<bool, REGION0_A>;
impl REGION0_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> REGION0_A {
        match self.bits {
            false => REGION0_A::DISABLED,
            true => REGION0_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == REGION0_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == REGION0_A::ENABLED
    }
}
#[doc = "Write proxy for field `REGION0`"]
pub struct REGION0_W<'a> {
    w: &'a mut W,
}
impl<'a> REGION0_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: REGION0_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Protection disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(REGION0_A::DISABLED)
    }
    #[doc = "Protection enable"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(REGION0_A::ENABLED)
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
#[doc = "Enable protection for region 1. Write '0' has no effect.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum REGION1_A {
    #[doc = "0: Protection disabled"]
    DISABLED = 0,
    #[doc = "1: Protection enable"]
    ENABLED = 1,
}
impl From<REGION1_A> for bool {
    #[inline(always)]
    fn from(variant: REGION1_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `REGION1`"]
pub type REGION1_R = crate::R<bool, REGION1_A>;
impl REGION1_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> REGION1_A {
        match self.bits {
            false => REGION1_A::DISABLED,
            true => REGION1_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == REGION1_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == REGION1_A::ENABLED
    }
}
#[doc = "Write proxy for field `REGION1`"]
pub struct REGION1_W<'a> {
    w: &'a mut W,
}
impl<'a> REGION1_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: REGION1_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Protection disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(REGION1_A::DISABLED)
    }
    #[doc = "Protection enable"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(REGION1_A::ENABLED)
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
#[doc = "Enable protection for region 2. Write '0' has no effect.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum REGION2_A {
    #[doc = "0: Protection disabled"]
    DISABLED = 0,
    #[doc = "1: Protection enable"]
    ENABLED = 1,
}
impl From<REGION2_A> for bool {
    #[inline(always)]
    fn from(variant: REGION2_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `REGION2`"]
pub type REGION2_R = crate::R<bool, REGION2_A>;
impl REGION2_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> REGION2_A {
        match self.bits {
            false => REGION2_A::DISABLED,
            true => REGION2_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == REGION2_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == REGION2_A::ENABLED
    }
}
#[doc = "Write proxy for field `REGION2`"]
pub struct REGION2_W<'a> {
    w: &'a mut W,
}
impl<'a> REGION2_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: REGION2_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Protection disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(REGION2_A::DISABLED)
    }
    #[doc = "Protection enable"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(REGION2_A::ENABLED)
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
#[doc = "Enable protection for region 3. Write '0' has no effect.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum REGION3_A {
    #[doc = "0: Protection disabled"]
    DISABLED = 0,
    #[doc = "1: Protection enable"]
    ENABLED = 1,
}
impl From<REGION3_A> for bool {
    #[inline(always)]
    fn from(variant: REGION3_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `REGION3`"]
pub type REGION3_R = crate::R<bool, REGION3_A>;
impl REGION3_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> REGION3_A {
        match self.bits {
            false => REGION3_A::DISABLED,
            true => REGION3_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == REGION3_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == REGION3_A::ENABLED
    }
}
#[doc = "Write proxy for field `REGION3`"]
pub struct REGION3_W<'a> {
    w: &'a mut W,
}
impl<'a> REGION3_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: REGION3_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Protection disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(REGION3_A::DISABLED)
    }
    #[doc = "Protection enable"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(REGION3_A::ENABLED)
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
#[doc = "Enable protection for region 4. Write '0' has no effect.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum REGION4_A {
    #[doc = "0: Protection disabled"]
    DISABLED = 0,
    #[doc = "1: Protection enable"]
    ENABLED = 1,
}
impl From<REGION4_A> for bool {
    #[inline(always)]
    fn from(variant: REGION4_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `REGION4`"]
pub type REGION4_R = crate::R<bool, REGION4_A>;
impl REGION4_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> REGION4_A {
        match self.bits {
            false => REGION4_A::DISABLED,
            true => REGION4_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == REGION4_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == REGION4_A::ENABLED
    }
}
#[doc = "Write proxy for field `REGION4`"]
pub struct REGION4_W<'a> {
    w: &'a mut W,
}
impl<'a> REGION4_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: REGION4_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Protection disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(REGION4_A::DISABLED)
    }
    #[doc = "Protection enable"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(REGION4_A::ENABLED)
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
#[doc = "Enable protection for region 5. Write '0' has no effect.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum REGION5_A {
    #[doc = "0: Protection disabled"]
    DISABLED = 0,
    #[doc = "1: Protection enable"]
    ENABLED = 1,
}
impl From<REGION5_A> for bool {
    #[inline(always)]
    fn from(variant: REGION5_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `REGION5`"]
pub type REGION5_R = crate::R<bool, REGION5_A>;
impl REGION5_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> REGION5_A {
        match self.bits {
            false => REGION5_A::DISABLED,
            true => REGION5_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == REGION5_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == REGION5_A::ENABLED
    }
}
#[doc = "Write proxy for field `REGION5`"]
pub struct REGION5_W<'a> {
    w: &'a mut W,
}
impl<'a> REGION5_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: REGION5_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Protection disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(REGION5_A::DISABLED)
    }
    #[doc = "Protection enable"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(REGION5_A::ENABLED)
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
#[doc = "Enable protection for region 6. Write '0' has no effect.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum REGION6_A {
    #[doc = "0: Protection disabled"]
    DISABLED = 0,
    #[doc = "1: Protection enable"]
    ENABLED = 1,
}
impl From<REGION6_A> for bool {
    #[inline(always)]
    fn from(variant: REGION6_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `REGION6`"]
pub type REGION6_R = crate::R<bool, REGION6_A>;
impl REGION6_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> REGION6_A {
        match self.bits {
            false => REGION6_A::DISABLED,
            true => REGION6_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == REGION6_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == REGION6_A::ENABLED
    }
}
#[doc = "Write proxy for field `REGION6`"]
pub struct REGION6_W<'a> {
    w: &'a mut W,
}
impl<'a> REGION6_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: REGION6_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Protection disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(REGION6_A::DISABLED)
    }
    #[doc = "Protection enable"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(REGION6_A::ENABLED)
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
#[doc = "Enable protection for region 7. Write '0' has no effect.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum REGION7_A {
    #[doc = "0: Protection disabled"]
    DISABLED = 0,
    #[doc = "1: Protection enable"]
    ENABLED = 1,
}
impl From<REGION7_A> for bool {
    #[inline(always)]
    fn from(variant: REGION7_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `REGION7`"]
pub type REGION7_R = crate::R<bool, REGION7_A>;
impl REGION7_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> REGION7_A {
        match self.bits {
            false => REGION7_A::DISABLED,
            true => REGION7_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == REGION7_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == REGION7_A::ENABLED
    }
}
#[doc = "Write proxy for field `REGION7`"]
pub struct REGION7_W<'a> {
    w: &'a mut W,
}
impl<'a> REGION7_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: REGION7_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Protection disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(REGION7_A::DISABLED)
    }
    #[doc = "Protection enable"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(REGION7_A::ENABLED)
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
#[doc = "Enable protection for region 8. Write '0' has no effect.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum REGION8_A {
    #[doc = "0: Protection disabled"]
    DISABLED = 0,
    #[doc = "1: Protection enable"]
    ENABLED = 1,
}
impl From<REGION8_A> for bool {
    #[inline(always)]
    fn from(variant: REGION8_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `REGION8`"]
pub type REGION8_R = crate::R<bool, REGION8_A>;
impl REGION8_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> REGION8_A {
        match self.bits {
            false => REGION8_A::DISABLED,
            true => REGION8_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == REGION8_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == REGION8_A::ENABLED
    }
}
#[doc = "Write proxy for field `REGION8`"]
pub struct REGION8_W<'a> {
    w: &'a mut W,
}
impl<'a> REGION8_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: REGION8_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Protection disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(REGION8_A::DISABLED)
    }
    #[doc = "Protection enable"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(REGION8_A::ENABLED)
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
#[doc = "Enable protection for region 9. Write '0' has no effect.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum REGION9_A {
    #[doc = "0: Protection disabled"]
    DISABLED = 0,
    #[doc = "1: Protection enable"]
    ENABLED = 1,
}
impl From<REGION9_A> for bool {
    #[inline(always)]
    fn from(variant: REGION9_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `REGION9`"]
pub type REGION9_R = crate::R<bool, REGION9_A>;
impl REGION9_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> REGION9_A {
        match self.bits {
            false => REGION9_A::DISABLED,
            true => REGION9_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == REGION9_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == REGION9_A::ENABLED
    }
}
#[doc = "Write proxy for field `REGION9`"]
pub struct REGION9_W<'a> {
    w: &'a mut W,
}
impl<'a> REGION9_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: REGION9_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Protection disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(REGION9_A::DISABLED)
    }
    #[doc = "Protection enable"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(REGION9_A::ENABLED)
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
#[doc = "Enable protection for region 10. Write '0' has no effect.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum REGION10_A {
    #[doc = "0: Protection disabled"]
    DISABLED = 0,
    #[doc = "1: Protection enable"]
    ENABLED = 1,
}
impl From<REGION10_A> for bool {
    #[inline(always)]
    fn from(variant: REGION10_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `REGION10`"]
pub type REGION10_R = crate::R<bool, REGION10_A>;
impl REGION10_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> REGION10_A {
        match self.bits {
            false => REGION10_A::DISABLED,
            true => REGION10_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == REGION10_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == REGION10_A::ENABLED
    }
}
#[doc = "Write proxy for field `REGION10`"]
pub struct REGION10_W<'a> {
    w: &'a mut W,
}
impl<'a> REGION10_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: REGION10_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Protection disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(REGION10_A::DISABLED)
    }
    #[doc = "Protection enable"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(REGION10_A::ENABLED)
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
#[doc = "Enable protection for region 11. Write '0' has no effect.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum REGION11_A {
    #[doc = "0: Protection disabled"]
    DISABLED = 0,
    #[doc = "1: Protection enable"]
    ENABLED = 1,
}
impl From<REGION11_A> for bool {
    #[inline(always)]
    fn from(variant: REGION11_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `REGION11`"]
pub type REGION11_R = crate::R<bool, REGION11_A>;
impl REGION11_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> REGION11_A {
        match self.bits {
            false => REGION11_A::DISABLED,
            true => REGION11_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == REGION11_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == REGION11_A::ENABLED
    }
}
#[doc = "Write proxy for field `REGION11`"]
pub struct REGION11_W<'a> {
    w: &'a mut W,
}
impl<'a> REGION11_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: REGION11_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Protection disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(REGION11_A::DISABLED)
    }
    #[doc = "Protection enable"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(REGION11_A::ENABLED)
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
#[doc = "Enable protection for region 12. Write '0' has no effect.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum REGION12_A {
    #[doc = "0: Protection disabled"]
    DISABLED = 0,
    #[doc = "1: Protection enable"]
    ENABLED = 1,
}
impl From<REGION12_A> for bool {
    #[inline(always)]
    fn from(variant: REGION12_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `REGION12`"]
pub type REGION12_R = crate::R<bool, REGION12_A>;
impl REGION12_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> REGION12_A {
        match self.bits {
            false => REGION12_A::DISABLED,
            true => REGION12_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == REGION12_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == REGION12_A::ENABLED
    }
}
#[doc = "Write proxy for field `REGION12`"]
pub struct REGION12_W<'a> {
    w: &'a mut W,
}
impl<'a> REGION12_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: REGION12_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Protection disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(REGION12_A::DISABLED)
    }
    #[doc = "Protection enable"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(REGION12_A::ENABLED)
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
#[doc = "Enable protection for region 13. Write '0' has no effect.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum REGION13_A {
    #[doc = "0: Protection disabled"]
    DISABLED = 0,
    #[doc = "1: Protection enable"]
    ENABLED = 1,
}
impl From<REGION13_A> for bool {
    #[inline(always)]
    fn from(variant: REGION13_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `REGION13`"]
pub type REGION13_R = crate::R<bool, REGION13_A>;
impl REGION13_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> REGION13_A {
        match self.bits {
            false => REGION13_A::DISABLED,
            true => REGION13_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == REGION13_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == REGION13_A::ENABLED
    }
}
#[doc = "Write proxy for field `REGION13`"]
pub struct REGION13_W<'a> {
    w: &'a mut W,
}
impl<'a> REGION13_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: REGION13_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Protection disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(REGION13_A::DISABLED)
    }
    #[doc = "Protection enable"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(REGION13_A::ENABLED)
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
#[doc = "Enable protection for region 14. Write '0' has no effect.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum REGION14_A {
    #[doc = "0: Protection disabled"]
    DISABLED = 0,
    #[doc = "1: Protection enable"]
    ENABLED = 1,
}
impl From<REGION14_A> for bool {
    #[inline(always)]
    fn from(variant: REGION14_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `REGION14`"]
pub type REGION14_R = crate::R<bool, REGION14_A>;
impl REGION14_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> REGION14_A {
        match self.bits {
            false => REGION14_A::DISABLED,
            true => REGION14_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == REGION14_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == REGION14_A::ENABLED
    }
}
#[doc = "Write proxy for field `REGION14`"]
pub struct REGION14_W<'a> {
    w: &'a mut W,
}
impl<'a> REGION14_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: REGION14_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Protection disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(REGION14_A::DISABLED)
    }
    #[doc = "Protection enable"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(REGION14_A::ENABLED)
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
#[doc = "Enable protection for region 15. Write '0' has no effect.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum REGION15_A {
    #[doc = "0: Protection disabled"]
    DISABLED = 0,
    #[doc = "1: Protection enable"]
    ENABLED = 1,
}
impl From<REGION15_A> for bool {
    #[inline(always)]
    fn from(variant: REGION15_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `REGION15`"]
pub type REGION15_R = crate::R<bool, REGION15_A>;
impl REGION15_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> REGION15_A {
        match self.bits {
            false => REGION15_A::DISABLED,
            true => REGION15_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == REGION15_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == REGION15_A::ENABLED
    }
}
#[doc = "Write proxy for field `REGION15`"]
pub struct REGION15_W<'a> {
    w: &'a mut W,
}
impl<'a> REGION15_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: REGION15_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Protection disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(REGION15_A::DISABLED)
    }
    #[doc = "Protection enable"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(REGION15_A::ENABLED)
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
#[doc = "Enable protection for region 16. Write '0' has no effect.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum REGION16_A {
    #[doc = "0: Protection disabled"]
    DISABLED = 0,
    #[doc = "1: Protection enable"]
    ENABLED = 1,
}
impl From<REGION16_A> for bool {
    #[inline(always)]
    fn from(variant: REGION16_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `REGION16`"]
pub type REGION16_R = crate::R<bool, REGION16_A>;
impl REGION16_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> REGION16_A {
        match self.bits {
            false => REGION16_A::DISABLED,
            true => REGION16_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == REGION16_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == REGION16_A::ENABLED
    }
}
#[doc = "Write proxy for field `REGION16`"]
pub struct REGION16_W<'a> {
    w: &'a mut W,
}
impl<'a> REGION16_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: REGION16_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Protection disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(REGION16_A::DISABLED)
    }
    #[doc = "Protection enable"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(REGION16_A::ENABLED)
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
#[doc = "Enable protection for region 17. Write '0' has no effect.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum REGION17_A {
    #[doc = "0: Protection disabled"]
    DISABLED = 0,
    #[doc = "1: Protection enable"]
    ENABLED = 1,
}
impl From<REGION17_A> for bool {
    #[inline(always)]
    fn from(variant: REGION17_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `REGION17`"]
pub type REGION17_R = crate::R<bool, REGION17_A>;
impl REGION17_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> REGION17_A {
        match self.bits {
            false => REGION17_A::DISABLED,
            true => REGION17_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == REGION17_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == REGION17_A::ENABLED
    }
}
#[doc = "Write proxy for field `REGION17`"]
pub struct REGION17_W<'a> {
    w: &'a mut W,
}
impl<'a> REGION17_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: REGION17_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Protection disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(REGION17_A::DISABLED)
    }
    #[doc = "Protection enable"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(REGION17_A::ENABLED)
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
#[doc = "Enable protection for region 18. Write '0' has no effect.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum REGION18_A {
    #[doc = "0: Protection disabled"]
    DISABLED = 0,
    #[doc = "1: Protection enable"]
    ENABLED = 1,
}
impl From<REGION18_A> for bool {
    #[inline(always)]
    fn from(variant: REGION18_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `REGION18`"]
pub type REGION18_R = crate::R<bool, REGION18_A>;
impl REGION18_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> REGION18_A {
        match self.bits {
            false => REGION18_A::DISABLED,
            true => REGION18_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == REGION18_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == REGION18_A::ENABLED
    }
}
#[doc = "Write proxy for field `REGION18`"]
pub struct REGION18_W<'a> {
    w: &'a mut W,
}
impl<'a> REGION18_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: REGION18_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Protection disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(REGION18_A::DISABLED)
    }
    #[doc = "Protection enable"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(REGION18_A::ENABLED)
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
#[doc = "Enable protection for region 19. Write '0' has no effect.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum REGION19_A {
    #[doc = "0: Protection disabled"]
    DISABLED = 0,
    #[doc = "1: Protection enable"]
    ENABLED = 1,
}
impl From<REGION19_A> for bool {
    #[inline(always)]
    fn from(variant: REGION19_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `REGION19`"]
pub type REGION19_R = crate::R<bool, REGION19_A>;
impl REGION19_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> REGION19_A {
        match self.bits {
            false => REGION19_A::DISABLED,
            true => REGION19_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == REGION19_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == REGION19_A::ENABLED
    }
}
#[doc = "Write proxy for field `REGION19`"]
pub struct REGION19_W<'a> {
    w: &'a mut W,
}
impl<'a> REGION19_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: REGION19_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Protection disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(REGION19_A::DISABLED)
    }
    #[doc = "Protection enable"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(REGION19_A::ENABLED)
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
#[doc = "Enable protection for region 20. Write '0' has no effect.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum REGION20_A {
    #[doc = "0: Protection disabled"]
    DISABLED = 0,
    #[doc = "1: Protection enable"]
    ENABLED = 1,
}
impl From<REGION20_A> for bool {
    #[inline(always)]
    fn from(variant: REGION20_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `REGION20`"]
pub type REGION20_R = crate::R<bool, REGION20_A>;
impl REGION20_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> REGION20_A {
        match self.bits {
            false => REGION20_A::DISABLED,
            true => REGION20_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == REGION20_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == REGION20_A::ENABLED
    }
}
#[doc = "Write proxy for field `REGION20`"]
pub struct REGION20_W<'a> {
    w: &'a mut W,
}
impl<'a> REGION20_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: REGION20_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Protection disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(REGION20_A::DISABLED)
    }
    #[doc = "Protection enable"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(REGION20_A::ENABLED)
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
#[doc = "Enable protection for region 21. Write '0' has no effect.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum REGION21_A {
    #[doc = "0: Protection disabled"]
    DISABLED = 0,
    #[doc = "1: Protection enable"]
    ENABLED = 1,
}
impl From<REGION21_A> for bool {
    #[inline(always)]
    fn from(variant: REGION21_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `REGION21`"]
pub type REGION21_R = crate::R<bool, REGION21_A>;
impl REGION21_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> REGION21_A {
        match self.bits {
            false => REGION21_A::DISABLED,
            true => REGION21_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == REGION21_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == REGION21_A::ENABLED
    }
}
#[doc = "Write proxy for field `REGION21`"]
pub struct REGION21_W<'a> {
    w: &'a mut W,
}
impl<'a> REGION21_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: REGION21_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Protection disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(REGION21_A::DISABLED)
    }
    #[doc = "Protection enable"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(REGION21_A::ENABLED)
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
#[doc = "Enable protection for region 22. Write '0' has no effect.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum REGION22_A {
    #[doc = "0: Protection disabled"]
    DISABLED = 0,
    #[doc = "1: Protection enable"]
    ENABLED = 1,
}
impl From<REGION22_A> for bool {
    #[inline(always)]
    fn from(variant: REGION22_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `REGION22`"]
pub type REGION22_R = crate::R<bool, REGION22_A>;
impl REGION22_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> REGION22_A {
        match self.bits {
            false => REGION22_A::DISABLED,
            true => REGION22_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == REGION22_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == REGION22_A::ENABLED
    }
}
#[doc = "Write proxy for field `REGION22`"]
pub struct REGION22_W<'a> {
    w: &'a mut W,
}
impl<'a> REGION22_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: REGION22_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Protection disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(REGION22_A::DISABLED)
    }
    #[doc = "Protection enable"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(REGION22_A::ENABLED)
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
#[doc = "Enable protection for region 23. Write '0' has no effect.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum REGION23_A {
    #[doc = "0: Protection disabled"]
    DISABLED = 0,
    #[doc = "1: Protection enable"]
    ENABLED = 1,
}
impl From<REGION23_A> for bool {
    #[inline(always)]
    fn from(variant: REGION23_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `REGION23`"]
pub type REGION23_R = crate::R<bool, REGION23_A>;
impl REGION23_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> REGION23_A {
        match self.bits {
            false => REGION23_A::DISABLED,
            true => REGION23_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == REGION23_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == REGION23_A::ENABLED
    }
}
#[doc = "Write proxy for field `REGION23`"]
pub struct REGION23_W<'a> {
    w: &'a mut W,
}
impl<'a> REGION23_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: REGION23_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Protection disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(REGION23_A::DISABLED)
    }
    #[doc = "Protection enable"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(REGION23_A::ENABLED)
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
#[doc = "Enable protection for region 24. Write '0' has no effect.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum REGION24_A {
    #[doc = "0: Protection disabled"]
    DISABLED = 0,
    #[doc = "1: Protection enable"]
    ENABLED = 1,
}
impl From<REGION24_A> for bool {
    #[inline(always)]
    fn from(variant: REGION24_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `REGION24`"]
pub type REGION24_R = crate::R<bool, REGION24_A>;
impl REGION24_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> REGION24_A {
        match self.bits {
            false => REGION24_A::DISABLED,
            true => REGION24_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == REGION24_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == REGION24_A::ENABLED
    }
}
#[doc = "Write proxy for field `REGION24`"]
pub struct REGION24_W<'a> {
    w: &'a mut W,
}
impl<'a> REGION24_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: REGION24_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Protection disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(REGION24_A::DISABLED)
    }
    #[doc = "Protection enable"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(REGION24_A::ENABLED)
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
#[doc = "Enable protection for region 25. Write '0' has no effect.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum REGION25_A {
    #[doc = "0: Protection disabled"]
    DISABLED = 0,
    #[doc = "1: Protection enable"]
    ENABLED = 1,
}
impl From<REGION25_A> for bool {
    #[inline(always)]
    fn from(variant: REGION25_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `REGION25`"]
pub type REGION25_R = crate::R<bool, REGION25_A>;
impl REGION25_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> REGION25_A {
        match self.bits {
            false => REGION25_A::DISABLED,
            true => REGION25_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == REGION25_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == REGION25_A::ENABLED
    }
}
#[doc = "Write proxy for field `REGION25`"]
pub struct REGION25_W<'a> {
    w: &'a mut W,
}
impl<'a> REGION25_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: REGION25_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Protection disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(REGION25_A::DISABLED)
    }
    #[doc = "Protection enable"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(REGION25_A::ENABLED)
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
#[doc = "Enable protection for region 26. Write '0' has no effect.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum REGION26_A {
    #[doc = "0: Protection disabled"]
    DISABLED = 0,
    #[doc = "1: Protection enable"]
    ENABLED = 1,
}
impl From<REGION26_A> for bool {
    #[inline(always)]
    fn from(variant: REGION26_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `REGION26`"]
pub type REGION26_R = crate::R<bool, REGION26_A>;
impl REGION26_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> REGION26_A {
        match self.bits {
            false => REGION26_A::DISABLED,
            true => REGION26_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == REGION26_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == REGION26_A::ENABLED
    }
}
#[doc = "Write proxy for field `REGION26`"]
pub struct REGION26_W<'a> {
    w: &'a mut W,
}
impl<'a> REGION26_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: REGION26_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Protection disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(REGION26_A::DISABLED)
    }
    #[doc = "Protection enable"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(REGION26_A::ENABLED)
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
#[doc = "Enable protection for region 27. Write '0' has no effect.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum REGION27_A {
    #[doc = "0: Protection disabled"]
    DISABLED = 0,
    #[doc = "1: Protection enable"]
    ENABLED = 1,
}
impl From<REGION27_A> for bool {
    #[inline(always)]
    fn from(variant: REGION27_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `REGION27`"]
pub type REGION27_R = crate::R<bool, REGION27_A>;
impl REGION27_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> REGION27_A {
        match self.bits {
            false => REGION27_A::DISABLED,
            true => REGION27_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == REGION27_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == REGION27_A::ENABLED
    }
}
#[doc = "Write proxy for field `REGION27`"]
pub struct REGION27_W<'a> {
    w: &'a mut W,
}
impl<'a> REGION27_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: REGION27_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Protection disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(REGION27_A::DISABLED)
    }
    #[doc = "Protection enable"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(REGION27_A::ENABLED)
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
#[doc = "Enable protection for region 28. Write '0' has no effect.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum REGION28_A {
    #[doc = "0: Protection disabled"]
    DISABLED = 0,
    #[doc = "1: Protection enable"]
    ENABLED = 1,
}
impl From<REGION28_A> for bool {
    #[inline(always)]
    fn from(variant: REGION28_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `REGION28`"]
pub type REGION28_R = crate::R<bool, REGION28_A>;
impl REGION28_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> REGION28_A {
        match self.bits {
            false => REGION28_A::DISABLED,
            true => REGION28_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == REGION28_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == REGION28_A::ENABLED
    }
}
#[doc = "Write proxy for field `REGION28`"]
pub struct REGION28_W<'a> {
    w: &'a mut W,
}
impl<'a> REGION28_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: REGION28_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Protection disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(REGION28_A::DISABLED)
    }
    #[doc = "Protection enable"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(REGION28_A::ENABLED)
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
#[doc = "Enable protection for region 29. Write '0' has no effect.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum REGION29_A {
    #[doc = "0: Protection disabled"]
    DISABLED = 0,
    #[doc = "1: Protection enable"]
    ENABLED = 1,
}
impl From<REGION29_A> for bool {
    #[inline(always)]
    fn from(variant: REGION29_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `REGION29`"]
pub type REGION29_R = crate::R<bool, REGION29_A>;
impl REGION29_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> REGION29_A {
        match self.bits {
            false => REGION29_A::DISABLED,
            true => REGION29_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == REGION29_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == REGION29_A::ENABLED
    }
}
#[doc = "Write proxy for field `REGION29`"]
pub struct REGION29_W<'a> {
    w: &'a mut W,
}
impl<'a> REGION29_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: REGION29_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Protection disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(REGION29_A::DISABLED)
    }
    #[doc = "Protection enable"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(REGION29_A::ENABLED)
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
#[doc = "Enable protection for region 30. Write '0' has no effect.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum REGION30_A {
    #[doc = "0: Protection disabled"]
    DISABLED = 0,
    #[doc = "1: Protection enable"]
    ENABLED = 1,
}
impl From<REGION30_A> for bool {
    #[inline(always)]
    fn from(variant: REGION30_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `REGION30`"]
pub type REGION30_R = crate::R<bool, REGION30_A>;
impl REGION30_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> REGION30_A {
        match self.bits {
            false => REGION30_A::DISABLED,
            true => REGION30_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == REGION30_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == REGION30_A::ENABLED
    }
}
#[doc = "Write proxy for field `REGION30`"]
pub struct REGION30_W<'a> {
    w: &'a mut W,
}
impl<'a> REGION30_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: REGION30_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Protection disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(REGION30_A::DISABLED)
    }
    #[doc = "Protection enable"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(REGION30_A::ENABLED)
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
#[doc = "Enable protection for region 31. Write '0' has no effect.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum REGION31_A {
    #[doc = "0: Protection disabled"]
    DISABLED = 0,
    #[doc = "1: Protection enable"]
    ENABLED = 1,
}
impl From<REGION31_A> for bool {
    #[inline(always)]
    fn from(variant: REGION31_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `REGION31`"]
pub type REGION31_R = crate::R<bool, REGION31_A>;
impl REGION31_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> REGION31_A {
        match self.bits {
            false => REGION31_A::DISABLED,
            true => REGION31_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == REGION31_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == REGION31_A::ENABLED
    }
}
#[doc = "Write proxy for field `REGION31`"]
pub struct REGION31_W<'a> {
    w: &'a mut W,
}
impl<'a> REGION31_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: REGION31_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Protection disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(REGION31_A::DISABLED)
    }
    #[doc = "Protection enable"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(REGION31_A::ENABLED)
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
    #[doc = "Bit 0 - Enable protection for region 0. Write '0' has no effect."]
    #[inline(always)]
    pub fn region0(&self) -> REGION0_R {
        REGION0_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - Enable protection for region 1. Write '0' has no effect."]
    #[inline(always)]
    pub fn region1(&self) -> REGION1_R {
        REGION1_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - Enable protection for region 2. Write '0' has no effect."]
    #[inline(always)]
    pub fn region2(&self) -> REGION2_R {
        REGION2_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - Enable protection for region 3. Write '0' has no effect."]
    #[inline(always)]
    pub fn region3(&self) -> REGION3_R {
        REGION3_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - Enable protection for region 4. Write '0' has no effect."]
    #[inline(always)]
    pub fn region4(&self) -> REGION4_R {
        REGION4_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - Enable protection for region 5. Write '0' has no effect."]
    #[inline(always)]
    pub fn region5(&self) -> REGION5_R {
        REGION5_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - Enable protection for region 6. Write '0' has no effect."]
    #[inline(always)]
    pub fn region6(&self) -> REGION6_R {
        REGION6_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7 - Enable protection for region 7. Write '0' has no effect."]
    #[inline(always)]
    pub fn region7(&self) -> REGION7_R {
        REGION7_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 8 - Enable protection for region 8. Write '0' has no effect."]
    #[inline(always)]
    pub fn region8(&self) -> REGION8_R {
        REGION8_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 9 - Enable protection for region 9. Write '0' has no effect."]
    #[inline(always)]
    pub fn region9(&self) -> REGION9_R {
        REGION9_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 10 - Enable protection for region 10. Write '0' has no effect."]
    #[inline(always)]
    pub fn region10(&self) -> REGION10_R {
        REGION10_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    #[doc = "Bit 11 - Enable protection for region 11. Write '0' has no effect."]
    #[inline(always)]
    pub fn region11(&self) -> REGION11_R {
        REGION11_R::new(((self.bits >> 11) & 0x01) != 0)
    }
    #[doc = "Bit 12 - Enable protection for region 12. Write '0' has no effect."]
    #[inline(always)]
    pub fn region12(&self) -> REGION12_R {
        REGION12_R::new(((self.bits >> 12) & 0x01) != 0)
    }
    #[doc = "Bit 13 - Enable protection for region 13. Write '0' has no effect."]
    #[inline(always)]
    pub fn region13(&self) -> REGION13_R {
        REGION13_R::new(((self.bits >> 13) & 0x01) != 0)
    }
    #[doc = "Bit 14 - Enable protection for region 14. Write '0' has no effect."]
    #[inline(always)]
    pub fn region14(&self) -> REGION14_R {
        REGION14_R::new(((self.bits >> 14) & 0x01) != 0)
    }
    #[doc = "Bit 15 - Enable protection for region 15. Write '0' has no effect."]
    #[inline(always)]
    pub fn region15(&self) -> REGION15_R {
        REGION15_R::new(((self.bits >> 15) & 0x01) != 0)
    }
    #[doc = "Bit 16 - Enable protection for region 16. Write '0' has no effect."]
    #[inline(always)]
    pub fn region16(&self) -> REGION16_R {
        REGION16_R::new(((self.bits >> 16) & 0x01) != 0)
    }
    #[doc = "Bit 17 - Enable protection for region 17. Write '0' has no effect."]
    #[inline(always)]
    pub fn region17(&self) -> REGION17_R {
        REGION17_R::new(((self.bits >> 17) & 0x01) != 0)
    }
    #[doc = "Bit 18 - Enable protection for region 18. Write '0' has no effect."]
    #[inline(always)]
    pub fn region18(&self) -> REGION18_R {
        REGION18_R::new(((self.bits >> 18) & 0x01) != 0)
    }
    #[doc = "Bit 19 - Enable protection for region 19. Write '0' has no effect."]
    #[inline(always)]
    pub fn region19(&self) -> REGION19_R {
        REGION19_R::new(((self.bits >> 19) & 0x01) != 0)
    }
    #[doc = "Bit 20 - Enable protection for region 20. Write '0' has no effect."]
    #[inline(always)]
    pub fn region20(&self) -> REGION20_R {
        REGION20_R::new(((self.bits >> 20) & 0x01) != 0)
    }
    #[doc = "Bit 21 - Enable protection for region 21. Write '0' has no effect."]
    #[inline(always)]
    pub fn region21(&self) -> REGION21_R {
        REGION21_R::new(((self.bits >> 21) & 0x01) != 0)
    }
    #[doc = "Bit 22 - Enable protection for region 22. Write '0' has no effect."]
    #[inline(always)]
    pub fn region22(&self) -> REGION22_R {
        REGION22_R::new(((self.bits >> 22) & 0x01) != 0)
    }
    #[doc = "Bit 23 - Enable protection for region 23. Write '0' has no effect."]
    #[inline(always)]
    pub fn region23(&self) -> REGION23_R {
        REGION23_R::new(((self.bits >> 23) & 0x01) != 0)
    }
    #[doc = "Bit 24 - Enable protection for region 24. Write '0' has no effect."]
    #[inline(always)]
    pub fn region24(&self) -> REGION24_R {
        REGION24_R::new(((self.bits >> 24) & 0x01) != 0)
    }
    #[doc = "Bit 25 - Enable protection for region 25. Write '0' has no effect."]
    #[inline(always)]
    pub fn region25(&self) -> REGION25_R {
        REGION25_R::new(((self.bits >> 25) & 0x01) != 0)
    }
    #[doc = "Bit 26 - Enable protection for region 26. Write '0' has no effect."]
    #[inline(always)]
    pub fn region26(&self) -> REGION26_R {
        REGION26_R::new(((self.bits >> 26) & 0x01) != 0)
    }
    #[doc = "Bit 27 - Enable protection for region 27. Write '0' has no effect."]
    #[inline(always)]
    pub fn region27(&self) -> REGION27_R {
        REGION27_R::new(((self.bits >> 27) & 0x01) != 0)
    }
    #[doc = "Bit 28 - Enable protection for region 28. Write '0' has no effect."]
    #[inline(always)]
    pub fn region28(&self) -> REGION28_R {
        REGION28_R::new(((self.bits >> 28) & 0x01) != 0)
    }
    #[doc = "Bit 29 - Enable protection for region 29. Write '0' has no effect."]
    #[inline(always)]
    pub fn region29(&self) -> REGION29_R {
        REGION29_R::new(((self.bits >> 29) & 0x01) != 0)
    }
    #[doc = "Bit 30 - Enable protection for region 30. Write '0' has no effect."]
    #[inline(always)]
    pub fn region30(&self) -> REGION30_R {
        REGION30_R::new(((self.bits >> 30) & 0x01) != 0)
    }
    #[doc = "Bit 31 - Enable protection for region 31. Write '0' has no effect."]
    #[inline(always)]
    pub fn region31(&self) -> REGION31_R {
        REGION31_R::new(((self.bits >> 31) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Enable protection for region 0. Write '0' has no effect."]
    #[inline(always)]
    pub fn region0(&mut self) -> REGION0_W {
        REGION0_W { w: self }
    }
    #[doc = "Bit 1 - Enable protection for region 1. Write '0' has no effect."]
    #[inline(always)]
    pub fn region1(&mut self) -> REGION1_W {
        REGION1_W { w: self }
    }
    #[doc = "Bit 2 - Enable protection for region 2. Write '0' has no effect."]
    #[inline(always)]
    pub fn region2(&mut self) -> REGION2_W {
        REGION2_W { w: self }
    }
    #[doc = "Bit 3 - Enable protection for region 3. Write '0' has no effect."]
    #[inline(always)]
    pub fn region3(&mut self) -> REGION3_W {
        REGION3_W { w: self }
    }
    #[doc = "Bit 4 - Enable protection for region 4. Write '0' has no effect."]
    #[inline(always)]
    pub fn region4(&mut self) -> REGION4_W {
        REGION4_W { w: self }
    }
    #[doc = "Bit 5 - Enable protection for region 5. Write '0' has no effect."]
    #[inline(always)]
    pub fn region5(&mut self) -> REGION5_W {
        REGION5_W { w: self }
    }
    #[doc = "Bit 6 - Enable protection for region 6. Write '0' has no effect."]
    #[inline(always)]
    pub fn region6(&mut self) -> REGION6_W {
        REGION6_W { w: self }
    }
    #[doc = "Bit 7 - Enable protection for region 7. Write '0' has no effect."]
    #[inline(always)]
    pub fn region7(&mut self) -> REGION7_W {
        REGION7_W { w: self }
    }
    #[doc = "Bit 8 - Enable protection for region 8. Write '0' has no effect."]
    #[inline(always)]
    pub fn region8(&mut self) -> REGION8_W {
        REGION8_W { w: self }
    }
    #[doc = "Bit 9 - Enable protection for region 9. Write '0' has no effect."]
    #[inline(always)]
    pub fn region9(&mut self) -> REGION9_W {
        REGION9_W { w: self }
    }
    #[doc = "Bit 10 - Enable protection for region 10. Write '0' has no effect."]
    #[inline(always)]
    pub fn region10(&mut self) -> REGION10_W {
        REGION10_W { w: self }
    }
    #[doc = "Bit 11 - Enable protection for region 11. Write '0' has no effect."]
    #[inline(always)]
    pub fn region11(&mut self) -> REGION11_W {
        REGION11_W { w: self }
    }
    #[doc = "Bit 12 - Enable protection for region 12. Write '0' has no effect."]
    #[inline(always)]
    pub fn region12(&mut self) -> REGION12_W {
        REGION12_W { w: self }
    }
    #[doc = "Bit 13 - Enable protection for region 13. Write '0' has no effect."]
    #[inline(always)]
    pub fn region13(&mut self) -> REGION13_W {
        REGION13_W { w: self }
    }
    #[doc = "Bit 14 - Enable protection for region 14. Write '0' has no effect."]
    #[inline(always)]
    pub fn region14(&mut self) -> REGION14_W {
        REGION14_W { w: self }
    }
    #[doc = "Bit 15 - Enable protection for region 15. Write '0' has no effect."]
    #[inline(always)]
    pub fn region15(&mut self) -> REGION15_W {
        REGION15_W { w: self }
    }
    #[doc = "Bit 16 - Enable protection for region 16. Write '0' has no effect."]
    #[inline(always)]
    pub fn region16(&mut self) -> REGION16_W {
        REGION16_W { w: self }
    }
    #[doc = "Bit 17 - Enable protection for region 17. Write '0' has no effect."]
    #[inline(always)]
    pub fn region17(&mut self) -> REGION17_W {
        REGION17_W { w: self }
    }
    #[doc = "Bit 18 - Enable protection for region 18. Write '0' has no effect."]
    #[inline(always)]
    pub fn region18(&mut self) -> REGION18_W {
        REGION18_W { w: self }
    }
    #[doc = "Bit 19 - Enable protection for region 19. Write '0' has no effect."]
    #[inline(always)]
    pub fn region19(&mut self) -> REGION19_W {
        REGION19_W { w: self }
    }
    #[doc = "Bit 20 - Enable protection for region 20. Write '0' has no effect."]
    #[inline(always)]
    pub fn region20(&mut self) -> REGION20_W {
        REGION20_W { w: self }
    }
    #[doc = "Bit 21 - Enable protection for region 21. Write '0' has no effect."]
    #[inline(always)]
    pub fn region21(&mut self) -> REGION21_W {
        REGION21_W { w: self }
    }
    #[doc = "Bit 22 - Enable protection for region 22. Write '0' has no effect."]
    #[inline(always)]
    pub fn region22(&mut self) -> REGION22_W {
        REGION22_W { w: self }
    }
    #[doc = "Bit 23 - Enable protection for region 23. Write '0' has no effect."]
    #[inline(always)]
    pub fn region23(&mut self) -> REGION23_W {
        REGION23_W { w: self }
    }
    #[doc = "Bit 24 - Enable protection for region 24. Write '0' has no effect."]
    #[inline(always)]
    pub fn region24(&mut self) -> REGION24_W {
        REGION24_W { w: self }
    }
    #[doc = "Bit 25 - Enable protection for region 25. Write '0' has no effect."]
    #[inline(always)]
    pub fn region25(&mut self) -> REGION25_W {
        REGION25_W { w: self }
    }
    #[doc = "Bit 26 - Enable protection for region 26. Write '0' has no effect."]
    #[inline(always)]
    pub fn region26(&mut self) -> REGION26_W {
        REGION26_W { w: self }
    }
    #[doc = "Bit 27 - Enable protection for region 27. Write '0' has no effect."]
    #[inline(always)]
    pub fn region27(&mut self) -> REGION27_W {
        REGION27_W { w: self }
    }
    #[doc = "Bit 28 - Enable protection for region 28. Write '0' has no effect."]
    #[inline(always)]
    pub fn region28(&mut self) -> REGION28_W {
        REGION28_W { w: self }
    }
    #[doc = "Bit 29 - Enable protection for region 29. Write '0' has no effect."]
    #[inline(always)]
    pub fn region29(&mut self) -> REGION29_W {
        REGION29_W { w: self }
    }
    #[doc = "Bit 30 - Enable protection for region 30. Write '0' has no effect."]
    #[inline(always)]
    pub fn region30(&mut self) -> REGION30_W {
        REGION30_W { w: self }
    }
    #[doc = "Bit 31 - Enable protection for region 31. Write '0' has no effect."]
    #[inline(always)]
    pub fn region31(&mut self) -> REGION31_W {
        REGION31_W { w: self }
    }
}
