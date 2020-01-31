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
    #[doc = "0: Read: no overrun occured"]
    NOTPRESENT = 0,
    #[doc = "1: Read: overrun occured"]
    PRESENT = 1,
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
            false => OVERRUN_A::NOTPRESENT,
            true => OVERRUN_A::PRESENT,
        }
    }
    #[doc = "Checks if the value of the field is `NOTPRESENT`"]
    #[inline(always)]
    pub fn is_not_present(&self) -> bool {
        *self == OVERRUN_A::NOTPRESENT
    }
    #[doc = "Checks if the value of the field is `PRESENT`"]
    #[inline(always)]
    pub fn is_present(&self) -> bool {
        *self == OVERRUN_A::PRESENT
    }
}
#[doc = "Overrun error\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum OVERRUN_AW {
    #[doc = "1: Write: clear error on writing '1'"]
    CLEAR = 1,
}
impl From<OVERRUN_AW> for bool {
    #[inline(always)]
    fn from(variant: OVERRUN_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Write proxy for field `OVERRUN`"]
pub struct OVERRUN_W<'a> {
    w: &'a mut W,
}
impl<'a> OVERRUN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: OVERRUN_AW) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Write: clear error on writing '1'"]
    #[inline(always)]
    pub fn clear(self) -> &'a mut W {
        self.variant(OVERRUN_AW::CLEAR)
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
    #[doc = "0: Read: error not present"]
    NOTPRESENT = 0,
    #[doc = "1: Read: error present"]
    PRESENT = 1,
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
            false => ANACK_A::NOTPRESENT,
            true => ANACK_A::PRESENT,
        }
    }
    #[doc = "Checks if the value of the field is `NOTPRESENT`"]
    #[inline(always)]
    pub fn is_not_present(&self) -> bool {
        *self == ANACK_A::NOTPRESENT
    }
    #[doc = "Checks if the value of the field is `PRESENT`"]
    #[inline(always)]
    pub fn is_present(&self) -> bool {
        *self == ANACK_A::PRESENT
    }
}
#[doc = "NACK received after sending the address (write '1' to clear)\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ANACK_AW {
    #[doc = "1: Write: clear error on writing '1'"]
    CLEAR = 1,
}
impl From<ANACK_AW> for bool {
    #[inline(always)]
    fn from(variant: ANACK_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Write proxy for field `ANACK`"]
pub struct ANACK_W<'a> {
    w: &'a mut W,
}
impl<'a> ANACK_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: ANACK_AW) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Write: clear error on writing '1'"]
    #[inline(always)]
    pub fn clear(self) -> &'a mut W {
        self.variant(ANACK_AW::CLEAR)
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
    #[doc = "0: Read: error not present"]
    NOTPRESENT = 0,
    #[doc = "1: Read: error present"]
    PRESENT = 1,
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
            false => DNACK_A::NOTPRESENT,
            true => DNACK_A::PRESENT,
        }
    }
    #[doc = "Checks if the value of the field is `NOTPRESENT`"]
    #[inline(always)]
    pub fn is_not_present(&self) -> bool {
        *self == DNACK_A::NOTPRESENT
    }
    #[doc = "Checks if the value of the field is `PRESENT`"]
    #[inline(always)]
    pub fn is_present(&self) -> bool {
        *self == DNACK_A::PRESENT
    }
}
#[doc = "NACK received after sending a data byte (write '1' to clear)\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DNACK_AW {
    #[doc = "1: Write: clear error on writing '1'"]
    CLEAR = 1,
}
impl From<DNACK_AW> for bool {
    #[inline(always)]
    fn from(variant: DNACK_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Write proxy for field `DNACK`"]
pub struct DNACK_W<'a> {
    w: &'a mut W,
}
impl<'a> DNACK_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: DNACK_AW) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Write: clear error on writing '1'"]
    #[inline(always)]
    pub fn clear(self) -> &'a mut W {
        self.variant(DNACK_AW::CLEAR)
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
