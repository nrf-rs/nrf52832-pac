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
    #[doc = "0: VDD * 1/8 selected as reference"]
    REF1_8VDD = 0,
    #[doc = "1: VDD * 2/8 selected as reference"]
    REF2_8VDD = 1,
    #[doc = "2: VDD * 3/8 selected as reference"]
    REF3_8VDD = 2,
    #[doc = "3: VDD * 4/8 selected as reference"]
    REF4_8VDD = 3,
    #[doc = "4: VDD * 5/8 selected as reference"]
    REF5_8VDD = 4,
    #[doc = "5: VDD * 6/8 selected as reference"]
    REF6_8VDD = 5,
    #[doc = "6: VDD * 7/8 selected as reference"]
    REF7_8VDD = 6,
    #[doc = "7: External analog reference selected"]
    AREF = 7,
    #[doc = "8: VDD * 1/16 selected as reference"]
    REF1_16VDD = 8,
    #[doc = "9: VDD * 3/16 selected as reference"]
    REF3_16VDD = 9,
    #[doc = "10: VDD * 5/16 selected as reference"]
    REF5_16VDD = 10,
    #[doc = "11: VDD * 7/16 selected as reference"]
    REF7_16VDD = 11,
    #[doc = "12: VDD * 9/16 selected as reference"]
    REF9_16VDD = 12,
    #[doc = "13: VDD * 11/16 selected as reference"]
    REF11_16VDD = 13,
    #[doc = "14: VDD * 13/16 selected as reference"]
    REF13_16VDD = 14,
    #[doc = "15: VDD * 15/16 selected as reference"]
    REF15_16VDD = 15,
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
    pub fn variant(&self) -> REFSEL_A {
        match self.bits {
            0 => REFSEL_A::REF1_8VDD,
            1 => REFSEL_A::REF2_8VDD,
            2 => REFSEL_A::REF3_8VDD,
            3 => REFSEL_A::REF4_8VDD,
            4 => REFSEL_A::REF5_8VDD,
            5 => REFSEL_A::REF6_8VDD,
            6 => REFSEL_A::REF7_8VDD,
            7 => REFSEL_A::AREF,
            8 => REFSEL_A::REF1_16VDD,
            9 => REFSEL_A::REF3_16VDD,
            10 => REFSEL_A::REF5_16VDD,
            11 => REFSEL_A::REF7_16VDD,
            12 => REFSEL_A::REF9_16VDD,
            13 => REFSEL_A::REF11_16VDD,
            14 => REFSEL_A::REF13_16VDD,
            15 => REFSEL_A::REF15_16VDD,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `REF1_8VDD`"]
    #[inline(always)]
    pub fn is_ref1_8vdd(&self) -> bool {
        *self == REFSEL_A::REF1_8VDD
    }
    #[doc = "Checks if the value of the field is `REF2_8VDD`"]
    #[inline(always)]
    pub fn is_ref2_8vdd(&self) -> bool {
        *self == REFSEL_A::REF2_8VDD
    }
    #[doc = "Checks if the value of the field is `REF3_8VDD`"]
    #[inline(always)]
    pub fn is_ref3_8vdd(&self) -> bool {
        *self == REFSEL_A::REF3_8VDD
    }
    #[doc = "Checks if the value of the field is `REF4_8VDD`"]
    #[inline(always)]
    pub fn is_ref4_8vdd(&self) -> bool {
        *self == REFSEL_A::REF4_8VDD
    }
    #[doc = "Checks if the value of the field is `REF5_8VDD`"]
    #[inline(always)]
    pub fn is_ref5_8vdd(&self) -> bool {
        *self == REFSEL_A::REF5_8VDD
    }
    #[doc = "Checks if the value of the field is `REF6_8VDD`"]
    #[inline(always)]
    pub fn is_ref6_8vdd(&self) -> bool {
        *self == REFSEL_A::REF6_8VDD
    }
    #[doc = "Checks if the value of the field is `REF7_8VDD`"]
    #[inline(always)]
    pub fn is_ref7_8vdd(&self) -> bool {
        *self == REFSEL_A::REF7_8VDD
    }
    #[doc = "Checks if the value of the field is `AREF`"]
    #[inline(always)]
    pub fn is_aref(&self) -> bool {
        *self == REFSEL_A::AREF
    }
    #[doc = "Checks if the value of the field is `REF1_16VDD`"]
    #[inline(always)]
    pub fn is_ref1_16vdd(&self) -> bool {
        *self == REFSEL_A::REF1_16VDD
    }
    #[doc = "Checks if the value of the field is `REF3_16VDD`"]
    #[inline(always)]
    pub fn is_ref3_16vdd(&self) -> bool {
        *self == REFSEL_A::REF3_16VDD
    }
    #[doc = "Checks if the value of the field is `REF5_16VDD`"]
    #[inline(always)]
    pub fn is_ref5_16vdd(&self) -> bool {
        *self == REFSEL_A::REF5_16VDD
    }
    #[doc = "Checks if the value of the field is `REF7_16VDD`"]
    #[inline(always)]
    pub fn is_ref7_16vdd(&self) -> bool {
        *self == REFSEL_A::REF7_16VDD
    }
    #[doc = "Checks if the value of the field is `REF9_16VDD`"]
    #[inline(always)]
    pub fn is_ref9_16vdd(&self) -> bool {
        *self == REFSEL_A::REF9_16VDD
    }
    #[doc = "Checks if the value of the field is `REF11_16VDD`"]
    #[inline(always)]
    pub fn is_ref11_16vdd(&self) -> bool {
        *self == REFSEL_A::REF11_16VDD
    }
    #[doc = "Checks if the value of the field is `REF13_16VDD`"]
    #[inline(always)]
    pub fn is_ref13_16vdd(&self) -> bool {
        *self == REFSEL_A::REF13_16VDD
    }
    #[doc = "Checks if the value of the field is `REF15_16VDD`"]
    #[inline(always)]
    pub fn is_ref15_16vdd(&self) -> bool {
        *self == REFSEL_A::REF15_16VDD
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
        {
            self.bits(variant.into())
        }
    }
    #[doc = "VDD * 1/8 selected as reference"]
    #[inline(always)]
    pub fn ref1_8vdd(self) -> &'a mut W {
        self.variant(REFSEL_A::REF1_8VDD)
    }
    #[doc = "VDD * 2/8 selected as reference"]
    #[inline(always)]
    pub fn ref2_8vdd(self) -> &'a mut W {
        self.variant(REFSEL_A::REF2_8VDD)
    }
    #[doc = "VDD * 3/8 selected as reference"]
    #[inline(always)]
    pub fn ref3_8vdd(self) -> &'a mut W {
        self.variant(REFSEL_A::REF3_8VDD)
    }
    #[doc = "VDD * 4/8 selected as reference"]
    #[inline(always)]
    pub fn ref4_8vdd(self) -> &'a mut W {
        self.variant(REFSEL_A::REF4_8VDD)
    }
    #[doc = "VDD * 5/8 selected as reference"]
    #[inline(always)]
    pub fn ref5_8vdd(self) -> &'a mut W {
        self.variant(REFSEL_A::REF5_8VDD)
    }
    #[doc = "VDD * 6/8 selected as reference"]
    #[inline(always)]
    pub fn ref6_8vdd(self) -> &'a mut W {
        self.variant(REFSEL_A::REF6_8VDD)
    }
    #[doc = "VDD * 7/8 selected as reference"]
    #[inline(always)]
    pub fn ref7_8vdd(self) -> &'a mut W {
        self.variant(REFSEL_A::REF7_8VDD)
    }
    #[doc = "External analog reference selected"]
    #[inline(always)]
    pub fn aref(self) -> &'a mut W {
        self.variant(REFSEL_A::AREF)
    }
    #[doc = "VDD * 1/16 selected as reference"]
    #[inline(always)]
    pub fn ref1_16vdd(self) -> &'a mut W {
        self.variant(REFSEL_A::REF1_16VDD)
    }
    #[doc = "VDD * 3/16 selected as reference"]
    #[inline(always)]
    pub fn ref3_16vdd(self) -> &'a mut W {
        self.variant(REFSEL_A::REF3_16VDD)
    }
    #[doc = "VDD * 5/16 selected as reference"]
    #[inline(always)]
    pub fn ref5_16vdd(self) -> &'a mut W {
        self.variant(REFSEL_A::REF5_16VDD)
    }
    #[doc = "VDD * 7/16 selected as reference"]
    #[inline(always)]
    pub fn ref7_16vdd(self) -> &'a mut W {
        self.variant(REFSEL_A::REF7_16VDD)
    }
    #[doc = "VDD * 9/16 selected as reference"]
    #[inline(always)]
    pub fn ref9_16vdd(self) -> &'a mut W {
        self.variant(REFSEL_A::REF9_16VDD)
    }
    #[doc = "VDD * 11/16 selected as reference"]
    #[inline(always)]
    pub fn ref11_16vdd(self) -> &'a mut W {
        self.variant(REFSEL_A::REF11_16VDD)
    }
    #[doc = "VDD * 13/16 selected as reference"]
    #[inline(always)]
    pub fn ref13_16vdd(self) -> &'a mut W {
        self.variant(REFSEL_A::REF13_16VDD)
    }
    #[doc = "VDD * 15/16 selected as reference"]
    #[inline(always)]
    pub fn ref15_16vdd(self) -> &'a mut W {
        self.variant(REFSEL_A::REF15_16VDD)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x0f) | ((value as u32) & 0x0f);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:3 - Reference select"]
    #[inline(always)]
    pub fn refsel(&self) -> REFSEL_R {
        REFSEL_R::new((self.bits & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:3 - Reference select"]
    #[inline(always)]
    pub fn refsel(&mut self) -> REFSEL_W {
        REFSEL_W { w: self }
    }
}
