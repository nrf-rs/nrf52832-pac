#[doc = "Reader of register ISOURCE"]
pub type R = crate::R<u32, super::ISOURCE>;
#[doc = "Writer for register ISOURCE"]
pub type W = crate::W<u32, super::ISOURCE>;
#[doc = "Register ISOURCE `reset()`'s with value 0"]
impl crate::ResetValue for super::ISOURCE {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Comparator hysteresis\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum ISOURCE_A {
    #[doc = "0: Current source disabled"]
    OFF = 0,
    #[doc = "1: Current source enabled (+/- 2.5 uA)"]
    IEN2MA5 = 1,
    #[doc = "2: Current source enabled (+/- 5 uA)"]
    IEN5MA = 2,
    #[doc = "3: Current source enabled (+/- 10 uA)"]
    IEN10MA = 3,
}
impl From<ISOURCE_A> for u8 {
    #[inline(always)]
    fn from(variant: ISOURCE_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `ISOURCE`"]
pub type ISOURCE_R = crate::R<u8, ISOURCE_A>;
impl ISOURCE_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ISOURCE_A {
        match self.bits {
            0 => ISOURCE_A::OFF,
            1 => ISOURCE_A::IEN2MA5,
            2 => ISOURCE_A::IEN5MA,
            3 => ISOURCE_A::IEN10MA,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `OFF`"]
    #[inline(always)]
    pub fn is_off(&self) -> bool {
        *self == ISOURCE_A::OFF
    }
    #[doc = "Checks if the value of the field is `IEN2MA5`"]
    #[inline(always)]
    pub fn is_ien2m_a5(&self) -> bool {
        *self == ISOURCE_A::IEN2MA5
    }
    #[doc = "Checks if the value of the field is `IEN5MA`"]
    #[inline(always)]
    pub fn is_ien5m_a(&self) -> bool {
        *self == ISOURCE_A::IEN5MA
    }
    #[doc = "Checks if the value of the field is `IEN10MA`"]
    #[inline(always)]
    pub fn is_ien10m_a(&self) -> bool {
        *self == ISOURCE_A::IEN10MA
    }
}
#[doc = "Write proxy for field `ISOURCE`"]
pub struct ISOURCE_W<'a> {
    w: &'a mut W,
}
impl<'a> ISOURCE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: ISOURCE_A) -> &'a mut W {
        {
            self.bits(variant.into())
        }
    }
    #[doc = "Current source disabled"]
    #[inline(always)]
    pub fn off(self) -> &'a mut W {
        self.variant(ISOURCE_A::OFF)
    }
    #[doc = "Current source enabled (+/- 2.5 uA)"]
    #[inline(always)]
    pub fn ien2m_a5(self) -> &'a mut W {
        self.variant(ISOURCE_A::IEN2MA5)
    }
    #[doc = "Current source enabled (+/- 5 uA)"]
    #[inline(always)]
    pub fn ien5m_a(self) -> &'a mut W {
        self.variant(ISOURCE_A::IEN5MA)
    }
    #[doc = "Current source enabled (+/- 10 uA)"]
    #[inline(always)]
    pub fn ien10m_a(self) -> &'a mut W {
        self.variant(ISOURCE_A::IEN10MA)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x03) | ((value as u32) & 0x03);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:1 - Comparator hysteresis"]
    #[inline(always)]
    pub fn isource(&self) -> ISOURCE_R {
        ISOURCE_R::new((self.bits & 0x03) as u8)
    }
}
impl W {
    #[doc = "Bits 0:1 - Comparator hysteresis"]
    #[inline(always)]
    pub fn isource(&mut self) -> ISOURCE_W {
        ISOURCE_W { w: self }
    }
}
