#[doc = "Reader of register CONFIG"]
pub type R = crate::R<u32, super::CONFIG>;
#[doc = "Writer for register CONFIG"]
pub type W = crate::W<u32, super::CONFIG>;
#[doc = "Register CONFIG `reset()`'s with value 0"]
impl crate::ResetValue for super::CONFIG {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Bit order\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ORDER_A {
    #[doc = "0: Most significant bit shifted out first"]
    MSBFIRST = 0,
    #[doc = "1: Least significant bit shifted out first"]
    LSBFIRST = 1,
}
impl From<ORDER_A> for bool {
    #[inline(always)]
    fn from(variant: ORDER_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `ORDER`"]
pub type ORDER_R = crate::R<bool, ORDER_A>;
impl ORDER_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ORDER_A {
        match self.bits {
            false => ORDER_A::MSBFIRST,
            true => ORDER_A::LSBFIRST,
        }
    }
    #[doc = "Checks if the value of the field is `MSBFIRST`"]
    #[inline(always)]
    pub fn is_msb_first(&self) -> bool {
        *self == ORDER_A::MSBFIRST
    }
    #[doc = "Checks if the value of the field is `LSBFIRST`"]
    #[inline(always)]
    pub fn is_lsb_first(&self) -> bool {
        *self == ORDER_A::LSBFIRST
    }
}
#[doc = "Write proxy for field `ORDER`"]
pub struct ORDER_W<'a> {
    w: &'a mut W,
}
impl<'a> ORDER_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: ORDER_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Most significant bit shifted out first"]
    #[inline(always)]
    pub fn msb_first(self) -> &'a mut W {
        self.variant(ORDER_A::MSBFIRST)
    }
    #[doc = "Least significant bit shifted out first"]
    #[inline(always)]
    pub fn lsb_first(self) -> &'a mut W {
        self.variant(ORDER_A::LSBFIRST)
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
#[doc = "Serial clock (SCK) phase\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CPHA_A {
    #[doc = "0: Sample on leading edge of clock, shift serial data on trailing edge"]
    LEADING = 0,
    #[doc = "1: Sample on trailing edge of clock, shift serial data on leading edge"]
    TRAILING = 1,
}
impl From<CPHA_A> for bool {
    #[inline(always)]
    fn from(variant: CPHA_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `CPHA`"]
pub type CPHA_R = crate::R<bool, CPHA_A>;
impl CPHA_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CPHA_A {
        match self.bits {
            false => CPHA_A::LEADING,
            true => CPHA_A::TRAILING,
        }
    }
    #[doc = "Checks if the value of the field is `LEADING`"]
    #[inline(always)]
    pub fn is_leading(&self) -> bool {
        *self == CPHA_A::LEADING
    }
    #[doc = "Checks if the value of the field is `TRAILING`"]
    #[inline(always)]
    pub fn is_trailing(&self) -> bool {
        *self == CPHA_A::TRAILING
    }
}
#[doc = "Write proxy for field `CPHA`"]
pub struct CPHA_W<'a> {
    w: &'a mut W,
}
impl<'a> CPHA_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CPHA_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Sample on leading edge of clock, shift serial data on trailing edge"]
    #[inline(always)]
    pub fn leading(self) -> &'a mut W {
        self.variant(CPHA_A::LEADING)
    }
    #[doc = "Sample on trailing edge of clock, shift serial data on leading edge"]
    #[inline(always)]
    pub fn trailing(self) -> &'a mut W {
        self.variant(CPHA_A::TRAILING)
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
#[doc = "Serial clock (SCK) polarity\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CPOL_A {
    #[doc = "0: Active high"]
    ACTIVEHIGH = 0,
    #[doc = "1: Active low"]
    ACTIVELOW = 1,
}
impl From<CPOL_A> for bool {
    #[inline(always)]
    fn from(variant: CPOL_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `CPOL`"]
pub type CPOL_R = crate::R<bool, CPOL_A>;
impl CPOL_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CPOL_A {
        match self.bits {
            false => CPOL_A::ACTIVEHIGH,
            true => CPOL_A::ACTIVELOW,
        }
    }
    #[doc = "Checks if the value of the field is `ACTIVEHIGH`"]
    #[inline(always)]
    pub fn is_active_high(&self) -> bool {
        *self == CPOL_A::ACTIVEHIGH
    }
    #[doc = "Checks if the value of the field is `ACTIVELOW`"]
    #[inline(always)]
    pub fn is_active_low(&self) -> bool {
        *self == CPOL_A::ACTIVELOW
    }
}
#[doc = "Write proxy for field `CPOL`"]
pub struct CPOL_W<'a> {
    w: &'a mut W,
}
impl<'a> CPOL_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CPOL_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Active high"]
    #[inline(always)]
    pub fn active_high(self) -> &'a mut W {
        self.variant(CPOL_A::ACTIVEHIGH)
    }
    #[doc = "Active low"]
    #[inline(always)]
    pub fn active_low(self) -> &'a mut W {
        self.variant(CPOL_A::ACTIVELOW)
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
impl R {
    #[doc = "Bit 0 - Bit order"]
    #[inline(always)]
    pub fn order(&self) -> ORDER_R {
        ORDER_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - Serial clock (SCK) phase"]
    #[inline(always)]
    pub fn cpha(&self) -> CPHA_R {
        CPHA_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - Serial clock (SCK) polarity"]
    #[inline(always)]
    pub fn cpol(&self) -> CPOL_R {
        CPOL_R::new(((self.bits >> 2) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Bit order"]
    #[inline(always)]
    pub fn order(&mut self) -> ORDER_W {
        ORDER_W { w: self }
    }
    #[doc = "Bit 1 - Serial clock (SCK) phase"]
    #[inline(always)]
    pub fn cpha(&mut self) -> CPHA_W {
        CPHA_W { w: self }
    }
    #[doc = "Bit 2 - Serial clock (SCK) polarity"]
    #[inline(always)]
    pub fn cpol(&mut self) -> CPOL_W {
        CPOL_W { w: self }
    }
}
