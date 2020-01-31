#[doc = "Reader of register MODE"]
pub type R = crate::R<u32, super::MODE>;
#[doc = "Writer for register MODE"]
pub type W = crate::W<u32, super::MODE>;
#[doc = "Register MODE `reset()`'s with value 0x01"]
impl crate::ResetValue for super::MODE {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0x01
    }
}
#[doc = "The mode of operation to be used\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum MODE_A {
    #[doc = "0: AES CCM packet encryption mode"]
    ENCRYPTION = 0,
    #[doc = "1: AES CCM packet decryption mode"]
    DECRYPTION = 1,
}
impl From<MODE_A> for bool {
    #[inline(always)]
    fn from(variant: MODE_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `MODE`"]
pub type MODE_R = crate::R<bool, MODE_A>;
impl MODE_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> MODE_A {
        match self.bits {
            false => MODE_A::ENCRYPTION,
            true => MODE_A::DECRYPTION,
        }
    }
    #[doc = "Checks if the value of the field is `ENCRYPTION`"]
    #[inline(always)]
    pub fn is_encryption(&self) -> bool {
        *self == MODE_A::ENCRYPTION
    }
    #[doc = "Checks if the value of the field is `DECRYPTION`"]
    #[inline(always)]
    pub fn is_decryption(&self) -> bool {
        *self == MODE_A::DECRYPTION
    }
}
#[doc = "Write proxy for field `MODE`"]
pub struct MODE_W<'a> {
    w: &'a mut W,
}
impl<'a> MODE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: MODE_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "AES CCM packet encryption mode"]
    #[inline(always)]
    pub fn encryption(self) -> &'a mut W {
        self.variant(MODE_A::ENCRYPTION)
    }
    #[doc = "AES CCM packet decryption mode"]
    #[inline(always)]
    pub fn decryption(self) -> &'a mut W {
        self.variant(MODE_A::DECRYPTION)
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
#[doc = "Data rate that the CCM shall run in synch with\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DATARATE_A {
    #[doc = "0: In synch with 1 Mbit data rate"]
    _1MBIT = 0,
    #[doc = "1: In synch with 2 Mbit data rate"]
    _2MBIT = 1,
}
impl From<DATARATE_A> for bool {
    #[inline(always)]
    fn from(variant: DATARATE_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `DATARATE`"]
pub type DATARATE_R = crate::R<bool, DATARATE_A>;
impl DATARATE_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DATARATE_A {
        match self.bits {
            false => DATARATE_A::_1MBIT,
            true => DATARATE_A::_2MBIT,
        }
    }
    #[doc = "Checks if the value of the field is `_1MBIT`"]
    #[inline(always)]
    pub fn is_1mbit(&self) -> bool {
        *self == DATARATE_A::_1MBIT
    }
    #[doc = "Checks if the value of the field is `_2MBIT`"]
    #[inline(always)]
    pub fn is_2mbit(&self) -> bool {
        *self == DATARATE_A::_2MBIT
    }
}
#[doc = "Write proxy for field `DATARATE`"]
pub struct DATARATE_W<'a> {
    w: &'a mut W,
}
impl<'a> DATARATE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: DATARATE_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "In synch with 1 Mbit data rate"]
    #[inline(always)]
    pub fn _1mbit(self) -> &'a mut W {
        self.variant(DATARATE_A::_1MBIT)
    }
    #[doc = "In synch with 2 Mbit data rate"]
    #[inline(always)]
    pub fn _2mbit(self) -> &'a mut W {
        self.variant(DATARATE_A::_2MBIT)
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
        self.w.bits = (self.w.bits & !(0x01 << 16)) | (((value as u32) & 0x01) << 16);
        self.w
    }
}
#[doc = "Packet length configuration\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum LENGTH_A {
    #[doc = "0: Default length. Effective length of LENGTH field is 5-bit"]
    DEFAULT = 0,
    #[doc = "1: Extended length. Effective length of LENGTH field is 8-bit"]
    EXTENDED = 1,
}
impl From<LENGTH_A> for bool {
    #[inline(always)]
    fn from(variant: LENGTH_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `LENGTH`"]
pub type LENGTH_R = crate::R<bool, LENGTH_A>;
impl LENGTH_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> LENGTH_A {
        match self.bits {
            false => LENGTH_A::DEFAULT,
            true => LENGTH_A::EXTENDED,
        }
    }
    #[doc = "Checks if the value of the field is `DEFAULT`"]
    #[inline(always)]
    pub fn is_default(&self) -> bool {
        *self == LENGTH_A::DEFAULT
    }
    #[doc = "Checks if the value of the field is `EXTENDED`"]
    #[inline(always)]
    pub fn is_extended(&self) -> bool {
        *self == LENGTH_A::EXTENDED
    }
}
#[doc = "Write proxy for field `LENGTH`"]
pub struct LENGTH_W<'a> {
    w: &'a mut W,
}
impl<'a> LENGTH_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: LENGTH_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Default length. Effective length of LENGTH field is 5-bit"]
    #[inline(always)]
    pub fn default(self) -> &'a mut W {
        self.variant(LENGTH_A::DEFAULT)
    }
    #[doc = "Extended length. Effective length of LENGTH field is 8-bit"]
    #[inline(always)]
    pub fn extended(self) -> &'a mut W {
        self.variant(LENGTH_A::EXTENDED)
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
        self.w.bits = (self.w.bits & !(0x01 << 24)) | (((value as u32) & 0x01) << 24);
        self.w
    }
}
impl R {
    #[doc = "Bit 0 - The mode of operation to be used"]
    #[inline(always)]
    pub fn mode(&self) -> MODE_R {
        MODE_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 16 - Data rate that the CCM shall run in synch with"]
    #[inline(always)]
    pub fn datarate(&self) -> DATARATE_R {
        DATARATE_R::new(((self.bits >> 16) & 0x01) != 0)
    }
    #[doc = "Bit 24 - Packet length configuration"]
    #[inline(always)]
    pub fn length(&self) -> LENGTH_R {
        LENGTH_R::new(((self.bits >> 24) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - The mode of operation to be used"]
    #[inline(always)]
    pub fn mode(&mut self) -> MODE_W {
        MODE_W { w: self }
    }
    #[doc = "Bit 16 - Data rate that the CCM shall run in synch with"]
    #[inline(always)]
    pub fn datarate(&mut self) -> DATARATE_W {
        DATARATE_W { w: self }
    }
    #[doc = "Bit 24 - Packet length configuration"]
    #[inline(always)]
    pub fn length(&mut self) -> LENGTH_W {
        LENGTH_W { w: self }
    }
}
