#[doc = "Reader of register MODECNF0"]
pub type R = crate::R<u32, super::MODECNF0>;
#[doc = "Writer for register MODECNF0"]
pub type W = crate::W<u32, super::MODECNF0>;
#[doc = "Register MODECNF0 `reset()`'s with value 0x0200"]
impl crate::ResetValue for super::MODECNF0 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0x0200
    }
}
#[doc = "Radio ramp-up time\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RU_A {
    #[doc = "0: Default ramp-up time (tRXEN), compatible with firmware written for nRF51"]
    DEFAULT = 0,
    #[doc = "1: Fast ramp-up (tRXEN,FAST), see electrical specification for more information"]
    FAST = 1,
}
impl From<RU_A> for bool {
    #[inline(always)]
    fn from(variant: RU_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `RU`"]
pub type RU_R = crate::R<bool, RU_A>;
impl RU_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RU_A {
        match self.bits {
            false => RU_A::DEFAULT,
            true => RU_A::FAST,
        }
    }
    #[doc = "Checks if the value of the field is `DEFAULT`"]
    #[inline(always)]
    pub fn is_default(&self) -> bool {
        *self == RU_A::DEFAULT
    }
    #[doc = "Checks if the value of the field is `FAST`"]
    #[inline(always)]
    pub fn is_fast(&self) -> bool {
        *self == RU_A::FAST
    }
}
#[doc = "Write proxy for field `RU`"]
pub struct RU_W<'a> {
    w: &'a mut W,
}
impl<'a> RU_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: RU_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Default ramp-up time (tRXEN), compatible with firmware written for nRF51"]
    #[inline(always)]
    pub fn default(self) -> &'a mut W {
        self.variant(RU_A::DEFAULT)
    }
    #[doc = "Fast ramp-up (tRXEN,FAST), see electrical specification for more information"]
    #[inline(always)]
    pub fn fast(self) -> &'a mut W {
        self.variant(RU_A::FAST)
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
#[doc = "Default TX value\n\nValue on reset: 2"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum DTX_A {
    #[doc = "0: Transmit '1'"]
    B1 = 0,
    #[doc = "1: Transmit '0'"]
    B0 = 1,
    #[doc = "2: Transmit center frequency"]
    CENTER = 2,
}
impl From<DTX_A> for u8 {
    #[inline(always)]
    fn from(variant: DTX_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `DTX`"]
pub type DTX_R = crate::R<u8, DTX_A>;
impl DTX_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, DTX_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(DTX_A::B1),
            1 => Val(DTX_A::B0),
            2 => Val(DTX_A::CENTER),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `B1`"]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == DTX_A::B1
    }
    #[doc = "Checks if the value of the field is `B0`"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == DTX_A::B0
    }
    #[doc = "Checks if the value of the field is `CENTER`"]
    #[inline(always)]
    pub fn is_center(&self) -> bool {
        *self == DTX_A::CENTER
    }
}
#[doc = "Write proxy for field `DTX`"]
pub struct DTX_W<'a> {
    w: &'a mut W,
}
impl<'a> DTX_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: DTX_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "Transmit '1'"]
    #[inline(always)]
    pub fn b1(self) -> &'a mut W {
        self.variant(DTX_A::B1)
    }
    #[doc = "Transmit '0'"]
    #[inline(always)]
    pub fn b0(self) -> &'a mut W {
        self.variant(DTX_A::B0)
    }
    #[doc = "Transmit center frequency"]
    #[inline(always)]
    pub fn center(self) -> &'a mut W {
        self.variant(DTX_A::CENTER)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 8)) | (((value as u32) & 0x03) << 8);
        self.w
    }
}
impl R {
    #[doc = "Bit 0 - Radio ramp-up time"]
    #[inline(always)]
    pub fn ru(&self) -> RU_R {
        RU_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bits 8:9 - Default TX value"]
    #[inline(always)]
    pub fn dtx(&self) -> DTX_R {
        DTX_R::new(((self.bits >> 8) & 0x03) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - Radio ramp-up time"]
    #[inline(always)]
    pub fn ru(&mut self) -> RU_W {
        RU_W { w: self }
    }
    #[doc = "Bits 8:9 - Default TX value"]
    #[inline(always)]
    pub fn dtx(&mut self) -> DTX_W {
        DTX_W { w: self }
    }
}
