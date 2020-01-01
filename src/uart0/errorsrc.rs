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
    #[doc = "0: Read: error not present"]
    NOTPRESENT = 0,
    #[doc = "1: Read: error present"]
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
    #[doc = "Read: error not present"]
    #[inline(always)]
    pub fn not_present(self) -> &'a mut W {
        self.variant(OVERRUN_A::NOTPRESENT)
    }
    #[doc = "Read: error present"]
    #[inline(always)]
    pub fn present(self) -> &'a mut W {
        self.variant(OVERRUN_A::PRESENT)
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
#[doc = "Parity error\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PARITY_A {
    #[doc = "0: Read: error not present"]
    NOTPRESENT = 0,
    #[doc = "1: Read: error present"]
    PRESENT = 1,
}
impl From<PARITY_A> for bool {
    #[inline(always)]
    fn from(variant: PARITY_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `PARITY`"]
pub type PARITY_R = crate::R<bool, PARITY_A>;
impl PARITY_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PARITY_A {
        match self.bits {
            false => PARITY_A::NOTPRESENT,
            true => PARITY_A::PRESENT,
        }
    }
    #[doc = "Checks if the value of the field is `NOTPRESENT`"]
    #[inline(always)]
    pub fn is_not_present(&self) -> bool {
        *self == PARITY_A::NOTPRESENT
    }
    #[doc = "Checks if the value of the field is `PRESENT`"]
    #[inline(always)]
    pub fn is_present(&self) -> bool {
        *self == PARITY_A::PRESENT
    }
}
#[doc = "Write proxy for field `PARITY`"]
pub struct PARITY_W<'a> {
    w: &'a mut W,
}
impl<'a> PARITY_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PARITY_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Read: error not present"]
    #[inline(always)]
    pub fn not_present(self) -> &'a mut W {
        self.variant(PARITY_A::NOTPRESENT)
    }
    #[doc = "Read: error present"]
    #[inline(always)]
    pub fn present(self) -> &'a mut W {
        self.variant(PARITY_A::PRESENT)
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
#[doc = "Framing error occurred\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum FRAMING_A {
    #[doc = "0: Read: error not present"]
    NOTPRESENT = 0,
    #[doc = "1: Read: error present"]
    PRESENT = 1,
}
impl From<FRAMING_A> for bool {
    #[inline(always)]
    fn from(variant: FRAMING_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `FRAMING`"]
pub type FRAMING_R = crate::R<bool, FRAMING_A>;
impl FRAMING_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> FRAMING_A {
        match self.bits {
            false => FRAMING_A::NOTPRESENT,
            true => FRAMING_A::PRESENT,
        }
    }
    #[doc = "Checks if the value of the field is `NOTPRESENT`"]
    #[inline(always)]
    pub fn is_not_present(&self) -> bool {
        *self == FRAMING_A::NOTPRESENT
    }
    #[doc = "Checks if the value of the field is `PRESENT`"]
    #[inline(always)]
    pub fn is_present(&self) -> bool {
        *self == FRAMING_A::PRESENT
    }
}
#[doc = "Write proxy for field `FRAMING`"]
pub struct FRAMING_W<'a> {
    w: &'a mut W,
}
impl<'a> FRAMING_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: FRAMING_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Read: error not present"]
    #[inline(always)]
    pub fn not_present(self) -> &'a mut W {
        self.variant(FRAMING_A::NOTPRESENT)
    }
    #[doc = "Read: error present"]
    #[inline(always)]
    pub fn present(self) -> &'a mut W {
        self.variant(FRAMING_A::PRESENT)
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
#[doc = "Break condition\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum BREAK_A {
    #[doc = "0: Read: error not present"]
    NOTPRESENT = 0,
    #[doc = "1: Read: error present"]
    PRESENT = 1,
}
impl From<BREAK_A> for bool {
    #[inline(always)]
    fn from(variant: BREAK_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `BREAK`"]
pub type BREAK_R = crate::R<bool, BREAK_A>;
impl BREAK_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> BREAK_A {
        match self.bits {
            false => BREAK_A::NOTPRESENT,
            true => BREAK_A::PRESENT,
        }
    }
    #[doc = "Checks if the value of the field is `NOTPRESENT`"]
    #[inline(always)]
    pub fn is_not_present(&self) -> bool {
        *self == BREAK_A::NOTPRESENT
    }
    #[doc = "Checks if the value of the field is `PRESENT`"]
    #[inline(always)]
    pub fn is_present(&self) -> bool {
        *self == BREAK_A::PRESENT
    }
}
#[doc = "Write proxy for field `BREAK`"]
pub struct BREAK_W<'a> {
    w: &'a mut W,
}
impl<'a> BREAK_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: BREAK_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Read: error not present"]
    #[inline(always)]
    pub fn not_present(self) -> &'a mut W {
        self.variant(BREAK_A::NOTPRESENT)
    }
    #[doc = "Read: error present"]
    #[inline(always)]
    pub fn present(self) -> &'a mut W {
        self.variant(BREAK_A::PRESENT)
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
    #[doc = "Bit 0 - Overrun error"]
    #[inline(always)]
    pub fn overrun(&self) -> OVERRUN_R {
        OVERRUN_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - Parity error"]
    #[inline(always)]
    pub fn parity(&self) -> PARITY_R {
        PARITY_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - Framing error occurred"]
    #[inline(always)]
    pub fn framing(&self) -> FRAMING_R {
        FRAMING_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - Break condition"]
    #[inline(always)]
    pub fn break_(&self) -> BREAK_R {
        BREAK_R::new(((self.bits >> 3) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Overrun error"]
    #[inline(always)]
    pub fn overrun(&mut self) -> OVERRUN_W {
        OVERRUN_W { w: self }
    }
    #[doc = "Bit 1 - Parity error"]
    #[inline(always)]
    pub fn parity(&mut self) -> PARITY_W {
        PARITY_W { w: self }
    }
    #[doc = "Bit 2 - Framing error occurred"]
    #[inline(always)]
    pub fn framing(&mut self) -> FRAMING_W {
        FRAMING_W { w: self }
    }
    #[doc = "Bit 3 - Break condition"]
    #[inline(always)]
    pub fn break_(&mut self) -> BREAK_W {
        BREAK_W { w: self }
    }
}
