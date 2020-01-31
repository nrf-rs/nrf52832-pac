#[doc = "Reader of register ERASEUICR"]
pub type R = crate::R<u32, super::ERASEUICR>;
#[doc = "Writer for register ERASEUICR"]
pub type W = crate::W<u32, super::ERASEUICR>;
#[doc = "Register ERASEUICR `reset()`'s with value 0"]
impl crate::ResetValue for super::ERASEUICR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Register starting erase of all User Information Configuration Registers. Note that code erase has to be enabled by CONFIG.EEN before the UICR can be erased.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ERASEUICR_A {
    #[doc = "0: No operation"]
    NOOPERATION = 0,
    #[doc = "1: Start erase of UICR"]
    ERASE = 1,
}
impl From<ERASEUICR_A> for bool {
    #[inline(always)]
    fn from(variant: ERASEUICR_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `ERASEUICR`"]
pub type ERASEUICR_R = crate::R<bool, ERASEUICR_A>;
impl ERASEUICR_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ERASEUICR_A {
        match self.bits {
            false => ERASEUICR_A::NOOPERATION,
            true => ERASEUICR_A::ERASE,
        }
    }
    #[doc = "Checks if the value of the field is `NOOPERATION`"]
    #[inline(always)]
    pub fn is_no_operation(&self) -> bool {
        *self == ERASEUICR_A::NOOPERATION
    }
    #[doc = "Checks if the value of the field is `ERASE`"]
    #[inline(always)]
    pub fn is_erase(&self) -> bool {
        *self == ERASEUICR_A::ERASE
    }
}
#[doc = "Write proxy for field `ERASEUICR`"]
pub struct ERASEUICR_W<'a> {
    w: &'a mut W,
}
impl<'a> ERASEUICR_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: ERASEUICR_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "No operation"]
    #[inline(always)]
    pub fn no_operation(self) -> &'a mut W {
        self.variant(ERASEUICR_A::NOOPERATION)
    }
    #[doc = "Start erase of UICR"]
    #[inline(always)]
    pub fn erase(self) -> &'a mut W {
        self.variant(ERASEUICR_A::ERASE)
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
    #[doc = "Bit 0 - Register starting erase of all User Information Configuration Registers. Note that code erase has to be enabled by CONFIG.EEN before the UICR can be erased."]
    #[inline(always)]
    pub fn eraseuicr(&self) -> ERASEUICR_R {
        ERASEUICR_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Register starting erase of all User Information Configuration Registers. Note that code erase has to be enabled by CONFIG.EEN before the UICR can be erased."]
    #[inline(always)]
    pub fn eraseuicr(&mut self) -> ERASEUICR_W {
        ERASEUICR_W { w: self }
    }
}
