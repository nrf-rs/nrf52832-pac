#[doc = "Reader of register NFCPINS"]
pub type R = crate::R<u32, super::NFCPINS>;
#[doc = "Writer for register NFCPINS"]
pub type W = crate::W<u32, super::NFCPINS>;
#[doc = "Register NFCPINS `reset()`'s with value 0xffff_ffff"]
impl crate::ResetValue for super::NFCPINS {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0xffff_ffff
    }
}
#[doc = "Setting of pins dedicated to NFC functionality\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PROTECT_A {
    #[doc = "0: Operation as GPIO pins. Same protection as normal GPIO pins"]
    DISABLED = 0,
    #[doc = "1: Operation as NFC antenna pins. Configures the protection for NFC operation"]
    NFC = 1,
}
impl From<PROTECT_A> for bool {
    #[inline(always)]
    fn from(variant: PROTECT_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `PROTECT`"]
pub type PROTECT_R = crate::R<bool, PROTECT_A>;
impl PROTECT_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PROTECT_A {
        match self.bits {
            false => PROTECT_A::DISABLED,
            true => PROTECT_A::NFC,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == PROTECT_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `NFC`"]
    #[inline(always)]
    pub fn is_nfc(&self) -> bool {
        *self == PROTECT_A::NFC
    }
}
#[doc = "Write proxy for field `PROTECT`"]
pub struct PROTECT_W<'a> {
    w: &'a mut W,
}
impl<'a> PROTECT_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PROTECT_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Operation as GPIO pins. Same protection as normal GPIO pins"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(PROTECT_A::DISABLED)
    }
    #[doc = "Operation as NFC antenna pins. Configures the protection for NFC operation"]
    #[inline(always)]
    pub fn nfc(self) -> &'a mut W {
        self.variant(PROTECT_A::NFC)
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
impl R {
    #[doc = "Bit 0 - Setting of pins dedicated to NFC functionality"]
    #[inline(always)]
    pub fn protect(&self) -> PROTECT_R {
        PROTECT_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Setting of pins dedicated to NFC functionality"]
    #[inline(always)]
    pub fn protect(&mut self) -> PROTECT_W {
        PROTECT_W { w: self }
    }
}
