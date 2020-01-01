#[doc = "Reader of register DACNF"]
pub type R = crate::R<u32, super::DACNF>;
#[doc = "Writer for register DACNF"]
pub type W = crate::W<u32, super::DACNF>;
#[doc = "Register DACNF `reset()`'s with value 0"]
impl crate::ResetValue for super::DACNF {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Enable or disable device address matching using device address 0\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ENA0_A {
    #[doc = "0: Disabled"]
    DISABLED = 0,
    #[doc = "1: Enabled"]
    ENABLED = 1,
}
impl From<ENA0_A> for bool {
    #[inline(always)]
    fn from(variant: ENA0_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `ENA0`"]
pub type ENA0_R = crate::R<bool, ENA0_A>;
impl ENA0_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ENA0_A {
        match self.bits {
            false => ENA0_A::DISABLED,
            true => ENA0_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == ENA0_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == ENA0_A::ENABLED
    }
}
#[doc = "Write proxy for field `ENA0`"]
pub struct ENA0_W<'a> {
    w: &'a mut W,
}
impl<'a> ENA0_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: ENA0_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(ENA0_A::DISABLED)
    }
    #[doc = "Enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(ENA0_A::ENABLED)
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
#[doc = "Enable or disable device address matching using device address 1\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ENA1_A {
    #[doc = "0: Disabled"]
    DISABLED = 0,
    #[doc = "1: Enabled"]
    ENABLED = 1,
}
impl From<ENA1_A> for bool {
    #[inline(always)]
    fn from(variant: ENA1_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `ENA1`"]
pub type ENA1_R = crate::R<bool, ENA1_A>;
impl ENA1_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ENA1_A {
        match self.bits {
            false => ENA1_A::DISABLED,
            true => ENA1_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == ENA1_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == ENA1_A::ENABLED
    }
}
#[doc = "Write proxy for field `ENA1`"]
pub struct ENA1_W<'a> {
    w: &'a mut W,
}
impl<'a> ENA1_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: ENA1_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(ENA1_A::DISABLED)
    }
    #[doc = "Enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(ENA1_A::ENABLED)
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
#[doc = "Enable or disable device address matching using device address 2\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ENA2_A {
    #[doc = "0: Disabled"]
    DISABLED = 0,
    #[doc = "1: Enabled"]
    ENABLED = 1,
}
impl From<ENA2_A> for bool {
    #[inline(always)]
    fn from(variant: ENA2_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `ENA2`"]
pub type ENA2_R = crate::R<bool, ENA2_A>;
impl ENA2_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ENA2_A {
        match self.bits {
            false => ENA2_A::DISABLED,
            true => ENA2_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == ENA2_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == ENA2_A::ENABLED
    }
}
#[doc = "Write proxy for field `ENA2`"]
pub struct ENA2_W<'a> {
    w: &'a mut W,
}
impl<'a> ENA2_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: ENA2_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(ENA2_A::DISABLED)
    }
    #[doc = "Enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(ENA2_A::ENABLED)
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
#[doc = "Enable or disable device address matching using device address 3\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ENA3_A {
    #[doc = "0: Disabled"]
    DISABLED = 0,
    #[doc = "1: Enabled"]
    ENABLED = 1,
}
impl From<ENA3_A> for bool {
    #[inline(always)]
    fn from(variant: ENA3_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `ENA3`"]
pub type ENA3_R = crate::R<bool, ENA3_A>;
impl ENA3_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ENA3_A {
        match self.bits {
            false => ENA3_A::DISABLED,
            true => ENA3_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == ENA3_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == ENA3_A::ENABLED
    }
}
#[doc = "Write proxy for field `ENA3`"]
pub struct ENA3_W<'a> {
    w: &'a mut W,
}
impl<'a> ENA3_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: ENA3_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(ENA3_A::DISABLED)
    }
    #[doc = "Enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(ENA3_A::ENABLED)
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
#[doc = "Enable or disable device address matching using device address 4\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ENA4_A {
    #[doc = "0: Disabled"]
    DISABLED = 0,
    #[doc = "1: Enabled"]
    ENABLED = 1,
}
impl From<ENA4_A> for bool {
    #[inline(always)]
    fn from(variant: ENA4_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `ENA4`"]
pub type ENA4_R = crate::R<bool, ENA4_A>;
impl ENA4_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ENA4_A {
        match self.bits {
            false => ENA4_A::DISABLED,
            true => ENA4_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == ENA4_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == ENA4_A::ENABLED
    }
}
#[doc = "Write proxy for field `ENA4`"]
pub struct ENA4_W<'a> {
    w: &'a mut W,
}
impl<'a> ENA4_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: ENA4_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(ENA4_A::DISABLED)
    }
    #[doc = "Enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(ENA4_A::ENABLED)
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
#[doc = "Enable or disable device address matching using device address 5\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ENA5_A {
    #[doc = "0: Disabled"]
    DISABLED = 0,
    #[doc = "1: Enabled"]
    ENABLED = 1,
}
impl From<ENA5_A> for bool {
    #[inline(always)]
    fn from(variant: ENA5_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `ENA5`"]
pub type ENA5_R = crate::R<bool, ENA5_A>;
impl ENA5_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ENA5_A {
        match self.bits {
            false => ENA5_A::DISABLED,
            true => ENA5_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == ENA5_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == ENA5_A::ENABLED
    }
}
#[doc = "Write proxy for field `ENA5`"]
pub struct ENA5_W<'a> {
    w: &'a mut W,
}
impl<'a> ENA5_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: ENA5_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(ENA5_A::DISABLED)
    }
    #[doc = "Enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(ENA5_A::ENABLED)
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
#[doc = "Enable or disable device address matching using device address 6\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ENA6_A {
    #[doc = "0: Disabled"]
    DISABLED = 0,
    #[doc = "1: Enabled"]
    ENABLED = 1,
}
impl From<ENA6_A> for bool {
    #[inline(always)]
    fn from(variant: ENA6_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `ENA6`"]
pub type ENA6_R = crate::R<bool, ENA6_A>;
impl ENA6_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ENA6_A {
        match self.bits {
            false => ENA6_A::DISABLED,
            true => ENA6_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == ENA6_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == ENA6_A::ENABLED
    }
}
#[doc = "Write proxy for field `ENA6`"]
pub struct ENA6_W<'a> {
    w: &'a mut W,
}
impl<'a> ENA6_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: ENA6_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(ENA6_A::DISABLED)
    }
    #[doc = "Enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(ENA6_A::ENABLED)
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
#[doc = "Enable or disable device address matching using device address 7\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ENA7_A {
    #[doc = "0: Disabled"]
    DISABLED = 0,
    #[doc = "1: Enabled"]
    ENABLED = 1,
}
impl From<ENA7_A> for bool {
    #[inline(always)]
    fn from(variant: ENA7_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `ENA7`"]
pub type ENA7_R = crate::R<bool, ENA7_A>;
impl ENA7_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ENA7_A {
        match self.bits {
            false => ENA7_A::DISABLED,
            true => ENA7_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == ENA7_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == ENA7_A::ENABLED
    }
}
#[doc = "Write proxy for field `ENA7`"]
pub struct ENA7_W<'a> {
    w: &'a mut W,
}
impl<'a> ENA7_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: ENA7_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(ENA7_A::DISABLED)
    }
    #[doc = "Enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(ENA7_A::ENABLED)
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
#[doc = "Reader of field `TXADD0`"]
pub type TXADD0_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `TXADD0`"]
pub struct TXADD0_W<'a> {
    w: &'a mut W,
}
impl<'a> TXADD0_W<'a> {
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
#[doc = "Reader of field `TXADD1`"]
pub type TXADD1_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `TXADD1`"]
pub struct TXADD1_W<'a> {
    w: &'a mut W,
}
impl<'a> TXADD1_W<'a> {
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
#[doc = "Reader of field `TXADD2`"]
pub type TXADD2_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `TXADD2`"]
pub struct TXADD2_W<'a> {
    w: &'a mut W,
}
impl<'a> TXADD2_W<'a> {
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
#[doc = "Reader of field `TXADD3`"]
pub type TXADD3_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `TXADD3`"]
pub struct TXADD3_W<'a> {
    w: &'a mut W,
}
impl<'a> TXADD3_W<'a> {
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
#[doc = "Reader of field `TXADD4`"]
pub type TXADD4_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `TXADD4`"]
pub struct TXADD4_W<'a> {
    w: &'a mut W,
}
impl<'a> TXADD4_W<'a> {
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
#[doc = "Reader of field `TXADD5`"]
pub type TXADD5_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `TXADD5`"]
pub struct TXADD5_W<'a> {
    w: &'a mut W,
}
impl<'a> TXADD5_W<'a> {
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
#[doc = "Reader of field `TXADD6`"]
pub type TXADD6_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `TXADD6`"]
pub struct TXADD6_W<'a> {
    w: &'a mut W,
}
impl<'a> TXADD6_W<'a> {
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
#[doc = "Reader of field `TXADD7`"]
pub type TXADD7_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `TXADD7`"]
pub struct TXADD7_W<'a> {
    w: &'a mut W,
}
impl<'a> TXADD7_W<'a> {
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
    #[doc = "Bit 0 - Enable or disable device address matching using device address 0"]
    #[inline(always)]
    pub fn ena0(&self) -> ENA0_R {
        ENA0_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - Enable or disable device address matching using device address 1"]
    #[inline(always)]
    pub fn ena1(&self) -> ENA1_R {
        ENA1_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - Enable or disable device address matching using device address 2"]
    #[inline(always)]
    pub fn ena2(&self) -> ENA2_R {
        ENA2_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - Enable or disable device address matching using device address 3"]
    #[inline(always)]
    pub fn ena3(&self) -> ENA3_R {
        ENA3_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - Enable or disable device address matching using device address 4"]
    #[inline(always)]
    pub fn ena4(&self) -> ENA4_R {
        ENA4_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - Enable or disable device address matching using device address 5"]
    #[inline(always)]
    pub fn ena5(&self) -> ENA5_R {
        ENA5_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - Enable or disable device address matching using device address 6"]
    #[inline(always)]
    pub fn ena6(&self) -> ENA6_R {
        ENA6_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7 - Enable or disable device address matching using device address 7"]
    #[inline(always)]
    pub fn ena7(&self) -> ENA7_R {
        ENA7_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 8 - TxAdd for device address 0"]
    #[inline(always)]
    pub fn txadd0(&self) -> TXADD0_R {
        TXADD0_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 9 - TxAdd for device address 1"]
    #[inline(always)]
    pub fn txadd1(&self) -> TXADD1_R {
        TXADD1_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 10 - TxAdd for device address 2"]
    #[inline(always)]
    pub fn txadd2(&self) -> TXADD2_R {
        TXADD2_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    #[doc = "Bit 11 - TxAdd for device address 3"]
    #[inline(always)]
    pub fn txadd3(&self) -> TXADD3_R {
        TXADD3_R::new(((self.bits >> 11) & 0x01) != 0)
    }
    #[doc = "Bit 12 - TxAdd for device address 4"]
    #[inline(always)]
    pub fn txadd4(&self) -> TXADD4_R {
        TXADD4_R::new(((self.bits >> 12) & 0x01) != 0)
    }
    #[doc = "Bit 13 - TxAdd for device address 5"]
    #[inline(always)]
    pub fn txadd5(&self) -> TXADD5_R {
        TXADD5_R::new(((self.bits >> 13) & 0x01) != 0)
    }
    #[doc = "Bit 14 - TxAdd for device address 6"]
    #[inline(always)]
    pub fn txadd6(&self) -> TXADD6_R {
        TXADD6_R::new(((self.bits >> 14) & 0x01) != 0)
    }
    #[doc = "Bit 15 - TxAdd for device address 7"]
    #[inline(always)]
    pub fn txadd7(&self) -> TXADD7_R {
        TXADD7_R::new(((self.bits >> 15) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Enable or disable device address matching using device address 0"]
    #[inline(always)]
    pub fn ena0(&mut self) -> ENA0_W {
        ENA0_W { w: self }
    }
    #[doc = "Bit 1 - Enable or disable device address matching using device address 1"]
    #[inline(always)]
    pub fn ena1(&mut self) -> ENA1_W {
        ENA1_W { w: self }
    }
    #[doc = "Bit 2 - Enable or disable device address matching using device address 2"]
    #[inline(always)]
    pub fn ena2(&mut self) -> ENA2_W {
        ENA2_W { w: self }
    }
    #[doc = "Bit 3 - Enable or disable device address matching using device address 3"]
    #[inline(always)]
    pub fn ena3(&mut self) -> ENA3_W {
        ENA3_W { w: self }
    }
    #[doc = "Bit 4 - Enable or disable device address matching using device address 4"]
    #[inline(always)]
    pub fn ena4(&mut self) -> ENA4_W {
        ENA4_W { w: self }
    }
    #[doc = "Bit 5 - Enable or disable device address matching using device address 5"]
    #[inline(always)]
    pub fn ena5(&mut self) -> ENA5_W {
        ENA5_W { w: self }
    }
    #[doc = "Bit 6 - Enable or disable device address matching using device address 6"]
    #[inline(always)]
    pub fn ena6(&mut self) -> ENA6_W {
        ENA6_W { w: self }
    }
    #[doc = "Bit 7 - Enable or disable device address matching using device address 7"]
    #[inline(always)]
    pub fn ena7(&mut self) -> ENA7_W {
        ENA7_W { w: self }
    }
    #[doc = "Bit 8 - TxAdd for device address 0"]
    #[inline(always)]
    pub fn txadd0(&mut self) -> TXADD0_W {
        TXADD0_W { w: self }
    }
    #[doc = "Bit 9 - TxAdd for device address 1"]
    #[inline(always)]
    pub fn txadd1(&mut self) -> TXADD1_W {
        TXADD1_W { w: self }
    }
    #[doc = "Bit 10 - TxAdd for device address 2"]
    #[inline(always)]
    pub fn txadd2(&mut self) -> TXADD2_W {
        TXADD2_W { w: self }
    }
    #[doc = "Bit 11 - TxAdd for device address 3"]
    #[inline(always)]
    pub fn txadd3(&mut self) -> TXADD3_W {
        TXADD3_W { w: self }
    }
    #[doc = "Bit 12 - TxAdd for device address 4"]
    #[inline(always)]
    pub fn txadd4(&mut self) -> TXADD4_W {
        TXADD4_W { w: self }
    }
    #[doc = "Bit 13 - TxAdd for device address 5"]
    #[inline(always)]
    pub fn txadd5(&mut self) -> TXADD5_W {
        TXADD5_W { w: self }
    }
    #[doc = "Bit 14 - TxAdd for device address 6"]
    #[inline(always)]
    pub fn txadd6(&mut self) -> TXADD6_W {
        TXADD6_W { w: self }
    }
    #[doc = "Bit 15 - TxAdd for device address 7"]
    #[inline(always)]
    pub fn txadd7(&mut self) -> TXADD7_W {
        TXADD7_W { w: self }
    }
}
