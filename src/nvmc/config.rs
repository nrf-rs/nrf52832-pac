#[doc = "Reader of register CONFIG"]
pub type R = crate::R<u32, super::CONFIG>;
#[doc = "Writer for register CONFIG"]
pub type W = crate::W<u32, super::CONFIG>;
#[doc = "Register CONFIG `reset()`'s with value 0"]
impl crate::ResetValue for super::CONFIG {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Program memory access mode. It is strongly recommended to only activate erase and write modes when they are actively used. Enabling write or erase will invalidate the cache and keep it invalidated.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum WEN_A {
    #[doc = "0: Read only access"]
    REN = 0,
    #[doc = "1: Write Enabled"]
    WEN = 1,
    #[doc = "2: Erase enabled"]
    EEN = 2,
}
impl From<WEN_A> for u8 {
    #[inline(always)]
    fn from(variant: WEN_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `WEN`"]
pub type WEN_R = crate::R<u8, WEN_A>;
impl WEN_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, WEN_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(WEN_A::REN),
            1 => Val(WEN_A::WEN),
            2 => Val(WEN_A::EEN),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `REN`"]
    #[inline(always)]
    pub fn is_ren(&self) -> bool {
        *self == WEN_A::REN
    }
    #[doc = "Checks if the value of the field is `WEN`"]
    #[inline(always)]
    pub fn is_wen(&self) -> bool {
        *self == WEN_A::WEN
    }
    #[doc = "Checks if the value of the field is `EEN`"]
    #[inline(always)]
    pub fn is_een(&self) -> bool {
        *self == WEN_A::EEN
    }
}
#[doc = "Write proxy for field `WEN`"]
pub struct WEN_W<'a> {
    w: &'a mut W,
}
impl<'a> WEN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: WEN_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "Read only access"]
    #[inline(always)]
    pub fn ren(self) -> &'a mut W {
        self.variant(WEN_A::REN)
    }
    #[doc = "Write Enabled"]
    #[inline(always)]
    pub fn wen(self) -> &'a mut W {
        self.variant(WEN_A::WEN)
    }
    #[doc = "Erase enabled"]
    #[inline(always)]
    pub fn een(self) -> &'a mut W {
        self.variant(WEN_A::EEN)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x03) | ((value as u32) & 0x03);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:1 - Program memory access mode. It is strongly recommended to only activate erase and write modes when they are actively used. Enabling write or erase will invalidate the cache and keep it invalidated."]
    #[inline(always)]
    pub fn wen(&self) -> WEN_R {
        WEN_R::new((self.bits & 0x03) as u8)
    }
}
impl W {
    #[doc = "Bits 0:1 - Program memory access mode. It is strongly recommended to only activate erase and write modes when they are actively used. Enabling write or erase will invalidate the cache and keep it invalidated."]
    #[inline(always)]
    pub fn wen(&mut self) -> WEN_W {
        WEN_W { w: self }
    }
}
