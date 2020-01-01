#[doc = "Reader of register REPORTPER"]
pub type R = crate::R<u32, super::REPORTPER>;
#[doc = "Writer for register REPORTPER"]
pub type W = crate::W<u32, super::REPORTPER>;
#[doc = "Register REPORTPER `reset()`'s with value 0"]
impl crate::ResetValue for super::REPORTPER {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Specifies the number of samples to be accumulated in the ACC register before the REPORTRDY and DBLRDY events can be generated\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum REPORTPER_A {
    #[doc = "0: 10 samples / report"]
    _10SMPL = 0,
    #[doc = "1: 40 samples / report"]
    _40SMPL = 1,
    #[doc = "2: 80 samples / report"]
    _80SMPL = 2,
    #[doc = "3: 120 samples / report"]
    _120SMPL = 3,
    #[doc = "4: 160 samples / report"]
    _160SMPL = 4,
    #[doc = "5: 200 samples / report"]
    _200SMPL = 5,
    #[doc = "6: 240 samples / report"]
    _240SMPL = 6,
    #[doc = "7: 280 samples / report"]
    _280SMPL = 7,
    #[doc = "8: 1 sample / report"]
    _1SMPL = 8,
}
impl From<REPORTPER_A> for u8 {
    #[inline(always)]
    fn from(variant: REPORTPER_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `REPORTPER`"]
pub type REPORTPER_R = crate::R<u8, REPORTPER_A>;
impl REPORTPER_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, REPORTPER_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(REPORTPER_A::_10SMPL),
            1 => Val(REPORTPER_A::_40SMPL),
            2 => Val(REPORTPER_A::_80SMPL),
            3 => Val(REPORTPER_A::_120SMPL),
            4 => Val(REPORTPER_A::_160SMPL),
            5 => Val(REPORTPER_A::_200SMPL),
            6 => Val(REPORTPER_A::_240SMPL),
            7 => Val(REPORTPER_A::_280SMPL),
            8 => Val(REPORTPER_A::_1SMPL),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `_10SMPL`"]
    #[inline(always)]
    pub fn is_10smpl(&self) -> bool {
        *self == REPORTPER_A::_10SMPL
    }
    #[doc = "Checks if the value of the field is `_40SMPL`"]
    #[inline(always)]
    pub fn is_40smpl(&self) -> bool {
        *self == REPORTPER_A::_40SMPL
    }
    #[doc = "Checks if the value of the field is `_80SMPL`"]
    #[inline(always)]
    pub fn is_80smpl(&self) -> bool {
        *self == REPORTPER_A::_80SMPL
    }
    #[doc = "Checks if the value of the field is `_120SMPL`"]
    #[inline(always)]
    pub fn is_120smpl(&self) -> bool {
        *self == REPORTPER_A::_120SMPL
    }
    #[doc = "Checks if the value of the field is `_160SMPL`"]
    #[inline(always)]
    pub fn is_160smpl(&self) -> bool {
        *self == REPORTPER_A::_160SMPL
    }
    #[doc = "Checks if the value of the field is `_200SMPL`"]
    #[inline(always)]
    pub fn is_200smpl(&self) -> bool {
        *self == REPORTPER_A::_200SMPL
    }
    #[doc = "Checks if the value of the field is `_240SMPL`"]
    #[inline(always)]
    pub fn is_240smpl(&self) -> bool {
        *self == REPORTPER_A::_240SMPL
    }
    #[doc = "Checks if the value of the field is `_280SMPL`"]
    #[inline(always)]
    pub fn is_280smpl(&self) -> bool {
        *self == REPORTPER_A::_280SMPL
    }
    #[doc = "Checks if the value of the field is `_1SMPL`"]
    #[inline(always)]
    pub fn is_1smpl(&self) -> bool {
        *self == REPORTPER_A::_1SMPL
    }
}
#[doc = "Write proxy for field `REPORTPER`"]
pub struct REPORTPER_W<'a> {
    w: &'a mut W,
}
impl<'a> REPORTPER_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: REPORTPER_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "10 samples / report"]
    #[inline(always)]
    pub fn _10smpl(self) -> &'a mut W {
        self.variant(REPORTPER_A::_10SMPL)
    }
    #[doc = "40 samples / report"]
    #[inline(always)]
    pub fn _40smpl(self) -> &'a mut W {
        self.variant(REPORTPER_A::_40SMPL)
    }
    #[doc = "80 samples / report"]
    #[inline(always)]
    pub fn _80smpl(self) -> &'a mut W {
        self.variant(REPORTPER_A::_80SMPL)
    }
    #[doc = "120 samples / report"]
    #[inline(always)]
    pub fn _120smpl(self) -> &'a mut W {
        self.variant(REPORTPER_A::_120SMPL)
    }
    #[doc = "160 samples / report"]
    #[inline(always)]
    pub fn _160smpl(self) -> &'a mut W {
        self.variant(REPORTPER_A::_160SMPL)
    }
    #[doc = "200 samples / report"]
    #[inline(always)]
    pub fn _200smpl(self) -> &'a mut W {
        self.variant(REPORTPER_A::_200SMPL)
    }
    #[doc = "240 samples / report"]
    #[inline(always)]
    pub fn _240smpl(self) -> &'a mut W {
        self.variant(REPORTPER_A::_240SMPL)
    }
    #[doc = "280 samples / report"]
    #[inline(always)]
    pub fn _280smpl(self) -> &'a mut W {
        self.variant(REPORTPER_A::_280SMPL)
    }
    #[doc = "1 sample / report"]
    #[inline(always)]
    pub fn _1smpl(self) -> &'a mut W {
        self.variant(REPORTPER_A::_1SMPL)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x0f) | ((value as u32) & 0x0f);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:3 - Specifies the number of samples to be accumulated in the ACC register before the REPORTRDY and DBLRDY events can be generated"]
    #[inline(always)]
    pub fn reportper(&self) -> REPORTPER_R {
        REPORTPER_R::new((self.bits & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:3 - Specifies the number of samples to be accumulated in the ACC register before the REPORTRDY and DBLRDY events can be generated"]
    #[inline(always)]
    pub fn reportper(&mut self) -> REPORTPER_W {
        REPORTPER_W { w: self }
    }
}
