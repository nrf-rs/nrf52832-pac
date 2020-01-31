#[doc = "Reader of register STATUS"]
pub type R = crate::R<u32, super::STATUS>;
#[doc = "Writer for register STATUS"]
pub type W = crate::W<u32, super::STATUS>;
#[doc = "Register STATUS `reset()`'s with value 0"]
impl crate::ResetValue for super::STATUS {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "TX buffer over-read detected, and prevented\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum OVERREAD_A {
    #[doc = "0: Read: error not present"]
    NOTPRESENT = 0,
    #[doc = "1: Read: error present"]
    PRESENT = 1,
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
            false => OVERREAD_A::NOTPRESENT,
            true => OVERREAD_A::PRESENT,
        }
    }
    #[doc = "Checks if the value of the field is `NOTPRESENT`"]
    #[inline(always)]
    pub fn is_not_present(&self) -> bool {
        *self == OVERREAD_A::NOTPRESENT
    }
    #[doc = "Checks if the value of the field is `PRESENT`"]
    #[inline(always)]
    pub fn is_present(&self) -> bool {
        *self == OVERREAD_A::PRESENT
    }
}
#[doc = "TX buffer over-read detected, and prevented\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum OVERREAD_AW {
    #[doc = "1: Write: clear error on writing '1'"]
    CLEAR = 1,
}
impl From<OVERREAD_AW> for bool {
    #[inline(always)]
    fn from(variant: OVERREAD_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Write proxy for field `OVERREAD`"]
pub struct OVERREAD_W<'a> {
    w: &'a mut W,
}
impl<'a> OVERREAD_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: OVERREAD_AW) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Write: clear error on writing '1'"]
    #[inline(always)]
    pub fn clear(self) -> &'a mut W {
        self.variant(OVERREAD_AW::CLEAR)
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
#[doc = "RX buffer overflow detected, and prevented\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum OVERFLOW_A {
    #[doc = "0: Read: error not present"]
    NOTPRESENT = 0,
    #[doc = "1: Read: error present"]
    PRESENT = 1,
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
            false => OVERFLOW_A::NOTPRESENT,
            true => OVERFLOW_A::PRESENT,
        }
    }
    #[doc = "Checks if the value of the field is `NOTPRESENT`"]
    #[inline(always)]
    pub fn is_not_present(&self) -> bool {
        *self == OVERFLOW_A::NOTPRESENT
    }
    #[doc = "Checks if the value of the field is `PRESENT`"]
    #[inline(always)]
    pub fn is_present(&self) -> bool {
        *self == OVERFLOW_A::PRESENT
    }
}
#[doc = "RX buffer overflow detected, and prevented\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum OVERFLOW_AW {
    #[doc = "1: Write: clear error on writing '1'"]
    CLEAR = 1,
}
impl From<OVERFLOW_AW> for bool {
    #[inline(always)]
    fn from(variant: OVERFLOW_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Write proxy for field `OVERFLOW`"]
pub struct OVERFLOW_W<'a> {
    w: &'a mut W,
}
impl<'a> OVERFLOW_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: OVERFLOW_AW) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Write: clear error on writing '1'"]
    #[inline(always)]
    pub fn clear(self) -> &'a mut W {
        self.variant(OVERFLOW_AW::CLEAR)
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
impl R {
    #[doc = "Bit 0 - TX buffer over-read detected, and prevented"]
    #[inline(always)]
    pub fn overread(&self) -> OVERREAD_R {
        OVERREAD_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - RX buffer overflow detected, and prevented"]
    #[inline(always)]
    pub fn overflow(&self) -> OVERFLOW_R {
        OVERFLOW_R::new(((self.bits >> 1) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - TX buffer over-read detected, and prevented"]
    #[inline(always)]
    pub fn overread(&mut self) -> OVERREAD_W {
        OVERREAD_W { w: self }
    }
    #[doc = "Bit 1 - RX buffer overflow detected, and prevented"]
    #[inline(always)]
    pub fn overflow(&mut self) -> OVERFLOW_W {
        OVERFLOW_W { w: self }
    }
}
