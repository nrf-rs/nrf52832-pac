#[doc = "Reader of register LIST"]
pub type R = crate::R<u32, super::LIST>;
#[doc = "Writer for register LIST"]
pub type W = crate::W<u32, super::LIST>;
#[doc = "Register LIST `reset()`'s with value 0"]
impl crate::ResetValue for super::LIST {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "List type\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum LIST_A {
    #[doc = "0: Disable EasyDMA list"]
    DISABLED = 0,
    #[doc = "1: Use array list"]
    ARRAYLIST = 1,
}
impl From<LIST_A> for u8 {
    #[inline(always)]
    fn from(variant: LIST_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `LIST`"]
pub type LIST_R = crate::R<u8, LIST_A>;
impl LIST_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, LIST_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(LIST_A::DISABLED),
            1 => Val(LIST_A::ARRAYLIST),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == LIST_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ARRAYLIST`"]
    #[inline(always)]
    pub fn is_array_list(&self) -> bool {
        *self == LIST_A::ARRAYLIST
    }
}
#[doc = "Write proxy for field `LIST`"]
pub struct LIST_W<'a> {
    w: &'a mut W,
}
impl<'a> LIST_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: LIST_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "Disable EasyDMA list"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(LIST_A::DISABLED)
    }
    #[doc = "Use array list"]
    #[inline(always)]
    pub fn array_list(self) -> &'a mut W {
        self.variant(LIST_A::ARRAYLIST)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x07) | ((value as u32) & 0x07);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:2 - List type"]
    #[inline(always)]
    pub fn list(&self) -> LIST_R {
        LIST_R::new((self.bits & 0x07) as u8)
    }
}
impl W {
    #[doc = "Bits 0:2 - List type"]
    #[inline(always)]
    pub fn list(&mut self) -> LIST_W {
        LIST_W { w: self }
    }
}
