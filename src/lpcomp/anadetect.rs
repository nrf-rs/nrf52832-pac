#[doc = "Reader of register ANADETECT"]
pub type R = crate::R<u32, super::ANADETECT>;
#[doc = "Writer for register ANADETECT"]
pub type W = crate::W<u32, super::ANADETECT>;
#[doc = "Register ANADETECT `reset()`'s with value 0"]
impl crate::ResetValue for super::ANADETECT {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Analog detect configuration\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum ANADETECT_A {
    #[doc = "0: Generate ANADETECT on crossing, both upward crossing and downward crossing"]
    CROSS = 0,
    #[doc = "1: Generate ANADETECT on upward crossing only"]
    UP = 1,
    #[doc = "2: Generate ANADETECT on downward crossing only"]
    DOWN = 2,
}
impl From<ANADETECT_A> for u8 {
    #[inline(always)]
    fn from(variant: ANADETECT_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `ANADETECT`"]
pub type ANADETECT_R = crate::R<u8, ANADETECT_A>;
impl ANADETECT_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, ANADETECT_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(ANADETECT_A::CROSS),
            1 => Val(ANADETECT_A::UP),
            2 => Val(ANADETECT_A::DOWN),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `CROSS`"]
    #[inline(always)]
    pub fn is_cross(&self) -> bool {
        *self == ANADETECT_A::CROSS
    }
    #[doc = "Checks if the value of the field is `UP`"]
    #[inline(always)]
    pub fn is_up(&self) -> bool {
        *self == ANADETECT_A::UP
    }
    #[doc = "Checks if the value of the field is `DOWN`"]
    #[inline(always)]
    pub fn is_down(&self) -> bool {
        *self == ANADETECT_A::DOWN
    }
}
#[doc = "Write proxy for field `ANADETECT`"]
pub struct ANADETECT_W<'a> {
    w: &'a mut W,
}
impl<'a> ANADETECT_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: ANADETECT_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "Generate ANADETECT on crossing, both upward crossing and downward crossing"]
    #[inline(always)]
    pub fn cross(self) -> &'a mut W {
        self.variant(ANADETECT_A::CROSS)
    }
    #[doc = "Generate ANADETECT on upward crossing only"]
    #[inline(always)]
    pub fn up(self) -> &'a mut W {
        self.variant(ANADETECT_A::UP)
    }
    #[doc = "Generate ANADETECT on downward crossing only"]
    #[inline(always)]
    pub fn down(self) -> &'a mut W {
        self.variant(ANADETECT_A::DOWN)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x03) | ((value as u32) & 0x03);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:1 - Analog detect configuration"]
    #[inline(always)]
    pub fn anadetect(&self) -> ANADETECT_R {
        ANADETECT_R::new((self.bits & 0x03) as u8)
    }
}
impl W {
    #[doc = "Bits 0:1 - Analog detect configuration"]
    #[inline(always)]
    pub fn anadetect(&mut self) -> ANADETECT_W {
        ANADETECT_W { w: self }
    }
}
