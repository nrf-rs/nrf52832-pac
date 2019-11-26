#[doc = "Reader of register PREFIX1"]
pub type R = crate::R<u32, super::PREFIX1>;
#[doc = "Writer for register PREFIX1"]
pub type W = crate::W<u32, super::PREFIX1>;
#[doc = "Register PREFIX1 `reset()`'s with value 0"]
impl crate::ResetValue for super::PREFIX1 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `AP4`"]
pub type AP4_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `AP4`"]
pub struct AP4_W<'a> {
    w: &'a mut W,
}
impl<'a> AP4_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xff) | ((value as u32) & 0xff);
        self.w
    }
}
#[doc = "Reader of field `AP5`"]
pub type AP5_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `AP5`"]
pub struct AP5_W<'a> {
    w: &'a mut W,
}
impl<'a> AP5_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xff << 8)) | (((value as u32) & 0xff) << 8);
        self.w
    }
}
#[doc = "Reader of field `AP6`"]
pub type AP6_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `AP6`"]
pub struct AP6_W<'a> {
    w: &'a mut W,
}
impl<'a> AP6_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xff << 16)) | (((value as u32) & 0xff) << 16);
        self.w
    }
}
#[doc = "Reader of field `AP7`"]
pub type AP7_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `AP7`"]
pub struct AP7_W<'a> {
    w: &'a mut W,
}
impl<'a> AP7_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xff << 24)) | (((value as u32) & 0xff) << 24);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:7 - Address prefix 4."]
    #[inline(always)]
    pub fn ap4(&self) -> AP4_R {
        AP4_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - Address prefix 5."]
    #[inline(always)]
    pub fn ap5(&self) -> AP5_R {
        AP5_R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:23 - Address prefix 6."]
    #[inline(always)]
    pub fn ap6(&self) -> AP6_R {
        AP6_R::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 24:31 - Address prefix 7."]
    #[inline(always)]
    pub fn ap7(&self) -> AP7_R {
        AP7_R::new(((self.bits >> 24) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - Address prefix 4."]
    #[inline(always)]
    pub fn ap4(&mut self) -> AP4_W {
        AP4_W { w: self }
    }
    #[doc = "Bits 8:15 - Address prefix 5."]
    #[inline(always)]
    pub fn ap5(&mut self) -> AP5_W {
        AP5_W { w: self }
    }
    #[doc = "Bits 16:23 - Address prefix 6."]
    #[inline(always)]
    pub fn ap6(&mut self) -> AP6_W {
        AP6_W { w: self }
    }
    #[doc = "Bits 24:31 - Address prefix 7."]
    #[inline(always)]
    pub fn ap7(&mut self) -> AP7_W {
        AP7_W { w: self }
    }
}
