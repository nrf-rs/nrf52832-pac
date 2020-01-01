#[doc = "Reader of register REFSEL"]
pub type R = crate::R<u32, super::REFSEL>;
#[doc = "Writer for register REFSEL"]
pub type W = crate::W<u32, super::REFSEL>;
#[doc = "Register REFSEL `reset()`'s with value 0x04"]
impl crate::ResetValue for super::REFSEL {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0x04
    }
}
#[doc = "Reference select\n\nValue on reset: 4"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum REFSEL_A {
    #[doc = "0: VREF = internal 1.2 V reference (VDD &gt;= 1.7 V)"]
    INT1V2 = 0,
    #[doc = "1: VREF = internal 1.8 V reference (VDD &gt;= VREF + 0.2 V)"]
    INT1V8 = 1,
    #[doc = "2: VREF = internal 2.4 V reference (VDD &gt;= VREF + 0.2 V)"]
    INT2V4 = 2,
    #[doc = "4: VREF = VDD"]
    VDD = 4,
    #[doc = "7: VREF = AREF (VDD &gt;= VREF &gt;= AREFMIN)"]
    AREF = 7,
}
impl From<REFSEL_A> for u8 {
    #[inline(always)]
    fn from(variant: REFSEL_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `REFSEL`"]
pub type REFSEL_R = crate::R<u8, REFSEL_A>;
impl REFSEL_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, REFSEL_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(REFSEL_A::INT1V2),
            1 => Val(REFSEL_A::INT1V8),
            2 => Val(REFSEL_A::INT2V4),
            4 => Val(REFSEL_A::VDD),
            7 => Val(REFSEL_A::AREF),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `INT1V2`"]
    #[inline(always)]
    pub fn is_int1v2(&self) -> bool {
        *self == REFSEL_A::INT1V2
    }
    #[doc = "Checks if the value of the field is `INT1V8`"]
    #[inline(always)]
    pub fn is_int1v8(&self) -> bool {
        *self == REFSEL_A::INT1V8
    }
    #[doc = "Checks if the value of the field is `INT2V4`"]
    #[inline(always)]
    pub fn is_int2v4(&self) -> bool {
        *self == REFSEL_A::INT2V4
    }
    #[doc = "Checks if the value of the field is `VDD`"]
    #[inline(always)]
    pub fn is_vdd(&self) -> bool {
        *self == REFSEL_A::VDD
    }
    #[doc = "Checks if the value of the field is `AREF`"]
    #[inline(always)]
    pub fn is_aref(&self) -> bool {
        *self == REFSEL_A::AREF
    }
}
#[doc = "Write proxy for field `REFSEL`"]
pub struct REFSEL_W<'a> {
    w: &'a mut W,
}
impl<'a> REFSEL_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: REFSEL_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "VREF = internal 1.2 V reference (VDD &gt;= 1.7 V)"]
    #[inline(always)]
    pub fn int1v2(self) -> &'a mut W {
        self.variant(REFSEL_A::INT1V2)
    }
    #[doc = "VREF = internal 1.8 V reference (VDD &gt;= VREF + 0.2 V)"]
    #[inline(always)]
    pub fn int1v8(self) -> &'a mut W {
        self.variant(REFSEL_A::INT1V8)
    }
    #[doc = "VREF = internal 2.4 V reference (VDD &gt;= VREF + 0.2 V)"]
    #[inline(always)]
    pub fn int2v4(self) -> &'a mut W {
        self.variant(REFSEL_A::INT2V4)
    }
    #[doc = "VREF = VDD"]
    #[inline(always)]
    pub fn vdd(self) -> &'a mut W {
        self.variant(REFSEL_A::VDD)
    }
    #[doc = "VREF = AREF (VDD &gt;= VREF &gt;= AREFMIN)"]
    #[inline(always)]
    pub fn aref(self) -> &'a mut W {
        self.variant(REFSEL_A::AREF)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x07) | ((value as u32) & 0x07);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:2 - Reference select"]
    #[inline(always)]
    pub fn refsel(&self) -> REFSEL_R {
        REFSEL_R::new((self.bits & 0x07) as u8)
    }
}
impl W {
    #[doc = "Bits 0:2 - Reference select"]
    #[inline(always)]
    pub fn refsel(&mut self) -> REFSEL_W {
        REFSEL_W { w: self }
    }
}
