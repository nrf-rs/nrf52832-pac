#[doc = "Reader of register INTEN"]
pub type R = crate::R<u32, super::INTEN>;
#[doc = "Writer for register INTEN"]
pub type W = crate::W<u32, super::INTEN>;
#[doc = "Register INTEN `reset()`'s with value 0"]
impl crate::ResetValue for super::INTEN {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Enable or disable interrupt for TRIGGERED\\[0\\]
event\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TRIGGERED0_A {
    #[doc = "0: Disable"]
    DISABLED = 0,
    #[doc = "1: Enable"]
    ENABLED = 1,
}
impl From<TRIGGERED0_A> for bool {
    #[inline(always)]
    fn from(variant: TRIGGERED0_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `TRIGGERED0`"]
pub type TRIGGERED0_R = crate::R<bool, TRIGGERED0_A>;
impl TRIGGERED0_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TRIGGERED0_A {
        match self.bits {
            false => TRIGGERED0_A::DISABLED,
            true => TRIGGERED0_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == TRIGGERED0_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == TRIGGERED0_A::ENABLED
    }
}
#[doc = "Write proxy for field `TRIGGERED0`"]
pub struct TRIGGERED0_W<'a> {
    w: &'a mut W,
}
impl<'a> TRIGGERED0_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: TRIGGERED0_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Disable"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(TRIGGERED0_A::DISABLED)
    }
    #[doc = "Enable"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(TRIGGERED0_A::ENABLED)
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
#[doc = "Enable or disable interrupt for TRIGGERED\\[1\\]
event\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TRIGGERED1_A {
    #[doc = "0: Disable"]
    DISABLED = 0,
    #[doc = "1: Enable"]
    ENABLED = 1,
}
impl From<TRIGGERED1_A> for bool {
    #[inline(always)]
    fn from(variant: TRIGGERED1_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `TRIGGERED1`"]
pub type TRIGGERED1_R = crate::R<bool, TRIGGERED1_A>;
impl TRIGGERED1_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TRIGGERED1_A {
        match self.bits {
            false => TRIGGERED1_A::DISABLED,
            true => TRIGGERED1_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == TRIGGERED1_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == TRIGGERED1_A::ENABLED
    }
}
#[doc = "Write proxy for field `TRIGGERED1`"]
pub struct TRIGGERED1_W<'a> {
    w: &'a mut W,
}
impl<'a> TRIGGERED1_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: TRIGGERED1_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Disable"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(TRIGGERED1_A::DISABLED)
    }
    #[doc = "Enable"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(TRIGGERED1_A::ENABLED)
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
#[doc = "Enable or disable interrupt for TRIGGERED\\[2\\]
event\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TRIGGERED2_A {
    #[doc = "0: Disable"]
    DISABLED = 0,
    #[doc = "1: Enable"]
    ENABLED = 1,
}
impl From<TRIGGERED2_A> for bool {
    #[inline(always)]
    fn from(variant: TRIGGERED2_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `TRIGGERED2`"]
pub type TRIGGERED2_R = crate::R<bool, TRIGGERED2_A>;
impl TRIGGERED2_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TRIGGERED2_A {
        match self.bits {
            false => TRIGGERED2_A::DISABLED,
            true => TRIGGERED2_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == TRIGGERED2_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == TRIGGERED2_A::ENABLED
    }
}
#[doc = "Write proxy for field `TRIGGERED2`"]
pub struct TRIGGERED2_W<'a> {
    w: &'a mut W,
}
impl<'a> TRIGGERED2_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: TRIGGERED2_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Disable"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(TRIGGERED2_A::DISABLED)
    }
    #[doc = "Enable"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(TRIGGERED2_A::ENABLED)
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
#[doc = "Enable or disable interrupt for TRIGGERED\\[3\\]
event\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TRIGGERED3_A {
    #[doc = "0: Disable"]
    DISABLED = 0,
    #[doc = "1: Enable"]
    ENABLED = 1,
}
impl From<TRIGGERED3_A> for bool {
    #[inline(always)]
    fn from(variant: TRIGGERED3_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `TRIGGERED3`"]
pub type TRIGGERED3_R = crate::R<bool, TRIGGERED3_A>;
impl TRIGGERED3_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TRIGGERED3_A {
        match self.bits {
            false => TRIGGERED3_A::DISABLED,
            true => TRIGGERED3_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == TRIGGERED3_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == TRIGGERED3_A::ENABLED
    }
}
#[doc = "Write proxy for field `TRIGGERED3`"]
pub struct TRIGGERED3_W<'a> {
    w: &'a mut W,
}
impl<'a> TRIGGERED3_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: TRIGGERED3_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Disable"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(TRIGGERED3_A::DISABLED)
    }
    #[doc = "Enable"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(TRIGGERED3_A::ENABLED)
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
#[doc = "Enable or disable interrupt for TRIGGERED\\[4\\]
event\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TRIGGERED4_A {
    #[doc = "0: Disable"]
    DISABLED = 0,
    #[doc = "1: Enable"]
    ENABLED = 1,
}
impl From<TRIGGERED4_A> for bool {
    #[inline(always)]
    fn from(variant: TRIGGERED4_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `TRIGGERED4`"]
pub type TRIGGERED4_R = crate::R<bool, TRIGGERED4_A>;
impl TRIGGERED4_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TRIGGERED4_A {
        match self.bits {
            false => TRIGGERED4_A::DISABLED,
            true => TRIGGERED4_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == TRIGGERED4_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == TRIGGERED4_A::ENABLED
    }
}
#[doc = "Write proxy for field `TRIGGERED4`"]
pub struct TRIGGERED4_W<'a> {
    w: &'a mut W,
}
impl<'a> TRIGGERED4_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: TRIGGERED4_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Disable"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(TRIGGERED4_A::DISABLED)
    }
    #[doc = "Enable"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(TRIGGERED4_A::ENABLED)
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
#[doc = "Enable or disable interrupt for TRIGGERED\\[5\\]
event\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TRIGGERED5_A {
    #[doc = "0: Disable"]
    DISABLED = 0,
    #[doc = "1: Enable"]
    ENABLED = 1,
}
impl From<TRIGGERED5_A> for bool {
    #[inline(always)]
    fn from(variant: TRIGGERED5_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `TRIGGERED5`"]
pub type TRIGGERED5_R = crate::R<bool, TRIGGERED5_A>;
impl TRIGGERED5_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TRIGGERED5_A {
        match self.bits {
            false => TRIGGERED5_A::DISABLED,
            true => TRIGGERED5_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == TRIGGERED5_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == TRIGGERED5_A::ENABLED
    }
}
#[doc = "Write proxy for field `TRIGGERED5`"]
pub struct TRIGGERED5_W<'a> {
    w: &'a mut W,
}
impl<'a> TRIGGERED5_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: TRIGGERED5_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Disable"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(TRIGGERED5_A::DISABLED)
    }
    #[doc = "Enable"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(TRIGGERED5_A::ENABLED)
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
#[doc = "Enable or disable interrupt for TRIGGERED\\[6\\]
event\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TRIGGERED6_A {
    #[doc = "0: Disable"]
    DISABLED = 0,
    #[doc = "1: Enable"]
    ENABLED = 1,
}
impl From<TRIGGERED6_A> for bool {
    #[inline(always)]
    fn from(variant: TRIGGERED6_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `TRIGGERED6`"]
pub type TRIGGERED6_R = crate::R<bool, TRIGGERED6_A>;
impl TRIGGERED6_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TRIGGERED6_A {
        match self.bits {
            false => TRIGGERED6_A::DISABLED,
            true => TRIGGERED6_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == TRIGGERED6_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == TRIGGERED6_A::ENABLED
    }
}
#[doc = "Write proxy for field `TRIGGERED6`"]
pub struct TRIGGERED6_W<'a> {
    w: &'a mut W,
}
impl<'a> TRIGGERED6_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: TRIGGERED6_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Disable"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(TRIGGERED6_A::DISABLED)
    }
    #[doc = "Enable"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(TRIGGERED6_A::ENABLED)
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
#[doc = "Enable or disable interrupt for TRIGGERED\\[7\\]
event\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TRIGGERED7_A {
    #[doc = "0: Disable"]
    DISABLED = 0,
    #[doc = "1: Enable"]
    ENABLED = 1,
}
impl From<TRIGGERED7_A> for bool {
    #[inline(always)]
    fn from(variant: TRIGGERED7_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `TRIGGERED7`"]
pub type TRIGGERED7_R = crate::R<bool, TRIGGERED7_A>;
impl TRIGGERED7_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TRIGGERED7_A {
        match self.bits {
            false => TRIGGERED7_A::DISABLED,
            true => TRIGGERED7_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == TRIGGERED7_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == TRIGGERED7_A::ENABLED
    }
}
#[doc = "Write proxy for field `TRIGGERED7`"]
pub struct TRIGGERED7_W<'a> {
    w: &'a mut W,
}
impl<'a> TRIGGERED7_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: TRIGGERED7_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Disable"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(TRIGGERED7_A::DISABLED)
    }
    #[doc = "Enable"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(TRIGGERED7_A::ENABLED)
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
#[doc = "Enable or disable interrupt for TRIGGERED\\[8\\]
event\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TRIGGERED8_A {
    #[doc = "0: Disable"]
    DISABLED = 0,
    #[doc = "1: Enable"]
    ENABLED = 1,
}
impl From<TRIGGERED8_A> for bool {
    #[inline(always)]
    fn from(variant: TRIGGERED8_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `TRIGGERED8`"]
pub type TRIGGERED8_R = crate::R<bool, TRIGGERED8_A>;
impl TRIGGERED8_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TRIGGERED8_A {
        match self.bits {
            false => TRIGGERED8_A::DISABLED,
            true => TRIGGERED8_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == TRIGGERED8_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == TRIGGERED8_A::ENABLED
    }
}
#[doc = "Write proxy for field `TRIGGERED8`"]
pub struct TRIGGERED8_W<'a> {
    w: &'a mut W,
}
impl<'a> TRIGGERED8_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: TRIGGERED8_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Disable"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(TRIGGERED8_A::DISABLED)
    }
    #[doc = "Enable"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(TRIGGERED8_A::ENABLED)
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
#[doc = "Enable or disable interrupt for TRIGGERED\\[9\\]
event\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TRIGGERED9_A {
    #[doc = "0: Disable"]
    DISABLED = 0,
    #[doc = "1: Enable"]
    ENABLED = 1,
}
impl From<TRIGGERED9_A> for bool {
    #[inline(always)]
    fn from(variant: TRIGGERED9_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `TRIGGERED9`"]
pub type TRIGGERED9_R = crate::R<bool, TRIGGERED9_A>;
impl TRIGGERED9_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TRIGGERED9_A {
        match self.bits {
            false => TRIGGERED9_A::DISABLED,
            true => TRIGGERED9_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == TRIGGERED9_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == TRIGGERED9_A::ENABLED
    }
}
#[doc = "Write proxy for field `TRIGGERED9`"]
pub struct TRIGGERED9_W<'a> {
    w: &'a mut W,
}
impl<'a> TRIGGERED9_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: TRIGGERED9_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Disable"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(TRIGGERED9_A::DISABLED)
    }
    #[doc = "Enable"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(TRIGGERED9_A::ENABLED)
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
#[doc = "Enable or disable interrupt for TRIGGERED\\[10\\]
event\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TRIGGERED10_A {
    #[doc = "0: Disable"]
    DISABLED = 0,
    #[doc = "1: Enable"]
    ENABLED = 1,
}
impl From<TRIGGERED10_A> for bool {
    #[inline(always)]
    fn from(variant: TRIGGERED10_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `TRIGGERED10`"]
pub type TRIGGERED10_R = crate::R<bool, TRIGGERED10_A>;
impl TRIGGERED10_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TRIGGERED10_A {
        match self.bits {
            false => TRIGGERED10_A::DISABLED,
            true => TRIGGERED10_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == TRIGGERED10_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == TRIGGERED10_A::ENABLED
    }
}
#[doc = "Write proxy for field `TRIGGERED10`"]
pub struct TRIGGERED10_W<'a> {
    w: &'a mut W,
}
impl<'a> TRIGGERED10_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: TRIGGERED10_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Disable"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(TRIGGERED10_A::DISABLED)
    }
    #[doc = "Enable"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(TRIGGERED10_A::ENABLED)
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
#[doc = "Enable or disable interrupt for TRIGGERED\\[11\\]
event\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TRIGGERED11_A {
    #[doc = "0: Disable"]
    DISABLED = 0,
    #[doc = "1: Enable"]
    ENABLED = 1,
}
impl From<TRIGGERED11_A> for bool {
    #[inline(always)]
    fn from(variant: TRIGGERED11_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `TRIGGERED11`"]
pub type TRIGGERED11_R = crate::R<bool, TRIGGERED11_A>;
impl TRIGGERED11_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TRIGGERED11_A {
        match self.bits {
            false => TRIGGERED11_A::DISABLED,
            true => TRIGGERED11_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == TRIGGERED11_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == TRIGGERED11_A::ENABLED
    }
}
#[doc = "Write proxy for field `TRIGGERED11`"]
pub struct TRIGGERED11_W<'a> {
    w: &'a mut W,
}
impl<'a> TRIGGERED11_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: TRIGGERED11_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Disable"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(TRIGGERED11_A::DISABLED)
    }
    #[doc = "Enable"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(TRIGGERED11_A::ENABLED)
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
#[doc = "Enable or disable interrupt for TRIGGERED\\[12\\]
event\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TRIGGERED12_A {
    #[doc = "0: Disable"]
    DISABLED = 0,
    #[doc = "1: Enable"]
    ENABLED = 1,
}
impl From<TRIGGERED12_A> for bool {
    #[inline(always)]
    fn from(variant: TRIGGERED12_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `TRIGGERED12`"]
pub type TRIGGERED12_R = crate::R<bool, TRIGGERED12_A>;
impl TRIGGERED12_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TRIGGERED12_A {
        match self.bits {
            false => TRIGGERED12_A::DISABLED,
            true => TRIGGERED12_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == TRIGGERED12_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == TRIGGERED12_A::ENABLED
    }
}
#[doc = "Write proxy for field `TRIGGERED12`"]
pub struct TRIGGERED12_W<'a> {
    w: &'a mut W,
}
impl<'a> TRIGGERED12_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: TRIGGERED12_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Disable"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(TRIGGERED12_A::DISABLED)
    }
    #[doc = "Enable"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(TRIGGERED12_A::ENABLED)
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
#[doc = "Enable or disable interrupt for TRIGGERED\\[13\\]
event\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TRIGGERED13_A {
    #[doc = "0: Disable"]
    DISABLED = 0,
    #[doc = "1: Enable"]
    ENABLED = 1,
}
impl From<TRIGGERED13_A> for bool {
    #[inline(always)]
    fn from(variant: TRIGGERED13_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `TRIGGERED13`"]
pub type TRIGGERED13_R = crate::R<bool, TRIGGERED13_A>;
impl TRIGGERED13_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TRIGGERED13_A {
        match self.bits {
            false => TRIGGERED13_A::DISABLED,
            true => TRIGGERED13_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == TRIGGERED13_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == TRIGGERED13_A::ENABLED
    }
}
#[doc = "Write proxy for field `TRIGGERED13`"]
pub struct TRIGGERED13_W<'a> {
    w: &'a mut W,
}
impl<'a> TRIGGERED13_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: TRIGGERED13_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Disable"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(TRIGGERED13_A::DISABLED)
    }
    #[doc = "Enable"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(TRIGGERED13_A::ENABLED)
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
#[doc = "Enable or disable interrupt for TRIGGERED\\[14\\]
event\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TRIGGERED14_A {
    #[doc = "0: Disable"]
    DISABLED = 0,
    #[doc = "1: Enable"]
    ENABLED = 1,
}
impl From<TRIGGERED14_A> for bool {
    #[inline(always)]
    fn from(variant: TRIGGERED14_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `TRIGGERED14`"]
pub type TRIGGERED14_R = crate::R<bool, TRIGGERED14_A>;
impl TRIGGERED14_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TRIGGERED14_A {
        match self.bits {
            false => TRIGGERED14_A::DISABLED,
            true => TRIGGERED14_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == TRIGGERED14_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == TRIGGERED14_A::ENABLED
    }
}
#[doc = "Write proxy for field `TRIGGERED14`"]
pub struct TRIGGERED14_W<'a> {
    w: &'a mut W,
}
impl<'a> TRIGGERED14_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: TRIGGERED14_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Disable"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(TRIGGERED14_A::DISABLED)
    }
    #[doc = "Enable"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(TRIGGERED14_A::ENABLED)
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
#[doc = "Enable or disable interrupt for TRIGGERED\\[15\\]
event\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TRIGGERED15_A {
    #[doc = "0: Disable"]
    DISABLED = 0,
    #[doc = "1: Enable"]
    ENABLED = 1,
}
impl From<TRIGGERED15_A> for bool {
    #[inline(always)]
    fn from(variant: TRIGGERED15_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `TRIGGERED15`"]
pub type TRIGGERED15_R = crate::R<bool, TRIGGERED15_A>;
impl TRIGGERED15_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TRIGGERED15_A {
        match self.bits {
            false => TRIGGERED15_A::DISABLED,
            true => TRIGGERED15_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == TRIGGERED15_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == TRIGGERED15_A::ENABLED
    }
}
#[doc = "Write proxy for field `TRIGGERED15`"]
pub struct TRIGGERED15_W<'a> {
    w: &'a mut W,
}
impl<'a> TRIGGERED15_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: TRIGGERED15_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Disable"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(TRIGGERED15_A::DISABLED)
    }
    #[doc = "Enable"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(TRIGGERED15_A::ENABLED)
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
impl R {
    #[doc = "Bit 0 - Enable or disable interrupt for TRIGGERED\\[0\\]
event"]
    #[inline(always)]
    pub fn triggered0(&self) -> TRIGGERED0_R {
        TRIGGERED0_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - Enable or disable interrupt for TRIGGERED\\[1\\]
event"]
    #[inline(always)]
    pub fn triggered1(&self) -> TRIGGERED1_R {
        TRIGGERED1_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - Enable or disable interrupt for TRIGGERED\\[2\\]
event"]
    #[inline(always)]
    pub fn triggered2(&self) -> TRIGGERED2_R {
        TRIGGERED2_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - Enable or disable interrupt for TRIGGERED\\[3\\]
event"]
    #[inline(always)]
    pub fn triggered3(&self) -> TRIGGERED3_R {
        TRIGGERED3_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - Enable or disable interrupt for TRIGGERED\\[4\\]
event"]
    #[inline(always)]
    pub fn triggered4(&self) -> TRIGGERED4_R {
        TRIGGERED4_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - Enable or disable interrupt for TRIGGERED\\[5\\]
event"]
    #[inline(always)]
    pub fn triggered5(&self) -> TRIGGERED5_R {
        TRIGGERED5_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - Enable or disable interrupt for TRIGGERED\\[6\\]
event"]
    #[inline(always)]
    pub fn triggered6(&self) -> TRIGGERED6_R {
        TRIGGERED6_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7 - Enable or disable interrupt for TRIGGERED\\[7\\]
event"]
    #[inline(always)]
    pub fn triggered7(&self) -> TRIGGERED7_R {
        TRIGGERED7_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 8 - Enable or disable interrupt for TRIGGERED\\[8\\]
event"]
    #[inline(always)]
    pub fn triggered8(&self) -> TRIGGERED8_R {
        TRIGGERED8_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 9 - Enable or disable interrupt for TRIGGERED\\[9\\]
event"]
    #[inline(always)]
    pub fn triggered9(&self) -> TRIGGERED9_R {
        TRIGGERED9_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 10 - Enable or disable interrupt for TRIGGERED\\[10\\]
event"]
    #[inline(always)]
    pub fn triggered10(&self) -> TRIGGERED10_R {
        TRIGGERED10_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    #[doc = "Bit 11 - Enable or disable interrupt for TRIGGERED\\[11\\]
event"]
    #[inline(always)]
    pub fn triggered11(&self) -> TRIGGERED11_R {
        TRIGGERED11_R::new(((self.bits >> 11) & 0x01) != 0)
    }
    #[doc = "Bit 12 - Enable or disable interrupt for TRIGGERED\\[12\\]
event"]
    #[inline(always)]
    pub fn triggered12(&self) -> TRIGGERED12_R {
        TRIGGERED12_R::new(((self.bits >> 12) & 0x01) != 0)
    }
    #[doc = "Bit 13 - Enable or disable interrupt for TRIGGERED\\[13\\]
event"]
    #[inline(always)]
    pub fn triggered13(&self) -> TRIGGERED13_R {
        TRIGGERED13_R::new(((self.bits >> 13) & 0x01) != 0)
    }
    #[doc = "Bit 14 - Enable or disable interrupt for TRIGGERED\\[14\\]
event"]
    #[inline(always)]
    pub fn triggered14(&self) -> TRIGGERED14_R {
        TRIGGERED14_R::new(((self.bits >> 14) & 0x01) != 0)
    }
    #[doc = "Bit 15 - Enable or disable interrupt for TRIGGERED\\[15\\]
event"]
    #[inline(always)]
    pub fn triggered15(&self) -> TRIGGERED15_R {
        TRIGGERED15_R::new(((self.bits >> 15) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Enable or disable interrupt for TRIGGERED\\[0\\]
event"]
    #[inline(always)]
    pub fn triggered0(&mut self) -> TRIGGERED0_W {
        TRIGGERED0_W { w: self }
    }
    #[doc = "Bit 1 - Enable or disable interrupt for TRIGGERED\\[1\\]
event"]
    #[inline(always)]
    pub fn triggered1(&mut self) -> TRIGGERED1_W {
        TRIGGERED1_W { w: self }
    }
    #[doc = "Bit 2 - Enable or disable interrupt for TRIGGERED\\[2\\]
event"]
    #[inline(always)]
    pub fn triggered2(&mut self) -> TRIGGERED2_W {
        TRIGGERED2_W { w: self }
    }
    #[doc = "Bit 3 - Enable or disable interrupt for TRIGGERED\\[3\\]
event"]
    #[inline(always)]
    pub fn triggered3(&mut self) -> TRIGGERED3_W {
        TRIGGERED3_W { w: self }
    }
    #[doc = "Bit 4 - Enable or disable interrupt for TRIGGERED\\[4\\]
event"]
    #[inline(always)]
    pub fn triggered4(&mut self) -> TRIGGERED4_W {
        TRIGGERED4_W { w: self }
    }
    #[doc = "Bit 5 - Enable or disable interrupt for TRIGGERED\\[5\\]
event"]
    #[inline(always)]
    pub fn triggered5(&mut self) -> TRIGGERED5_W {
        TRIGGERED5_W { w: self }
    }
    #[doc = "Bit 6 - Enable or disable interrupt for TRIGGERED\\[6\\]
event"]
    #[inline(always)]
    pub fn triggered6(&mut self) -> TRIGGERED6_W {
        TRIGGERED6_W { w: self }
    }
    #[doc = "Bit 7 - Enable or disable interrupt for TRIGGERED\\[7\\]
event"]
    #[inline(always)]
    pub fn triggered7(&mut self) -> TRIGGERED7_W {
        TRIGGERED7_W { w: self }
    }
    #[doc = "Bit 8 - Enable or disable interrupt for TRIGGERED\\[8\\]
event"]
    #[inline(always)]
    pub fn triggered8(&mut self) -> TRIGGERED8_W {
        TRIGGERED8_W { w: self }
    }
    #[doc = "Bit 9 - Enable or disable interrupt for TRIGGERED\\[9\\]
event"]
    #[inline(always)]
    pub fn triggered9(&mut self) -> TRIGGERED9_W {
        TRIGGERED9_W { w: self }
    }
    #[doc = "Bit 10 - Enable or disable interrupt for TRIGGERED\\[10\\]
event"]
    #[inline(always)]
    pub fn triggered10(&mut self) -> TRIGGERED10_W {
        TRIGGERED10_W { w: self }
    }
    #[doc = "Bit 11 - Enable or disable interrupt for TRIGGERED\\[11\\]
event"]
    #[inline(always)]
    pub fn triggered11(&mut self) -> TRIGGERED11_W {
        TRIGGERED11_W { w: self }
    }
    #[doc = "Bit 12 - Enable or disable interrupt for TRIGGERED\\[12\\]
event"]
    #[inline(always)]
    pub fn triggered12(&mut self) -> TRIGGERED12_W {
        TRIGGERED12_W { w: self }
    }
    #[doc = "Bit 13 - Enable or disable interrupt for TRIGGERED\\[13\\]
event"]
    #[inline(always)]
    pub fn triggered13(&mut self) -> TRIGGERED13_W {
        TRIGGERED13_W { w: self }
    }
    #[doc = "Bit 14 - Enable or disable interrupt for TRIGGERED\\[14\\]
event"]
    #[inline(always)]
    pub fn triggered14(&mut self) -> TRIGGERED14_W {
        TRIGGERED14_W { w: self }
    }
    #[doc = "Bit 15 - Enable or disable interrupt for TRIGGERED\\[15\\]
event"]
    #[inline(always)]
    pub fn triggered15(&mut self) -> TRIGGERED15_W {
        TRIGGERED15_W { w: self }
    }
}
