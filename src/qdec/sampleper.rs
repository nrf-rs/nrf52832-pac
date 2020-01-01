#[doc = "Reader of register SAMPLEPER"]
pub type R = crate::R<u32, super::SAMPLEPER>;
#[doc = "Writer for register SAMPLEPER"]
pub type W = crate::W<u32, super::SAMPLEPER>;
#[doc = "Register SAMPLEPER `reset()`'s with value 0"]
impl crate::ResetValue for super::SAMPLEPER {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Sample period. The SAMPLE register will be updated for every new sample\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum SAMPLEPER_A {
    #[doc = "0: 128 us"]
    _128US = 0,
    #[doc = "1: 256 us"]
    _256US = 1,
    #[doc = "2: 512 us"]
    _512US = 2,
    #[doc = "3: 1024 us"]
    _1024US = 3,
    #[doc = "4: 2048 us"]
    _2048US = 4,
    #[doc = "5: 4096 us"]
    _4096US = 5,
    #[doc = "6: 8192 us"]
    _8192US = 6,
    #[doc = "7: 16384 us"]
    _16384US = 7,
    #[doc = "8: 32768 us"]
    _32MS = 8,
    #[doc = "9: 65536 us"]
    _65MS = 9,
    #[doc = "10: 131072 us"]
    _131MS = 10,
}
impl From<SAMPLEPER_A> for u8 {
    #[inline(always)]
    fn from(variant: SAMPLEPER_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `SAMPLEPER`"]
pub type SAMPLEPER_R = crate::R<u8, SAMPLEPER_A>;
impl SAMPLEPER_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, SAMPLEPER_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(SAMPLEPER_A::_128US),
            1 => Val(SAMPLEPER_A::_256US),
            2 => Val(SAMPLEPER_A::_512US),
            3 => Val(SAMPLEPER_A::_1024US),
            4 => Val(SAMPLEPER_A::_2048US),
            5 => Val(SAMPLEPER_A::_4096US),
            6 => Val(SAMPLEPER_A::_8192US),
            7 => Val(SAMPLEPER_A::_16384US),
            8 => Val(SAMPLEPER_A::_32MS),
            9 => Val(SAMPLEPER_A::_65MS),
            10 => Val(SAMPLEPER_A::_131MS),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `_128US`"]
    #[inline(always)]
    pub fn is_128us(&self) -> bool {
        *self == SAMPLEPER_A::_128US
    }
    #[doc = "Checks if the value of the field is `_256US`"]
    #[inline(always)]
    pub fn is_256us(&self) -> bool {
        *self == SAMPLEPER_A::_256US
    }
    #[doc = "Checks if the value of the field is `_512US`"]
    #[inline(always)]
    pub fn is_512us(&self) -> bool {
        *self == SAMPLEPER_A::_512US
    }
    #[doc = "Checks if the value of the field is `_1024US`"]
    #[inline(always)]
    pub fn is_1024us(&self) -> bool {
        *self == SAMPLEPER_A::_1024US
    }
    #[doc = "Checks if the value of the field is `_2048US`"]
    #[inline(always)]
    pub fn is_2048us(&self) -> bool {
        *self == SAMPLEPER_A::_2048US
    }
    #[doc = "Checks if the value of the field is `_4096US`"]
    #[inline(always)]
    pub fn is_4096us(&self) -> bool {
        *self == SAMPLEPER_A::_4096US
    }
    #[doc = "Checks if the value of the field is `_8192US`"]
    #[inline(always)]
    pub fn is_8192us(&self) -> bool {
        *self == SAMPLEPER_A::_8192US
    }
    #[doc = "Checks if the value of the field is `_16384US`"]
    #[inline(always)]
    pub fn is_16384us(&self) -> bool {
        *self == SAMPLEPER_A::_16384US
    }
    #[doc = "Checks if the value of the field is `_32MS`"]
    #[inline(always)]
    pub fn is_32ms(&self) -> bool {
        *self == SAMPLEPER_A::_32MS
    }
    #[doc = "Checks if the value of the field is `_65MS`"]
    #[inline(always)]
    pub fn is_65ms(&self) -> bool {
        *self == SAMPLEPER_A::_65MS
    }
    #[doc = "Checks if the value of the field is `_131MS`"]
    #[inline(always)]
    pub fn is_131ms(&self) -> bool {
        *self == SAMPLEPER_A::_131MS
    }
}
#[doc = "Write proxy for field `SAMPLEPER`"]
pub struct SAMPLEPER_W<'a> {
    w: &'a mut W,
}
impl<'a> SAMPLEPER_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SAMPLEPER_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "128 us"]
    #[inline(always)]
    pub fn _128us(self) -> &'a mut W {
        self.variant(SAMPLEPER_A::_128US)
    }
    #[doc = "256 us"]
    #[inline(always)]
    pub fn _256us(self) -> &'a mut W {
        self.variant(SAMPLEPER_A::_256US)
    }
    #[doc = "512 us"]
    #[inline(always)]
    pub fn _512us(self) -> &'a mut W {
        self.variant(SAMPLEPER_A::_512US)
    }
    #[doc = "1024 us"]
    #[inline(always)]
    pub fn _1024us(self) -> &'a mut W {
        self.variant(SAMPLEPER_A::_1024US)
    }
    #[doc = "2048 us"]
    #[inline(always)]
    pub fn _2048us(self) -> &'a mut W {
        self.variant(SAMPLEPER_A::_2048US)
    }
    #[doc = "4096 us"]
    #[inline(always)]
    pub fn _4096us(self) -> &'a mut W {
        self.variant(SAMPLEPER_A::_4096US)
    }
    #[doc = "8192 us"]
    #[inline(always)]
    pub fn _8192us(self) -> &'a mut W {
        self.variant(SAMPLEPER_A::_8192US)
    }
    #[doc = "16384 us"]
    #[inline(always)]
    pub fn _16384us(self) -> &'a mut W {
        self.variant(SAMPLEPER_A::_16384US)
    }
    #[doc = "32768 us"]
    #[inline(always)]
    pub fn _32ms(self) -> &'a mut W {
        self.variant(SAMPLEPER_A::_32MS)
    }
    #[doc = "65536 us"]
    #[inline(always)]
    pub fn _65ms(self) -> &'a mut W {
        self.variant(SAMPLEPER_A::_65MS)
    }
    #[doc = "131072 us"]
    #[inline(always)]
    pub fn _131ms(self) -> &'a mut W {
        self.variant(SAMPLEPER_A::_131MS)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x0f) | ((value as u32) & 0x0f);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:3 - Sample period. The SAMPLE register will be updated for every new sample"]
    #[inline(always)]
    pub fn sampleper(&self) -> SAMPLEPER_R {
        SAMPLEPER_R::new((self.bits & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:3 - Sample period. The SAMPLE register will be updated for every new sample"]
    #[inline(always)]
    pub fn sampleper(&mut self) -> SAMPLEPER_W {
        SAMPLEPER_W { w: self }
    }
}
