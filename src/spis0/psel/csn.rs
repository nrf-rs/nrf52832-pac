#[doc = "Reader of register CSN"]
pub type R = crate::R<u32, super::CSN>;
#[doc = "Writer for register CSN"]
pub type W = crate::W<u32, super::CSN>;
#[doc = "Register CSN `reset()`'s with value 0xffff_ffff"]
impl crate::ResetValue for super::CSN {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0xffff_ffff
    }
}
#[doc = "Reader of field `PIN`"]
pub type PIN_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `PIN`"]
pub struct PIN_W<'a> {
    w: &'a mut W,
}
impl<'a> PIN_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x1f) | ((value as u32) & 0x1f);
        self.w
    }
}
#[doc = "Connection\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CONNECT_A {
    #[doc = "1: Disconnect"]
    DISCONNECTED = 1,
    #[doc = "0: Connect"]
    CONNECTED = 0,
}
impl From<CONNECT_A> for bool {
    #[inline(always)]
    fn from(variant: CONNECT_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `CONNECT`"]
pub type CONNECT_R = crate::R<bool, CONNECT_A>;
impl CONNECT_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CONNECT_A {
        match self.bits {
            true => CONNECT_A::DISCONNECTED,
            false => CONNECT_A::CONNECTED,
        }
    }
    #[doc = "Checks if the value of the field is `DISCONNECTED`"]
    #[inline(always)]
    pub fn is_disconnected(&self) -> bool {
        *self == CONNECT_A::DISCONNECTED
    }
    #[doc = "Checks if the value of the field is `CONNECTED`"]
    #[inline(always)]
    pub fn is_connected(&self) -> bool {
        *self == CONNECT_A::CONNECTED
    }
}
#[doc = "Write proxy for field `CONNECT`"]
pub struct CONNECT_W<'a> {
    w: &'a mut W,
}
impl<'a> CONNECT_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CONNECT_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Disconnect"]
    #[inline(always)]
    pub fn disconnected(self) -> &'a mut W {
        self.variant(CONNECT_A::DISCONNECTED)
    }
    #[doc = "Connect"]
    #[inline(always)]
    pub fn connected(self) -> &'a mut W {
        self.variant(CONNECT_A::CONNECTED)
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
        self.w.bits = (self.w.bits & !(0x01 << 31)) | (((value as u32) & 0x01) << 31);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:4 - Pin number"]
    #[inline(always)]
    pub fn pin(&self) -> PIN_R {
        PIN_R::new((self.bits & 0x1f) as u8)
    }
    #[doc = "Bit 31 - Connection"]
    #[inline(always)]
    pub fn connect(&self) -> CONNECT_R {
        CONNECT_R::new(((self.bits >> 31) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bits 0:4 - Pin number"]
    #[inline(always)]
    pub fn pin(&mut self) -> PIN_W {
        PIN_W { w: self }
    }
    #[doc = "Bit 31 - Connection"]
    #[inline(always)]
    pub fn connect(&mut self) -> CONNECT_W {
        CONNECT_W { w: self }
    }
}
