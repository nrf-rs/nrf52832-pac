#[doc = "Reader of register RXADDRESSES"]
pub type R = crate::R<u32, super::RXADDRESSES>;
#[doc = "Writer for register RXADDRESSES"]
pub type W = crate::W<u32, super::RXADDRESSES>;
#[doc = "Register RXADDRESSES `reset()`'s with value 0"]
impl crate::ResetValue for super::RXADDRESSES {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Enable or disable reception on logical address 0.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ADDR0_A {
    #[doc = "0: Disable"]
    DISABLED = 0,
    #[doc = "1: Enable"]
    ENABLED = 1,
}
impl From<ADDR0_A> for bool {
    #[inline(always)]
    fn from(variant: ADDR0_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `ADDR0`"]
pub type ADDR0_R = crate::R<bool, ADDR0_A>;
impl ADDR0_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ADDR0_A {
        match self.bits {
            false => ADDR0_A::DISABLED,
            true => ADDR0_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == ADDR0_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == ADDR0_A::ENABLED
    }
}
#[doc = "Write proxy for field `ADDR0`"]
pub struct ADDR0_W<'a> {
    w: &'a mut W,
}
impl<'a> ADDR0_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: ADDR0_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Disable"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(ADDR0_A::DISABLED)
    }
    #[doc = "Enable"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(ADDR0_A::ENABLED)
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
#[doc = "Enable or disable reception on logical address 1.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ADDR1_A {
    #[doc = "0: Disable"]
    DISABLED = 0,
    #[doc = "1: Enable"]
    ENABLED = 1,
}
impl From<ADDR1_A> for bool {
    #[inline(always)]
    fn from(variant: ADDR1_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `ADDR1`"]
pub type ADDR1_R = crate::R<bool, ADDR1_A>;
impl ADDR1_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ADDR1_A {
        match self.bits {
            false => ADDR1_A::DISABLED,
            true => ADDR1_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == ADDR1_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == ADDR1_A::ENABLED
    }
}
#[doc = "Write proxy for field `ADDR1`"]
pub struct ADDR1_W<'a> {
    w: &'a mut W,
}
impl<'a> ADDR1_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: ADDR1_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Disable"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(ADDR1_A::DISABLED)
    }
    #[doc = "Enable"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(ADDR1_A::ENABLED)
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
#[doc = "Enable or disable reception on logical address 2.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ADDR2_A {
    #[doc = "0: Disable"]
    DISABLED = 0,
    #[doc = "1: Enable"]
    ENABLED = 1,
}
impl From<ADDR2_A> for bool {
    #[inline(always)]
    fn from(variant: ADDR2_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `ADDR2`"]
pub type ADDR2_R = crate::R<bool, ADDR2_A>;
impl ADDR2_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ADDR2_A {
        match self.bits {
            false => ADDR2_A::DISABLED,
            true => ADDR2_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == ADDR2_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == ADDR2_A::ENABLED
    }
}
#[doc = "Write proxy for field `ADDR2`"]
pub struct ADDR2_W<'a> {
    w: &'a mut W,
}
impl<'a> ADDR2_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: ADDR2_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Disable"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(ADDR2_A::DISABLED)
    }
    #[doc = "Enable"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(ADDR2_A::ENABLED)
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
#[doc = "Enable or disable reception on logical address 3.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ADDR3_A {
    #[doc = "0: Disable"]
    DISABLED = 0,
    #[doc = "1: Enable"]
    ENABLED = 1,
}
impl From<ADDR3_A> for bool {
    #[inline(always)]
    fn from(variant: ADDR3_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `ADDR3`"]
pub type ADDR3_R = crate::R<bool, ADDR3_A>;
impl ADDR3_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ADDR3_A {
        match self.bits {
            false => ADDR3_A::DISABLED,
            true => ADDR3_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == ADDR3_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == ADDR3_A::ENABLED
    }
}
#[doc = "Write proxy for field `ADDR3`"]
pub struct ADDR3_W<'a> {
    w: &'a mut W,
}
impl<'a> ADDR3_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: ADDR3_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Disable"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(ADDR3_A::DISABLED)
    }
    #[doc = "Enable"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(ADDR3_A::ENABLED)
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
#[doc = "Enable or disable reception on logical address 4.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ADDR4_A {
    #[doc = "0: Disable"]
    DISABLED = 0,
    #[doc = "1: Enable"]
    ENABLED = 1,
}
impl From<ADDR4_A> for bool {
    #[inline(always)]
    fn from(variant: ADDR4_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `ADDR4`"]
pub type ADDR4_R = crate::R<bool, ADDR4_A>;
impl ADDR4_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ADDR4_A {
        match self.bits {
            false => ADDR4_A::DISABLED,
            true => ADDR4_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == ADDR4_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == ADDR4_A::ENABLED
    }
}
#[doc = "Write proxy for field `ADDR4`"]
pub struct ADDR4_W<'a> {
    w: &'a mut W,
}
impl<'a> ADDR4_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: ADDR4_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Disable"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(ADDR4_A::DISABLED)
    }
    #[doc = "Enable"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(ADDR4_A::ENABLED)
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
#[doc = "Enable or disable reception on logical address 5.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ADDR5_A {
    #[doc = "0: Disable"]
    DISABLED = 0,
    #[doc = "1: Enable"]
    ENABLED = 1,
}
impl From<ADDR5_A> for bool {
    #[inline(always)]
    fn from(variant: ADDR5_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `ADDR5`"]
pub type ADDR5_R = crate::R<bool, ADDR5_A>;
impl ADDR5_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ADDR5_A {
        match self.bits {
            false => ADDR5_A::DISABLED,
            true => ADDR5_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == ADDR5_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == ADDR5_A::ENABLED
    }
}
#[doc = "Write proxy for field `ADDR5`"]
pub struct ADDR5_W<'a> {
    w: &'a mut W,
}
impl<'a> ADDR5_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: ADDR5_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Disable"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(ADDR5_A::DISABLED)
    }
    #[doc = "Enable"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(ADDR5_A::ENABLED)
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
#[doc = "Enable or disable reception on logical address 6.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ADDR6_A {
    #[doc = "0: Disable"]
    DISABLED = 0,
    #[doc = "1: Enable"]
    ENABLED = 1,
}
impl From<ADDR6_A> for bool {
    #[inline(always)]
    fn from(variant: ADDR6_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `ADDR6`"]
pub type ADDR6_R = crate::R<bool, ADDR6_A>;
impl ADDR6_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ADDR6_A {
        match self.bits {
            false => ADDR6_A::DISABLED,
            true => ADDR6_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == ADDR6_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == ADDR6_A::ENABLED
    }
}
#[doc = "Write proxy for field `ADDR6`"]
pub struct ADDR6_W<'a> {
    w: &'a mut W,
}
impl<'a> ADDR6_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: ADDR6_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Disable"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(ADDR6_A::DISABLED)
    }
    #[doc = "Enable"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(ADDR6_A::ENABLED)
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
#[doc = "Enable or disable reception on logical address 7.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ADDR7_A {
    #[doc = "0: Disable"]
    DISABLED = 0,
    #[doc = "1: Enable"]
    ENABLED = 1,
}
impl From<ADDR7_A> for bool {
    #[inline(always)]
    fn from(variant: ADDR7_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `ADDR7`"]
pub type ADDR7_R = crate::R<bool, ADDR7_A>;
impl ADDR7_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ADDR7_A {
        match self.bits {
            false => ADDR7_A::DISABLED,
            true => ADDR7_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == ADDR7_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == ADDR7_A::ENABLED
    }
}
#[doc = "Write proxy for field `ADDR7`"]
pub struct ADDR7_W<'a> {
    w: &'a mut W,
}
impl<'a> ADDR7_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: ADDR7_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Disable"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(ADDR7_A::DISABLED)
    }
    #[doc = "Enable"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(ADDR7_A::ENABLED)
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
    #[doc = "Bit 0 - Enable or disable reception on logical address 0."]
    #[inline(always)]
    pub fn addr0(&self) -> ADDR0_R {
        ADDR0_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - Enable or disable reception on logical address 1."]
    #[inline(always)]
    pub fn addr1(&self) -> ADDR1_R {
        ADDR1_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - Enable or disable reception on logical address 2."]
    #[inline(always)]
    pub fn addr2(&self) -> ADDR2_R {
        ADDR2_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - Enable or disable reception on logical address 3."]
    #[inline(always)]
    pub fn addr3(&self) -> ADDR3_R {
        ADDR3_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - Enable or disable reception on logical address 4."]
    #[inline(always)]
    pub fn addr4(&self) -> ADDR4_R {
        ADDR4_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - Enable or disable reception on logical address 5."]
    #[inline(always)]
    pub fn addr5(&self) -> ADDR5_R {
        ADDR5_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - Enable or disable reception on logical address 6."]
    #[inline(always)]
    pub fn addr6(&self) -> ADDR6_R {
        ADDR6_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7 - Enable or disable reception on logical address 7."]
    #[inline(always)]
    pub fn addr7(&self) -> ADDR7_R {
        ADDR7_R::new(((self.bits >> 7) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Enable or disable reception on logical address 0."]
    #[inline(always)]
    pub fn addr0(&mut self) -> ADDR0_W {
        ADDR0_W { w: self }
    }
    #[doc = "Bit 1 - Enable or disable reception on logical address 1."]
    #[inline(always)]
    pub fn addr1(&mut self) -> ADDR1_W {
        ADDR1_W { w: self }
    }
    #[doc = "Bit 2 - Enable or disable reception on logical address 2."]
    #[inline(always)]
    pub fn addr2(&mut self) -> ADDR2_W {
        ADDR2_W { w: self }
    }
    #[doc = "Bit 3 - Enable or disable reception on logical address 3."]
    #[inline(always)]
    pub fn addr3(&mut self) -> ADDR3_W {
        ADDR3_W { w: self }
    }
    #[doc = "Bit 4 - Enable or disable reception on logical address 4."]
    #[inline(always)]
    pub fn addr4(&mut self) -> ADDR4_W {
        ADDR4_W { w: self }
    }
    #[doc = "Bit 5 - Enable or disable reception on logical address 5."]
    #[inline(always)]
    pub fn addr5(&mut self) -> ADDR5_W {
        ADDR5_W { w: self }
    }
    #[doc = "Bit 6 - Enable or disable reception on logical address 6."]
    #[inline(always)]
    pub fn addr6(&mut self) -> ADDR6_W {
        ADDR6_W { w: self }
    }
    #[doc = "Bit 7 - Enable or disable reception on logical address 7."]
    #[inline(always)]
    pub fn addr7(&mut self) -> ADDR7_W {
        ADDR7_W { w: self }
    }
}
