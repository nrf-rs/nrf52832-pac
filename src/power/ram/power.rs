#[doc = "Reader of register POWER"]
pub type R = crate::R<u32, super::POWER>;
#[doc = "Writer for register POWER"]
pub type W = crate::W<u32, super::POWER>;
#[doc = "Register POWER `reset()`'s with value 0xffff"]
impl crate::ResetValue for super::POWER {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0xffff
    }
}
#[doc = "Keep RAM section S0 ON or OFF in System ON mode.\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum S0POWER_A {
    #[doc = "0: Off"]
    OFF = 0,
    #[doc = "1: On"]
    ON = 1,
}
impl From<S0POWER_A> for bool {
    #[inline(always)]
    fn from(variant: S0POWER_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `S0POWER`"]
pub type S0POWER_R = crate::R<bool, S0POWER_A>;
impl S0POWER_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> S0POWER_A {
        match self.bits {
            false => S0POWER_A::OFF,
            true => S0POWER_A::ON,
        }
    }
    #[doc = "Checks if the value of the field is `OFF`"]
    #[inline(always)]
    pub fn is_off(&self) -> bool {
        *self == S0POWER_A::OFF
    }
    #[doc = "Checks if the value of the field is `ON`"]
    #[inline(always)]
    pub fn is_on(&self) -> bool {
        *self == S0POWER_A::ON
    }
}
#[doc = "Write proxy for field `S0POWER`"]
pub struct S0POWER_W<'a> {
    w: &'a mut W,
}
impl<'a> S0POWER_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: S0POWER_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Off"]
    #[inline(always)]
    pub fn off(self) -> &'a mut W {
        self.variant(S0POWER_A::OFF)
    }
    #[doc = "On"]
    #[inline(always)]
    pub fn on(self) -> &'a mut W {
        self.variant(S0POWER_A::ON)
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
#[doc = "Keep RAM section S1 ON or OFF in System ON mode.\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum S1POWER_A {
    #[doc = "0: Off"]
    OFF = 0,
    #[doc = "1: On"]
    ON = 1,
}
impl From<S1POWER_A> for bool {
    #[inline(always)]
    fn from(variant: S1POWER_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `S1POWER`"]
pub type S1POWER_R = crate::R<bool, S1POWER_A>;
impl S1POWER_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> S1POWER_A {
        match self.bits {
            false => S1POWER_A::OFF,
            true => S1POWER_A::ON,
        }
    }
    #[doc = "Checks if the value of the field is `OFF`"]
    #[inline(always)]
    pub fn is_off(&self) -> bool {
        *self == S1POWER_A::OFF
    }
    #[doc = "Checks if the value of the field is `ON`"]
    #[inline(always)]
    pub fn is_on(&self) -> bool {
        *self == S1POWER_A::ON
    }
}
#[doc = "Write proxy for field `S1POWER`"]
pub struct S1POWER_W<'a> {
    w: &'a mut W,
}
impl<'a> S1POWER_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: S1POWER_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Off"]
    #[inline(always)]
    pub fn off(self) -> &'a mut W {
        self.variant(S1POWER_A::OFF)
    }
    #[doc = "On"]
    #[inline(always)]
    pub fn on(self) -> &'a mut W {
        self.variant(S1POWER_A::ON)
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
#[doc = "Keep retention on RAM section S0 when RAM section is in OFF\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum S0RETENTION_A {
    #[doc = "0: Off"]
    OFF = 0,
    #[doc = "1: On"]
    ON = 1,
}
impl From<S0RETENTION_A> for bool {
    #[inline(always)]
    fn from(variant: S0RETENTION_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `S0RETENTION`"]
pub type S0RETENTION_R = crate::R<bool, S0RETENTION_A>;
impl S0RETENTION_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> S0RETENTION_A {
        match self.bits {
            false => S0RETENTION_A::OFF,
            true => S0RETENTION_A::ON,
        }
    }
    #[doc = "Checks if the value of the field is `OFF`"]
    #[inline(always)]
    pub fn is_off(&self) -> bool {
        *self == S0RETENTION_A::OFF
    }
    #[doc = "Checks if the value of the field is `ON`"]
    #[inline(always)]
    pub fn is_on(&self) -> bool {
        *self == S0RETENTION_A::ON
    }
}
#[doc = "Write proxy for field `S0RETENTION`"]
pub struct S0RETENTION_W<'a> {
    w: &'a mut W,
}
impl<'a> S0RETENTION_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: S0RETENTION_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Off"]
    #[inline(always)]
    pub fn off(self) -> &'a mut W {
        self.variant(S0RETENTION_A::OFF)
    }
    #[doc = "On"]
    #[inline(always)]
    pub fn on(self) -> &'a mut W {
        self.variant(S0RETENTION_A::ON)
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
#[doc = "Keep retention on RAM section S1 when RAM section is in OFF\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum S1RETENTION_A {
    #[doc = "0: Off"]
    OFF = 0,
    #[doc = "1: On"]
    ON = 1,
}
impl From<S1RETENTION_A> for bool {
    #[inline(always)]
    fn from(variant: S1RETENTION_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `S1RETENTION`"]
pub type S1RETENTION_R = crate::R<bool, S1RETENTION_A>;
impl S1RETENTION_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> S1RETENTION_A {
        match self.bits {
            false => S1RETENTION_A::OFF,
            true => S1RETENTION_A::ON,
        }
    }
    #[doc = "Checks if the value of the field is `OFF`"]
    #[inline(always)]
    pub fn is_off(&self) -> bool {
        *self == S1RETENTION_A::OFF
    }
    #[doc = "Checks if the value of the field is `ON`"]
    #[inline(always)]
    pub fn is_on(&self) -> bool {
        *self == S1RETENTION_A::ON
    }
}
#[doc = "Write proxy for field `S1RETENTION`"]
pub struct S1RETENTION_W<'a> {
    w: &'a mut W,
}
impl<'a> S1RETENTION_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: S1RETENTION_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Off"]
    #[inline(always)]
    pub fn off(self) -> &'a mut W {
        self.variant(S1RETENTION_A::OFF)
    }
    #[doc = "On"]
    #[inline(always)]
    pub fn on(self) -> &'a mut W {
        self.variant(S1RETENTION_A::ON)
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
impl R {
    #[doc = "Bit 0 - Keep RAM section S0 ON or OFF in System ON mode."]
    #[inline(always)]
    pub fn s0power(&self) -> S0POWER_R {
        S0POWER_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - Keep RAM section S1 ON or OFF in System ON mode."]
    #[inline(always)]
    pub fn s1power(&self) -> S1POWER_R {
        S1POWER_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 16 - Keep retention on RAM section S0 when RAM section is in OFF"]
    #[inline(always)]
    pub fn s0retention(&self) -> S0RETENTION_R {
        S0RETENTION_R::new(((self.bits >> 16) & 0x01) != 0)
    }
    #[doc = "Bit 17 - Keep retention on RAM section S1 when RAM section is in OFF"]
    #[inline(always)]
    pub fn s1retention(&self) -> S1RETENTION_R {
        S1RETENTION_R::new(((self.bits >> 17) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Keep RAM section S0 ON or OFF in System ON mode."]
    #[inline(always)]
    pub fn s0power(&mut self) -> S0POWER_W {
        S0POWER_W { w: self }
    }
    #[doc = "Bit 1 - Keep RAM section S1 ON or OFF in System ON mode."]
    #[inline(always)]
    pub fn s1power(&mut self) -> S1POWER_W {
        S1POWER_W { w: self }
    }
    #[doc = "Bit 16 - Keep retention on RAM section S0 when RAM section is in OFF"]
    #[inline(always)]
    pub fn s0retention(&mut self) -> S0RETENTION_W {
        S0RETENTION_W { w: self }
    }
    #[doc = "Bit 17 - Keep retention on RAM section S1 when RAM section is in OFF"]
    #[inline(always)]
    pub fn s1retention(&mut self) -> S1RETENTION_W {
        S1RETENTION_W { w: self }
    }
}
