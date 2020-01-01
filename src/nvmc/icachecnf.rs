#[doc = "Reader of register ICACHECNF"]
pub type R = crate::R<u32, super::ICACHECNF>;
#[doc = "Writer for register ICACHECNF"]
pub type W = crate::W<u32, super::ICACHECNF>;
#[doc = "Register ICACHECNF `reset()`'s with value 0"]
impl crate::ResetValue for super::ICACHECNF {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Cache enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CACHEEN_A {
    #[doc = "0: Disable cache. Invalidates all cache entries."]
    DISABLED = 0,
    #[doc = "1: Enable cache"]
    ENABLED = 1,
}
impl From<CACHEEN_A> for bool {
    #[inline(always)]
    fn from(variant: CACHEEN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `CACHEEN`"]
pub type CACHEEN_R = crate::R<bool, CACHEEN_A>;
impl CACHEEN_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CACHEEN_A {
        match self.bits {
            false => CACHEEN_A::DISABLED,
            true => CACHEEN_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == CACHEEN_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == CACHEEN_A::ENABLED
    }
}
#[doc = "Write proxy for field `CACHEEN`"]
pub struct CACHEEN_W<'a> {
    w: &'a mut W,
}
impl<'a> CACHEEN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CACHEEN_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Disable cache. Invalidates all cache entries."]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(CACHEEN_A::DISABLED)
    }
    #[doc = "Enable cache"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(CACHEEN_A::ENABLED)
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
#[doc = "Cache profiling enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CACHEPROFEN_A {
    #[doc = "0: Disable cache profiling"]
    DISABLED = 0,
    #[doc = "1: Enable cache profiling"]
    ENABLED = 1,
}
impl From<CACHEPROFEN_A> for bool {
    #[inline(always)]
    fn from(variant: CACHEPROFEN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `CACHEPROFEN`"]
pub type CACHEPROFEN_R = crate::R<bool, CACHEPROFEN_A>;
impl CACHEPROFEN_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CACHEPROFEN_A {
        match self.bits {
            false => CACHEPROFEN_A::DISABLED,
            true => CACHEPROFEN_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == CACHEPROFEN_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == CACHEPROFEN_A::ENABLED
    }
}
#[doc = "Write proxy for field `CACHEPROFEN`"]
pub struct CACHEPROFEN_W<'a> {
    w: &'a mut W,
}
impl<'a> CACHEPROFEN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CACHEPROFEN_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Disable cache profiling"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(CACHEPROFEN_A::DISABLED)
    }
    #[doc = "Enable cache profiling"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(CACHEPROFEN_A::ENABLED)
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
        self.w.bits = (self.w.bits & !(0x01 << 8)) | (((value as u32) & 0x01) << 8);
        self.w
    }
}
impl R {
    #[doc = "Bit 0 - Cache enable"]
    #[inline(always)]
    pub fn cacheen(&self) -> CACHEEN_R {
        CACHEEN_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 8 - Cache profiling enable"]
    #[inline(always)]
    pub fn cacheprofen(&self) -> CACHEPROFEN_R {
        CACHEPROFEN_R::new(((self.bits >> 8) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Cache enable"]
    #[inline(always)]
    pub fn cacheen(&mut self) -> CACHEEN_W {
        CACHEEN_W { w: self }
    }
    #[doc = "Bit 8 - Cache profiling enable"]
    #[inline(always)]
    pub fn cacheprofen(&mut self) -> CACHEPROFEN_W {
        CACHEPROFEN_W { w: self }
    }
}
