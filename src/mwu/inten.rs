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
#[doc = "Enable or disable interrupt for REGION\\[0\\].WA event\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum REGION0WA_A {
    #[doc = "0: Disable"]
    DISABLED = 0,
    #[doc = "1: Enable"]
    ENABLED = 1,
}
impl From<REGION0WA_A> for bool {
    #[inline(always)]
    fn from(variant: REGION0WA_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `REGION0WA`"]
pub type REGION0WA_R = crate::R<bool, REGION0WA_A>;
impl REGION0WA_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> REGION0WA_A {
        match self.bits {
            false => REGION0WA_A::DISABLED,
            true => REGION0WA_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == REGION0WA_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == REGION0WA_A::ENABLED
    }
}
#[doc = "Write proxy for field `REGION0WA`"]
pub struct REGION0WA_W<'a> {
    w: &'a mut W,
}
impl<'a> REGION0WA_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: REGION0WA_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Disable"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(REGION0WA_A::DISABLED)
    }
    #[doc = "Enable"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(REGION0WA_A::ENABLED)
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
#[doc = "Enable or disable interrupt for REGION\\[0\\].RA event\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum REGION0RA_A {
    #[doc = "0: Disable"]
    DISABLED = 0,
    #[doc = "1: Enable"]
    ENABLED = 1,
}
impl From<REGION0RA_A> for bool {
    #[inline(always)]
    fn from(variant: REGION0RA_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `REGION0RA`"]
pub type REGION0RA_R = crate::R<bool, REGION0RA_A>;
impl REGION0RA_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> REGION0RA_A {
        match self.bits {
            false => REGION0RA_A::DISABLED,
            true => REGION0RA_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == REGION0RA_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == REGION0RA_A::ENABLED
    }
}
#[doc = "Write proxy for field `REGION0RA`"]
pub struct REGION0RA_W<'a> {
    w: &'a mut W,
}
impl<'a> REGION0RA_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: REGION0RA_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Disable"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(REGION0RA_A::DISABLED)
    }
    #[doc = "Enable"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(REGION0RA_A::ENABLED)
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
#[doc = "Enable or disable interrupt for REGION\\[1\\].WA event\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum REGION1WA_A {
    #[doc = "0: Disable"]
    DISABLED = 0,
    #[doc = "1: Enable"]
    ENABLED = 1,
}
impl From<REGION1WA_A> for bool {
    #[inline(always)]
    fn from(variant: REGION1WA_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `REGION1WA`"]
pub type REGION1WA_R = crate::R<bool, REGION1WA_A>;
impl REGION1WA_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> REGION1WA_A {
        match self.bits {
            false => REGION1WA_A::DISABLED,
            true => REGION1WA_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == REGION1WA_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == REGION1WA_A::ENABLED
    }
}
#[doc = "Write proxy for field `REGION1WA`"]
pub struct REGION1WA_W<'a> {
    w: &'a mut W,
}
impl<'a> REGION1WA_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: REGION1WA_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Disable"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(REGION1WA_A::DISABLED)
    }
    #[doc = "Enable"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(REGION1WA_A::ENABLED)
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
#[doc = "Enable or disable interrupt for REGION\\[1\\].RA event\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum REGION1RA_A {
    #[doc = "0: Disable"]
    DISABLED = 0,
    #[doc = "1: Enable"]
    ENABLED = 1,
}
impl From<REGION1RA_A> for bool {
    #[inline(always)]
    fn from(variant: REGION1RA_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `REGION1RA`"]
pub type REGION1RA_R = crate::R<bool, REGION1RA_A>;
impl REGION1RA_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> REGION1RA_A {
        match self.bits {
            false => REGION1RA_A::DISABLED,
            true => REGION1RA_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == REGION1RA_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == REGION1RA_A::ENABLED
    }
}
#[doc = "Write proxy for field `REGION1RA`"]
pub struct REGION1RA_W<'a> {
    w: &'a mut W,
}
impl<'a> REGION1RA_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: REGION1RA_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Disable"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(REGION1RA_A::DISABLED)
    }
    #[doc = "Enable"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(REGION1RA_A::ENABLED)
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
#[doc = "Enable or disable interrupt for REGION\\[2\\].WA event\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum REGION2WA_A {
    #[doc = "0: Disable"]
    DISABLED = 0,
    #[doc = "1: Enable"]
    ENABLED = 1,
}
impl From<REGION2WA_A> for bool {
    #[inline(always)]
    fn from(variant: REGION2WA_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `REGION2WA`"]
pub type REGION2WA_R = crate::R<bool, REGION2WA_A>;
impl REGION2WA_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> REGION2WA_A {
        match self.bits {
            false => REGION2WA_A::DISABLED,
            true => REGION2WA_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == REGION2WA_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == REGION2WA_A::ENABLED
    }
}
#[doc = "Write proxy for field `REGION2WA`"]
pub struct REGION2WA_W<'a> {
    w: &'a mut W,
}
impl<'a> REGION2WA_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: REGION2WA_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Disable"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(REGION2WA_A::DISABLED)
    }
    #[doc = "Enable"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(REGION2WA_A::ENABLED)
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
#[doc = "Enable or disable interrupt for REGION\\[2\\].RA event\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum REGION2RA_A {
    #[doc = "0: Disable"]
    DISABLED = 0,
    #[doc = "1: Enable"]
    ENABLED = 1,
}
impl From<REGION2RA_A> for bool {
    #[inline(always)]
    fn from(variant: REGION2RA_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `REGION2RA`"]
pub type REGION2RA_R = crate::R<bool, REGION2RA_A>;
impl REGION2RA_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> REGION2RA_A {
        match self.bits {
            false => REGION2RA_A::DISABLED,
            true => REGION2RA_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == REGION2RA_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == REGION2RA_A::ENABLED
    }
}
#[doc = "Write proxy for field `REGION2RA`"]
pub struct REGION2RA_W<'a> {
    w: &'a mut W,
}
impl<'a> REGION2RA_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: REGION2RA_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Disable"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(REGION2RA_A::DISABLED)
    }
    #[doc = "Enable"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(REGION2RA_A::ENABLED)
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
#[doc = "Enable or disable interrupt for REGION\\[3\\].WA event\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum REGION3WA_A {
    #[doc = "0: Disable"]
    DISABLED = 0,
    #[doc = "1: Enable"]
    ENABLED = 1,
}
impl From<REGION3WA_A> for bool {
    #[inline(always)]
    fn from(variant: REGION3WA_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `REGION3WA`"]
pub type REGION3WA_R = crate::R<bool, REGION3WA_A>;
impl REGION3WA_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> REGION3WA_A {
        match self.bits {
            false => REGION3WA_A::DISABLED,
            true => REGION3WA_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == REGION3WA_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == REGION3WA_A::ENABLED
    }
}
#[doc = "Write proxy for field `REGION3WA`"]
pub struct REGION3WA_W<'a> {
    w: &'a mut W,
}
impl<'a> REGION3WA_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: REGION3WA_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Disable"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(REGION3WA_A::DISABLED)
    }
    #[doc = "Enable"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(REGION3WA_A::ENABLED)
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
#[doc = "Enable or disable interrupt for REGION\\[3\\].RA event\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum REGION3RA_A {
    #[doc = "0: Disable"]
    DISABLED = 0,
    #[doc = "1: Enable"]
    ENABLED = 1,
}
impl From<REGION3RA_A> for bool {
    #[inline(always)]
    fn from(variant: REGION3RA_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `REGION3RA`"]
pub type REGION3RA_R = crate::R<bool, REGION3RA_A>;
impl REGION3RA_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> REGION3RA_A {
        match self.bits {
            false => REGION3RA_A::DISABLED,
            true => REGION3RA_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == REGION3RA_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == REGION3RA_A::ENABLED
    }
}
#[doc = "Write proxy for field `REGION3RA`"]
pub struct REGION3RA_W<'a> {
    w: &'a mut W,
}
impl<'a> REGION3RA_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: REGION3RA_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Disable"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(REGION3RA_A::DISABLED)
    }
    #[doc = "Enable"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(REGION3RA_A::ENABLED)
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
#[doc = "Enable or disable interrupt for PREGION\\[0\\].WA event\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PREGION0WA_A {
    #[doc = "0: Disable"]
    DISABLED = 0,
    #[doc = "1: Enable"]
    ENABLED = 1,
}
impl From<PREGION0WA_A> for bool {
    #[inline(always)]
    fn from(variant: PREGION0WA_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `PREGION0WA`"]
pub type PREGION0WA_R = crate::R<bool, PREGION0WA_A>;
impl PREGION0WA_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PREGION0WA_A {
        match self.bits {
            false => PREGION0WA_A::DISABLED,
            true => PREGION0WA_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == PREGION0WA_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == PREGION0WA_A::ENABLED
    }
}
#[doc = "Write proxy for field `PREGION0WA`"]
pub struct PREGION0WA_W<'a> {
    w: &'a mut W,
}
impl<'a> PREGION0WA_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PREGION0WA_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Disable"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(PREGION0WA_A::DISABLED)
    }
    #[doc = "Enable"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(PREGION0WA_A::ENABLED)
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
#[doc = "Enable or disable interrupt for PREGION\\[0\\].RA event\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PREGION0RA_A {
    #[doc = "0: Disable"]
    DISABLED = 0,
    #[doc = "1: Enable"]
    ENABLED = 1,
}
impl From<PREGION0RA_A> for bool {
    #[inline(always)]
    fn from(variant: PREGION0RA_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `PREGION0RA`"]
pub type PREGION0RA_R = crate::R<bool, PREGION0RA_A>;
impl PREGION0RA_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PREGION0RA_A {
        match self.bits {
            false => PREGION0RA_A::DISABLED,
            true => PREGION0RA_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == PREGION0RA_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == PREGION0RA_A::ENABLED
    }
}
#[doc = "Write proxy for field `PREGION0RA`"]
pub struct PREGION0RA_W<'a> {
    w: &'a mut W,
}
impl<'a> PREGION0RA_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PREGION0RA_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Disable"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(PREGION0RA_A::DISABLED)
    }
    #[doc = "Enable"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(PREGION0RA_A::ENABLED)
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
#[doc = "Enable or disable interrupt for PREGION\\[1\\].WA event\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PREGION1WA_A {
    #[doc = "0: Disable"]
    DISABLED = 0,
    #[doc = "1: Enable"]
    ENABLED = 1,
}
impl From<PREGION1WA_A> for bool {
    #[inline(always)]
    fn from(variant: PREGION1WA_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `PREGION1WA`"]
pub type PREGION1WA_R = crate::R<bool, PREGION1WA_A>;
impl PREGION1WA_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PREGION1WA_A {
        match self.bits {
            false => PREGION1WA_A::DISABLED,
            true => PREGION1WA_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == PREGION1WA_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == PREGION1WA_A::ENABLED
    }
}
#[doc = "Write proxy for field `PREGION1WA`"]
pub struct PREGION1WA_W<'a> {
    w: &'a mut W,
}
impl<'a> PREGION1WA_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PREGION1WA_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Disable"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(PREGION1WA_A::DISABLED)
    }
    #[doc = "Enable"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(PREGION1WA_A::ENABLED)
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
#[doc = "Enable or disable interrupt for PREGION\\[1\\].RA event\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PREGION1RA_A {
    #[doc = "0: Disable"]
    DISABLED = 0,
    #[doc = "1: Enable"]
    ENABLED = 1,
}
impl From<PREGION1RA_A> for bool {
    #[inline(always)]
    fn from(variant: PREGION1RA_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `PREGION1RA`"]
pub type PREGION1RA_R = crate::R<bool, PREGION1RA_A>;
impl PREGION1RA_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PREGION1RA_A {
        match self.bits {
            false => PREGION1RA_A::DISABLED,
            true => PREGION1RA_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == PREGION1RA_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == PREGION1RA_A::ENABLED
    }
}
#[doc = "Write proxy for field `PREGION1RA`"]
pub struct PREGION1RA_W<'a> {
    w: &'a mut W,
}
impl<'a> PREGION1RA_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PREGION1RA_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Disable"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(PREGION1RA_A::DISABLED)
    }
    #[doc = "Enable"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(PREGION1RA_A::ENABLED)
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
impl R {
    #[doc = "Bit 0 - Enable or disable interrupt for REGION\\[0\\].WA event"]
    #[inline(always)]
    pub fn region0wa(&self) -> REGION0WA_R {
        REGION0WA_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - Enable or disable interrupt for REGION\\[0\\].RA event"]
    #[inline(always)]
    pub fn region0ra(&self) -> REGION0RA_R {
        REGION0RA_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - Enable or disable interrupt for REGION\\[1\\].WA event"]
    #[inline(always)]
    pub fn region1wa(&self) -> REGION1WA_R {
        REGION1WA_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - Enable or disable interrupt for REGION\\[1\\].RA event"]
    #[inline(always)]
    pub fn region1ra(&self) -> REGION1RA_R {
        REGION1RA_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - Enable or disable interrupt for REGION\\[2\\].WA event"]
    #[inline(always)]
    pub fn region2wa(&self) -> REGION2WA_R {
        REGION2WA_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - Enable or disable interrupt for REGION\\[2\\].RA event"]
    #[inline(always)]
    pub fn region2ra(&self) -> REGION2RA_R {
        REGION2RA_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - Enable or disable interrupt for REGION\\[3\\].WA event"]
    #[inline(always)]
    pub fn region3wa(&self) -> REGION3WA_R {
        REGION3WA_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7 - Enable or disable interrupt for REGION\\[3\\].RA event"]
    #[inline(always)]
    pub fn region3ra(&self) -> REGION3RA_R {
        REGION3RA_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 24 - Enable or disable interrupt for PREGION\\[0\\].WA event"]
    #[inline(always)]
    pub fn pregion0wa(&self) -> PREGION0WA_R {
        PREGION0WA_R::new(((self.bits >> 24) & 0x01) != 0)
    }
    #[doc = "Bit 25 - Enable or disable interrupt for PREGION\\[0\\].RA event"]
    #[inline(always)]
    pub fn pregion0ra(&self) -> PREGION0RA_R {
        PREGION0RA_R::new(((self.bits >> 25) & 0x01) != 0)
    }
    #[doc = "Bit 26 - Enable or disable interrupt for PREGION\\[1\\].WA event"]
    #[inline(always)]
    pub fn pregion1wa(&self) -> PREGION1WA_R {
        PREGION1WA_R::new(((self.bits >> 26) & 0x01) != 0)
    }
    #[doc = "Bit 27 - Enable or disable interrupt for PREGION\\[1\\].RA event"]
    #[inline(always)]
    pub fn pregion1ra(&self) -> PREGION1RA_R {
        PREGION1RA_R::new(((self.bits >> 27) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Enable or disable interrupt for REGION\\[0\\].WA event"]
    #[inline(always)]
    pub fn region0wa(&mut self) -> REGION0WA_W {
        REGION0WA_W { w: self }
    }
    #[doc = "Bit 1 - Enable or disable interrupt for REGION\\[0\\].RA event"]
    #[inline(always)]
    pub fn region0ra(&mut self) -> REGION0RA_W {
        REGION0RA_W { w: self }
    }
    #[doc = "Bit 2 - Enable or disable interrupt for REGION\\[1\\].WA event"]
    #[inline(always)]
    pub fn region1wa(&mut self) -> REGION1WA_W {
        REGION1WA_W { w: self }
    }
    #[doc = "Bit 3 - Enable or disable interrupt for REGION\\[1\\].RA event"]
    #[inline(always)]
    pub fn region1ra(&mut self) -> REGION1RA_W {
        REGION1RA_W { w: self }
    }
    #[doc = "Bit 4 - Enable or disable interrupt for REGION\\[2\\].WA event"]
    #[inline(always)]
    pub fn region2wa(&mut self) -> REGION2WA_W {
        REGION2WA_W { w: self }
    }
    #[doc = "Bit 5 - Enable or disable interrupt for REGION\\[2\\].RA event"]
    #[inline(always)]
    pub fn region2ra(&mut self) -> REGION2RA_W {
        REGION2RA_W { w: self }
    }
    #[doc = "Bit 6 - Enable or disable interrupt for REGION\\[3\\].WA event"]
    #[inline(always)]
    pub fn region3wa(&mut self) -> REGION3WA_W {
        REGION3WA_W { w: self }
    }
    #[doc = "Bit 7 - Enable or disable interrupt for REGION\\[3\\].RA event"]
    #[inline(always)]
    pub fn region3ra(&mut self) -> REGION3RA_W {
        REGION3RA_W { w: self }
    }
    #[doc = "Bit 24 - Enable or disable interrupt for PREGION\\[0\\].WA event"]
    #[inline(always)]
    pub fn pregion0wa(&mut self) -> PREGION0WA_W {
        PREGION0WA_W { w: self }
    }
    #[doc = "Bit 25 - Enable or disable interrupt for PREGION\\[0\\].RA event"]
    #[inline(always)]
    pub fn pregion0ra(&mut self) -> PREGION0RA_W {
        PREGION0RA_W { w: self }
    }
    #[doc = "Bit 26 - Enable or disable interrupt for PREGION\\[1\\].WA event"]
    #[inline(always)]
    pub fn pregion1wa(&mut self) -> PREGION1WA_W {
        PREGION1WA_W { w: self }
    }
    #[doc = "Bit 27 - Enable or disable interrupt for PREGION\\[1\\].RA event"]
    #[inline(always)]
    pub fn pregion1ra(&mut self) -> PREGION1RA_W {
        PREGION1RA_W { w: self }
    }
}
