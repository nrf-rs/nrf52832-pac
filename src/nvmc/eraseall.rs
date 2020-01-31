#[doc = "Reader of register ERASEALL"]
pub type R = crate::R<u32, super::ERASEALL>;
#[doc = "Writer for register ERASEALL"]
pub type W = crate::W<u32, super::ERASEALL>;
#[doc = "Register ERASEALL `reset()`'s with value 0"]
impl crate::ResetValue for super::ERASEALL {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Erase all non-volatile memory including UICR registers. Note that code erase has to be enabled by CONFIG.EEN before the UICR can be erased.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ERASEALL_A {
    #[doc = "0: No operation"]
    NOOPERATION = 0,
    #[doc = "1: Start chip erase"]
    ERASE = 1,
}
impl From<ERASEALL_A> for bool {
    #[inline(always)]
    fn from(variant: ERASEALL_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `ERASEALL`"]
pub type ERASEALL_R = crate::R<bool, ERASEALL_A>;
impl ERASEALL_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ERASEALL_A {
        match self.bits {
            false => ERASEALL_A::NOOPERATION,
            true => ERASEALL_A::ERASE,
        }
    }
    #[doc = "Checks if the value of the field is `NOOPERATION`"]
    #[inline(always)]
    pub fn is_no_operation(&self) -> bool {
        *self == ERASEALL_A::NOOPERATION
    }
    #[doc = "Checks if the value of the field is `ERASE`"]
    #[inline(always)]
    pub fn is_erase(&self) -> bool {
        *self == ERASEALL_A::ERASE
    }
}
#[doc = "Write proxy for field `ERASEALL`"]
pub struct ERASEALL_W<'a> {
    w: &'a mut W,
}
impl<'a> ERASEALL_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: ERASEALL_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "No operation"]
    #[inline(always)]
    pub fn no_operation(self) -> &'a mut W {
        self.variant(ERASEALL_A::NOOPERATION)
    }
    #[doc = "Start chip erase"]
    #[inline(always)]
    pub fn erase(self) -> &'a mut W {
        self.variant(ERASEALL_A::ERASE)
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
    #[doc = "Bit 0 - Erase all non-volatile memory including UICR registers. Note that code erase has to be enabled by CONFIG.EEN before the UICR can be erased."]
    #[inline(always)]
    pub fn eraseall(&self) -> ERASEALL_R {
        ERASEALL_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Erase all non-volatile memory including UICR registers. Note that code erase has to be enabled by CONFIG.EEN before the UICR can be erased."]
    #[inline(always)]
    pub fn eraseall(&mut self) -> ERASEALL_W {
        ERASEALL_W { w: self }
    }
}
