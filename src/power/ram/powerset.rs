#[doc = "Writer for register POWERSET"]
pub type W = crate::W<u32, super::POWERSET>;
#[doc = "Register POWERSET `reset()`'s with value 0xffff"]
impl crate::ResetValue for super::POWERSET {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0xffff
    }
}
#[doc = "Keep RAM section S0 of RAM0 on or off in System ON mode\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum S0POWER_AW {
    #[doc = "1: On"]
    ON = 1,
}
impl From<S0POWER_AW> for bool {
    #[inline(always)]
    fn from(variant: S0POWER_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Write proxy for field `S0POWER`"]
pub struct S0POWER_W<'a> {
    w: &'a mut W,
}
impl<'a> S0POWER_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: S0POWER_AW) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "On"]
    #[inline(always)]
    pub fn on(self) -> &'a mut W {
        self.variant(S0POWER_AW::ON)
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
#[doc = "Keep RAM section S1 of RAM0 on or off in System ON mode\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum S1POWER_AW {
    #[doc = "1: On"]
    ON = 1,
}
impl From<S1POWER_AW> for bool {
    #[inline(always)]
    fn from(variant: S1POWER_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Write proxy for field `S1POWER`"]
pub struct S1POWER_W<'a> {
    w: &'a mut W,
}
impl<'a> S1POWER_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: S1POWER_AW) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "On"]
    #[inline(always)]
    pub fn on(self) -> &'a mut W {
        self.variant(S1POWER_AW::ON)
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
#[doc = "Keep retention on RAM section S0 when RAM section is switched off\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum S0RETENTION_AW {
    #[doc = "1: On"]
    ON = 1,
}
impl From<S0RETENTION_AW> for bool {
    #[inline(always)]
    fn from(variant: S0RETENTION_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Write proxy for field `S0RETENTION`"]
pub struct S0RETENTION_W<'a> {
    w: &'a mut W,
}
impl<'a> S0RETENTION_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: S0RETENTION_AW) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "On"]
    #[inline(always)]
    pub fn on(self) -> &'a mut W {
        self.variant(S0RETENTION_AW::ON)
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
#[doc = "Keep retention on RAM section S1 when RAM section is switched off\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum S1RETENTION_AW {
    #[doc = "1: On"]
    ON = 1,
}
impl From<S1RETENTION_AW> for bool {
    #[inline(always)]
    fn from(variant: S1RETENTION_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Write proxy for field `S1RETENTION`"]
pub struct S1RETENTION_W<'a> {
    w: &'a mut W,
}
impl<'a> S1RETENTION_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: S1RETENTION_AW) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "On"]
    #[inline(always)]
    pub fn on(self) -> &'a mut W {
        self.variant(S1RETENTION_AW::ON)
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
impl W {
    #[doc = "Bit 0 - Keep RAM section S0 of RAM0 on or off in System ON mode"]
    #[inline(always)]
    pub fn s0power(&mut self) -> S0POWER_W {
        S0POWER_W { w: self }
    }
    #[doc = "Bit 1 - Keep RAM section S1 of RAM0 on or off in System ON mode"]
    #[inline(always)]
    pub fn s1power(&mut self) -> S1POWER_W {
        S1POWER_W { w: self }
    }
    #[doc = "Bit 16 - Keep retention on RAM section S0 when RAM section is switched off"]
    #[inline(always)]
    pub fn s0retention(&mut self) -> S0RETENTION_W {
        S0RETENTION_W { w: self }
    }
    #[doc = "Bit 17 - Keep retention on RAM section S1 when RAM section is switched off"]
    #[inline(always)]
    pub fn s1retention(&mut self) -> S1RETENTION_W {
        S1RETENTION_W { w: self }
    }
}
