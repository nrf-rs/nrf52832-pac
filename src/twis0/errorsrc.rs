#[doc = "Reader of register ERRORSRC"]
pub type R = crate::R<u32, super::ERRORSRC>;
#[doc = "Writer for register ERRORSRC"]
pub type W = crate::W<u32, super::ERRORSRC>;
#[doc = "Register ERRORSRC `reset()`'s with value 0"]
impl crate::ResetValue for super::ERRORSRC {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "RX buffer overflow detected, and prevented\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum OVERFLOW_A {
    #[doc = "0: Error did not occur"]
    NOTDETECTED = 0,
    #[doc = "1: Error occurred"]
    DETECTED = 1,
}
impl From<OVERFLOW_A> for bool {
    #[inline(always)]
    fn from(variant: OVERFLOW_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `OVERFLOW`"]
pub type OVERFLOW_R = crate::R<bool, OVERFLOW_A>;
impl OVERFLOW_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> OVERFLOW_A {
        match self.bits {
            false => OVERFLOW_A::NOTDETECTED,
            true => OVERFLOW_A::DETECTED,
        }
    }
    #[doc = "Checks if the value of the field is `NOTDETECTED`"]
    #[inline(always)]
    pub fn is_not_detected(&self) -> bool {
        *self == OVERFLOW_A::NOTDETECTED
    }
    #[doc = "Checks if the value of the field is `DETECTED`"]
    #[inline(always)]
    pub fn is_detected(&self) -> bool {
        *self == OVERFLOW_A::DETECTED
    }
}
#[doc = "Write proxy for field `OVERFLOW`"]
pub struct OVERFLOW_W<'a> {
    w: &'a mut W,
}
impl<'a> OVERFLOW_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: OVERFLOW_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Error did not occur"]
    #[inline(always)]
    pub fn not_detected(self) -> &'a mut W {
        self.variant(OVERFLOW_A::NOTDETECTED)
    }
    #[doc = "Error occurred"]
    #[inline(always)]
    pub fn detected(self) -> &'a mut W {
        self.variant(OVERFLOW_A::DETECTED)
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
#[doc = "NACK sent after receiving a data byte\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DNACK_A {
    #[doc = "0: Error did not occur"]
    NOTRECEIVED = 0,
    #[doc = "1: Error occurred"]
    RECEIVED = 1,
}
impl From<DNACK_A> for bool {
    #[inline(always)]
    fn from(variant: DNACK_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `DNACK`"]
pub type DNACK_R = crate::R<bool, DNACK_A>;
impl DNACK_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DNACK_A {
        match self.bits {
            false => DNACK_A::NOTRECEIVED,
            true => DNACK_A::RECEIVED,
        }
    }
    #[doc = "Checks if the value of the field is `NOTRECEIVED`"]
    #[inline(always)]
    pub fn is_not_received(&self) -> bool {
        *self == DNACK_A::NOTRECEIVED
    }
    #[doc = "Checks if the value of the field is `RECEIVED`"]
    #[inline(always)]
    pub fn is_received(&self) -> bool {
        *self == DNACK_A::RECEIVED
    }
}
#[doc = "Write proxy for field `DNACK`"]
pub struct DNACK_W<'a> {
    w: &'a mut W,
}
impl<'a> DNACK_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: DNACK_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Error did not occur"]
    #[inline(always)]
    pub fn not_received(self) -> &'a mut W {
        self.variant(DNACK_A::NOTRECEIVED)
    }
    #[doc = "Error occurred"]
    #[inline(always)]
    pub fn received(self) -> &'a mut W {
        self.variant(DNACK_A::RECEIVED)
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
#[doc = "TX buffer over-read detected, and prevented\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum OVERREAD_A {
    #[doc = "0: Error did not occur"]
    NOTDETECTED = 0,
    #[doc = "1: Error occurred"]
    DETECTED = 1,
}
impl From<OVERREAD_A> for bool {
    #[inline(always)]
    fn from(variant: OVERREAD_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `OVERREAD`"]
pub type OVERREAD_R = crate::R<bool, OVERREAD_A>;
impl OVERREAD_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> OVERREAD_A {
        match self.bits {
            false => OVERREAD_A::NOTDETECTED,
            true => OVERREAD_A::DETECTED,
        }
    }
    #[doc = "Checks if the value of the field is `NOTDETECTED`"]
    #[inline(always)]
    pub fn is_not_detected(&self) -> bool {
        *self == OVERREAD_A::NOTDETECTED
    }
    #[doc = "Checks if the value of the field is `DETECTED`"]
    #[inline(always)]
    pub fn is_detected(&self) -> bool {
        *self == OVERREAD_A::DETECTED
    }
}
#[doc = "Write proxy for field `OVERREAD`"]
pub struct OVERREAD_W<'a> {
    w: &'a mut W,
}
impl<'a> OVERREAD_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: OVERREAD_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Error did not occur"]
    #[inline(always)]
    pub fn not_detected(self) -> &'a mut W {
        self.variant(OVERREAD_A::NOTDETECTED)
    }
    #[doc = "Error occurred"]
    #[inline(always)]
    pub fn detected(self) -> &'a mut W {
        self.variant(OVERREAD_A::DETECTED)
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
impl R {
    #[doc = "Bit 0 - RX buffer overflow detected, and prevented"]
    #[inline(always)]
    pub fn overflow(&self) -> OVERFLOW_R {
        OVERFLOW_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 2 - NACK sent after receiving a data byte"]
    #[inline(always)]
    pub fn dnack(&self) -> DNACK_R {
        DNACK_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - TX buffer over-read detected, and prevented"]
    #[inline(always)]
    pub fn overread(&self) -> OVERREAD_R {
        OVERREAD_R::new(((self.bits >> 3) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - RX buffer overflow detected, and prevented"]
    #[inline(always)]
    pub fn overflow(&mut self) -> OVERFLOW_W {
        OVERFLOW_W { w: self }
    }
    #[doc = "Bit 2 - NACK sent after receiving a data byte"]
    #[inline(always)]
    pub fn dnack(&mut self) -> DNACK_W {
        DNACK_W { w: self }
    }
    #[doc = "Bit 3 - TX buffer over-read detected, and prevented"]
    #[inline(always)]
    pub fn overread(&mut self) -> OVERREAD_W {
        OVERREAD_W { w: self }
    }
}
