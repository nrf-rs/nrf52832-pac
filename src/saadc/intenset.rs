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
        self.w.bits = (self.w.bits & !0x01) | ((value as u32) & 0x01);
        self.w
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
        self.w.bits = (self.w.bits & !(0x01 << 1)) | (((value as u32) & 0x01) << 1);
        self.w
    }
}
#[doc = "Write '1' to Enable interrupt for DONE event\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DONE_A {
    #[doc = "0: Read: Disabled"]
    DISABLED = 0,
    #[doc = "1: Read: Enabled"]
    ENABLED = 1,
}
impl From<DONE_A> for bool {
    #[inline(always)]
    fn from(variant: DONE_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `DONE`"]
pub type DONE_R = crate::R<bool, DONE_A>;
impl DONE_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DONE_A {
        match self.bits {
            false => DONE_A::DISABLED,
            true => DONE_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == DONE_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == DONE_A::ENABLED
    }
}
#[doc = "Write '1' to Enable interrupt for DONE event\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DONE_AW {
    #[doc = "1: Enable"]
    SET = 1,
}
impl From<DONE_AW> for bool {
    #[inline(always)]
    fn from(variant: DONE_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Write proxy for field `DONE`"]
pub struct DONE_W<'a> {
    w: &'a mut W,
}
impl<'a> DONE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: DONE_AW) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Enable"]
    #[inline(always)]
    pub fn set(self) -> &'a mut W {
        self.variant(DONE_AW::SET)
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
#[doc = "Write '1' to Enable interrupt for RESULTDONE event\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RESULTDONE_A {
    #[doc = "0: Read: Disabled"]
    DISABLED = 0,
    #[doc = "1: Read: Enabled"]
    ENABLED = 1,
}
impl From<RESULTDONE_A> for bool {
    #[inline(always)]
    fn from(variant: RESULTDONE_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `RESULTDONE`"]
pub type RESULTDONE_R = crate::R<bool, RESULTDONE_A>;
impl RESULTDONE_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RESULTDONE_A {
        match self.bits {
            false => RESULTDONE_A::DISABLED,
            true => RESULTDONE_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == RESULTDONE_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == RESULTDONE_A::ENABLED
    }
}
#[doc = "Write '1' to Enable interrupt for RESULTDONE event\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RESULTDONE_AW {
    #[doc = "1: Enable"]
    SET = 1,
}
impl From<RESULTDONE_AW> for bool {
    #[inline(always)]
    fn from(variant: RESULTDONE_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Write proxy for field `RESULTDONE`"]
pub struct RESULTDONE_W<'a> {
    w: &'a mut W,
}
impl<'a> RESULTDONE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: RESULTDONE_AW) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Enable"]
    #[inline(always)]
    pub fn set(self) -> &'a mut W {
        self.variant(RESULTDONE_AW::SET)
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
#[doc = "Write '1' to Enable interrupt for CALIBRATEDONE event\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CALIBRATEDONE_A {
    #[doc = "0: Read: Disabled"]
    DISABLED = 0,
    #[doc = "1: Read: Enabled"]
    ENABLED = 1,
}
impl From<CALIBRATEDONE_A> for bool {
    #[inline(always)]
    fn from(variant: CALIBRATEDONE_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `CALIBRATEDONE`"]
pub type CALIBRATEDONE_R = crate::R<bool, CALIBRATEDONE_A>;
impl CALIBRATEDONE_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CALIBRATEDONE_A {
        match self.bits {
            false => CALIBRATEDONE_A::DISABLED,
            true => CALIBRATEDONE_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == CALIBRATEDONE_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == CALIBRATEDONE_A::ENABLED
    }
}
#[doc = "Write '1' to Enable interrupt for CALIBRATEDONE event\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CALIBRATEDONE_AW {
    #[doc = "1: Enable"]
    SET = 1,
}
impl From<CALIBRATEDONE_AW> for bool {
    #[inline(always)]
    fn from(variant: CALIBRATEDONE_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Write proxy for field `CALIBRATEDONE`"]
pub struct CALIBRATEDONE_W<'a> {
    w: &'a mut W,
}
impl<'a> CALIBRATEDONE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CALIBRATEDONE_AW) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Enable"]
    #[inline(always)]
    pub fn set(self) -> &'a mut W {
        self.variant(CALIBRATEDONE_AW::SET)
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
        self.w.bits = (self.w.bits & !(0x01 << 5)) | (((value as u32) & 0x01) << 5);
        self.w
    }
}
#[doc = "Write '1' to Enable interrupt for CH\\[0\\].LIMITH event\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CH0LIMITH_A {
    #[doc = "0: Read: Disabled"]
    DISABLED = 0,
    #[doc = "1: Read: Enabled"]
    ENABLED = 1,
}
impl From<CH0LIMITH_A> for bool {
    #[inline(always)]
    fn from(variant: CH0LIMITH_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `CH0LIMITH`"]
pub type CH0LIMITH_R = crate::R<bool, CH0LIMITH_A>;
impl CH0LIMITH_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CH0LIMITH_A {
        match self.bits {
            false => CH0LIMITH_A::DISABLED,
            true => CH0LIMITH_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == CH0LIMITH_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == CH0LIMITH_A::ENABLED
    }
}
#[doc = "Write '1' to Enable interrupt for CH\\[0\\].LIMITH event\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CH0LIMITH_AW {
    #[doc = "1: Enable"]
    SET = 1,
}
impl From<CH0LIMITH_AW> for bool {
    #[inline(always)]
    fn from(variant: CH0LIMITH_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Write proxy for field `CH0LIMITH`"]
pub struct CH0LIMITH_W<'a> {
    w: &'a mut W,
}
impl<'a> CH0LIMITH_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CH0LIMITH_AW) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Enable"]
    #[inline(always)]
    pub fn set(self) -> &'a mut W {
        self.variant(CH0LIMITH_AW::SET)
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
#[doc = "Write '1' to Enable interrupt for CH\\[0\\].LIMITL event\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CH0LIMITL_A {
    #[doc = "0: Read: Disabled"]
    DISABLED = 0,
    #[doc = "1: Read: Enabled"]
    ENABLED = 1,
}
impl From<CH0LIMITL_A> for bool {
    #[inline(always)]
    fn from(variant: CH0LIMITL_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `CH0LIMITL`"]
pub type CH0LIMITL_R = crate::R<bool, CH0LIMITL_A>;
impl CH0LIMITL_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CH0LIMITL_A {
        match self.bits {
            false => CH0LIMITL_A::DISABLED,
            true => CH0LIMITL_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == CH0LIMITL_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == CH0LIMITL_A::ENABLED
    }
}
#[doc = "Write '1' to Enable interrupt for CH\\[0\\].LIMITL event\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CH0LIMITL_AW {
    #[doc = "1: Enable"]
    SET = 1,
}
impl From<CH0LIMITL_AW> for bool {
    #[inline(always)]
    fn from(variant: CH0LIMITL_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Write proxy for field `CH0LIMITL`"]
pub struct CH0LIMITL_W<'a> {
    w: &'a mut W,
}
impl<'a> CH0LIMITL_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CH0LIMITL_AW) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Enable"]
    #[inline(always)]
    pub fn set(self) -> &'a mut W {
        self.variant(CH0LIMITL_AW::SET)
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
#[doc = "Write '1' to Enable interrupt for CH\\[1\\].LIMITH event\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CH1LIMITH_A {
    #[doc = "0: Read: Disabled"]
    DISABLED = 0,
    #[doc = "1: Read: Enabled"]
    ENABLED = 1,
}
impl From<CH1LIMITH_A> for bool {
    #[inline(always)]
    fn from(variant: CH1LIMITH_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `CH1LIMITH`"]
pub type CH1LIMITH_R = crate::R<bool, CH1LIMITH_A>;
impl CH1LIMITH_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CH1LIMITH_A {
        match self.bits {
            false => CH1LIMITH_A::DISABLED,
            true => CH1LIMITH_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == CH1LIMITH_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == CH1LIMITH_A::ENABLED
    }
}
#[doc = "Write '1' to Enable interrupt for CH\\[1\\].LIMITH event\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CH1LIMITH_AW {
    #[doc = "1: Enable"]
    SET = 1,
}
impl From<CH1LIMITH_AW> for bool {
    #[inline(always)]
    fn from(variant: CH1LIMITH_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Write proxy for field `CH1LIMITH`"]
pub struct CH1LIMITH_W<'a> {
    w: &'a mut W,
}
impl<'a> CH1LIMITH_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CH1LIMITH_AW) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Enable"]
    #[inline(always)]
    pub fn set(self) -> &'a mut W {
        self.variant(CH1LIMITH_AW::SET)
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
#[doc = "Write '1' to Enable interrupt for CH\\[1\\].LIMITL event\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CH1LIMITL_A {
    #[doc = "0: Read: Disabled"]
    DISABLED = 0,
    #[doc = "1: Read: Enabled"]
    ENABLED = 1,
}
impl From<CH1LIMITL_A> for bool {
    #[inline(always)]
    fn from(variant: CH1LIMITL_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `CH1LIMITL`"]
pub type CH1LIMITL_R = crate::R<bool, CH1LIMITL_A>;
impl CH1LIMITL_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CH1LIMITL_A {
        match self.bits {
            false => CH1LIMITL_A::DISABLED,
            true => CH1LIMITL_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == CH1LIMITL_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == CH1LIMITL_A::ENABLED
    }
}
#[doc = "Write '1' to Enable interrupt for CH\\[1\\].LIMITL event\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CH1LIMITL_AW {
    #[doc = "1: Enable"]
    SET = 1,
}
impl From<CH1LIMITL_AW> for bool {
    #[inline(always)]
    fn from(variant: CH1LIMITL_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Write proxy for field `CH1LIMITL`"]
pub struct CH1LIMITL_W<'a> {
    w: &'a mut W,
}
impl<'a> CH1LIMITL_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CH1LIMITL_AW) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Enable"]
    #[inline(always)]
    pub fn set(self) -> &'a mut W {
        self.variant(CH1LIMITL_AW::SET)
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
#[doc = "Write '1' to Enable interrupt for CH\\[2\\].LIMITH event\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CH2LIMITH_A {
    #[doc = "0: Read: Disabled"]
    DISABLED = 0,
    #[doc = "1: Read: Enabled"]
    ENABLED = 1,
}
impl From<CH2LIMITH_A> for bool {
    #[inline(always)]
    fn from(variant: CH2LIMITH_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `CH2LIMITH`"]
pub type CH2LIMITH_R = crate::R<bool, CH2LIMITH_A>;
impl CH2LIMITH_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CH2LIMITH_A {
        match self.bits {
            false => CH2LIMITH_A::DISABLED,
            true => CH2LIMITH_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == CH2LIMITH_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == CH2LIMITH_A::ENABLED
    }
}
#[doc = "Write '1' to Enable interrupt for CH\\[2\\].LIMITH event\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CH2LIMITH_AW {
    #[doc = "1: Enable"]
    SET = 1,
}
impl From<CH2LIMITH_AW> for bool {
    #[inline(always)]
    fn from(variant: CH2LIMITH_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Write proxy for field `CH2LIMITH`"]
pub struct CH2LIMITH_W<'a> {
    w: &'a mut W,
}
impl<'a> CH2LIMITH_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CH2LIMITH_AW) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Enable"]
    #[inline(always)]
    pub fn set(self) -> &'a mut W {
        self.variant(CH2LIMITH_AW::SET)
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
#[doc = "Write '1' to Enable interrupt for CH\\[2\\].LIMITL event\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CH2LIMITL_A {
    #[doc = "0: Read: Disabled"]
    DISABLED = 0,
    #[doc = "1: Read: Enabled"]
    ENABLED = 1,
}
impl From<CH2LIMITL_A> for bool {
    #[inline(always)]
    fn from(variant: CH2LIMITL_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `CH2LIMITL`"]
pub type CH2LIMITL_R = crate::R<bool, CH2LIMITL_A>;
impl CH2LIMITL_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CH2LIMITL_A {
        match self.bits {
            false => CH2LIMITL_A::DISABLED,
            true => CH2LIMITL_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == CH2LIMITL_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == CH2LIMITL_A::ENABLED
    }
}
#[doc = "Write '1' to Enable interrupt for CH\\[2\\].LIMITL event\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CH2LIMITL_AW {
    #[doc = "1: Enable"]
    SET = 1,
}
impl From<CH2LIMITL_AW> for bool {
    #[inline(always)]
    fn from(variant: CH2LIMITL_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Write proxy for field `CH2LIMITL`"]
pub struct CH2LIMITL_W<'a> {
    w: &'a mut W,
}
impl<'a> CH2LIMITL_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CH2LIMITL_AW) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Enable"]
    #[inline(always)]
    pub fn set(self) -> &'a mut W {
        self.variant(CH2LIMITL_AW::SET)
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
#[doc = "Write '1' to Enable interrupt for CH\\[3\\].LIMITH event\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CH3LIMITH_A {
    #[doc = "0: Read: Disabled"]
    DISABLED = 0,
    #[doc = "1: Read: Enabled"]
    ENABLED = 1,
}
impl From<CH3LIMITH_A> for bool {
    #[inline(always)]
    fn from(variant: CH3LIMITH_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `CH3LIMITH`"]
pub type CH3LIMITH_R = crate::R<bool, CH3LIMITH_A>;
impl CH3LIMITH_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CH3LIMITH_A {
        match self.bits {
            false => CH3LIMITH_A::DISABLED,
            true => CH3LIMITH_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == CH3LIMITH_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == CH3LIMITH_A::ENABLED
    }
}
#[doc = "Write '1' to Enable interrupt for CH\\[3\\].LIMITH event\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CH3LIMITH_AW {
    #[doc = "1: Enable"]
    SET = 1,
}
impl From<CH3LIMITH_AW> for bool {
    #[inline(always)]
    fn from(variant: CH3LIMITH_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Write proxy for field `CH3LIMITH`"]
pub struct CH3LIMITH_W<'a> {
    w: &'a mut W,
}
impl<'a> CH3LIMITH_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CH3LIMITH_AW) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Enable"]
    #[inline(always)]
    pub fn set(self) -> &'a mut W {
        self.variant(CH3LIMITH_AW::SET)
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
#[doc = "Write '1' to Enable interrupt for CH\\[3\\].LIMITL event\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CH3LIMITL_A {
    #[doc = "0: Read: Disabled"]
    DISABLED = 0,
    #[doc = "1: Read: Enabled"]
    ENABLED = 1,
}
impl From<CH3LIMITL_A> for bool {
    #[inline(always)]
    fn from(variant: CH3LIMITL_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `CH3LIMITL`"]
pub type CH3LIMITL_R = crate::R<bool, CH3LIMITL_A>;
impl CH3LIMITL_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CH3LIMITL_A {
        match self.bits {
            false => CH3LIMITL_A::DISABLED,
            true => CH3LIMITL_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == CH3LIMITL_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == CH3LIMITL_A::ENABLED
    }
}
#[doc = "Write '1' to Enable interrupt for CH\\[3\\].LIMITL event\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CH3LIMITL_AW {
    #[doc = "1: Enable"]
    SET = 1,
}
impl From<CH3LIMITL_AW> for bool {
    #[inline(always)]
    fn from(variant: CH3LIMITL_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Write proxy for field `CH3LIMITL`"]
pub struct CH3LIMITL_W<'a> {
    w: &'a mut W,
}
impl<'a> CH3LIMITL_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CH3LIMITL_AW) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Enable"]
    #[inline(always)]
    pub fn set(self) -> &'a mut W {
        self.variant(CH3LIMITL_AW::SET)
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
#[doc = "Write '1' to Enable interrupt for CH\\[4\\].LIMITH event\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CH4LIMITH_A {
    #[doc = "0: Read: Disabled"]
    DISABLED = 0,
    #[doc = "1: Read: Enabled"]
    ENABLED = 1,
}
impl From<CH4LIMITH_A> for bool {
    #[inline(always)]
    fn from(variant: CH4LIMITH_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `CH4LIMITH`"]
pub type CH4LIMITH_R = crate::R<bool, CH4LIMITH_A>;
impl CH4LIMITH_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CH4LIMITH_A {
        match self.bits {
            false => CH4LIMITH_A::DISABLED,
            true => CH4LIMITH_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == CH4LIMITH_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == CH4LIMITH_A::ENABLED
    }
}
#[doc = "Write '1' to Enable interrupt for CH\\[4\\].LIMITH event\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CH4LIMITH_AW {
    #[doc = "1: Enable"]
    SET = 1,
}
impl From<CH4LIMITH_AW> for bool {
    #[inline(always)]
    fn from(variant: CH4LIMITH_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Write proxy for field `CH4LIMITH`"]
pub struct CH4LIMITH_W<'a> {
    w: &'a mut W,
}
impl<'a> CH4LIMITH_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CH4LIMITH_AW) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Enable"]
    #[inline(always)]
    pub fn set(self) -> &'a mut W {
        self.variant(CH4LIMITH_AW::SET)
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
#[doc = "Write '1' to Enable interrupt for CH\\[4\\].LIMITL event\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CH4LIMITL_A {
    #[doc = "0: Read: Disabled"]
    DISABLED = 0,
    #[doc = "1: Read: Enabled"]
    ENABLED = 1,
}
impl From<CH4LIMITL_A> for bool {
    #[inline(always)]
    fn from(variant: CH4LIMITL_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `CH4LIMITL`"]
pub type CH4LIMITL_R = crate::R<bool, CH4LIMITL_A>;
impl CH4LIMITL_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CH4LIMITL_A {
        match self.bits {
            false => CH4LIMITL_A::DISABLED,
            true => CH4LIMITL_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == CH4LIMITL_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == CH4LIMITL_A::ENABLED
    }
}
#[doc = "Write '1' to Enable interrupt for CH\\[4\\].LIMITL event\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CH4LIMITL_AW {
    #[doc = "1: Enable"]
    SET = 1,
}
impl From<CH4LIMITL_AW> for bool {
    #[inline(always)]
    fn from(variant: CH4LIMITL_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Write proxy for field `CH4LIMITL`"]
pub struct CH4LIMITL_W<'a> {
    w: &'a mut W,
}
impl<'a> CH4LIMITL_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CH4LIMITL_AW) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Enable"]
    #[inline(always)]
    pub fn set(self) -> &'a mut W {
        self.variant(CH4LIMITL_AW::SET)
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
#[doc = "Write '1' to Enable interrupt for CH\\[5\\].LIMITH event\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CH5LIMITH_A {
    #[doc = "0: Read: Disabled"]
    DISABLED = 0,
    #[doc = "1: Read: Enabled"]
    ENABLED = 1,
}
impl From<CH5LIMITH_A> for bool {
    #[inline(always)]
    fn from(variant: CH5LIMITH_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `CH5LIMITH`"]
pub type CH5LIMITH_R = crate::R<bool, CH5LIMITH_A>;
impl CH5LIMITH_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CH5LIMITH_A {
        match self.bits {
            false => CH5LIMITH_A::DISABLED,
            true => CH5LIMITH_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == CH5LIMITH_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == CH5LIMITH_A::ENABLED
    }
}
#[doc = "Write '1' to Enable interrupt for CH\\[5\\].LIMITH event\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CH5LIMITH_AW {
    #[doc = "1: Enable"]
    SET = 1,
}
impl From<CH5LIMITH_AW> for bool {
    #[inline(always)]
    fn from(variant: CH5LIMITH_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Write proxy for field `CH5LIMITH`"]
pub struct CH5LIMITH_W<'a> {
    w: &'a mut W,
}
impl<'a> CH5LIMITH_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CH5LIMITH_AW) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Enable"]
    #[inline(always)]
    pub fn set(self) -> &'a mut W {
        self.variant(CH5LIMITH_AW::SET)
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
#[doc = "Write '1' to Enable interrupt for CH\\[5\\].LIMITL event\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CH5LIMITL_A {
    #[doc = "0: Read: Disabled"]
    DISABLED = 0,
    #[doc = "1: Read: Enabled"]
    ENABLED = 1,
}
impl From<CH5LIMITL_A> for bool {
    #[inline(always)]
    fn from(variant: CH5LIMITL_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `CH5LIMITL`"]
pub type CH5LIMITL_R = crate::R<bool, CH5LIMITL_A>;
impl CH5LIMITL_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CH5LIMITL_A {
        match self.bits {
            false => CH5LIMITL_A::DISABLED,
            true => CH5LIMITL_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == CH5LIMITL_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == CH5LIMITL_A::ENABLED
    }
}
#[doc = "Write '1' to Enable interrupt for CH\\[5\\].LIMITL event\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CH5LIMITL_AW {
    #[doc = "1: Enable"]
    SET = 1,
}
impl From<CH5LIMITL_AW> for bool {
    #[inline(always)]
    fn from(variant: CH5LIMITL_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Write proxy for field `CH5LIMITL`"]
pub struct CH5LIMITL_W<'a> {
    w: &'a mut W,
}
impl<'a> CH5LIMITL_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CH5LIMITL_AW) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Enable"]
    #[inline(always)]
    pub fn set(self) -> &'a mut W {
        self.variant(CH5LIMITL_AW::SET)
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
#[doc = "Write '1' to Enable interrupt for CH\\[6\\].LIMITH event\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CH6LIMITH_A {
    #[doc = "0: Read: Disabled"]
    DISABLED = 0,
    #[doc = "1: Read: Enabled"]
    ENABLED = 1,
}
impl From<CH6LIMITH_A> for bool {
    #[inline(always)]
    fn from(variant: CH6LIMITH_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `CH6LIMITH`"]
pub type CH6LIMITH_R = crate::R<bool, CH6LIMITH_A>;
impl CH6LIMITH_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CH6LIMITH_A {
        match self.bits {
            false => CH6LIMITH_A::DISABLED,
            true => CH6LIMITH_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == CH6LIMITH_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == CH6LIMITH_A::ENABLED
    }
}
#[doc = "Write '1' to Enable interrupt for CH\\[6\\].LIMITH event\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CH6LIMITH_AW {
    #[doc = "1: Enable"]
    SET = 1,
}
impl From<CH6LIMITH_AW> for bool {
    #[inline(always)]
    fn from(variant: CH6LIMITH_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Write proxy for field `CH6LIMITH`"]
pub struct CH6LIMITH_W<'a> {
    w: &'a mut W,
}
impl<'a> CH6LIMITH_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CH6LIMITH_AW) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Enable"]
    #[inline(always)]
    pub fn set(self) -> &'a mut W {
        self.variant(CH6LIMITH_AW::SET)
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
#[doc = "Write '1' to Enable interrupt for CH\\[6\\].LIMITL event\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CH6LIMITL_A {
    #[doc = "0: Read: Disabled"]
    DISABLED = 0,
    #[doc = "1: Read: Enabled"]
    ENABLED = 1,
}
impl From<CH6LIMITL_A> for bool {
    #[inline(always)]
    fn from(variant: CH6LIMITL_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `CH6LIMITL`"]
pub type CH6LIMITL_R = crate::R<bool, CH6LIMITL_A>;
impl CH6LIMITL_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CH6LIMITL_A {
        match self.bits {
            false => CH6LIMITL_A::DISABLED,
            true => CH6LIMITL_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == CH6LIMITL_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == CH6LIMITL_A::ENABLED
    }
}
#[doc = "Write '1' to Enable interrupt for CH\\[6\\].LIMITL event\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CH6LIMITL_AW {
    #[doc = "1: Enable"]
    SET = 1,
}
impl From<CH6LIMITL_AW> for bool {
    #[inline(always)]
    fn from(variant: CH6LIMITL_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Write proxy for field `CH6LIMITL`"]
pub struct CH6LIMITL_W<'a> {
    w: &'a mut W,
}
impl<'a> CH6LIMITL_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CH6LIMITL_AW) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Enable"]
    #[inline(always)]
    pub fn set(self) -> &'a mut W {
        self.variant(CH6LIMITL_AW::SET)
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
#[doc = "Write '1' to Enable interrupt for CH\\[7\\].LIMITH event\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CH7LIMITH_A {
    #[doc = "0: Read: Disabled"]
    DISABLED = 0,
    #[doc = "1: Read: Enabled"]
    ENABLED = 1,
}
impl From<CH7LIMITH_A> for bool {
    #[inline(always)]
    fn from(variant: CH7LIMITH_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `CH7LIMITH`"]
pub type CH7LIMITH_R = crate::R<bool, CH7LIMITH_A>;
impl CH7LIMITH_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CH7LIMITH_A {
        match self.bits {
            false => CH7LIMITH_A::DISABLED,
            true => CH7LIMITH_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == CH7LIMITH_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == CH7LIMITH_A::ENABLED
    }
}
#[doc = "Write '1' to Enable interrupt for CH\\[7\\].LIMITH event\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CH7LIMITH_AW {
    #[doc = "1: Enable"]
    SET = 1,
}
impl From<CH7LIMITH_AW> for bool {
    #[inline(always)]
    fn from(variant: CH7LIMITH_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Write proxy for field `CH7LIMITH`"]
pub struct CH7LIMITH_W<'a> {
    w: &'a mut W,
}
impl<'a> CH7LIMITH_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CH7LIMITH_AW) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Enable"]
    #[inline(always)]
    pub fn set(self) -> &'a mut W {
        self.variant(CH7LIMITH_AW::SET)
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
#[doc = "Write '1' to Enable interrupt for CH\\[7\\].LIMITL event\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CH7LIMITL_A {
    #[doc = "0: Read: Disabled"]
    DISABLED = 0,
    #[doc = "1: Read: Enabled"]
    ENABLED = 1,
}
impl From<CH7LIMITL_A> for bool {
    #[inline(always)]
    fn from(variant: CH7LIMITL_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `CH7LIMITL`"]
pub type CH7LIMITL_R = crate::R<bool, CH7LIMITL_A>;
impl CH7LIMITL_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CH7LIMITL_A {
        match self.bits {
            false => CH7LIMITL_A::DISABLED,
            true => CH7LIMITL_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == CH7LIMITL_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == CH7LIMITL_A::ENABLED
    }
}
#[doc = "Write '1' to Enable interrupt for CH\\[7\\].LIMITL event\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CH7LIMITL_AW {
    #[doc = "1: Enable"]
    SET = 1,
}
impl From<CH7LIMITL_AW> for bool {
    #[inline(always)]
    fn from(variant: CH7LIMITL_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Write proxy for field `CH7LIMITL`"]
pub struct CH7LIMITL_W<'a> {
    w: &'a mut W,
}
impl<'a> CH7LIMITL_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CH7LIMITL_AW) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Enable"]
    #[inline(always)]
    pub fn set(self) -> &'a mut W {
        self.variant(CH7LIMITL_AW::SET)
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
impl R {
    #[doc = "Bit 0 - Write '1' to Enable interrupt for STARTED event"]
    #[inline(always)]
    pub fn started(&self) -> STARTED_R {
        STARTED_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - Write '1' to Enable interrupt for END event"]
    #[inline(always)]
    pub fn end(&self) -> END_R {
        END_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - Write '1' to Enable interrupt for DONE event"]
    #[inline(always)]
    pub fn done(&self) -> DONE_R {
        DONE_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - Write '1' to Enable interrupt for RESULTDONE event"]
    #[inline(always)]
    pub fn resultdone(&self) -> RESULTDONE_R {
        RESULTDONE_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - Write '1' to Enable interrupt for CALIBRATEDONE event"]
    #[inline(always)]
    pub fn calibratedone(&self) -> CALIBRATEDONE_R {
        CALIBRATEDONE_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - Write '1' to Enable interrupt for STOPPED event"]
    #[inline(always)]
    pub fn stopped(&self) -> STOPPED_R {
        STOPPED_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - Write '1' to Enable interrupt for CH\\[0\\].LIMITH event"]
    #[inline(always)]
    pub fn ch0limith(&self) -> CH0LIMITH_R {
        CH0LIMITH_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7 - Write '1' to Enable interrupt for CH\\[0\\].LIMITL event"]
    #[inline(always)]
    pub fn ch0limitl(&self) -> CH0LIMITL_R {
        CH0LIMITL_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 8 - Write '1' to Enable interrupt for CH\\[1\\].LIMITH event"]
    #[inline(always)]
    pub fn ch1limith(&self) -> CH1LIMITH_R {
        CH1LIMITH_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 9 - Write '1' to Enable interrupt for CH\\[1\\].LIMITL event"]
    #[inline(always)]
    pub fn ch1limitl(&self) -> CH1LIMITL_R {
        CH1LIMITL_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 10 - Write '1' to Enable interrupt for CH\\[2\\].LIMITH event"]
    #[inline(always)]
    pub fn ch2limith(&self) -> CH2LIMITH_R {
        CH2LIMITH_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    #[doc = "Bit 11 - Write '1' to Enable interrupt for CH\\[2\\].LIMITL event"]
    #[inline(always)]
    pub fn ch2limitl(&self) -> CH2LIMITL_R {
        CH2LIMITL_R::new(((self.bits >> 11) & 0x01) != 0)
    }
    #[doc = "Bit 12 - Write '1' to Enable interrupt for CH\\[3\\].LIMITH event"]
    #[inline(always)]
    pub fn ch3limith(&self) -> CH3LIMITH_R {
        CH3LIMITH_R::new(((self.bits >> 12) & 0x01) != 0)
    }
    #[doc = "Bit 13 - Write '1' to Enable interrupt for CH\\[3\\].LIMITL event"]
    #[inline(always)]
    pub fn ch3limitl(&self) -> CH3LIMITL_R {
        CH3LIMITL_R::new(((self.bits >> 13) & 0x01) != 0)
    }
    #[doc = "Bit 14 - Write '1' to Enable interrupt for CH\\[4\\].LIMITH event"]
    #[inline(always)]
    pub fn ch4limith(&self) -> CH4LIMITH_R {
        CH4LIMITH_R::new(((self.bits >> 14) & 0x01) != 0)
    }
    #[doc = "Bit 15 - Write '1' to Enable interrupt for CH\\[4\\].LIMITL event"]
    #[inline(always)]
    pub fn ch4limitl(&self) -> CH4LIMITL_R {
        CH4LIMITL_R::new(((self.bits >> 15) & 0x01) != 0)
    }
    #[doc = "Bit 16 - Write '1' to Enable interrupt for CH\\[5\\].LIMITH event"]
    #[inline(always)]
    pub fn ch5limith(&self) -> CH5LIMITH_R {
        CH5LIMITH_R::new(((self.bits >> 16) & 0x01) != 0)
    }
    #[doc = "Bit 17 - Write '1' to Enable interrupt for CH\\[5\\].LIMITL event"]
    #[inline(always)]
    pub fn ch5limitl(&self) -> CH5LIMITL_R {
        CH5LIMITL_R::new(((self.bits >> 17) & 0x01) != 0)
    }
    #[doc = "Bit 18 - Write '1' to Enable interrupt for CH\\[6\\].LIMITH event"]
    #[inline(always)]
    pub fn ch6limith(&self) -> CH6LIMITH_R {
        CH6LIMITH_R::new(((self.bits >> 18) & 0x01) != 0)
    }
    #[doc = "Bit 19 - Write '1' to Enable interrupt for CH\\[6\\].LIMITL event"]
    #[inline(always)]
    pub fn ch6limitl(&self) -> CH6LIMITL_R {
        CH6LIMITL_R::new(((self.bits >> 19) & 0x01) != 0)
    }
    #[doc = "Bit 20 - Write '1' to Enable interrupt for CH\\[7\\].LIMITH event"]
    #[inline(always)]
    pub fn ch7limith(&self) -> CH7LIMITH_R {
        CH7LIMITH_R::new(((self.bits >> 20) & 0x01) != 0)
    }
    #[doc = "Bit 21 - Write '1' to Enable interrupt for CH\\[7\\].LIMITL event"]
    #[inline(always)]
    pub fn ch7limitl(&self) -> CH7LIMITL_R {
        CH7LIMITL_R::new(((self.bits >> 21) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Write '1' to Enable interrupt for STARTED event"]
    #[inline(always)]
    pub fn started(&mut self) -> STARTED_W {
        STARTED_W { w: self }
    }
    #[doc = "Bit 1 - Write '1' to Enable interrupt for END event"]
    #[inline(always)]
    pub fn end(&mut self) -> END_W {
        END_W { w: self }
    }
    #[doc = "Bit 2 - Write '1' to Enable interrupt for DONE event"]
    #[inline(always)]
    pub fn done(&mut self) -> DONE_W {
        DONE_W { w: self }
    }
    #[doc = "Bit 3 - Write '1' to Enable interrupt for RESULTDONE event"]
    #[inline(always)]
    pub fn resultdone(&mut self) -> RESULTDONE_W {
        RESULTDONE_W { w: self }
    }
    #[doc = "Bit 4 - Write '1' to Enable interrupt for CALIBRATEDONE event"]
    #[inline(always)]
    pub fn calibratedone(&mut self) -> CALIBRATEDONE_W {
        CALIBRATEDONE_W { w: self }
    }
    #[doc = "Bit 5 - Write '1' to Enable interrupt for STOPPED event"]
    #[inline(always)]
    pub fn stopped(&mut self) -> STOPPED_W {
        STOPPED_W { w: self }
    }
    #[doc = "Bit 6 - Write '1' to Enable interrupt for CH\\[0\\].LIMITH event"]
    #[inline(always)]
    pub fn ch0limith(&mut self) -> CH0LIMITH_W {
        CH0LIMITH_W { w: self }
    }
    #[doc = "Bit 7 - Write '1' to Enable interrupt for CH\\[0\\].LIMITL event"]
    #[inline(always)]
    pub fn ch0limitl(&mut self) -> CH0LIMITL_W {
        CH0LIMITL_W { w: self }
    }
    #[doc = "Bit 8 - Write '1' to Enable interrupt for CH\\[1\\].LIMITH event"]
    #[inline(always)]
    pub fn ch1limith(&mut self) -> CH1LIMITH_W {
        CH1LIMITH_W { w: self }
    }
    #[doc = "Bit 9 - Write '1' to Enable interrupt for CH\\[1\\].LIMITL event"]
    #[inline(always)]
    pub fn ch1limitl(&mut self) -> CH1LIMITL_W {
        CH1LIMITL_W { w: self }
    }
    #[doc = "Bit 10 - Write '1' to Enable interrupt for CH\\[2\\].LIMITH event"]
    #[inline(always)]
    pub fn ch2limith(&mut self) -> CH2LIMITH_W {
        CH2LIMITH_W { w: self }
    }
    #[doc = "Bit 11 - Write '1' to Enable interrupt for CH\\[2\\].LIMITL event"]
    #[inline(always)]
    pub fn ch2limitl(&mut self) -> CH2LIMITL_W {
        CH2LIMITL_W { w: self }
    }
    #[doc = "Bit 12 - Write '1' to Enable interrupt for CH\\[3\\].LIMITH event"]
    #[inline(always)]
    pub fn ch3limith(&mut self) -> CH3LIMITH_W {
        CH3LIMITH_W { w: self }
    }
    #[doc = "Bit 13 - Write '1' to Enable interrupt for CH\\[3\\].LIMITL event"]
    #[inline(always)]
    pub fn ch3limitl(&mut self) -> CH3LIMITL_W {
        CH3LIMITL_W { w: self }
    }
    #[doc = "Bit 14 - Write '1' to Enable interrupt for CH\\[4\\].LIMITH event"]
    #[inline(always)]
    pub fn ch4limith(&mut self) -> CH4LIMITH_W {
        CH4LIMITH_W { w: self }
    }
    #[doc = "Bit 15 - Write '1' to Enable interrupt for CH\\[4\\].LIMITL event"]
    #[inline(always)]
    pub fn ch4limitl(&mut self) -> CH4LIMITL_W {
        CH4LIMITL_W { w: self }
    }
    #[doc = "Bit 16 - Write '1' to Enable interrupt for CH\\[5\\].LIMITH event"]
    #[inline(always)]
    pub fn ch5limith(&mut self) -> CH5LIMITH_W {
        CH5LIMITH_W { w: self }
    }
    #[doc = "Bit 17 - Write '1' to Enable interrupt for CH\\[5\\].LIMITL event"]
    #[inline(always)]
    pub fn ch5limitl(&mut self) -> CH5LIMITL_W {
        CH5LIMITL_W { w: self }
    }
    #[doc = "Bit 18 - Write '1' to Enable interrupt for CH\\[6\\].LIMITH event"]
    #[inline(always)]
    pub fn ch6limith(&mut self) -> CH6LIMITH_W {
        CH6LIMITH_W { w: self }
    }
    #[doc = "Bit 19 - Write '1' to Enable interrupt for CH\\[6\\].LIMITL event"]
    #[inline(always)]
    pub fn ch6limitl(&mut self) -> CH6LIMITL_W {
        CH6LIMITL_W { w: self }
    }
    #[doc = "Bit 20 - Write '1' to Enable interrupt for CH\\[7\\].LIMITH event"]
    #[inline(always)]
    pub fn ch7limith(&mut self) -> CH7LIMITH_W {
        CH7LIMITH_W { w: self }
    }
    #[doc = "Bit 21 - Write '1' to Enable interrupt for CH\\[7\\].LIMITL event"]
    #[inline(always)]
    pub fn ch7limitl(&mut self) -> CH7LIMITL_W {
        CH7LIMITL_W { w: self }
    }
}
