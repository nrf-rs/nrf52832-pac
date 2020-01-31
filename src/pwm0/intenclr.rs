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
#[doc = "Write '1' to Disable interrupt for STOPPED event\n\nValue on reset: 0"]
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
#[doc = "Write '1' to Disable interrupt for STOPPED event\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum STOPPED_AW {
    #[doc = "1: Disable"]
    CLEAR = 1,
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
    #[doc = "Disable"]
    #[inline(always)]
    pub fn clear(self) -> &'a mut W {
        self.variant(STOPPED_AW::CLEAR)
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
#[doc = "Write '1' to Disable interrupt for SEQSTARTED\\[0\\]
event\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SEQSTARTED0_A {
    #[doc = "0: Read: Disabled"]
    DISABLED = 0,
    #[doc = "1: Read: Enabled"]
    ENABLED = 1,
}
impl From<SEQSTARTED0_A> for bool {
    #[inline(always)]
    fn from(variant: SEQSTARTED0_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `SEQSTARTED0`"]
pub type SEQSTARTED0_R = crate::R<bool, SEQSTARTED0_A>;
impl SEQSTARTED0_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SEQSTARTED0_A {
        match self.bits {
            false => SEQSTARTED0_A::DISABLED,
            true => SEQSTARTED0_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == SEQSTARTED0_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == SEQSTARTED0_A::ENABLED
    }
}
#[doc = "Write '1' to Disable interrupt for SEQSTARTED\\[0\\]
event\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SEQSTARTED0_AW {
    #[doc = "1: Disable"]
    CLEAR = 1,
}
impl From<SEQSTARTED0_AW> for bool {
    #[inline(always)]
    fn from(variant: SEQSTARTED0_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Write proxy for field `SEQSTARTED0`"]
pub struct SEQSTARTED0_W<'a> {
    w: &'a mut W,
}
impl<'a> SEQSTARTED0_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SEQSTARTED0_AW) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Disable"]
    #[inline(always)]
    pub fn clear(self) -> &'a mut W {
        self.variant(SEQSTARTED0_AW::CLEAR)
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
#[doc = "Write '1' to Disable interrupt for SEQSTARTED\\[1\\]
event\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SEQSTARTED1_A {
    #[doc = "0: Read: Disabled"]
    DISABLED = 0,
    #[doc = "1: Read: Enabled"]
    ENABLED = 1,
}
impl From<SEQSTARTED1_A> for bool {
    #[inline(always)]
    fn from(variant: SEQSTARTED1_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `SEQSTARTED1`"]
pub type SEQSTARTED1_R = crate::R<bool, SEQSTARTED1_A>;
impl SEQSTARTED1_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SEQSTARTED1_A {
        match self.bits {
            false => SEQSTARTED1_A::DISABLED,
            true => SEQSTARTED1_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == SEQSTARTED1_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == SEQSTARTED1_A::ENABLED
    }
}
#[doc = "Write '1' to Disable interrupt for SEQSTARTED\\[1\\]
event\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SEQSTARTED1_AW {
    #[doc = "1: Disable"]
    CLEAR = 1,
}
impl From<SEQSTARTED1_AW> for bool {
    #[inline(always)]
    fn from(variant: SEQSTARTED1_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Write proxy for field `SEQSTARTED1`"]
pub struct SEQSTARTED1_W<'a> {
    w: &'a mut W,
}
impl<'a> SEQSTARTED1_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SEQSTARTED1_AW) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Disable"]
    #[inline(always)]
    pub fn clear(self) -> &'a mut W {
        self.variant(SEQSTARTED1_AW::CLEAR)
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
#[doc = "Write '1' to Disable interrupt for SEQEND\\[0\\]
event\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SEQEND0_A {
    #[doc = "0: Read: Disabled"]
    DISABLED = 0,
    #[doc = "1: Read: Enabled"]
    ENABLED = 1,
}
impl From<SEQEND0_A> for bool {
    #[inline(always)]
    fn from(variant: SEQEND0_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `SEQEND0`"]
pub type SEQEND0_R = crate::R<bool, SEQEND0_A>;
impl SEQEND0_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SEQEND0_A {
        match self.bits {
            false => SEQEND0_A::DISABLED,
            true => SEQEND0_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == SEQEND0_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == SEQEND0_A::ENABLED
    }
}
#[doc = "Write '1' to Disable interrupt for SEQEND\\[0\\]
event\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SEQEND0_AW {
    #[doc = "1: Disable"]
    CLEAR = 1,
}
impl From<SEQEND0_AW> for bool {
    #[inline(always)]
    fn from(variant: SEQEND0_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Write proxy for field `SEQEND0`"]
pub struct SEQEND0_W<'a> {
    w: &'a mut W,
}
impl<'a> SEQEND0_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SEQEND0_AW) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Disable"]
    #[inline(always)]
    pub fn clear(self) -> &'a mut W {
        self.variant(SEQEND0_AW::CLEAR)
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
#[doc = "Write '1' to Disable interrupt for SEQEND\\[1\\]
event\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SEQEND1_A {
    #[doc = "0: Read: Disabled"]
    DISABLED = 0,
    #[doc = "1: Read: Enabled"]
    ENABLED = 1,
}
impl From<SEQEND1_A> for bool {
    #[inline(always)]
    fn from(variant: SEQEND1_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `SEQEND1`"]
pub type SEQEND1_R = crate::R<bool, SEQEND1_A>;
impl SEQEND1_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SEQEND1_A {
        match self.bits {
            false => SEQEND1_A::DISABLED,
            true => SEQEND1_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == SEQEND1_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == SEQEND1_A::ENABLED
    }
}
#[doc = "Write '1' to Disable interrupt for SEQEND\\[1\\]
event\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SEQEND1_AW {
    #[doc = "1: Disable"]
    CLEAR = 1,
}
impl From<SEQEND1_AW> for bool {
    #[inline(always)]
    fn from(variant: SEQEND1_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Write proxy for field `SEQEND1`"]
pub struct SEQEND1_W<'a> {
    w: &'a mut W,
}
impl<'a> SEQEND1_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SEQEND1_AW) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Disable"]
    #[inline(always)]
    pub fn clear(self) -> &'a mut W {
        self.variant(SEQEND1_AW::CLEAR)
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
#[doc = "Write '1' to Disable interrupt for PWMPERIODEND event\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PWMPERIODEND_A {
    #[doc = "0: Read: Disabled"]
    DISABLED = 0,
    #[doc = "1: Read: Enabled"]
    ENABLED = 1,
}
impl From<PWMPERIODEND_A> for bool {
    #[inline(always)]
    fn from(variant: PWMPERIODEND_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `PWMPERIODEND`"]
pub type PWMPERIODEND_R = crate::R<bool, PWMPERIODEND_A>;
impl PWMPERIODEND_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PWMPERIODEND_A {
        match self.bits {
            false => PWMPERIODEND_A::DISABLED,
            true => PWMPERIODEND_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == PWMPERIODEND_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == PWMPERIODEND_A::ENABLED
    }
}
#[doc = "Write '1' to Disable interrupt for PWMPERIODEND event\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PWMPERIODEND_AW {
    #[doc = "1: Disable"]
    CLEAR = 1,
}
impl From<PWMPERIODEND_AW> for bool {
    #[inline(always)]
    fn from(variant: PWMPERIODEND_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Write proxy for field `PWMPERIODEND`"]
pub struct PWMPERIODEND_W<'a> {
    w: &'a mut W,
}
impl<'a> PWMPERIODEND_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PWMPERIODEND_AW) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Disable"]
    #[inline(always)]
    pub fn clear(self) -> &'a mut W {
        self.variant(PWMPERIODEND_AW::CLEAR)
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
#[doc = "Write '1' to Disable interrupt for LOOPSDONE event\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum LOOPSDONE_A {
    #[doc = "0: Read: Disabled"]
    DISABLED = 0,
    #[doc = "1: Read: Enabled"]
    ENABLED = 1,
}
impl From<LOOPSDONE_A> for bool {
    #[inline(always)]
    fn from(variant: LOOPSDONE_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `LOOPSDONE`"]
pub type LOOPSDONE_R = crate::R<bool, LOOPSDONE_A>;
impl LOOPSDONE_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> LOOPSDONE_A {
        match self.bits {
            false => LOOPSDONE_A::DISABLED,
            true => LOOPSDONE_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == LOOPSDONE_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == LOOPSDONE_A::ENABLED
    }
}
#[doc = "Write '1' to Disable interrupt for LOOPSDONE event\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum LOOPSDONE_AW {
    #[doc = "1: Disable"]
    CLEAR = 1,
}
impl From<LOOPSDONE_AW> for bool {
    #[inline(always)]
    fn from(variant: LOOPSDONE_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Write proxy for field `LOOPSDONE`"]
pub struct LOOPSDONE_W<'a> {
    w: &'a mut W,
}
impl<'a> LOOPSDONE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: LOOPSDONE_AW) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Disable"]
    #[inline(always)]
    pub fn clear(self) -> &'a mut W {
        self.variant(LOOPSDONE_AW::CLEAR)
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
impl R {
    #[doc = "Bit 1 - Write '1' to Disable interrupt for STOPPED event"]
    #[inline(always)]
    pub fn stopped(&self) -> STOPPED_R {
        STOPPED_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - Write '1' to Disable interrupt for SEQSTARTED\\[0\\]
event"]
    #[inline(always)]
    pub fn seqstarted0(&self) -> SEQSTARTED0_R {
        SEQSTARTED0_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - Write '1' to Disable interrupt for SEQSTARTED\\[1\\]
event"]
    #[inline(always)]
    pub fn seqstarted1(&self) -> SEQSTARTED1_R {
        SEQSTARTED1_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - Write '1' to Disable interrupt for SEQEND\\[0\\]
event"]
    #[inline(always)]
    pub fn seqend0(&self) -> SEQEND0_R {
        SEQEND0_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - Write '1' to Disable interrupt for SEQEND\\[1\\]
event"]
    #[inline(always)]
    pub fn seqend1(&self) -> SEQEND1_R {
        SEQEND1_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - Write '1' to Disable interrupt for PWMPERIODEND event"]
    #[inline(always)]
    pub fn pwmperiodend(&self) -> PWMPERIODEND_R {
        PWMPERIODEND_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7 - Write '1' to Disable interrupt for LOOPSDONE event"]
    #[inline(always)]
    pub fn loopsdone(&self) -> LOOPSDONE_R {
        LOOPSDONE_R::new(((self.bits >> 7) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 1 - Write '1' to Disable interrupt for STOPPED event"]
    #[inline(always)]
    pub fn stopped(&mut self) -> STOPPED_W {
        STOPPED_W { w: self }
    }
    #[doc = "Bit 2 - Write '1' to Disable interrupt for SEQSTARTED\\[0\\]
event"]
    #[inline(always)]
    pub fn seqstarted0(&mut self) -> SEQSTARTED0_W {
        SEQSTARTED0_W { w: self }
    }
    #[doc = "Bit 3 - Write '1' to Disable interrupt for SEQSTARTED\\[1\\]
event"]
    #[inline(always)]
    pub fn seqstarted1(&mut self) -> SEQSTARTED1_W {
        SEQSTARTED1_W { w: self }
    }
    #[doc = "Bit 4 - Write '1' to Disable interrupt for SEQEND\\[0\\]
event"]
    #[inline(always)]
    pub fn seqend0(&mut self) -> SEQEND0_W {
        SEQEND0_W { w: self }
    }
    #[doc = "Bit 5 - Write '1' to Disable interrupt for SEQEND\\[1\\]
event"]
    #[inline(always)]
    pub fn seqend1(&mut self) -> SEQEND1_W {
        SEQEND1_W { w: self }
    }
    #[doc = "Bit 6 - Write '1' to Disable interrupt for PWMPERIODEND event"]
    #[inline(always)]
    pub fn pwmperiodend(&mut self) -> PWMPERIODEND_W {
        PWMPERIODEND_W { w: self }
    }
    #[doc = "Bit 7 - Write '1' to Disable interrupt for LOOPSDONE event"]
    #[inline(always)]
    pub fn loopsdone(&mut self) -> LOOPSDONE_W {
        LOOPSDONE_W { w: self }
    }
}
