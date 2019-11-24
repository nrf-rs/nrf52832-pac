#[doc = "Reader of register ERRORSTATUS"]
pub type R = crate::R<u32, super::ERRORSTATUS>;
#[doc = "Writer for register ERRORSTATUS"]
pub type W = crate::W<u32, super::ERRORSTATUS>;
#[doc = "Register ERRORSTATUS `reset()`'s with value 0"]
impl crate::ResetValue for super::ERRORSTATUS {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `FRAMEDELAYTIMEOUT`"]
pub type FRAMEDELAYTIMEOUT_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `FRAMEDELAYTIMEOUT`"]
pub struct FRAMEDELAYTIMEOUT_W<'a> {
    w: &'a mut W,
}
impl<'a> FRAMEDELAYTIMEOUT_W<'a> {
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
#[doc = "Reader of field `NFCFIELDTOOSTRONG`"]
pub type NFCFIELDTOOSTRONG_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `NFCFIELDTOOSTRONG`"]
pub struct NFCFIELDTOOSTRONG_W<'a> {
    w: &'a mut W,
}
impl<'a> NFCFIELDTOOSTRONG_W<'a> {
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
#[doc = "Reader of field `NFCFIELDTOOWEAK`"]
pub type NFCFIELDTOOWEAK_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `NFCFIELDTOOWEAK`"]
pub struct NFCFIELDTOOWEAK_W<'a> {
    w: &'a mut W,
}
impl<'a> NFCFIELDTOOWEAK_W<'a> {
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
    #[doc = "Bit 0 - No STARTTX task triggered before expiration of the time set in FRAMEDELAYMAX"]
    #[inline(always)]
    pub fn framedelaytimeout(&self) -> FRAMEDELAYTIMEOUT_R {
        FRAMEDELAYTIMEOUT_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 2 - Field level is too high at max load resistance"]
    #[inline(always)]
    pub fn nfcfieldtoostrong(&self) -> NFCFIELDTOOSTRONG_R {
        NFCFIELDTOOSTRONG_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - Field level is too low at min load resistance"]
    #[inline(always)]
    pub fn nfcfieldtooweak(&self) -> NFCFIELDTOOWEAK_R {
        NFCFIELDTOOWEAK_R::new(((self.bits >> 3) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - No STARTTX task triggered before expiration of the time set in FRAMEDELAYMAX"]
    #[inline(always)]
    pub fn framedelaytimeout(&mut self) -> FRAMEDELAYTIMEOUT_W {
        FRAMEDELAYTIMEOUT_W { w: self }
    }
    #[doc = "Bit 2 - Field level is too high at max load resistance"]
    #[inline(always)]
    pub fn nfcfieldtoostrong(&mut self) -> NFCFIELDTOOSTRONG_W {
        NFCFIELDTOOSTRONG_W { w: self }
    }
    #[doc = "Bit 3 - Field level is too low at min load resistance"]
    #[inline(always)]
    pub fn nfcfieldtooweak(&mut self) -> NFCFIELDTOOWEAK_W {
        NFCFIELDTOOWEAK_W { w: self }
    }
}
