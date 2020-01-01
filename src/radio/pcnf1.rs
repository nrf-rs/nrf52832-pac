#[doc = "Reader of register PCNF1"]
pub type R = crate::R<u32, super::PCNF1>;
#[doc = "Writer for register PCNF1"]
pub type W = crate::W<u32, super::PCNF1>;
#[doc = "Register PCNF1 `reset()`'s with value 0"]
impl crate::ResetValue for super::PCNF1 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `MAXLEN`"]
pub type MAXLEN_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `MAXLEN`"]
pub struct MAXLEN_W<'a> {
    w: &'a mut W,
}
impl<'a> MAXLEN_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xff) | ((value as u32) & 0xff);
        self.w
    }
}
#[doc = "Reader of field `STATLEN`"]
pub type STATLEN_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `STATLEN`"]
pub struct STATLEN_W<'a> {
    w: &'a mut W,
}
impl<'a> STATLEN_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xff << 8)) | (((value as u32) & 0xff) << 8);
        self.w
    }
}
#[doc = "Reader of field `BALEN`"]
pub type BALEN_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `BALEN`"]
pub struct BALEN_W<'a> {
    w: &'a mut W,
}
impl<'a> BALEN_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 16)) | (((value as u32) & 0x07) << 16);
        self.w
    }
}
#[doc = "On air endianness of packet, this applies to the S0, LENGTH, S1 and the PAYLOAD fields.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ENDIAN_A {
    #[doc = "0: Least Significant bit on air first"]
    LITTLE = 0,
    #[doc = "1: Most significant bit on air first"]
    BIG = 1,
}
impl From<ENDIAN_A> for bool {
    #[inline(always)]
    fn from(variant: ENDIAN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `ENDIAN`"]
pub type ENDIAN_R = crate::R<bool, ENDIAN_A>;
impl ENDIAN_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ENDIAN_A {
        match self.bits {
            false => ENDIAN_A::LITTLE,
            true => ENDIAN_A::BIG,
        }
    }
    #[doc = "Checks if the value of the field is `LITTLE`"]
    #[inline(always)]
    pub fn is_little(&self) -> bool {
        *self == ENDIAN_A::LITTLE
    }
    #[doc = "Checks if the value of the field is `BIG`"]
    #[inline(always)]
    pub fn is_big(&self) -> bool {
        *self == ENDIAN_A::BIG
    }
}
#[doc = "Write proxy for field `ENDIAN`"]
pub struct ENDIAN_W<'a> {
    w: &'a mut W,
}
impl<'a> ENDIAN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: ENDIAN_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Least Significant bit on air first"]
    #[inline(always)]
    pub fn little(self) -> &'a mut W {
        self.variant(ENDIAN_A::LITTLE)
    }
    #[doc = "Most significant bit on air first"]
    #[inline(always)]
    pub fn big(self) -> &'a mut W {
        self.variant(ENDIAN_A::BIG)
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
#[doc = "Enable or disable packet whitening\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum WHITEEN_A {
    #[doc = "0: Disable"]
    DISABLED = 0,
    #[doc = "1: Enable"]
    ENABLED = 1,
}
impl From<WHITEEN_A> for bool {
    #[inline(always)]
    fn from(variant: WHITEEN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `WHITEEN`"]
pub type WHITEEN_R = crate::R<bool, WHITEEN_A>;
impl WHITEEN_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> WHITEEN_A {
        match self.bits {
            false => WHITEEN_A::DISABLED,
            true => WHITEEN_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == WHITEEN_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == WHITEEN_A::ENABLED
    }
}
#[doc = "Write proxy for field `WHITEEN`"]
pub struct WHITEEN_W<'a> {
    w: &'a mut W,
}
impl<'a> WHITEEN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: WHITEEN_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Disable"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(WHITEEN_A::DISABLED)
    }
    #[doc = "Enable"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(WHITEEN_A::ENABLED)
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
        self.w.bits = (self.w.bits & !(0x01 << 25)) | (((value as u32) & 0x01) << 25);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:7 - Maximum length of packet payload. If the packet payload is larger than MAXLEN, the radio will truncate the payload to MAXLEN."]
    #[inline(always)]
    pub fn maxlen(&self) -> MAXLEN_R {
        MAXLEN_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - Static length in number of bytes"]
    #[inline(always)]
    pub fn statlen(&self) -> STATLEN_R {
        STATLEN_R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:18 - Base address length in number of bytes"]
    #[inline(always)]
    pub fn balen(&self) -> BALEN_R {
        BALEN_R::new(((self.bits >> 16) & 0x07) as u8)
    }
    #[doc = "Bit 24 - On air endianness of packet, this applies to the S0, LENGTH, S1 and the PAYLOAD fields."]
    #[inline(always)]
    pub fn endian(&self) -> ENDIAN_R {
        ENDIAN_R::new(((self.bits >> 24) & 0x01) != 0)
    }
    #[doc = "Bit 25 - Enable or disable packet whitening"]
    #[inline(always)]
    pub fn whiteen(&self) -> WHITEEN_R {
        WHITEEN_R::new(((self.bits >> 25) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bits 0:7 - Maximum length of packet payload. If the packet payload is larger than MAXLEN, the radio will truncate the payload to MAXLEN."]
    #[inline(always)]
    pub fn maxlen(&mut self) -> MAXLEN_W {
        MAXLEN_W { w: self }
    }
    #[doc = "Bits 8:15 - Static length in number of bytes"]
    #[inline(always)]
    pub fn statlen(&mut self) -> STATLEN_W {
        STATLEN_W { w: self }
    }
    #[doc = "Bits 16:18 - Base address length in number of bytes"]
    #[inline(always)]
    pub fn balen(&mut self) -> BALEN_W {
        BALEN_W { w: self }
    }
    #[doc = "Bit 24 - On air endianness of packet, this applies to the S0, LENGTH, S1 and the PAYLOAD fields."]
    #[inline(always)]
    pub fn endian(&mut self) -> ENDIAN_W {
        ENDIAN_W { w: self }
    }
    #[doc = "Bit 25 - Enable or disable packet whitening"]
    #[inline(always)]
    pub fn whiteen(&mut self) -> WHITEEN_W {
        WHITEEN_W { w: self }
    }
}
