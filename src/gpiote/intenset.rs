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
#[doc = "Write '1' to Enable interrupt for IN\\[0\\]
event\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum IN0_A {
    #[doc = "0: Read: Disabled"]
    DISABLED = 0,
    #[doc = "1: Read: Enabled"]
    ENABLED = 1,
}
impl From<IN0_A> for bool {
    #[inline(always)]
    fn from(variant: IN0_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `IN0`"]
pub type IN0_R = crate::R<bool, IN0_A>;
impl IN0_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> IN0_A {
        match self.bits {
            false => IN0_A::DISABLED,
            true => IN0_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == IN0_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == IN0_A::ENABLED
    }
}
#[doc = "Write '1' to Enable interrupt for IN\\[0\\]
event\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum IN0_AW {
    #[doc = "1: Enable"]
    SET = 1,
}
impl From<IN0_AW> for bool {
    #[inline(always)]
    fn from(variant: IN0_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Write proxy for field `IN0`"]
pub struct IN0_W<'a> {
    w: &'a mut W,
}
impl<'a> IN0_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: IN0_AW) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Enable"]
    #[inline(always)]
    pub fn set(self) -> &'a mut W {
        self.variant(IN0_AW::SET)
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
#[doc = "Write '1' to Enable interrupt for IN\\[1\\]
event\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum IN1_A {
    #[doc = "0: Read: Disabled"]
    DISABLED = 0,
    #[doc = "1: Read: Enabled"]
    ENABLED = 1,
}
impl From<IN1_A> for bool {
    #[inline(always)]
    fn from(variant: IN1_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `IN1`"]
pub type IN1_R = crate::R<bool, IN1_A>;
impl IN1_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> IN1_A {
        match self.bits {
            false => IN1_A::DISABLED,
            true => IN1_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == IN1_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == IN1_A::ENABLED
    }
}
#[doc = "Write '1' to Enable interrupt for IN\\[1\\]
event\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum IN1_AW {
    #[doc = "1: Enable"]
    SET = 1,
}
impl From<IN1_AW> for bool {
    #[inline(always)]
    fn from(variant: IN1_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Write proxy for field `IN1`"]
pub struct IN1_W<'a> {
    w: &'a mut W,
}
impl<'a> IN1_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: IN1_AW) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Enable"]
    #[inline(always)]
    pub fn set(self) -> &'a mut W {
        self.variant(IN1_AW::SET)
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
#[doc = "Write '1' to Enable interrupt for IN\\[2\\]
event\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum IN2_A {
    #[doc = "0: Read: Disabled"]
    DISABLED = 0,
    #[doc = "1: Read: Enabled"]
    ENABLED = 1,
}
impl From<IN2_A> for bool {
    #[inline(always)]
    fn from(variant: IN2_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `IN2`"]
pub type IN2_R = crate::R<bool, IN2_A>;
impl IN2_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> IN2_A {
        match self.bits {
            false => IN2_A::DISABLED,
            true => IN2_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == IN2_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == IN2_A::ENABLED
    }
}
#[doc = "Write '1' to Enable interrupt for IN\\[2\\]
event\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum IN2_AW {
    #[doc = "1: Enable"]
    SET = 1,
}
impl From<IN2_AW> for bool {
    #[inline(always)]
    fn from(variant: IN2_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Write proxy for field `IN2`"]
pub struct IN2_W<'a> {
    w: &'a mut W,
}
impl<'a> IN2_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: IN2_AW) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Enable"]
    #[inline(always)]
    pub fn set(self) -> &'a mut W {
        self.variant(IN2_AW::SET)
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
#[doc = "Write '1' to Enable interrupt for IN\\[3\\]
event\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum IN3_A {
    #[doc = "0: Read: Disabled"]
    DISABLED = 0,
    #[doc = "1: Read: Enabled"]
    ENABLED = 1,
}
impl From<IN3_A> for bool {
    #[inline(always)]
    fn from(variant: IN3_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `IN3`"]
pub type IN3_R = crate::R<bool, IN3_A>;
impl IN3_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> IN3_A {
        match self.bits {
            false => IN3_A::DISABLED,
            true => IN3_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == IN3_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == IN3_A::ENABLED
    }
}
#[doc = "Write '1' to Enable interrupt for IN\\[3\\]
event\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum IN3_AW {
    #[doc = "1: Enable"]
    SET = 1,
}
impl From<IN3_AW> for bool {
    #[inline(always)]
    fn from(variant: IN3_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Write proxy for field `IN3`"]
pub struct IN3_W<'a> {
    w: &'a mut W,
}
impl<'a> IN3_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: IN3_AW) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Enable"]
    #[inline(always)]
    pub fn set(self) -> &'a mut W {
        self.variant(IN3_AW::SET)
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
#[doc = "Write '1' to Enable interrupt for IN\\[4\\]
event\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum IN4_A {
    #[doc = "0: Read: Disabled"]
    DISABLED = 0,
    #[doc = "1: Read: Enabled"]
    ENABLED = 1,
}
impl From<IN4_A> for bool {
    #[inline(always)]
    fn from(variant: IN4_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `IN4`"]
pub type IN4_R = crate::R<bool, IN4_A>;
impl IN4_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> IN4_A {
        match self.bits {
            false => IN4_A::DISABLED,
            true => IN4_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == IN4_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == IN4_A::ENABLED
    }
}
#[doc = "Write '1' to Enable interrupt for IN\\[4\\]
event\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum IN4_AW {
    #[doc = "1: Enable"]
    SET = 1,
}
impl From<IN4_AW> for bool {
    #[inline(always)]
    fn from(variant: IN4_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Write proxy for field `IN4`"]
pub struct IN4_W<'a> {
    w: &'a mut W,
}
impl<'a> IN4_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: IN4_AW) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Enable"]
    #[inline(always)]
    pub fn set(self) -> &'a mut W {
        self.variant(IN4_AW::SET)
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
#[doc = "Write '1' to Enable interrupt for IN\\[5\\]
event\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum IN5_A {
    #[doc = "0: Read: Disabled"]
    DISABLED = 0,
    #[doc = "1: Read: Enabled"]
    ENABLED = 1,
}
impl From<IN5_A> for bool {
    #[inline(always)]
    fn from(variant: IN5_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `IN5`"]
pub type IN5_R = crate::R<bool, IN5_A>;
impl IN5_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> IN5_A {
        match self.bits {
            false => IN5_A::DISABLED,
            true => IN5_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == IN5_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == IN5_A::ENABLED
    }
}
#[doc = "Write '1' to Enable interrupt for IN\\[5\\]
event\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum IN5_AW {
    #[doc = "1: Enable"]
    SET = 1,
}
impl From<IN5_AW> for bool {
    #[inline(always)]
    fn from(variant: IN5_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Write proxy for field `IN5`"]
pub struct IN5_W<'a> {
    w: &'a mut W,
}
impl<'a> IN5_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: IN5_AW) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Enable"]
    #[inline(always)]
    pub fn set(self) -> &'a mut W {
        self.variant(IN5_AW::SET)
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
#[doc = "Write '1' to Enable interrupt for IN\\[6\\]
event\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum IN6_A {
    #[doc = "0: Read: Disabled"]
    DISABLED = 0,
    #[doc = "1: Read: Enabled"]
    ENABLED = 1,
}
impl From<IN6_A> for bool {
    #[inline(always)]
    fn from(variant: IN6_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `IN6`"]
pub type IN6_R = crate::R<bool, IN6_A>;
impl IN6_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> IN6_A {
        match self.bits {
            false => IN6_A::DISABLED,
            true => IN6_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == IN6_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == IN6_A::ENABLED
    }
}
#[doc = "Write '1' to Enable interrupt for IN\\[6\\]
event\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum IN6_AW {
    #[doc = "1: Enable"]
    SET = 1,
}
impl From<IN6_AW> for bool {
    #[inline(always)]
    fn from(variant: IN6_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Write proxy for field `IN6`"]
pub struct IN6_W<'a> {
    w: &'a mut W,
}
impl<'a> IN6_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: IN6_AW) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Enable"]
    #[inline(always)]
    pub fn set(self) -> &'a mut W {
        self.variant(IN6_AW::SET)
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
#[doc = "Write '1' to Enable interrupt for IN\\[7\\]
event\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum IN7_A {
    #[doc = "0: Read: Disabled"]
    DISABLED = 0,
    #[doc = "1: Read: Enabled"]
    ENABLED = 1,
}
impl From<IN7_A> for bool {
    #[inline(always)]
    fn from(variant: IN7_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `IN7`"]
pub type IN7_R = crate::R<bool, IN7_A>;
impl IN7_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> IN7_A {
        match self.bits {
            false => IN7_A::DISABLED,
            true => IN7_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == IN7_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == IN7_A::ENABLED
    }
}
#[doc = "Write '1' to Enable interrupt for IN\\[7\\]
event\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum IN7_AW {
    #[doc = "1: Enable"]
    SET = 1,
}
impl From<IN7_AW> for bool {
    #[inline(always)]
    fn from(variant: IN7_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Write proxy for field `IN7`"]
pub struct IN7_W<'a> {
    w: &'a mut W,
}
impl<'a> IN7_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: IN7_AW) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Enable"]
    #[inline(always)]
    pub fn set(self) -> &'a mut W {
        self.variant(IN7_AW::SET)
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
#[doc = "Write '1' to Enable interrupt for PORT event\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PORT_A {
    #[doc = "0: Read: Disabled"]
    DISABLED = 0,
    #[doc = "1: Read: Enabled"]
    ENABLED = 1,
}
impl From<PORT_A> for bool {
    #[inline(always)]
    fn from(variant: PORT_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `PORT`"]
pub type PORT_R = crate::R<bool, PORT_A>;
impl PORT_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PORT_A {
        match self.bits {
            false => PORT_A::DISABLED,
            true => PORT_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == PORT_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == PORT_A::ENABLED
    }
}
#[doc = "Write '1' to Enable interrupt for PORT event\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PORT_AW {
    #[doc = "1: Enable"]
    SET = 1,
}
impl From<PORT_AW> for bool {
    #[inline(always)]
    fn from(variant: PORT_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Write proxy for field `PORT`"]
pub struct PORT_W<'a> {
    w: &'a mut W,
}
impl<'a> PORT_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PORT_AW) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Enable"]
    #[inline(always)]
    pub fn set(self) -> &'a mut W {
        self.variant(PORT_AW::SET)
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
    #[doc = "Bit 0 - Write '1' to Enable interrupt for IN\\[0\\]
event"]
    #[inline(always)]
    pub fn in0(&self) -> IN0_R {
        IN0_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - Write '1' to Enable interrupt for IN\\[1\\]
event"]
    #[inline(always)]
    pub fn in1(&self) -> IN1_R {
        IN1_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - Write '1' to Enable interrupt for IN\\[2\\]
event"]
    #[inline(always)]
    pub fn in2(&self) -> IN2_R {
        IN2_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - Write '1' to Enable interrupt for IN\\[3\\]
event"]
    #[inline(always)]
    pub fn in3(&self) -> IN3_R {
        IN3_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - Write '1' to Enable interrupt for IN\\[4\\]
event"]
    #[inline(always)]
    pub fn in4(&self) -> IN4_R {
        IN4_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - Write '1' to Enable interrupt for IN\\[5\\]
event"]
    #[inline(always)]
    pub fn in5(&self) -> IN5_R {
        IN5_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - Write '1' to Enable interrupt for IN\\[6\\]
event"]
    #[inline(always)]
    pub fn in6(&self) -> IN6_R {
        IN6_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7 - Write '1' to Enable interrupt for IN\\[7\\]
event"]
    #[inline(always)]
    pub fn in7(&self) -> IN7_R {
        IN7_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 31 - Write '1' to Enable interrupt for PORT event"]
    #[inline(always)]
    pub fn port(&self) -> PORT_R {
        PORT_R::new(((self.bits >> 31) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Write '1' to Enable interrupt for IN\\[0\\]
event"]
    #[inline(always)]
    pub fn in0(&mut self) -> IN0_W {
        IN0_W { w: self }
    }
    #[doc = "Bit 1 - Write '1' to Enable interrupt for IN\\[1\\]
event"]
    #[inline(always)]
    pub fn in1(&mut self) -> IN1_W {
        IN1_W { w: self }
    }
    #[doc = "Bit 2 - Write '1' to Enable interrupt for IN\\[2\\]
event"]
    #[inline(always)]
    pub fn in2(&mut self) -> IN2_W {
        IN2_W { w: self }
    }
    #[doc = "Bit 3 - Write '1' to Enable interrupt for IN\\[3\\]
event"]
    #[inline(always)]
    pub fn in3(&mut self) -> IN3_W {
        IN3_W { w: self }
    }
    #[doc = "Bit 4 - Write '1' to Enable interrupt for IN\\[4\\]
event"]
    #[inline(always)]
    pub fn in4(&mut self) -> IN4_W {
        IN4_W { w: self }
    }
    #[doc = "Bit 5 - Write '1' to Enable interrupt for IN\\[5\\]
event"]
    #[inline(always)]
    pub fn in5(&mut self) -> IN5_W {
        IN5_W { w: self }
    }
    #[doc = "Bit 6 - Write '1' to Enable interrupt for IN\\[6\\]
event"]
    #[inline(always)]
    pub fn in6(&mut self) -> IN6_W {
        IN6_W { w: self }
    }
    #[doc = "Bit 7 - Write '1' to Enable interrupt for IN\\[7\\]
event"]
    #[inline(always)]
    pub fn in7(&mut self) -> IN7_W {
        IN7_W { w: self }
    }
    #[doc = "Bit 31 - Write '1' to Enable interrupt for PORT event"]
    #[inline(always)]
    pub fn port(&mut self) -> PORT_W {
        PORT_W { w: self }
    }
}
