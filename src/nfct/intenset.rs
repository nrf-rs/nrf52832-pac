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
#[doc = "Write '1' to Enable interrupt for READY event\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum READY_A {
    #[doc = "0: Read: Disabled"]
    DISABLED = 0,
    #[doc = "1: Read: Enabled"]
    ENABLED = 1,
}
impl From<READY_A> for bool {
    #[inline(always)]
    fn from(variant: READY_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `READY`"]
pub type READY_R = crate::R<bool, READY_A>;
impl READY_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> READY_A {
        match self.bits {
            false => READY_A::DISABLED,
            true => READY_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == READY_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == READY_A::ENABLED
    }
}
#[doc = "Write '1' to Enable interrupt for READY event\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum READY_AW {
    #[doc = "1: Enable"]
    SET = 1,
}
impl From<READY_AW> for bool {
    #[inline(always)]
    fn from(variant: READY_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Write proxy for field `READY`"]
pub struct READY_W<'a> {
    w: &'a mut W,
}
impl<'a> READY_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: READY_AW) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Enable"]
    #[inline(always)]
    pub fn set(self) -> &'a mut W {
        self.variant(READY_AW::SET)
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
#[doc = "Write '1' to Enable interrupt for FIELDDETECTED event\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum FIELDDETECTED_A {
    #[doc = "0: Read: Disabled"]
    DISABLED = 0,
    #[doc = "1: Read: Enabled"]
    ENABLED = 1,
}
impl From<FIELDDETECTED_A> for bool {
    #[inline(always)]
    fn from(variant: FIELDDETECTED_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `FIELDDETECTED`"]
pub type FIELDDETECTED_R = crate::R<bool, FIELDDETECTED_A>;
impl FIELDDETECTED_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> FIELDDETECTED_A {
        match self.bits {
            false => FIELDDETECTED_A::DISABLED,
            true => FIELDDETECTED_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == FIELDDETECTED_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == FIELDDETECTED_A::ENABLED
    }
}
#[doc = "Write '1' to Enable interrupt for FIELDDETECTED event\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum FIELDDETECTED_AW {
    #[doc = "1: Enable"]
    SET = 1,
}
impl From<FIELDDETECTED_AW> for bool {
    #[inline(always)]
    fn from(variant: FIELDDETECTED_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Write proxy for field `FIELDDETECTED`"]
pub struct FIELDDETECTED_W<'a> {
    w: &'a mut W,
}
impl<'a> FIELDDETECTED_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: FIELDDETECTED_AW) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Enable"]
    #[inline(always)]
    pub fn set(self) -> &'a mut W {
        self.variant(FIELDDETECTED_AW::SET)
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
#[doc = "Write '1' to Enable interrupt for FIELDLOST event\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum FIELDLOST_A {
    #[doc = "0: Read: Disabled"]
    DISABLED = 0,
    #[doc = "1: Read: Enabled"]
    ENABLED = 1,
}
impl From<FIELDLOST_A> for bool {
    #[inline(always)]
    fn from(variant: FIELDLOST_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `FIELDLOST`"]
pub type FIELDLOST_R = crate::R<bool, FIELDLOST_A>;
impl FIELDLOST_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> FIELDLOST_A {
        match self.bits {
            false => FIELDLOST_A::DISABLED,
            true => FIELDLOST_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == FIELDLOST_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == FIELDLOST_A::ENABLED
    }
}
#[doc = "Write '1' to Enable interrupt for FIELDLOST event\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum FIELDLOST_AW {
    #[doc = "1: Enable"]
    SET = 1,
}
impl From<FIELDLOST_AW> for bool {
    #[inline(always)]
    fn from(variant: FIELDLOST_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Write proxy for field `FIELDLOST`"]
pub struct FIELDLOST_W<'a> {
    w: &'a mut W,
}
impl<'a> FIELDLOST_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: FIELDLOST_AW) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Enable"]
    #[inline(always)]
    pub fn set(self) -> &'a mut W {
        self.variant(FIELDLOST_AW::SET)
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
#[doc = "Write '1' to Enable interrupt for TXFRAMESTART event\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TXFRAMESTART_A {
    #[doc = "0: Read: Disabled"]
    DISABLED = 0,
    #[doc = "1: Read: Enabled"]
    ENABLED = 1,
}
impl From<TXFRAMESTART_A> for bool {
    #[inline(always)]
    fn from(variant: TXFRAMESTART_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `TXFRAMESTART`"]
pub type TXFRAMESTART_R = crate::R<bool, TXFRAMESTART_A>;
impl TXFRAMESTART_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TXFRAMESTART_A {
        match self.bits {
            false => TXFRAMESTART_A::DISABLED,
            true => TXFRAMESTART_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == TXFRAMESTART_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == TXFRAMESTART_A::ENABLED
    }
}
#[doc = "Write '1' to Enable interrupt for TXFRAMESTART event\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TXFRAMESTART_AW {
    #[doc = "1: Enable"]
    SET = 1,
}
impl From<TXFRAMESTART_AW> for bool {
    #[inline(always)]
    fn from(variant: TXFRAMESTART_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Write proxy for field `TXFRAMESTART`"]
pub struct TXFRAMESTART_W<'a> {
    w: &'a mut W,
}
impl<'a> TXFRAMESTART_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: TXFRAMESTART_AW) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Enable"]
    #[inline(always)]
    pub fn set(self) -> &'a mut W {
        self.variant(TXFRAMESTART_AW::SET)
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
#[doc = "Write '1' to Enable interrupt for TXFRAMEEND event\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TXFRAMEEND_A {
    #[doc = "0: Read: Disabled"]
    DISABLED = 0,
    #[doc = "1: Read: Enabled"]
    ENABLED = 1,
}
impl From<TXFRAMEEND_A> for bool {
    #[inline(always)]
    fn from(variant: TXFRAMEEND_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `TXFRAMEEND`"]
pub type TXFRAMEEND_R = crate::R<bool, TXFRAMEEND_A>;
impl TXFRAMEEND_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TXFRAMEEND_A {
        match self.bits {
            false => TXFRAMEEND_A::DISABLED,
            true => TXFRAMEEND_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == TXFRAMEEND_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == TXFRAMEEND_A::ENABLED
    }
}
#[doc = "Write '1' to Enable interrupt for TXFRAMEEND event\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TXFRAMEEND_AW {
    #[doc = "1: Enable"]
    SET = 1,
}
impl From<TXFRAMEEND_AW> for bool {
    #[inline(always)]
    fn from(variant: TXFRAMEEND_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Write proxy for field `TXFRAMEEND`"]
pub struct TXFRAMEEND_W<'a> {
    w: &'a mut W,
}
impl<'a> TXFRAMEEND_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: TXFRAMEEND_AW) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Enable"]
    #[inline(always)]
    pub fn set(self) -> &'a mut W {
        self.variant(TXFRAMEEND_AW::SET)
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
#[doc = "Write '1' to Enable interrupt for RXFRAMESTART event\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RXFRAMESTART_A {
    #[doc = "0: Read: Disabled"]
    DISABLED = 0,
    #[doc = "1: Read: Enabled"]
    ENABLED = 1,
}
impl From<RXFRAMESTART_A> for bool {
    #[inline(always)]
    fn from(variant: RXFRAMESTART_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `RXFRAMESTART`"]
pub type RXFRAMESTART_R = crate::R<bool, RXFRAMESTART_A>;
impl RXFRAMESTART_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RXFRAMESTART_A {
        match self.bits {
            false => RXFRAMESTART_A::DISABLED,
            true => RXFRAMESTART_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == RXFRAMESTART_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == RXFRAMESTART_A::ENABLED
    }
}
#[doc = "Write '1' to Enable interrupt for RXFRAMESTART event\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RXFRAMESTART_AW {
    #[doc = "1: Enable"]
    SET = 1,
}
impl From<RXFRAMESTART_AW> for bool {
    #[inline(always)]
    fn from(variant: RXFRAMESTART_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Write proxy for field `RXFRAMESTART`"]
pub struct RXFRAMESTART_W<'a> {
    w: &'a mut W,
}
impl<'a> RXFRAMESTART_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: RXFRAMESTART_AW) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Enable"]
    #[inline(always)]
    pub fn set(self) -> &'a mut W {
        self.variant(RXFRAMESTART_AW::SET)
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
#[doc = "Write '1' to Enable interrupt for RXFRAMEEND event\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RXFRAMEEND_A {
    #[doc = "0: Read: Disabled"]
    DISABLED = 0,
    #[doc = "1: Read: Enabled"]
    ENABLED = 1,
}
impl From<RXFRAMEEND_A> for bool {
    #[inline(always)]
    fn from(variant: RXFRAMEEND_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `RXFRAMEEND`"]
pub type RXFRAMEEND_R = crate::R<bool, RXFRAMEEND_A>;
impl RXFRAMEEND_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RXFRAMEEND_A {
        match self.bits {
            false => RXFRAMEEND_A::DISABLED,
            true => RXFRAMEEND_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == RXFRAMEEND_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == RXFRAMEEND_A::ENABLED
    }
}
#[doc = "Write '1' to Enable interrupt for RXFRAMEEND event\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RXFRAMEEND_AW {
    #[doc = "1: Enable"]
    SET = 1,
}
impl From<RXFRAMEEND_AW> for bool {
    #[inline(always)]
    fn from(variant: RXFRAMEEND_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Write proxy for field `RXFRAMEEND`"]
pub struct RXFRAMEEND_W<'a> {
    w: &'a mut W,
}
impl<'a> RXFRAMEEND_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: RXFRAMEEND_AW) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Enable"]
    #[inline(always)]
    pub fn set(self) -> &'a mut W {
        self.variant(RXFRAMEEND_AW::SET)
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
#[doc = "Write '1' to Enable interrupt for ERROR event\n\nValue on reset: 0"]
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
#[doc = "Write '1' to Enable interrupt for ERROR event\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ERROR_AW {
    #[doc = "1: Enable"]
    SET = 1,
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
    #[doc = "Enable"]
    #[inline(always)]
    pub fn set(self) -> &'a mut W {
        self.variant(ERROR_AW::SET)
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
#[doc = "Write '1' to Enable interrupt for RXERROR event\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RXERROR_A {
    #[doc = "0: Read: Disabled"]
    DISABLED = 0,
    #[doc = "1: Read: Enabled"]
    ENABLED = 1,
}
impl From<RXERROR_A> for bool {
    #[inline(always)]
    fn from(variant: RXERROR_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `RXERROR`"]
pub type RXERROR_R = crate::R<bool, RXERROR_A>;
impl RXERROR_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RXERROR_A {
        match self.bits {
            false => RXERROR_A::DISABLED,
            true => RXERROR_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == RXERROR_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == RXERROR_A::ENABLED
    }
}
#[doc = "Write '1' to Enable interrupt for RXERROR event\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RXERROR_AW {
    #[doc = "1: Enable"]
    SET = 1,
}
impl From<RXERROR_AW> for bool {
    #[inline(always)]
    fn from(variant: RXERROR_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Write proxy for field `RXERROR`"]
pub struct RXERROR_W<'a> {
    w: &'a mut W,
}
impl<'a> RXERROR_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: RXERROR_AW) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Enable"]
    #[inline(always)]
    pub fn set(self) -> &'a mut W {
        self.variant(RXERROR_AW::SET)
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
#[doc = "Write '1' to Enable interrupt for ENDRX event\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ENDRX_A {
    #[doc = "0: Read: Disabled"]
    DISABLED = 0,
    #[doc = "1: Read: Enabled"]
    ENABLED = 1,
}
impl From<ENDRX_A> for bool {
    #[inline(always)]
    fn from(variant: ENDRX_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `ENDRX`"]
pub type ENDRX_R = crate::R<bool, ENDRX_A>;
impl ENDRX_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ENDRX_A {
        match self.bits {
            false => ENDRX_A::DISABLED,
            true => ENDRX_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == ENDRX_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == ENDRX_A::ENABLED
    }
}
#[doc = "Write '1' to Enable interrupt for ENDRX event\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ENDRX_AW {
    #[doc = "1: Enable"]
    SET = 1,
}
impl From<ENDRX_AW> for bool {
    #[inline(always)]
    fn from(variant: ENDRX_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Write proxy for field `ENDRX`"]
pub struct ENDRX_W<'a> {
    w: &'a mut W,
}
impl<'a> ENDRX_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: ENDRX_AW) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Enable"]
    #[inline(always)]
    pub fn set(self) -> &'a mut W {
        self.variant(ENDRX_AW::SET)
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
#[doc = "Write '1' to Enable interrupt for ENDTX event\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ENDTX_A {
    #[doc = "0: Read: Disabled"]
    DISABLED = 0,
    #[doc = "1: Read: Enabled"]
    ENABLED = 1,
}
impl From<ENDTX_A> for bool {
    #[inline(always)]
    fn from(variant: ENDTX_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `ENDTX`"]
pub type ENDTX_R = crate::R<bool, ENDTX_A>;
impl ENDTX_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ENDTX_A {
        match self.bits {
            false => ENDTX_A::DISABLED,
            true => ENDTX_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == ENDTX_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == ENDTX_A::ENABLED
    }
}
#[doc = "Write '1' to Enable interrupt for ENDTX event\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ENDTX_AW {
    #[doc = "1: Enable"]
    SET = 1,
}
impl From<ENDTX_AW> for bool {
    #[inline(always)]
    fn from(variant: ENDTX_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Write proxy for field `ENDTX`"]
pub struct ENDTX_W<'a> {
    w: &'a mut W,
}
impl<'a> ENDTX_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: ENDTX_AW) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Enable"]
    #[inline(always)]
    pub fn set(self) -> &'a mut W {
        self.variant(ENDTX_AW::SET)
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
#[doc = "Write '1' to Enable interrupt for AUTOCOLRESSTARTED event\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum AUTOCOLRESSTARTED_A {
    #[doc = "0: Read: Disabled"]
    DISABLED = 0,
    #[doc = "1: Read: Enabled"]
    ENABLED = 1,
}
impl From<AUTOCOLRESSTARTED_A> for bool {
    #[inline(always)]
    fn from(variant: AUTOCOLRESSTARTED_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `AUTOCOLRESSTARTED`"]
pub type AUTOCOLRESSTARTED_R = crate::R<bool, AUTOCOLRESSTARTED_A>;
impl AUTOCOLRESSTARTED_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> AUTOCOLRESSTARTED_A {
        match self.bits {
            false => AUTOCOLRESSTARTED_A::DISABLED,
            true => AUTOCOLRESSTARTED_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == AUTOCOLRESSTARTED_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == AUTOCOLRESSTARTED_A::ENABLED
    }
}
#[doc = "Write '1' to Enable interrupt for AUTOCOLRESSTARTED event\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum AUTOCOLRESSTARTED_AW {
    #[doc = "1: Enable"]
    SET = 1,
}
impl From<AUTOCOLRESSTARTED_AW> for bool {
    #[inline(always)]
    fn from(variant: AUTOCOLRESSTARTED_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Write proxy for field `AUTOCOLRESSTARTED`"]
pub struct AUTOCOLRESSTARTED_W<'a> {
    w: &'a mut W,
}
impl<'a> AUTOCOLRESSTARTED_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: AUTOCOLRESSTARTED_AW) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Enable"]
    #[inline(always)]
    pub fn set(self) -> &'a mut W {
        self.variant(AUTOCOLRESSTARTED_AW::SET)
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
#[doc = "Write '1' to Enable interrupt for COLLISION event\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum COLLISION_A {
    #[doc = "0: Read: Disabled"]
    DISABLED = 0,
    #[doc = "1: Read: Enabled"]
    ENABLED = 1,
}
impl From<COLLISION_A> for bool {
    #[inline(always)]
    fn from(variant: COLLISION_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `COLLISION`"]
pub type COLLISION_R = crate::R<bool, COLLISION_A>;
impl COLLISION_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> COLLISION_A {
        match self.bits {
            false => COLLISION_A::DISABLED,
            true => COLLISION_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == COLLISION_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == COLLISION_A::ENABLED
    }
}
#[doc = "Write '1' to Enable interrupt for COLLISION event\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum COLLISION_AW {
    #[doc = "1: Enable"]
    SET = 1,
}
impl From<COLLISION_AW> for bool {
    #[inline(always)]
    fn from(variant: COLLISION_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Write proxy for field `COLLISION`"]
pub struct COLLISION_W<'a> {
    w: &'a mut W,
}
impl<'a> COLLISION_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: COLLISION_AW) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Enable"]
    #[inline(always)]
    pub fn set(self) -> &'a mut W {
        self.variant(COLLISION_AW::SET)
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
#[doc = "Write '1' to Enable interrupt for SELECTED event\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SELECTED_A {
    #[doc = "0: Read: Disabled"]
    DISABLED = 0,
    #[doc = "1: Read: Enabled"]
    ENABLED = 1,
}
impl From<SELECTED_A> for bool {
    #[inline(always)]
    fn from(variant: SELECTED_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `SELECTED`"]
pub type SELECTED_R = crate::R<bool, SELECTED_A>;
impl SELECTED_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SELECTED_A {
        match self.bits {
            false => SELECTED_A::DISABLED,
            true => SELECTED_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == SELECTED_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == SELECTED_A::ENABLED
    }
}
#[doc = "Write '1' to Enable interrupt for SELECTED event\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SELECTED_AW {
    #[doc = "1: Enable"]
    SET = 1,
}
impl From<SELECTED_AW> for bool {
    #[inline(always)]
    fn from(variant: SELECTED_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Write proxy for field `SELECTED`"]
pub struct SELECTED_W<'a> {
    w: &'a mut W,
}
impl<'a> SELECTED_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SELECTED_AW) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Enable"]
    #[inline(always)]
    pub fn set(self) -> &'a mut W {
        self.variant(SELECTED_AW::SET)
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
#[doc = "Write '1' to Enable interrupt for STARTED event\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum STARTED_A {
    #[doc = "0: Read: Disabled"]
    DISABLED = 0,
    #[doc = "1: Read: Enabled"]
    ENABLED = 1,
}
impl From<STARTED_A> for bool {
    #[inline(always)]
    fn from(variant: STARTED_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `STARTED`"]
pub type STARTED_R = crate::R<bool, STARTED_A>;
impl STARTED_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> STARTED_A {
        match self.bits {
            false => STARTED_A::DISABLED,
            true => STARTED_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == STARTED_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == STARTED_A::ENABLED
    }
}
#[doc = "Write '1' to Enable interrupt for STARTED event\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum STARTED_AW {
    #[doc = "1: Enable"]
    SET = 1,
}
impl From<STARTED_AW> for bool {
    #[inline(always)]
    fn from(variant: STARTED_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Write proxy for field `STARTED`"]
pub struct STARTED_W<'a> {
    w: &'a mut W,
}
impl<'a> STARTED_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: STARTED_AW) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Enable"]
    #[inline(always)]
    pub fn set(self) -> &'a mut W {
        self.variant(STARTED_AW::SET)
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
impl R {
    #[doc = "Bit 0 - Write '1' to Enable interrupt for READY event"]
    #[inline(always)]
    pub fn ready(&self) -> READY_R {
        READY_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - Write '1' to Enable interrupt for FIELDDETECTED event"]
    #[inline(always)]
    pub fn fielddetected(&self) -> FIELDDETECTED_R {
        FIELDDETECTED_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - Write '1' to Enable interrupt for FIELDLOST event"]
    #[inline(always)]
    pub fn fieldlost(&self) -> FIELDLOST_R {
        FIELDLOST_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - Write '1' to Enable interrupt for TXFRAMESTART event"]
    #[inline(always)]
    pub fn txframestart(&self) -> TXFRAMESTART_R {
        TXFRAMESTART_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - Write '1' to Enable interrupt for TXFRAMEEND event"]
    #[inline(always)]
    pub fn txframeend(&self) -> TXFRAMEEND_R {
        TXFRAMEEND_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - Write '1' to Enable interrupt for RXFRAMESTART event"]
    #[inline(always)]
    pub fn rxframestart(&self) -> RXFRAMESTART_R {
        RXFRAMESTART_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - Write '1' to Enable interrupt for RXFRAMEEND event"]
    #[inline(always)]
    pub fn rxframeend(&self) -> RXFRAMEEND_R {
        RXFRAMEEND_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7 - Write '1' to Enable interrupt for ERROR event"]
    #[inline(always)]
    pub fn error(&self) -> ERROR_R {
        ERROR_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 10 - Write '1' to Enable interrupt for RXERROR event"]
    #[inline(always)]
    pub fn rxerror(&self) -> RXERROR_R {
        RXERROR_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    #[doc = "Bit 11 - Write '1' to Enable interrupt for ENDRX event"]
    #[inline(always)]
    pub fn endrx(&self) -> ENDRX_R {
        ENDRX_R::new(((self.bits >> 11) & 0x01) != 0)
    }
    #[doc = "Bit 12 - Write '1' to Enable interrupt for ENDTX event"]
    #[inline(always)]
    pub fn endtx(&self) -> ENDTX_R {
        ENDTX_R::new(((self.bits >> 12) & 0x01) != 0)
    }
    #[doc = "Bit 14 - Write '1' to Enable interrupt for AUTOCOLRESSTARTED event"]
    #[inline(always)]
    pub fn autocolresstarted(&self) -> AUTOCOLRESSTARTED_R {
        AUTOCOLRESSTARTED_R::new(((self.bits >> 14) & 0x01) != 0)
    }
    #[doc = "Bit 18 - Write '1' to Enable interrupt for COLLISION event"]
    #[inline(always)]
    pub fn collision(&self) -> COLLISION_R {
        COLLISION_R::new(((self.bits >> 18) & 0x01) != 0)
    }
    #[doc = "Bit 19 - Write '1' to Enable interrupt for SELECTED event"]
    #[inline(always)]
    pub fn selected(&self) -> SELECTED_R {
        SELECTED_R::new(((self.bits >> 19) & 0x01) != 0)
    }
    #[doc = "Bit 20 - Write '1' to Enable interrupt for STARTED event"]
    #[inline(always)]
    pub fn started(&self) -> STARTED_R {
        STARTED_R::new(((self.bits >> 20) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Write '1' to Enable interrupt for READY event"]
    #[inline(always)]
    pub fn ready(&mut self) -> READY_W {
        READY_W { w: self }
    }
    #[doc = "Bit 1 - Write '1' to Enable interrupt for FIELDDETECTED event"]
    #[inline(always)]
    pub fn fielddetected(&mut self) -> FIELDDETECTED_W {
        FIELDDETECTED_W { w: self }
    }
    #[doc = "Bit 2 - Write '1' to Enable interrupt for FIELDLOST event"]
    #[inline(always)]
    pub fn fieldlost(&mut self) -> FIELDLOST_W {
        FIELDLOST_W { w: self }
    }
    #[doc = "Bit 3 - Write '1' to Enable interrupt for TXFRAMESTART event"]
    #[inline(always)]
    pub fn txframestart(&mut self) -> TXFRAMESTART_W {
        TXFRAMESTART_W { w: self }
    }
    #[doc = "Bit 4 - Write '1' to Enable interrupt for TXFRAMEEND event"]
    #[inline(always)]
    pub fn txframeend(&mut self) -> TXFRAMEEND_W {
        TXFRAMEEND_W { w: self }
    }
    #[doc = "Bit 5 - Write '1' to Enable interrupt for RXFRAMESTART event"]
    #[inline(always)]
    pub fn rxframestart(&mut self) -> RXFRAMESTART_W {
        RXFRAMESTART_W { w: self }
    }
    #[doc = "Bit 6 - Write '1' to Enable interrupt for RXFRAMEEND event"]
    #[inline(always)]
    pub fn rxframeend(&mut self) -> RXFRAMEEND_W {
        RXFRAMEEND_W { w: self }
    }
    #[doc = "Bit 7 - Write '1' to Enable interrupt for ERROR event"]
    #[inline(always)]
    pub fn error(&mut self) -> ERROR_W {
        ERROR_W { w: self }
    }
    #[doc = "Bit 10 - Write '1' to Enable interrupt for RXERROR event"]
    #[inline(always)]
    pub fn rxerror(&mut self) -> RXERROR_W {
        RXERROR_W { w: self }
    }
    #[doc = "Bit 11 - Write '1' to Enable interrupt for ENDRX event"]
    #[inline(always)]
    pub fn endrx(&mut self) -> ENDRX_W {
        ENDRX_W { w: self }
    }
    #[doc = "Bit 12 - Write '1' to Enable interrupt for ENDTX event"]
    #[inline(always)]
    pub fn endtx(&mut self) -> ENDTX_W {
        ENDTX_W { w: self }
    }
    #[doc = "Bit 14 - Write '1' to Enable interrupt for AUTOCOLRESSTARTED event"]
    #[inline(always)]
    pub fn autocolresstarted(&mut self) -> AUTOCOLRESSTARTED_W {
        AUTOCOLRESSTARTED_W { w: self }
    }
    #[doc = "Bit 18 - Write '1' to Enable interrupt for COLLISION event"]
    #[inline(always)]
    pub fn collision(&mut self) -> COLLISION_W {
        COLLISION_W { w: self }
    }
    #[doc = "Bit 19 - Write '1' to Enable interrupt for SELECTED event"]
    #[inline(always)]
    pub fn selected(&mut self) -> SELECTED_W {
        SELECTED_W { w: self }
    }
    #[doc = "Bit 20 - Write '1' to Enable interrupt for STARTED event"]
    #[inline(always)]
    pub fn started(&mut self) -> STARTED_W {
        STARTED_W { w: self }
    }
}
