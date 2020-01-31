#[doc = "Reader of register RX"]
pub type R = crate::R<u32, super::RX>;
#[doc = "Writer for register RX"]
pub type W = crate::W<u32, super::RX>;
#[doc = "Register RX `reset()`'s with value 0"]
impl crate::ResetValue for super::RX {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "No valid End of Frame detected\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CRCERROR_A {
    #[doc = "0: Valid CRC detected"]
    CRCCORRECT = 0,
    #[doc = "1: CRC received does not match local check"]
    CRCERROR = 1,
}
impl From<CRCERROR_A> for bool {
    #[inline(always)]
    fn from(variant: CRCERROR_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `CRCERROR`"]
pub type CRCERROR_R = crate::R<bool, CRCERROR_A>;
impl CRCERROR_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CRCERROR_A {
        match self.bits {
            false => CRCERROR_A::CRCCORRECT,
            true => CRCERROR_A::CRCERROR,
        }
    }
    #[doc = "Checks if the value of the field is `CRCCORRECT`"]
    #[inline(always)]
    pub fn is_crccorrect(&self) -> bool {
        *self == CRCERROR_A::CRCCORRECT
    }
    #[doc = "Checks if the value of the field is `CRCERROR`"]
    #[inline(always)]
    pub fn is_crcerror(&self) -> bool {
        *self == CRCERROR_A::CRCERROR
    }
}
#[doc = "Write proxy for field `CRCERROR`"]
pub struct CRCERROR_W<'a> {
    w: &'a mut W,
}
impl<'a> CRCERROR_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CRCERROR_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Valid CRC detected"]
    #[inline(always)]
    pub fn crccorrect(self) -> &'a mut W {
        self.variant(CRCERROR_A::CRCCORRECT)
    }
    #[doc = "CRC received does not match local check"]
    #[inline(always)]
    pub fn crcerror(self) -> &'a mut W {
        self.variant(CRCERROR_A::CRCERROR)
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
#[doc = "Parity status of received frame\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PARITYSTATUS_A {
    #[doc = "0: Frame received with parity OK"]
    PARITYOK = 0,
    #[doc = "1: Frame received with parity error"]
    PARITYERROR = 1,
}
impl From<PARITYSTATUS_A> for bool {
    #[inline(always)]
    fn from(variant: PARITYSTATUS_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `PARITYSTATUS`"]
pub type PARITYSTATUS_R = crate::R<bool, PARITYSTATUS_A>;
impl PARITYSTATUS_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PARITYSTATUS_A {
        match self.bits {
            false => PARITYSTATUS_A::PARITYOK,
            true => PARITYSTATUS_A::PARITYERROR,
        }
    }
    #[doc = "Checks if the value of the field is `PARITYOK`"]
    #[inline(always)]
    pub fn is_parity_ok(&self) -> bool {
        *self == PARITYSTATUS_A::PARITYOK
    }
    #[doc = "Checks if the value of the field is `PARITYERROR`"]
    #[inline(always)]
    pub fn is_parity_error(&self) -> bool {
        *self == PARITYSTATUS_A::PARITYERROR
    }
}
#[doc = "Write proxy for field `PARITYSTATUS`"]
pub struct PARITYSTATUS_W<'a> {
    w: &'a mut W,
}
impl<'a> PARITYSTATUS_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PARITYSTATUS_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Frame received with parity OK"]
    #[inline(always)]
    pub fn parity_ok(self) -> &'a mut W {
        self.variant(PARITYSTATUS_A::PARITYOK)
    }
    #[doc = "Frame received with parity error"]
    #[inline(always)]
    pub fn parity_error(self) -> &'a mut W {
        self.variant(PARITYSTATUS_A::PARITYERROR)
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
#[doc = "Overrun detected\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum OVERRUN_A {
    #[doc = "0: No overrun detected"]
    NOOVERRUN = 0,
    #[doc = "1: Overrun error"]
    OVERRUN = 1,
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
            false => OVERRUN_A::NOOVERRUN,
            true => OVERRUN_A::OVERRUN,
        }
    }
    #[doc = "Checks if the value of the field is `NOOVERRUN`"]
    #[inline(always)]
    pub fn is_no_overrun(&self) -> bool {
        *self == OVERRUN_A::NOOVERRUN
    }
    #[doc = "Checks if the value of the field is `OVERRUN`"]
    #[inline(always)]
    pub fn is_overrun(&self) -> bool {
        *self == OVERRUN_A::OVERRUN
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
    #[doc = "No overrun detected"]
    #[inline(always)]
    pub fn no_overrun(self) -> &'a mut W {
        self.variant(OVERRUN_A::NOOVERRUN)
    }
    #[doc = "Overrun error"]
    #[inline(always)]
    pub fn overrun(self) -> &'a mut W {
        self.variant(OVERRUN_A::OVERRUN)
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
    #[doc = "Bit 0 - No valid End of Frame detected"]
    #[inline(always)]
    pub fn crcerror(&self) -> CRCERROR_R {
        CRCERROR_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 2 - Parity status of received frame"]
    #[inline(always)]
    pub fn paritystatus(&self) -> PARITYSTATUS_R {
        PARITYSTATUS_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - Overrun detected"]
    #[inline(always)]
    pub fn overrun(&self) -> OVERRUN_R {
        OVERRUN_R::new(((self.bits >> 3) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - No valid End of Frame detected"]
    #[inline(always)]
    pub fn crcerror(&mut self) -> CRCERROR_W {
        CRCERROR_W { w: self }
    }
    #[doc = "Bit 2 - Parity status of received frame"]
    #[inline(always)]
    pub fn paritystatus(&mut self) -> PARITYSTATUS_W {
        PARITYSTATUS_W { w: self }
    }
    #[doc = "Bit 3 - Overrun detected"]
    #[inline(always)]
    pub fn overrun(&mut self) -> OVERRUN_W {
        OVERRUN_W { w: self }
    }
}
