#[doc = "Reader of register CONFIG3"]
pub type R = crate::R<u32, super::CONFIG3>;
#[doc = "Writer for register CONFIG3"]
pub type W = crate::W<u32, super::CONFIG3>;
#[doc = "Register CONFIG3 `reset()`'s with value 0"]
impl crate::ResetValue for super::CONFIG3 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Enable protection for region 96. Write '0' has no effect.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum REGION96_A {
    #[doc = "0: Protection disabled"]
    DISABLED = 0,
    #[doc = "1: Protection enabled"]
    ENABLED = 1,
}
impl From<REGION96_A> for bool {
    #[inline(always)]
    fn from(variant: REGION96_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `REGION96`"]
pub type REGION96_R = crate::R<bool, REGION96_A>;
impl REGION96_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> REGION96_A {
        match self.bits {
            false => REGION96_A::DISABLED,
            true => REGION96_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == REGION96_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == REGION96_A::ENABLED
    }
}
#[doc = "Write proxy for field `REGION96`"]
pub struct REGION96_W<'a> {
    w: &'a mut W,
}
impl<'a> REGION96_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: REGION96_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Protection disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(REGION96_A::DISABLED)
    }
    #[doc = "Protection enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(REGION96_A::ENABLED)
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
#[doc = "Enable protection for region 97. Write '0' has no effect.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum REGION97_A {
    #[doc = "0: Protection disabled"]
    DISABLED = 0,
    #[doc = "1: Protection enabled"]
    ENABLED = 1,
}
impl From<REGION97_A> for bool {
    #[inline(always)]
    fn from(variant: REGION97_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `REGION97`"]
pub type REGION97_R = crate::R<bool, REGION97_A>;
impl REGION97_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> REGION97_A {
        match self.bits {
            false => REGION97_A::DISABLED,
            true => REGION97_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == REGION97_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == REGION97_A::ENABLED
    }
}
#[doc = "Write proxy for field `REGION97`"]
pub struct REGION97_W<'a> {
    w: &'a mut W,
}
impl<'a> REGION97_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: REGION97_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Protection disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(REGION97_A::DISABLED)
    }
    #[doc = "Protection enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(REGION97_A::ENABLED)
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
#[doc = "Enable protection for region 98. Write '0' has no effect.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum REGION98_A {
    #[doc = "0: Protection disabled"]
    DISABLED = 0,
    #[doc = "1: Protection enabled"]
    ENABLED = 1,
}
impl From<REGION98_A> for bool {
    #[inline(always)]
    fn from(variant: REGION98_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `REGION98`"]
pub type REGION98_R = crate::R<bool, REGION98_A>;
impl REGION98_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> REGION98_A {
        match self.bits {
            false => REGION98_A::DISABLED,
            true => REGION98_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == REGION98_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == REGION98_A::ENABLED
    }
}
#[doc = "Write proxy for field `REGION98`"]
pub struct REGION98_W<'a> {
    w: &'a mut W,
}
impl<'a> REGION98_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: REGION98_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Protection disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(REGION98_A::DISABLED)
    }
    #[doc = "Protection enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(REGION98_A::ENABLED)
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
#[doc = "Enable protection for region 99. Write '0' has no effect.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum REGION99_A {
    #[doc = "0: Protection disabled"]
    DISABLED = 0,
    #[doc = "1: Protection enabled"]
    ENABLED = 1,
}
impl From<REGION99_A> for bool {
    #[inline(always)]
    fn from(variant: REGION99_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `REGION99`"]
pub type REGION99_R = crate::R<bool, REGION99_A>;
impl REGION99_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> REGION99_A {
        match self.bits {
            false => REGION99_A::DISABLED,
            true => REGION99_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == REGION99_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == REGION99_A::ENABLED
    }
}
#[doc = "Write proxy for field `REGION99`"]
pub struct REGION99_W<'a> {
    w: &'a mut W,
}
impl<'a> REGION99_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: REGION99_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Protection disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(REGION99_A::DISABLED)
    }
    #[doc = "Protection enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(REGION99_A::ENABLED)
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
#[doc = "Enable protection for region 100. Write '0' has no effect.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum REGION100_A {
    #[doc = "0: Protection disabled"]
    DISABLED = 0,
    #[doc = "1: Protection enabled"]
    ENABLED = 1,
}
impl From<REGION100_A> for bool {
    #[inline(always)]
    fn from(variant: REGION100_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `REGION100`"]
pub type REGION100_R = crate::R<bool, REGION100_A>;
impl REGION100_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> REGION100_A {
        match self.bits {
            false => REGION100_A::DISABLED,
            true => REGION100_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == REGION100_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == REGION100_A::ENABLED
    }
}
#[doc = "Write proxy for field `REGION100`"]
pub struct REGION100_W<'a> {
    w: &'a mut W,
}
impl<'a> REGION100_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: REGION100_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Protection disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(REGION100_A::DISABLED)
    }
    #[doc = "Protection enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(REGION100_A::ENABLED)
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
#[doc = "Enable protection for region 101. Write '0' has no effect.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum REGION101_A {
    #[doc = "0: Protection disabled"]
    DISABLED = 0,
    #[doc = "1: Protection enabled"]
    ENABLED = 1,
}
impl From<REGION101_A> for bool {
    #[inline(always)]
    fn from(variant: REGION101_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `REGION101`"]
pub type REGION101_R = crate::R<bool, REGION101_A>;
impl REGION101_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> REGION101_A {
        match self.bits {
            false => REGION101_A::DISABLED,
            true => REGION101_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == REGION101_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == REGION101_A::ENABLED
    }
}
#[doc = "Write proxy for field `REGION101`"]
pub struct REGION101_W<'a> {
    w: &'a mut W,
}
impl<'a> REGION101_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: REGION101_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Protection disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(REGION101_A::DISABLED)
    }
    #[doc = "Protection enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(REGION101_A::ENABLED)
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
#[doc = "Enable protection for region 102. Write '0' has no effect.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum REGION102_A {
    #[doc = "0: Protection disabled"]
    DISABLED = 0,
    #[doc = "1: Protection enabled"]
    ENABLED = 1,
}
impl From<REGION102_A> for bool {
    #[inline(always)]
    fn from(variant: REGION102_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `REGION102`"]
pub type REGION102_R = crate::R<bool, REGION102_A>;
impl REGION102_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> REGION102_A {
        match self.bits {
            false => REGION102_A::DISABLED,
            true => REGION102_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == REGION102_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == REGION102_A::ENABLED
    }
}
#[doc = "Write proxy for field `REGION102`"]
pub struct REGION102_W<'a> {
    w: &'a mut W,
}
impl<'a> REGION102_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: REGION102_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Protection disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(REGION102_A::DISABLED)
    }
    #[doc = "Protection enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(REGION102_A::ENABLED)
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
#[doc = "Enable protection for region 103. Write '0' has no effect.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum REGION103_A {
    #[doc = "0: Protection disabled"]
    DISABLED = 0,
    #[doc = "1: Protection enabled"]
    ENABLED = 1,
}
impl From<REGION103_A> for bool {
    #[inline(always)]
    fn from(variant: REGION103_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `REGION103`"]
pub type REGION103_R = crate::R<bool, REGION103_A>;
impl REGION103_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> REGION103_A {
        match self.bits {
            false => REGION103_A::DISABLED,
            true => REGION103_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == REGION103_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == REGION103_A::ENABLED
    }
}
#[doc = "Write proxy for field `REGION103`"]
pub struct REGION103_W<'a> {
    w: &'a mut W,
}
impl<'a> REGION103_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: REGION103_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Protection disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(REGION103_A::DISABLED)
    }
    #[doc = "Protection enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(REGION103_A::ENABLED)
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
#[doc = "Enable protection for region 104. Write '0' has no effect.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum REGION104_A {
    #[doc = "0: Protection disabled"]
    DISABLED = 0,
    #[doc = "1: Protection enabled"]
    ENABLED = 1,
}
impl From<REGION104_A> for bool {
    #[inline(always)]
    fn from(variant: REGION104_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `REGION104`"]
pub type REGION104_R = crate::R<bool, REGION104_A>;
impl REGION104_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> REGION104_A {
        match self.bits {
            false => REGION104_A::DISABLED,
            true => REGION104_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == REGION104_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == REGION104_A::ENABLED
    }
}
#[doc = "Write proxy for field `REGION104`"]
pub struct REGION104_W<'a> {
    w: &'a mut W,
}
impl<'a> REGION104_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: REGION104_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Protection disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(REGION104_A::DISABLED)
    }
    #[doc = "Protection enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(REGION104_A::ENABLED)
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
#[doc = "Enable protection for region 105. Write '0' has no effect.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum REGION105_A {
    #[doc = "0: Protection disabled"]
    DISABLED = 0,
    #[doc = "1: Protection enabled"]
    ENABLED = 1,
}
impl From<REGION105_A> for bool {
    #[inline(always)]
    fn from(variant: REGION105_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `REGION105`"]
pub type REGION105_R = crate::R<bool, REGION105_A>;
impl REGION105_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> REGION105_A {
        match self.bits {
            false => REGION105_A::DISABLED,
            true => REGION105_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == REGION105_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == REGION105_A::ENABLED
    }
}
#[doc = "Write proxy for field `REGION105`"]
pub struct REGION105_W<'a> {
    w: &'a mut W,
}
impl<'a> REGION105_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: REGION105_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Protection disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(REGION105_A::DISABLED)
    }
    #[doc = "Protection enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(REGION105_A::ENABLED)
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
#[doc = "Enable protection for region 106. Write '0' has no effect.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum REGION106_A {
    #[doc = "0: Protection disabled"]
    DISABLED = 0,
    #[doc = "1: Protection enabled"]
    ENABLED = 1,
}
impl From<REGION106_A> for bool {
    #[inline(always)]
    fn from(variant: REGION106_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `REGION106`"]
pub type REGION106_R = crate::R<bool, REGION106_A>;
impl REGION106_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> REGION106_A {
        match self.bits {
            false => REGION106_A::DISABLED,
            true => REGION106_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == REGION106_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == REGION106_A::ENABLED
    }
}
#[doc = "Write proxy for field `REGION106`"]
pub struct REGION106_W<'a> {
    w: &'a mut W,
}
impl<'a> REGION106_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: REGION106_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Protection disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(REGION106_A::DISABLED)
    }
    #[doc = "Protection enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(REGION106_A::ENABLED)
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
#[doc = "Enable protection for region 107. Write '0' has no effect.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum REGION107_A {
    #[doc = "0: Protection disabled"]
    DISABLED = 0,
    #[doc = "1: Protection enabled"]
    ENABLED = 1,
}
impl From<REGION107_A> for bool {
    #[inline(always)]
    fn from(variant: REGION107_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `REGION107`"]
pub type REGION107_R = crate::R<bool, REGION107_A>;
impl REGION107_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> REGION107_A {
        match self.bits {
            false => REGION107_A::DISABLED,
            true => REGION107_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == REGION107_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == REGION107_A::ENABLED
    }
}
#[doc = "Write proxy for field `REGION107`"]
pub struct REGION107_W<'a> {
    w: &'a mut W,
}
impl<'a> REGION107_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: REGION107_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Protection disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(REGION107_A::DISABLED)
    }
    #[doc = "Protection enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(REGION107_A::ENABLED)
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
#[doc = "Enable protection for region 108. Write '0' has no effect.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum REGION108_A {
    #[doc = "0: Protection disabled"]
    DISABLED = 0,
    #[doc = "1: Protection enabled"]
    ENABLED = 1,
}
impl From<REGION108_A> for bool {
    #[inline(always)]
    fn from(variant: REGION108_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `REGION108`"]
pub type REGION108_R = crate::R<bool, REGION108_A>;
impl REGION108_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> REGION108_A {
        match self.bits {
            false => REGION108_A::DISABLED,
            true => REGION108_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == REGION108_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == REGION108_A::ENABLED
    }
}
#[doc = "Write proxy for field `REGION108`"]
pub struct REGION108_W<'a> {
    w: &'a mut W,
}
impl<'a> REGION108_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: REGION108_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Protection disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(REGION108_A::DISABLED)
    }
    #[doc = "Protection enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(REGION108_A::ENABLED)
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
#[doc = "Enable protection for region 109. Write '0' has no effect.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum REGION109_A {
    #[doc = "0: Protection disabled"]
    DISABLED = 0,
    #[doc = "1: Protection enabled"]
    ENABLED = 1,
}
impl From<REGION109_A> for bool {
    #[inline(always)]
    fn from(variant: REGION109_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `REGION109`"]
pub type REGION109_R = crate::R<bool, REGION109_A>;
impl REGION109_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> REGION109_A {
        match self.bits {
            false => REGION109_A::DISABLED,
            true => REGION109_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == REGION109_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == REGION109_A::ENABLED
    }
}
#[doc = "Write proxy for field `REGION109`"]
pub struct REGION109_W<'a> {
    w: &'a mut W,
}
impl<'a> REGION109_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: REGION109_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Protection disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(REGION109_A::DISABLED)
    }
    #[doc = "Protection enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(REGION109_A::ENABLED)
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
#[doc = "Enable protection for region 110. Write '0' has no effect.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum REGION110_A {
    #[doc = "0: Protection disabled"]
    DISABLED = 0,
    #[doc = "1: Protection enabled"]
    ENABLED = 1,
}
impl From<REGION110_A> for bool {
    #[inline(always)]
    fn from(variant: REGION110_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `REGION110`"]
pub type REGION110_R = crate::R<bool, REGION110_A>;
impl REGION110_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> REGION110_A {
        match self.bits {
            false => REGION110_A::DISABLED,
            true => REGION110_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == REGION110_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == REGION110_A::ENABLED
    }
}
#[doc = "Write proxy for field `REGION110`"]
pub struct REGION110_W<'a> {
    w: &'a mut W,
}
impl<'a> REGION110_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: REGION110_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Protection disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(REGION110_A::DISABLED)
    }
    #[doc = "Protection enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(REGION110_A::ENABLED)
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
#[doc = "Enable protection for region 111. Write '0' has no effect.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum REGION111_A {
    #[doc = "0: Protection disabled"]
    DISABLED = 0,
    #[doc = "1: Protection enabled"]
    ENABLED = 1,
}
impl From<REGION111_A> for bool {
    #[inline(always)]
    fn from(variant: REGION111_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `REGION111`"]
pub type REGION111_R = crate::R<bool, REGION111_A>;
impl REGION111_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> REGION111_A {
        match self.bits {
            false => REGION111_A::DISABLED,
            true => REGION111_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == REGION111_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == REGION111_A::ENABLED
    }
}
#[doc = "Write proxy for field `REGION111`"]
pub struct REGION111_W<'a> {
    w: &'a mut W,
}
impl<'a> REGION111_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: REGION111_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Protection disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(REGION111_A::DISABLED)
    }
    #[doc = "Protection enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(REGION111_A::ENABLED)
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
#[doc = "Enable protection for region 112. Write '0' has no effect.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum REGION112_A {
    #[doc = "0: Protection disabled"]
    DISABLED = 0,
    #[doc = "1: Protection enabled"]
    ENABLED = 1,
}
impl From<REGION112_A> for bool {
    #[inline(always)]
    fn from(variant: REGION112_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `REGION112`"]
pub type REGION112_R = crate::R<bool, REGION112_A>;
impl REGION112_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> REGION112_A {
        match self.bits {
            false => REGION112_A::DISABLED,
            true => REGION112_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == REGION112_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == REGION112_A::ENABLED
    }
}
#[doc = "Write proxy for field `REGION112`"]
pub struct REGION112_W<'a> {
    w: &'a mut W,
}
impl<'a> REGION112_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: REGION112_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Protection disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(REGION112_A::DISABLED)
    }
    #[doc = "Protection enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(REGION112_A::ENABLED)
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
#[doc = "Enable protection for region 113. Write '0' has no effect.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum REGION113_A {
    #[doc = "0: Protection disabled"]
    DISABLED = 0,
    #[doc = "1: Protection enabled"]
    ENABLED = 1,
}
impl From<REGION113_A> for bool {
    #[inline(always)]
    fn from(variant: REGION113_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `REGION113`"]
pub type REGION113_R = crate::R<bool, REGION113_A>;
impl REGION113_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> REGION113_A {
        match self.bits {
            false => REGION113_A::DISABLED,
            true => REGION113_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == REGION113_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == REGION113_A::ENABLED
    }
}
#[doc = "Write proxy for field `REGION113`"]
pub struct REGION113_W<'a> {
    w: &'a mut W,
}
impl<'a> REGION113_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: REGION113_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Protection disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(REGION113_A::DISABLED)
    }
    #[doc = "Protection enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(REGION113_A::ENABLED)
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
#[doc = "Enable protection for region 114. Write '0' has no effect.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum REGION114_A {
    #[doc = "0: Protection disabled"]
    DISABLED = 0,
    #[doc = "1: Protection enabled"]
    ENABLED = 1,
}
impl From<REGION114_A> for bool {
    #[inline(always)]
    fn from(variant: REGION114_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `REGION114`"]
pub type REGION114_R = crate::R<bool, REGION114_A>;
impl REGION114_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> REGION114_A {
        match self.bits {
            false => REGION114_A::DISABLED,
            true => REGION114_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == REGION114_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == REGION114_A::ENABLED
    }
}
#[doc = "Write proxy for field `REGION114`"]
pub struct REGION114_W<'a> {
    w: &'a mut W,
}
impl<'a> REGION114_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: REGION114_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Protection disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(REGION114_A::DISABLED)
    }
    #[doc = "Protection enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(REGION114_A::ENABLED)
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
#[doc = "Enable protection for region 115. Write '0' has no effect.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum REGION115_A {
    #[doc = "0: Protection disabled"]
    DISABLED = 0,
    #[doc = "1: Protection enabled"]
    ENABLED = 1,
}
impl From<REGION115_A> for bool {
    #[inline(always)]
    fn from(variant: REGION115_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `REGION115`"]
pub type REGION115_R = crate::R<bool, REGION115_A>;
impl REGION115_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> REGION115_A {
        match self.bits {
            false => REGION115_A::DISABLED,
            true => REGION115_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == REGION115_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == REGION115_A::ENABLED
    }
}
#[doc = "Write proxy for field `REGION115`"]
pub struct REGION115_W<'a> {
    w: &'a mut W,
}
impl<'a> REGION115_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: REGION115_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Protection disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(REGION115_A::DISABLED)
    }
    #[doc = "Protection enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(REGION115_A::ENABLED)
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
#[doc = "Enable protection for region 116. Write '0' has no effect.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum REGION116_A {
    #[doc = "0: Protection disabled"]
    DISABLED = 0,
    #[doc = "1: Protection enabled"]
    ENABLED = 1,
}
impl From<REGION116_A> for bool {
    #[inline(always)]
    fn from(variant: REGION116_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `REGION116`"]
pub type REGION116_R = crate::R<bool, REGION116_A>;
impl REGION116_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> REGION116_A {
        match self.bits {
            false => REGION116_A::DISABLED,
            true => REGION116_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == REGION116_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == REGION116_A::ENABLED
    }
}
#[doc = "Write proxy for field `REGION116`"]
pub struct REGION116_W<'a> {
    w: &'a mut W,
}
impl<'a> REGION116_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: REGION116_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Protection disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(REGION116_A::DISABLED)
    }
    #[doc = "Protection enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(REGION116_A::ENABLED)
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
#[doc = "Enable protection for region 117. Write '0' has no effect.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum REGION117_A {
    #[doc = "0: Protection disabled"]
    DISABLED = 0,
    #[doc = "1: Protection enabled"]
    ENABLED = 1,
}
impl From<REGION117_A> for bool {
    #[inline(always)]
    fn from(variant: REGION117_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `REGION117`"]
pub type REGION117_R = crate::R<bool, REGION117_A>;
impl REGION117_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> REGION117_A {
        match self.bits {
            false => REGION117_A::DISABLED,
            true => REGION117_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == REGION117_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == REGION117_A::ENABLED
    }
}
#[doc = "Write proxy for field `REGION117`"]
pub struct REGION117_W<'a> {
    w: &'a mut W,
}
impl<'a> REGION117_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: REGION117_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Protection disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(REGION117_A::DISABLED)
    }
    #[doc = "Protection enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(REGION117_A::ENABLED)
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
#[doc = "Enable protection for region 118. Write '0' has no effect.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum REGION118_A {
    #[doc = "0: Protection disabled"]
    DISABLED = 0,
    #[doc = "1: Protection enabled"]
    ENABLED = 1,
}
impl From<REGION118_A> for bool {
    #[inline(always)]
    fn from(variant: REGION118_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `REGION118`"]
pub type REGION118_R = crate::R<bool, REGION118_A>;
impl REGION118_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> REGION118_A {
        match self.bits {
            false => REGION118_A::DISABLED,
            true => REGION118_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == REGION118_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == REGION118_A::ENABLED
    }
}
#[doc = "Write proxy for field `REGION118`"]
pub struct REGION118_W<'a> {
    w: &'a mut W,
}
impl<'a> REGION118_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: REGION118_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Protection disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(REGION118_A::DISABLED)
    }
    #[doc = "Protection enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(REGION118_A::ENABLED)
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
#[doc = "Enable protection for region 119. Write '0' has no effect.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum REGION119_A {
    #[doc = "0: Protection disabled"]
    DISABLED = 0,
    #[doc = "1: Protection enabled"]
    ENABLED = 1,
}
impl From<REGION119_A> for bool {
    #[inline(always)]
    fn from(variant: REGION119_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `REGION119`"]
pub type REGION119_R = crate::R<bool, REGION119_A>;
impl REGION119_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> REGION119_A {
        match self.bits {
            false => REGION119_A::DISABLED,
            true => REGION119_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == REGION119_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == REGION119_A::ENABLED
    }
}
#[doc = "Write proxy for field `REGION119`"]
pub struct REGION119_W<'a> {
    w: &'a mut W,
}
impl<'a> REGION119_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: REGION119_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Protection disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(REGION119_A::DISABLED)
    }
    #[doc = "Protection enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(REGION119_A::ENABLED)
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
#[doc = "Enable protection for region 120. Write '0' has no effect.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum REGION120_A {
    #[doc = "0: Protection disabled"]
    DISABLED = 0,
    #[doc = "1: Protection enabled"]
    ENABLED = 1,
}
impl From<REGION120_A> for bool {
    #[inline(always)]
    fn from(variant: REGION120_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `REGION120`"]
pub type REGION120_R = crate::R<bool, REGION120_A>;
impl REGION120_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> REGION120_A {
        match self.bits {
            false => REGION120_A::DISABLED,
            true => REGION120_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == REGION120_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == REGION120_A::ENABLED
    }
}
#[doc = "Write proxy for field `REGION120`"]
pub struct REGION120_W<'a> {
    w: &'a mut W,
}
impl<'a> REGION120_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: REGION120_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Protection disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(REGION120_A::DISABLED)
    }
    #[doc = "Protection enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(REGION120_A::ENABLED)
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
#[doc = "Enable protection for region 121. Write '0' has no effect.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum REGION121_A {
    #[doc = "0: Protection disabled"]
    DISABLED = 0,
    #[doc = "1: Protection enabled"]
    ENABLED = 1,
}
impl From<REGION121_A> for bool {
    #[inline(always)]
    fn from(variant: REGION121_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `REGION121`"]
pub type REGION121_R = crate::R<bool, REGION121_A>;
impl REGION121_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> REGION121_A {
        match self.bits {
            false => REGION121_A::DISABLED,
            true => REGION121_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == REGION121_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == REGION121_A::ENABLED
    }
}
#[doc = "Write proxy for field `REGION121`"]
pub struct REGION121_W<'a> {
    w: &'a mut W,
}
impl<'a> REGION121_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: REGION121_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Protection disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(REGION121_A::DISABLED)
    }
    #[doc = "Protection enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(REGION121_A::ENABLED)
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
#[doc = "Enable protection for region 122. Write '0' has no effect.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum REGION122_A {
    #[doc = "0: Protection disabled"]
    DISABLED = 0,
    #[doc = "1: Protection enabled"]
    ENABLED = 1,
}
impl From<REGION122_A> for bool {
    #[inline(always)]
    fn from(variant: REGION122_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `REGION122`"]
pub type REGION122_R = crate::R<bool, REGION122_A>;
impl REGION122_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> REGION122_A {
        match self.bits {
            false => REGION122_A::DISABLED,
            true => REGION122_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == REGION122_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == REGION122_A::ENABLED
    }
}
#[doc = "Write proxy for field `REGION122`"]
pub struct REGION122_W<'a> {
    w: &'a mut W,
}
impl<'a> REGION122_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: REGION122_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Protection disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(REGION122_A::DISABLED)
    }
    #[doc = "Protection enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(REGION122_A::ENABLED)
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
#[doc = "Enable protection for region 123. Write '0' has no effect.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum REGION123_A {
    #[doc = "0: Protection disabled"]
    DISABLED = 0,
    #[doc = "1: Protection enabled"]
    ENABLED = 1,
}
impl From<REGION123_A> for bool {
    #[inline(always)]
    fn from(variant: REGION123_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `REGION123`"]
pub type REGION123_R = crate::R<bool, REGION123_A>;
impl REGION123_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> REGION123_A {
        match self.bits {
            false => REGION123_A::DISABLED,
            true => REGION123_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == REGION123_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == REGION123_A::ENABLED
    }
}
#[doc = "Write proxy for field `REGION123`"]
pub struct REGION123_W<'a> {
    w: &'a mut W,
}
impl<'a> REGION123_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: REGION123_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Protection disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(REGION123_A::DISABLED)
    }
    #[doc = "Protection enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(REGION123_A::ENABLED)
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
#[doc = "Enable protection for region 124. Write '0' has no effect.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum REGION124_A {
    #[doc = "0: Protection disabled"]
    DISABLED = 0,
    #[doc = "1: Protection enabled"]
    ENABLED = 1,
}
impl From<REGION124_A> for bool {
    #[inline(always)]
    fn from(variant: REGION124_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `REGION124`"]
pub type REGION124_R = crate::R<bool, REGION124_A>;
impl REGION124_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> REGION124_A {
        match self.bits {
            false => REGION124_A::DISABLED,
            true => REGION124_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == REGION124_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == REGION124_A::ENABLED
    }
}
#[doc = "Write proxy for field `REGION124`"]
pub struct REGION124_W<'a> {
    w: &'a mut W,
}
impl<'a> REGION124_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: REGION124_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Protection disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(REGION124_A::DISABLED)
    }
    #[doc = "Protection enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(REGION124_A::ENABLED)
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
#[doc = "Enable protection for region 125. Write '0' has no effect.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum REGION125_A {
    #[doc = "0: Protection disabled"]
    DISABLED = 0,
    #[doc = "1: Protection enabled"]
    ENABLED = 1,
}
impl From<REGION125_A> for bool {
    #[inline(always)]
    fn from(variant: REGION125_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `REGION125`"]
pub type REGION125_R = crate::R<bool, REGION125_A>;
impl REGION125_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> REGION125_A {
        match self.bits {
            false => REGION125_A::DISABLED,
            true => REGION125_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == REGION125_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == REGION125_A::ENABLED
    }
}
#[doc = "Write proxy for field `REGION125`"]
pub struct REGION125_W<'a> {
    w: &'a mut W,
}
impl<'a> REGION125_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: REGION125_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Protection disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(REGION125_A::DISABLED)
    }
    #[doc = "Protection enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(REGION125_A::ENABLED)
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
#[doc = "Enable protection for region 126. Write '0' has no effect.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum REGION126_A {
    #[doc = "0: Protection disabled"]
    DISABLED = 0,
    #[doc = "1: Protection enabled"]
    ENABLED = 1,
}
impl From<REGION126_A> for bool {
    #[inline(always)]
    fn from(variant: REGION126_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `REGION126`"]
pub type REGION126_R = crate::R<bool, REGION126_A>;
impl REGION126_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> REGION126_A {
        match self.bits {
            false => REGION126_A::DISABLED,
            true => REGION126_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == REGION126_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == REGION126_A::ENABLED
    }
}
#[doc = "Write proxy for field `REGION126`"]
pub struct REGION126_W<'a> {
    w: &'a mut W,
}
impl<'a> REGION126_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: REGION126_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Protection disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(REGION126_A::DISABLED)
    }
    #[doc = "Protection enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(REGION126_A::ENABLED)
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
#[doc = "Enable protection for region 127. Write '0' has no effect.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum REGION127_A {
    #[doc = "0: Protection disabled"]
    DISABLED = 0,
    #[doc = "1: Protection enabled"]
    ENABLED = 1,
}
impl From<REGION127_A> for bool {
    #[inline(always)]
    fn from(variant: REGION127_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `REGION127`"]
pub type REGION127_R = crate::R<bool, REGION127_A>;
impl REGION127_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> REGION127_A {
        match self.bits {
            false => REGION127_A::DISABLED,
            true => REGION127_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == REGION127_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == REGION127_A::ENABLED
    }
}
#[doc = "Write proxy for field `REGION127`"]
pub struct REGION127_W<'a> {
    w: &'a mut W,
}
impl<'a> REGION127_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: REGION127_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Protection disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(REGION127_A::DISABLED)
    }
    #[doc = "Protection enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(REGION127_A::ENABLED)
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
    #[doc = "Bit 0 - Enable protection for region 96. Write '0' has no effect."]
    #[inline(always)]
    pub fn region96(&self) -> REGION96_R {
        REGION96_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - Enable protection for region 97. Write '0' has no effect."]
    #[inline(always)]
    pub fn region97(&self) -> REGION97_R {
        REGION97_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - Enable protection for region 98. Write '0' has no effect."]
    #[inline(always)]
    pub fn region98(&self) -> REGION98_R {
        REGION98_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - Enable protection for region 99. Write '0' has no effect."]
    #[inline(always)]
    pub fn region99(&self) -> REGION99_R {
        REGION99_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - Enable protection for region 100. Write '0' has no effect."]
    #[inline(always)]
    pub fn region100(&self) -> REGION100_R {
        REGION100_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - Enable protection for region 101. Write '0' has no effect."]
    #[inline(always)]
    pub fn region101(&self) -> REGION101_R {
        REGION101_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - Enable protection for region 102. Write '0' has no effect."]
    #[inline(always)]
    pub fn region102(&self) -> REGION102_R {
        REGION102_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7 - Enable protection for region 103. Write '0' has no effect."]
    #[inline(always)]
    pub fn region103(&self) -> REGION103_R {
        REGION103_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 8 - Enable protection for region 104. Write '0' has no effect."]
    #[inline(always)]
    pub fn region104(&self) -> REGION104_R {
        REGION104_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 9 - Enable protection for region 105. Write '0' has no effect."]
    #[inline(always)]
    pub fn region105(&self) -> REGION105_R {
        REGION105_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 10 - Enable protection for region 106. Write '0' has no effect."]
    #[inline(always)]
    pub fn region106(&self) -> REGION106_R {
        REGION106_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    #[doc = "Bit 11 - Enable protection for region 107. Write '0' has no effect."]
    #[inline(always)]
    pub fn region107(&self) -> REGION107_R {
        REGION107_R::new(((self.bits >> 11) & 0x01) != 0)
    }
    #[doc = "Bit 12 - Enable protection for region 108. Write '0' has no effect."]
    #[inline(always)]
    pub fn region108(&self) -> REGION108_R {
        REGION108_R::new(((self.bits >> 12) & 0x01) != 0)
    }
    #[doc = "Bit 13 - Enable protection for region 109. Write '0' has no effect."]
    #[inline(always)]
    pub fn region109(&self) -> REGION109_R {
        REGION109_R::new(((self.bits >> 13) & 0x01) != 0)
    }
    #[doc = "Bit 14 - Enable protection for region 110. Write '0' has no effect."]
    #[inline(always)]
    pub fn region110(&self) -> REGION110_R {
        REGION110_R::new(((self.bits >> 14) & 0x01) != 0)
    }
    #[doc = "Bit 15 - Enable protection for region 111. Write '0' has no effect."]
    #[inline(always)]
    pub fn region111(&self) -> REGION111_R {
        REGION111_R::new(((self.bits >> 15) & 0x01) != 0)
    }
    #[doc = "Bit 16 - Enable protection for region 112. Write '0' has no effect."]
    #[inline(always)]
    pub fn region112(&self) -> REGION112_R {
        REGION112_R::new(((self.bits >> 16) & 0x01) != 0)
    }
    #[doc = "Bit 17 - Enable protection for region 113. Write '0' has no effect."]
    #[inline(always)]
    pub fn region113(&self) -> REGION113_R {
        REGION113_R::new(((self.bits >> 17) & 0x01) != 0)
    }
    #[doc = "Bit 18 - Enable protection for region 114. Write '0' has no effect."]
    #[inline(always)]
    pub fn region114(&self) -> REGION114_R {
        REGION114_R::new(((self.bits >> 18) & 0x01) != 0)
    }
    #[doc = "Bit 19 - Enable protection for region 115. Write '0' has no effect."]
    #[inline(always)]
    pub fn region115(&self) -> REGION115_R {
        REGION115_R::new(((self.bits >> 19) & 0x01) != 0)
    }
    #[doc = "Bit 20 - Enable protection for region 116. Write '0' has no effect."]
    #[inline(always)]
    pub fn region116(&self) -> REGION116_R {
        REGION116_R::new(((self.bits >> 20) & 0x01) != 0)
    }
    #[doc = "Bit 21 - Enable protection for region 117. Write '0' has no effect."]
    #[inline(always)]
    pub fn region117(&self) -> REGION117_R {
        REGION117_R::new(((self.bits >> 21) & 0x01) != 0)
    }
    #[doc = "Bit 22 - Enable protection for region 118. Write '0' has no effect."]
    #[inline(always)]
    pub fn region118(&self) -> REGION118_R {
        REGION118_R::new(((self.bits >> 22) & 0x01) != 0)
    }
    #[doc = "Bit 23 - Enable protection for region 119. Write '0' has no effect."]
    #[inline(always)]
    pub fn region119(&self) -> REGION119_R {
        REGION119_R::new(((self.bits >> 23) & 0x01) != 0)
    }
    #[doc = "Bit 24 - Enable protection for region 120. Write '0' has no effect."]
    #[inline(always)]
    pub fn region120(&self) -> REGION120_R {
        REGION120_R::new(((self.bits >> 24) & 0x01) != 0)
    }
    #[doc = "Bit 25 - Enable protection for region 121. Write '0' has no effect."]
    #[inline(always)]
    pub fn region121(&self) -> REGION121_R {
        REGION121_R::new(((self.bits >> 25) & 0x01) != 0)
    }
    #[doc = "Bit 26 - Enable protection for region 122. Write '0' has no effect."]
    #[inline(always)]
    pub fn region122(&self) -> REGION122_R {
        REGION122_R::new(((self.bits >> 26) & 0x01) != 0)
    }
    #[doc = "Bit 27 - Enable protection for region 123. Write '0' has no effect."]
    #[inline(always)]
    pub fn region123(&self) -> REGION123_R {
        REGION123_R::new(((self.bits >> 27) & 0x01) != 0)
    }
    #[doc = "Bit 28 - Enable protection for region 124. Write '0' has no effect."]
    #[inline(always)]
    pub fn region124(&self) -> REGION124_R {
        REGION124_R::new(((self.bits >> 28) & 0x01) != 0)
    }
    #[doc = "Bit 29 - Enable protection for region 125. Write '0' has no effect."]
    #[inline(always)]
    pub fn region125(&self) -> REGION125_R {
        REGION125_R::new(((self.bits >> 29) & 0x01) != 0)
    }
    #[doc = "Bit 30 - Enable protection for region 126. Write '0' has no effect."]
    #[inline(always)]
    pub fn region126(&self) -> REGION126_R {
        REGION126_R::new(((self.bits >> 30) & 0x01) != 0)
    }
    #[doc = "Bit 31 - Enable protection for region 127. Write '0' has no effect."]
    #[inline(always)]
    pub fn region127(&self) -> REGION127_R {
        REGION127_R::new(((self.bits >> 31) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Enable protection for region 96. Write '0' has no effect."]
    #[inline(always)]
    pub fn region96(&mut self) -> REGION96_W {
        REGION96_W { w: self }
    }
    #[doc = "Bit 1 - Enable protection for region 97. Write '0' has no effect."]
    #[inline(always)]
    pub fn region97(&mut self) -> REGION97_W {
        REGION97_W { w: self }
    }
    #[doc = "Bit 2 - Enable protection for region 98. Write '0' has no effect."]
    #[inline(always)]
    pub fn region98(&mut self) -> REGION98_W {
        REGION98_W { w: self }
    }
    #[doc = "Bit 3 - Enable protection for region 99. Write '0' has no effect."]
    #[inline(always)]
    pub fn region99(&mut self) -> REGION99_W {
        REGION99_W { w: self }
    }
    #[doc = "Bit 4 - Enable protection for region 100. Write '0' has no effect."]
    #[inline(always)]
    pub fn region100(&mut self) -> REGION100_W {
        REGION100_W { w: self }
    }
    #[doc = "Bit 5 - Enable protection for region 101. Write '0' has no effect."]
    #[inline(always)]
    pub fn region101(&mut self) -> REGION101_W {
        REGION101_W { w: self }
    }
    #[doc = "Bit 6 - Enable protection for region 102. Write '0' has no effect."]
    #[inline(always)]
    pub fn region102(&mut self) -> REGION102_W {
        REGION102_W { w: self }
    }
    #[doc = "Bit 7 - Enable protection for region 103. Write '0' has no effect."]
    #[inline(always)]
    pub fn region103(&mut self) -> REGION103_W {
        REGION103_W { w: self }
    }
    #[doc = "Bit 8 - Enable protection for region 104. Write '0' has no effect."]
    #[inline(always)]
    pub fn region104(&mut self) -> REGION104_W {
        REGION104_W { w: self }
    }
    #[doc = "Bit 9 - Enable protection for region 105. Write '0' has no effect."]
    #[inline(always)]
    pub fn region105(&mut self) -> REGION105_W {
        REGION105_W { w: self }
    }
    #[doc = "Bit 10 - Enable protection for region 106. Write '0' has no effect."]
    #[inline(always)]
    pub fn region106(&mut self) -> REGION106_W {
        REGION106_W { w: self }
    }
    #[doc = "Bit 11 - Enable protection for region 107. Write '0' has no effect."]
    #[inline(always)]
    pub fn region107(&mut self) -> REGION107_W {
        REGION107_W { w: self }
    }
    #[doc = "Bit 12 - Enable protection for region 108. Write '0' has no effect."]
    #[inline(always)]
    pub fn region108(&mut self) -> REGION108_W {
        REGION108_W { w: self }
    }
    #[doc = "Bit 13 - Enable protection for region 109. Write '0' has no effect."]
    #[inline(always)]
    pub fn region109(&mut self) -> REGION109_W {
        REGION109_W { w: self }
    }
    #[doc = "Bit 14 - Enable protection for region 110. Write '0' has no effect."]
    #[inline(always)]
    pub fn region110(&mut self) -> REGION110_W {
        REGION110_W { w: self }
    }
    #[doc = "Bit 15 - Enable protection for region 111. Write '0' has no effect."]
    #[inline(always)]
    pub fn region111(&mut self) -> REGION111_W {
        REGION111_W { w: self }
    }
    #[doc = "Bit 16 - Enable protection for region 112. Write '0' has no effect."]
    #[inline(always)]
    pub fn region112(&mut self) -> REGION112_W {
        REGION112_W { w: self }
    }
    #[doc = "Bit 17 - Enable protection for region 113. Write '0' has no effect."]
    #[inline(always)]
    pub fn region113(&mut self) -> REGION113_W {
        REGION113_W { w: self }
    }
    #[doc = "Bit 18 - Enable protection for region 114. Write '0' has no effect."]
    #[inline(always)]
    pub fn region114(&mut self) -> REGION114_W {
        REGION114_W { w: self }
    }
    #[doc = "Bit 19 - Enable protection for region 115. Write '0' has no effect."]
    #[inline(always)]
    pub fn region115(&mut self) -> REGION115_W {
        REGION115_W { w: self }
    }
    #[doc = "Bit 20 - Enable protection for region 116. Write '0' has no effect."]
    #[inline(always)]
    pub fn region116(&mut self) -> REGION116_W {
        REGION116_W { w: self }
    }
    #[doc = "Bit 21 - Enable protection for region 117. Write '0' has no effect."]
    #[inline(always)]
    pub fn region117(&mut self) -> REGION117_W {
        REGION117_W { w: self }
    }
    #[doc = "Bit 22 - Enable protection for region 118. Write '0' has no effect."]
    #[inline(always)]
    pub fn region118(&mut self) -> REGION118_W {
        REGION118_W { w: self }
    }
    #[doc = "Bit 23 - Enable protection for region 119. Write '0' has no effect."]
    #[inline(always)]
    pub fn region119(&mut self) -> REGION119_W {
        REGION119_W { w: self }
    }
    #[doc = "Bit 24 - Enable protection for region 120. Write '0' has no effect."]
    #[inline(always)]
    pub fn region120(&mut self) -> REGION120_W {
        REGION120_W { w: self }
    }
    #[doc = "Bit 25 - Enable protection for region 121. Write '0' has no effect."]
    #[inline(always)]
    pub fn region121(&mut self) -> REGION121_W {
        REGION121_W { w: self }
    }
    #[doc = "Bit 26 - Enable protection for region 122. Write '0' has no effect."]
    #[inline(always)]
    pub fn region122(&mut self) -> REGION122_W {
        REGION122_W { w: self }
    }
    #[doc = "Bit 27 - Enable protection for region 123. Write '0' has no effect."]
    #[inline(always)]
    pub fn region123(&mut self) -> REGION123_W {
        REGION123_W { w: self }
    }
    #[doc = "Bit 28 - Enable protection for region 124. Write '0' has no effect."]
    #[inline(always)]
    pub fn region124(&mut self) -> REGION124_W {
        REGION124_W { w: self }
    }
    #[doc = "Bit 29 - Enable protection for region 125. Write '0' has no effect."]
    #[inline(always)]
    pub fn region125(&mut self) -> REGION125_W {
        REGION125_W { w: self }
    }
    #[doc = "Bit 30 - Enable protection for region 126. Write '0' has no effect."]
    #[inline(always)]
    pub fn region126(&mut self) -> REGION126_W {
        REGION126_W { w: self }
    }
    #[doc = "Bit 31 - Enable protection for region 127. Write '0' has no effect."]
    #[inline(always)]
    pub fn region127(&mut self) -> REGION127_W {
        REGION127_W { w: self }
    }
}
