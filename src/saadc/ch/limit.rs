#[doc = "Reader of register LIMIT"]
pub type R = crate::R<u32, super::LIMIT>;
#[doc = "Writer for register LIMIT"]
pub type W = crate::W<u32, super::LIMIT>;
#[doc = "Register LIMIT `reset()`'s with value 0x7fff_8000"]
impl crate::ResetValue for super::LIMIT {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0x7fff_8000
    }
}
#[doc = "Reader of field `LOW`"]
pub type LOW_R = crate::R<u16, u16>;
#[doc = "Write proxy for field `LOW`"]
pub struct LOW_W<'a> {
    w: &'a mut W,
}
impl<'a> LOW_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff) | ((value as u32) & 0xffff);
        self.w
    }
}
#[doc = "Reader of field `HIGH`"]
pub type HIGH_R = crate::R<u16, u16>;
#[doc = "Write proxy for field `HIGH`"]
pub struct HIGH_W<'a> {
    w: &'a mut W,
}
impl<'a> HIGH_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xffff << 16)) | (((value as u32) & 0xffff) << 16);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:15 - Low level limit"]
    #[inline(always)]
    pub fn low(&self) -> LOW_R {
        LOW_R::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 16:31 - High level limit"]
    #[inline(always)]
    pub fn high(&self) -> HIGH_R {
        HIGH_R::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - Low level limit"]
    #[inline(always)]
    pub fn low(&mut self) -> LOW_W {
        LOW_W { w: self }
    }
    #[doc = "Bits 16:31 - High level limit"]
    #[inline(always)]
    pub fn high(&mut self) -> HIGH_W {
        HIGH_W { w: self }
    }
}
