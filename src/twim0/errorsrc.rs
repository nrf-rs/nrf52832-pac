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
#[doc = "Overrun error\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum OVERRUN_A {
    #[doc = "0: Error did not occur"]
    NOTRECEIVED = 0,
    #[doc = "1: Error occurred"]
    RECEIVED = 1,
}
impl From<OVERRUN_A> for bool {
    #[inline(always)]
    fn from(variant: OVERRUN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `OVERRUN`"]
pub type OVERRUN_R = crate::R<bool, OVERRUN_A>;
impl OVERRUN_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> OVERRUN_A {
        match self.bits {
            false => OVERRUN_A::NOTRECEIVED,
            true => OVERRUN_A::RECEIVED,
        }
    }
    #[doc = "Checks if the value of the field is `NOTRECEIVED`"]
    #[inline(always)]
    pub fn is_not_received(&self) -> bool {
        *self == OVERRUN_A::NOTRECEIVED
    }
    #[doc = "Checks if the value of the field is `RECEIVED`"]
    #[inline(always)]
    pub fn is_received(&self) -> bool {
        *self == OVERRUN_A::RECEIVED
    }
}
#[doc = "Write proxy for field `OVERRUN`"]
pub struct OVERRUN_W<'a> {
    w: &'a mut W,
}
impl<'a> OVERRUN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: OVERRUN_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Error did not occur"]
    #[inline(always)]
    pub fn not_received(self) -> &'a mut W {
        self.variant(OVERRUN_A::NOTRECEIVED)
    }
    #[doc = "Error occurred"]
    #[inline(always)]
    pub fn received(self) -> &'a mut W {
        self.variant(OVERRUN_A::RECEIVED)
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
#[doc = "NACK received after sending the address (write '1' to clear)\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ANACK_A {
    #[doc = "0: Error did not occur"]
    NOTRECEIVED = 0,
    #[doc = "1: Error occurred"]
    RECEIVED = 1,
}
impl From<ANACK_A> for bool {
    #[inline(always)]
    fn from(variant: ANACK_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `ANACK`"]
pub type ANACK_R = crate::R<bool, ANACK_A>;
impl ANACK_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ANACK_A {
        match self.bits {
            false => ANACK_A::NOTRECEIVED,
            true => ANACK_A::RECEIVED,
        }
    }
    #[doc = "Checks if the value of the field is `NOTRECEIVED`"]
    #[inline(always)]
    pub fn is_not_received(&self) -> bool {
        *self == ANACK_A::NOTRECEIVED
    }
    #[doc = "Checks if the value of the field is `RECEIVED`"]
    #[inline(always)]
    pub fn is_received(&self) -> bool {
        *self == ANACK_A::RECEIVED
    }
}
#[doc = "Write proxy for field `ANACK`"]
pub struct ANACK_W<'a> {
    w: &'a mut W,
}
impl<'a> ANACK_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: ANACK_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Error did not occur"]
    #[inline(always)]
    pub fn not_received(self) -> &'a mut W {
        self.variant(ANACK_A::NOTRECEIVED)
    }
    #[doc = "Error occurred"]
    #[inline(always)]
    pub fn received(self) -> &'a mut W {
        self.variant(ANACK_A::RECEIVED)
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
#[doc = "NACK received after sending a data byte (write '1' to clear)\n\nValue on reset: 0"]
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
impl R {
    #[doc = "Bit 0 - Overrun error"]
    #[inline(always)]
    pub fn overrun(&self) -> OVERRUN_R {
        OVERRUN_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - NACK received after sending the address (write '1' to clear)"]
    #[inline(always)]
    pub fn anack(&self) -> ANACK_R {
        ANACK_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - NACK received after sending a data byte (write '1' to clear)"]
    #[inline(always)]
    pub fn dnack(&self) -> DNACK_R {
        DNACK_R::new(((self.bits >> 2) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Overrun error"]
    #[inline(always)]
    pub fn overrun(&mut self) -> OVERRUN_W {
        OVERRUN_W { w: self }
    }
    #[doc = "Bit 1 - NACK received after sending the address (write '1' to clear)"]
    #[inline(always)]
    pub fn anack(&mut self) -> ANACK_W {
        ANACK_W { w: self }
    }
    #[doc = "Bit 2 - NACK received after sending a data byte (write '1' to clear)"]
    #[inline(always)]
    pub fn dnack(&mut self) -> DNACK_W {
        DNACK_W { w: self }
    }
}
