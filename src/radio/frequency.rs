#[doc = "Reader of register FREQUENCY"]
pub type R = crate::R<u32, super::FREQUENCY>;
#[doc = "Writer for register FREQUENCY"]
pub type W = crate::W<u32, super::FREQUENCY>;
#[doc = "Register FREQUENCY `reset()`'s with value 0x02"]
impl crate::ResetValue for super::FREQUENCY {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0x02
    }
}
#[doc = "Reader of field `FREQUENCY`"]
pub type FREQUENCY_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `FREQUENCY`"]
pub struct FREQUENCY_W<'a> {
    w: &'a mut W,
}
impl<'a> FREQUENCY_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x7f) | ((value as u32) & 0x7f);
        self.w
    }
}
#[doc = "Channel map selection.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum MAP_A {
    #[doc = "0: Channel map between 2400 MHZ .. 2500 MHz"]
    DEFAULT = 0,
    #[doc = "1: Channel map between 2360 MHZ .. 2460 MHz"]
    LOW = 1,
}
impl From<MAP_A> for bool {
    #[inline(always)]
    fn from(variant: MAP_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `MAP`"]
pub type MAP_R = crate::R<bool, MAP_A>;
impl MAP_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> MAP_A {
        match self.bits {
            false => MAP_A::DEFAULT,
            true => MAP_A::LOW,
        }
    }
    #[doc = "Checks if the value of the field is `DEFAULT`"]
    #[inline(always)]
    pub fn is_default(&self) -> bool {
        *self == MAP_A::DEFAULT
    }
    #[doc = "Checks if the value of the field is `LOW`"]
    #[inline(always)]
    pub fn is_low(&self) -> bool {
        *self == MAP_A::LOW
    }
}
#[doc = "Write proxy for field `MAP`"]
pub struct MAP_W<'a> {
    w: &'a mut W,
}
impl<'a> MAP_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: MAP_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Channel map between 2400 MHZ .. 2500 MHz"]
    #[inline(always)]
    pub fn default(self) -> &'a mut W {
        self.variant(MAP_A::DEFAULT)
    }
    #[doc = "Channel map between 2360 MHZ .. 2460 MHz"]
    #[inline(always)]
    pub fn low(self) -> &'a mut W {
        self.variant(MAP_A::LOW)
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
impl R {
    #[doc = "Bits 0:6 - Radio channel frequency"]
    #[inline(always)]
    pub fn frequency(&self) -> FREQUENCY_R {
        FREQUENCY_R::new((self.bits & 0x7f) as u8)
    }
    #[doc = "Bit 8 - Channel map selection."]
    #[inline(always)]
    pub fn map(&self) -> MAP_R {
        MAP_R::new(((self.bits >> 8) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bits 0:6 - Radio channel frequency"]
    #[inline(always)]
    pub fn frequency(&mut self) -> FREQUENCY_W {
        FREQUENCY_W { w: self }
    }
    #[doc = "Bit 8 - Channel map selection."]
    #[inline(always)]
    pub fn map(&mut self) -> MAP_W {
        MAP_W { w: self }
    }
}
