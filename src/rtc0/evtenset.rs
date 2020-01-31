#[doc = "Reader of register EVTENSET"]
pub type R = crate::R<u32, super::EVTENSET>;
#[doc = "Writer for register EVTENSET"]
pub type W = crate::W<u32, super::EVTENSET>;
#[doc = "Register EVTENSET `reset()`'s with value 0"]
impl crate::ResetValue for super::EVTENSET {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Write '1' to Enable event routing for TICK event\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TICK_A {
    #[doc = "0: Read: Disabled"]
    DISABLED = 0,
    #[doc = "1: Read: Enabled"]
    ENABLED = 1,
}
impl From<TICK_A> for bool {
    #[inline(always)]
    fn from(variant: TICK_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `TICK`"]
pub type TICK_R = crate::R<bool, TICK_A>;
impl TICK_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TICK_A {
        match self.bits {
            false => TICK_A::DISABLED,
            true => TICK_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == TICK_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == TICK_A::ENABLED
    }
}
#[doc = "Write '1' to Enable event routing for TICK event\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TICK_AW {
    #[doc = "1: Enable"]
    SET = 1,
}
impl From<TICK_AW> for bool {
    #[inline(always)]
    fn from(variant: TICK_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Write proxy for field `TICK`"]
pub struct TICK_W<'a> {
    w: &'a mut W,
}
impl<'a> TICK_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: TICK_AW) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Enable"]
    #[inline(always)]
    pub fn set(self) -> &'a mut W {
        self.variant(TICK_AW::SET)
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
#[doc = "Write '1' to Enable event routing for OVRFLW event\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum OVRFLW_A {
    #[doc = "0: Read: Disabled"]
    DISABLED = 0,
    #[doc = "1: Read: Enabled"]
    ENABLED = 1,
}
impl From<OVRFLW_A> for bool {
    #[inline(always)]
    fn from(variant: OVRFLW_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `OVRFLW`"]
pub type OVRFLW_R = crate::R<bool, OVRFLW_A>;
impl OVRFLW_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> OVRFLW_A {
        match self.bits {
            false => OVRFLW_A::DISABLED,
            true => OVRFLW_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == OVRFLW_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == OVRFLW_A::ENABLED
    }
}
#[doc = "Write '1' to Enable event routing for OVRFLW event\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum OVRFLW_AW {
    #[doc = "1: Enable"]
    SET = 1,
}
impl From<OVRFLW_AW> for bool {
    #[inline(always)]
    fn from(variant: OVRFLW_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Write proxy for field `OVRFLW`"]
pub struct OVRFLW_W<'a> {
    w: &'a mut W,
}
impl<'a> OVRFLW_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: OVRFLW_AW) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Enable"]
    #[inline(always)]
    pub fn set(self) -> &'a mut W {
        self.variant(OVRFLW_AW::SET)
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
#[doc = "Write '1' to Enable event routing for COMPARE\\[0\\]
event\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum COMPARE0_A {
    #[doc = "0: Read: Disabled"]
    DISABLED = 0,
    #[doc = "1: Read: Enabled"]
    ENABLED = 1,
}
impl From<COMPARE0_A> for bool {
    #[inline(always)]
    fn from(variant: COMPARE0_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `COMPARE0`"]
pub type COMPARE0_R = crate::R<bool, COMPARE0_A>;
impl COMPARE0_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> COMPARE0_A {
        match self.bits {
            false => COMPARE0_A::DISABLED,
            true => COMPARE0_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == COMPARE0_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == COMPARE0_A::ENABLED
    }
}
#[doc = "Write '1' to Enable event routing for COMPARE\\[0\\]
event\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum COMPARE0_AW {
    #[doc = "1: Enable"]
    SET = 1,
}
impl From<COMPARE0_AW> for bool {
    #[inline(always)]
    fn from(variant: COMPARE0_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Write proxy for field `COMPARE0`"]
pub struct COMPARE0_W<'a> {
    w: &'a mut W,
}
impl<'a> COMPARE0_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: COMPARE0_AW) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Enable"]
    #[inline(always)]
    pub fn set(self) -> &'a mut W {
        self.variant(COMPARE0_AW::SET)
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
#[doc = "Write '1' to Enable event routing for COMPARE\\[1\\]
event\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum COMPARE1_A {
    #[doc = "0: Read: Disabled"]
    DISABLED = 0,
    #[doc = "1: Read: Enabled"]
    ENABLED = 1,
}
impl From<COMPARE1_A> for bool {
    #[inline(always)]
    fn from(variant: COMPARE1_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `COMPARE1`"]
pub type COMPARE1_R = crate::R<bool, COMPARE1_A>;
impl COMPARE1_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> COMPARE1_A {
        match self.bits {
            false => COMPARE1_A::DISABLED,
            true => COMPARE1_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == COMPARE1_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == COMPARE1_A::ENABLED
    }
}
#[doc = "Write '1' to Enable event routing for COMPARE\\[1\\]
event\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum COMPARE1_AW {
    #[doc = "1: Enable"]
    SET = 1,
}
impl From<COMPARE1_AW> for bool {
    #[inline(always)]
    fn from(variant: COMPARE1_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Write proxy for field `COMPARE1`"]
pub struct COMPARE1_W<'a> {
    w: &'a mut W,
}
impl<'a> COMPARE1_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: COMPARE1_AW) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Enable"]
    #[inline(always)]
    pub fn set(self) -> &'a mut W {
        self.variant(COMPARE1_AW::SET)
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
#[doc = "Write '1' to Enable event routing for COMPARE\\[2\\]
event\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum COMPARE2_A {
    #[doc = "0: Read: Disabled"]
    DISABLED = 0,
    #[doc = "1: Read: Enabled"]
    ENABLED = 1,
}
impl From<COMPARE2_A> for bool {
    #[inline(always)]
    fn from(variant: COMPARE2_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `COMPARE2`"]
pub type COMPARE2_R = crate::R<bool, COMPARE2_A>;
impl COMPARE2_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> COMPARE2_A {
        match self.bits {
            false => COMPARE2_A::DISABLED,
            true => COMPARE2_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == COMPARE2_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == COMPARE2_A::ENABLED
    }
}
#[doc = "Write '1' to Enable event routing for COMPARE\\[2\\]
event\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum COMPARE2_AW {
    #[doc = "1: Enable"]
    SET = 1,
}
impl From<COMPARE2_AW> for bool {
    #[inline(always)]
    fn from(variant: COMPARE2_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Write proxy for field `COMPARE2`"]
pub struct COMPARE2_W<'a> {
    w: &'a mut W,
}
impl<'a> COMPARE2_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: COMPARE2_AW) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Enable"]
    #[inline(always)]
    pub fn set(self) -> &'a mut W {
        self.variant(COMPARE2_AW::SET)
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
#[doc = "Write '1' to Enable event routing for COMPARE\\[3\\]
event\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum COMPARE3_A {
    #[doc = "0: Read: Disabled"]
    DISABLED = 0,
    #[doc = "1: Read: Enabled"]
    ENABLED = 1,
}
impl From<COMPARE3_A> for bool {
    #[inline(always)]
    fn from(variant: COMPARE3_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `COMPARE3`"]
pub type COMPARE3_R = crate::R<bool, COMPARE3_A>;
impl COMPARE3_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> COMPARE3_A {
        match self.bits {
            false => COMPARE3_A::DISABLED,
            true => COMPARE3_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == COMPARE3_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == COMPARE3_A::ENABLED
    }
}
#[doc = "Write '1' to Enable event routing for COMPARE\\[3\\]
event\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum COMPARE3_AW {
    #[doc = "1: Enable"]
    SET = 1,
}
impl From<COMPARE3_AW> for bool {
    #[inline(always)]
    fn from(variant: COMPARE3_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Write proxy for field `COMPARE3`"]
pub struct COMPARE3_W<'a> {
    w: &'a mut W,
}
impl<'a> COMPARE3_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: COMPARE3_AW) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Enable"]
    #[inline(always)]
    pub fn set(self) -> &'a mut W {
        self.variant(COMPARE3_AW::SET)
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
impl R {
    #[doc = "Bit 0 - Write '1' to Enable event routing for TICK event"]
    #[inline(always)]
    pub fn tick(&self) -> TICK_R {
        TICK_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - Write '1' to Enable event routing for OVRFLW event"]
    #[inline(always)]
    pub fn ovrflw(&self) -> OVRFLW_R {
        OVRFLW_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 16 - Write '1' to Enable event routing for COMPARE\\[0\\]
event"]
    #[inline(always)]
    pub fn compare0(&self) -> COMPARE0_R {
        COMPARE0_R::new(((self.bits >> 16) & 0x01) != 0)
    }
    #[doc = "Bit 17 - Write '1' to Enable event routing for COMPARE\\[1\\]
event"]
    #[inline(always)]
    pub fn compare1(&self) -> COMPARE1_R {
        COMPARE1_R::new(((self.bits >> 17) & 0x01) != 0)
    }
    #[doc = "Bit 18 - Write '1' to Enable event routing for COMPARE\\[2\\]
event"]
    #[inline(always)]
    pub fn compare2(&self) -> COMPARE2_R {
        COMPARE2_R::new(((self.bits >> 18) & 0x01) != 0)
    }
    #[doc = "Bit 19 - Write '1' to Enable event routing for COMPARE\\[3\\]
event"]
    #[inline(always)]
    pub fn compare3(&self) -> COMPARE3_R {
        COMPARE3_R::new(((self.bits >> 19) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Write '1' to Enable event routing for TICK event"]
    #[inline(always)]
    pub fn tick(&mut self) -> TICK_W {
        TICK_W { w: self }
    }
    #[doc = "Bit 1 - Write '1' to Enable event routing for OVRFLW event"]
    #[inline(always)]
    pub fn ovrflw(&mut self) -> OVRFLW_W {
        OVRFLW_W { w: self }
    }
    #[doc = "Bit 16 - Write '1' to Enable event routing for COMPARE\\[0\\]
event"]
    #[inline(always)]
    pub fn compare0(&mut self) -> COMPARE0_W {
        COMPARE0_W { w: self }
    }
    #[doc = "Bit 17 - Write '1' to Enable event routing for COMPARE\\[1\\]
event"]
    #[inline(always)]
    pub fn compare1(&mut self) -> COMPARE1_W {
        COMPARE1_W { w: self }
    }
    #[doc = "Bit 18 - Write '1' to Enable event routing for COMPARE\\[2\\]
event"]
    #[inline(always)]
    pub fn compare2(&mut self) -> COMPARE2_W {
        COMPARE2_W { w: self }
    }
    #[doc = "Bit 19 - Write '1' to Enable event routing for COMPARE\\[3\\]
event"]
    #[inline(always)]
    pub fn compare3(&mut self) -> COMPARE3_W {
        COMPARE3_W { w: self }
    }
}
