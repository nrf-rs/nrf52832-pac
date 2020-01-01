#[doc = "Reader of register POFCON"]
pub type R = crate::R<u32, super::POFCON>;
#[doc = "Writer for register POFCON"]
pub type W = crate::W<u32, super::POFCON>;
#[doc = "Register POFCON `reset()`'s with value 0"]
impl crate::ResetValue for super::POFCON {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Enable or disable power failure comparator\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum POF_A {
    #[doc = "0: Disable"]
    DISABLED = 0,
    #[doc = "1: Enable"]
    ENABLED = 1,
}
impl From<POF_A> for bool {
    #[inline(always)]
    fn from(variant: POF_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `POF`"]
pub type POF_R = crate::R<bool, POF_A>;
impl POF_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> POF_A {
        match self.bits {
            false => POF_A::DISABLED,
            true => POF_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == POF_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == POF_A::ENABLED
    }
}
#[doc = "Write proxy for field `POF`"]
pub struct POF_W<'a> {
    w: &'a mut W,
}
impl<'a> POF_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: POF_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Disable"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(POF_A::DISABLED)
    }
    #[doc = "Enable"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(POF_A::ENABLED)
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
#[doc = "Power failure comparator threshold setting\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum THRESHOLD_A {
    #[doc = "4: Set threshold to 1.7 V"]
    V17 = 4,
    #[doc = "5: Set threshold to 1.8 V"]
    V18 = 5,
    #[doc = "6: Set threshold to 1.9 V"]
    V19 = 6,
    #[doc = "7: Set threshold to 2.0 V"]
    V20 = 7,
    #[doc = "8: Set threshold to 2.1 V"]
    V21 = 8,
    #[doc = "9: Set threshold to 2.2 V"]
    V22 = 9,
    #[doc = "10: Set threshold to 2.3 V"]
    V23 = 10,
    #[doc = "11: Set threshold to 2.4 V"]
    V24 = 11,
    #[doc = "12: Set threshold to 2.5 V"]
    V25 = 12,
    #[doc = "13: Set threshold to 2.6 V"]
    V26 = 13,
    #[doc = "14: Set threshold to 2.7 V"]
    V27 = 14,
    #[doc = "15: Set threshold to 2.8 V"]
    V28 = 15,
}
impl From<THRESHOLD_A> for u8 {
    #[inline(always)]
    fn from(variant: THRESHOLD_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `THRESHOLD`"]
pub type THRESHOLD_R = crate::R<u8, THRESHOLD_A>;
impl THRESHOLD_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, THRESHOLD_A> {
        use crate::Variant::*;
        match self.bits {
            4 => Val(THRESHOLD_A::V17),
            5 => Val(THRESHOLD_A::V18),
            6 => Val(THRESHOLD_A::V19),
            7 => Val(THRESHOLD_A::V20),
            8 => Val(THRESHOLD_A::V21),
            9 => Val(THRESHOLD_A::V22),
            10 => Val(THRESHOLD_A::V23),
            11 => Val(THRESHOLD_A::V24),
            12 => Val(THRESHOLD_A::V25),
            13 => Val(THRESHOLD_A::V26),
            14 => Val(THRESHOLD_A::V27),
            15 => Val(THRESHOLD_A::V28),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `V17`"]
    #[inline(always)]
    pub fn is_v17(&self) -> bool {
        *self == THRESHOLD_A::V17
    }
    #[doc = "Checks if the value of the field is `V18`"]
    #[inline(always)]
    pub fn is_v18(&self) -> bool {
        *self == THRESHOLD_A::V18
    }
    #[doc = "Checks if the value of the field is `V19`"]
    #[inline(always)]
    pub fn is_v19(&self) -> bool {
        *self == THRESHOLD_A::V19
    }
    #[doc = "Checks if the value of the field is `V20`"]
    #[inline(always)]
    pub fn is_v20(&self) -> bool {
        *self == THRESHOLD_A::V20
    }
    #[doc = "Checks if the value of the field is `V21`"]
    #[inline(always)]
    pub fn is_v21(&self) -> bool {
        *self == THRESHOLD_A::V21
    }
    #[doc = "Checks if the value of the field is `V22`"]
    #[inline(always)]
    pub fn is_v22(&self) -> bool {
        *self == THRESHOLD_A::V22
    }
    #[doc = "Checks if the value of the field is `V23`"]
    #[inline(always)]
    pub fn is_v23(&self) -> bool {
        *self == THRESHOLD_A::V23
    }
    #[doc = "Checks if the value of the field is `V24`"]
    #[inline(always)]
    pub fn is_v24(&self) -> bool {
        *self == THRESHOLD_A::V24
    }
    #[doc = "Checks if the value of the field is `V25`"]
    #[inline(always)]
    pub fn is_v25(&self) -> bool {
        *self == THRESHOLD_A::V25
    }
    #[doc = "Checks if the value of the field is `V26`"]
    #[inline(always)]
    pub fn is_v26(&self) -> bool {
        *self == THRESHOLD_A::V26
    }
    #[doc = "Checks if the value of the field is `V27`"]
    #[inline(always)]
    pub fn is_v27(&self) -> bool {
        *self == THRESHOLD_A::V27
    }
    #[doc = "Checks if the value of the field is `V28`"]
    #[inline(always)]
    pub fn is_v28(&self) -> bool {
        *self == THRESHOLD_A::V28
    }
}
#[doc = "Write proxy for field `THRESHOLD`"]
pub struct THRESHOLD_W<'a> {
    w: &'a mut W,
}
impl<'a> THRESHOLD_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: THRESHOLD_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "Set threshold to 1.7 V"]
    #[inline(always)]
    pub fn v17(self) -> &'a mut W {
        self.variant(THRESHOLD_A::V17)
    }
    #[doc = "Set threshold to 1.8 V"]
    #[inline(always)]
    pub fn v18(self) -> &'a mut W {
        self.variant(THRESHOLD_A::V18)
    }
    #[doc = "Set threshold to 1.9 V"]
    #[inline(always)]
    pub fn v19(self) -> &'a mut W {
        self.variant(THRESHOLD_A::V19)
    }
    #[doc = "Set threshold to 2.0 V"]
    #[inline(always)]
    pub fn v20(self) -> &'a mut W {
        self.variant(THRESHOLD_A::V20)
    }
    #[doc = "Set threshold to 2.1 V"]
    #[inline(always)]
    pub fn v21(self) -> &'a mut W {
        self.variant(THRESHOLD_A::V21)
    }
    #[doc = "Set threshold to 2.2 V"]
    #[inline(always)]
    pub fn v22(self) -> &'a mut W {
        self.variant(THRESHOLD_A::V22)
    }
    #[doc = "Set threshold to 2.3 V"]
    #[inline(always)]
    pub fn v23(self) -> &'a mut W {
        self.variant(THRESHOLD_A::V23)
    }
    #[doc = "Set threshold to 2.4 V"]
    #[inline(always)]
    pub fn v24(self) -> &'a mut W {
        self.variant(THRESHOLD_A::V24)
    }
    #[doc = "Set threshold to 2.5 V"]
    #[inline(always)]
    pub fn v25(self) -> &'a mut W {
        self.variant(THRESHOLD_A::V25)
    }
    #[doc = "Set threshold to 2.6 V"]
    #[inline(always)]
    pub fn v26(self) -> &'a mut W {
        self.variant(THRESHOLD_A::V26)
    }
    #[doc = "Set threshold to 2.7 V"]
    #[inline(always)]
    pub fn v27(self) -> &'a mut W {
        self.variant(THRESHOLD_A::V27)
    }
    #[doc = "Set threshold to 2.8 V"]
    #[inline(always)]
    pub fn v28(self) -> &'a mut W {
        self.variant(THRESHOLD_A::V28)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 1)) | (((value as u32) & 0x0f) << 1);
        self.w
    }
}
impl R {
    #[doc = "Bit 0 - Enable or disable power failure comparator"]
    #[inline(always)]
    pub fn pof(&self) -> POF_R {
        POF_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bits 1:4 - Power failure comparator threshold setting"]
    #[inline(always)]
    pub fn threshold(&self) -> THRESHOLD_R {
        THRESHOLD_R::new(((self.bits >> 1) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - Enable or disable power failure comparator"]
    #[inline(always)]
    pub fn pof(&mut self) -> POF_W {
        POF_W { w: self }
    }
    #[doc = "Bits 1:4 - Power failure comparator threshold setting"]
    #[inline(always)]
    pub fn threshold(&mut self) -> THRESHOLD_W {
        THRESHOLD_W { w: self }
    }
}
