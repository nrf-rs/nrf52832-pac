#[doc = "Reader of register SELRES"]
pub type R = crate::R<u32, super::SELRES>;
#[doc = "Writer for register SELRES"]
pub type W = crate::W<u32, super::SELRES>;
#[doc = "Register SELRES `reset()`'s with value 0"]
impl crate::ResetValue for super::SELRES {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `RFU10`"]
pub type RFU10_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `RFU10`"]
pub struct RFU10_W<'a> {
    w: &'a mut W,
}
impl<'a> RFU10_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x03) | ((value as u32) & 0x03);
        self.w
    }
}
#[doc = "Cascade bit (controlled by hardware, write has no effect)\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CASCADE_A {
    #[doc = "0: NFCID1 complete"]
    COMPLETE = 0,
    #[doc = "1: NFCID1 not complete"]
    NOTCOMPLETE = 1,
}
impl From<CASCADE_A> for bool {
    #[inline(always)]
    fn from(variant: CASCADE_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `CASCADE`"]
pub type CASCADE_R = crate::R<bool, CASCADE_A>;
impl CASCADE_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CASCADE_A {
        match self.bits {
            false => CASCADE_A::COMPLETE,
            true => CASCADE_A::NOTCOMPLETE,
        }
    }
    #[doc = "Checks if the value of the field is `COMPLETE`"]
    #[inline(always)]
    pub fn is_complete(&self) -> bool {
        *self == CASCADE_A::COMPLETE
    }
    #[doc = "Checks if the value of the field is `NOTCOMPLETE`"]
    #[inline(always)]
    pub fn is_not_complete(&self) -> bool {
        *self == CASCADE_A::NOTCOMPLETE
    }
}
#[doc = "Write proxy for field `CASCADE`"]
pub struct CASCADE_W<'a> {
    w: &'a mut W,
}
impl<'a> CASCADE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CASCADE_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "NFCID1 complete"]
    #[inline(always)]
    pub fn complete(self) -> &'a mut W {
        self.variant(CASCADE_A::COMPLETE)
    }
    #[doc = "NFCID1 not complete"]
    #[inline(always)]
    pub fn not_complete(self) -> &'a mut W {
        self.variant(CASCADE_A::NOTCOMPLETE)
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
#[doc = "Reader of field `RFU43`"]
pub type RFU43_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `RFU43`"]
pub struct RFU43_W<'a> {
    w: &'a mut W,
}
impl<'a> RFU43_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 3)) | (((value as u32) & 0x03) << 3);
        self.w
    }
}
#[doc = "Reader of field `PROTOCOL`"]
pub type PROTOCOL_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `PROTOCOL`"]
pub struct PROTOCOL_W<'a> {
    w: &'a mut W,
}
impl<'a> PROTOCOL_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 5)) | (((value as u32) & 0x03) << 5);
        self.w
    }
}
#[doc = "Reader of field `RFU7`"]
pub type RFU7_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `RFU7`"]
pub struct RFU7_W<'a> {
    w: &'a mut W,
}
impl<'a> RFU7_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 7)) | (((value as u32) & 0x01) << 7);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:1 - Reserved for future use. Shall be 0."]
    #[inline(always)]
    pub fn rfu10(&self) -> RFU10_R {
        RFU10_R::new((self.bits & 0x03) as u8)
    }
    #[doc = "Bit 2 - Cascade bit (controlled by hardware, write has no effect)"]
    #[inline(always)]
    pub fn cascade(&self) -> CASCADE_R {
        CASCADE_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bits 3:4 - Reserved for future use. Shall be 0."]
    #[inline(always)]
    pub fn rfu43(&self) -> RFU43_R {
        RFU43_R::new(((self.bits >> 3) & 0x03) as u8)
    }
    #[doc = "Bits 5:6 - Protocol as defined by the b7:b6 of SEL_RES response in the NFC Forum, NFC Digital Protocol Technical Specification"]
    #[inline(always)]
    pub fn protocol(&self) -> PROTOCOL_R {
        PROTOCOL_R::new(((self.bits >> 5) & 0x03) as u8)
    }
    #[doc = "Bit 7 - Reserved for future use. Shall be 0."]
    #[inline(always)]
    pub fn rfu7(&self) -> RFU7_R {
        RFU7_R::new(((self.bits >> 7) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bits 0:1 - Reserved for future use. Shall be 0."]
    #[inline(always)]
    pub fn rfu10(&mut self) -> RFU10_W {
        RFU10_W { w: self }
    }
    #[doc = "Bit 2 - Cascade bit (controlled by hardware, write has no effect)"]
    #[inline(always)]
    pub fn cascade(&mut self) -> CASCADE_W {
        CASCADE_W { w: self }
    }
    #[doc = "Bits 3:4 - Reserved for future use. Shall be 0."]
    #[inline(always)]
    pub fn rfu43(&mut self) -> RFU43_W {
        RFU43_W { w: self }
    }
    #[doc = "Bits 5:6 - Protocol as defined by the b7:b6 of SEL_RES response in the NFC Forum, NFC Digital Protocol Technical Specification"]
    #[inline(always)]
    pub fn protocol(&mut self) -> PROTOCOL_W {
        PROTOCOL_W { w: self }
    }
    #[doc = "Bit 7 - Reserved for future use. Shall be 0."]
    #[inline(always)]
    pub fn rfu7(&mut self) -> RFU7_W {
        RFU7_W { w: self }
    }
}
