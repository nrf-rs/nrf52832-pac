#[doc = "Reader of register PREFIX0"]
pub type R = crate::R<u32, super::PREFIX0>;
#[doc = "Writer for register PREFIX0"]
pub type W = crate::W<u32, super::PREFIX0>;
#[doc = "Register PREFIX0 `reset()`'s with value 0"]
impl crate::ResetValue for super::PREFIX0 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `AP0`"]
pub type AP0_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `AP0`"]
pub struct AP0_W<'a> {
    w: &'a mut W,
}
impl<'a> AP0_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xff) | ((value as u32) & 0xff);
        self.w
    }
}
#[doc = "Reader of field `AP1`"]
pub type AP1_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `AP1`"]
pub struct AP1_W<'a> {
    w: &'a mut W,
}
impl<'a> AP1_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xff << 8)) | (((value as u32) & 0xff) << 8);
        self.w
    }
}
#[doc = "Reader of field `AP2`"]
pub type AP2_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `AP2`"]
pub struct AP2_W<'a> {
    w: &'a mut W,
}
impl<'a> AP2_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xff << 16)) | (((value as u32) & 0xff) << 16);
        self.w
    }
}
#[doc = "Reader of field `AP3`"]
pub type AP3_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `AP3`"]
pub struct AP3_W<'a> {
    w: &'a mut W,
}
impl<'a> AP3_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xff << 24)) | (((value as u32) & 0xff) << 24);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:7 - Address prefix 0."]
    #[inline(always)]
    pub fn ap0(&self) -> AP0_R {
        AP0_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - Address prefix 1."]
    #[inline(always)]
    pub fn ap1(&self) -> AP1_R {
        AP1_R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:23 - Address prefix 2."]
    #[inline(always)]
    pub fn ap2(&self) -> AP2_R {
        AP2_R::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 24:31 - Address prefix 3."]
    #[inline(always)]
    pub fn ap3(&self) -> AP3_R {
        AP3_R::new(((self.bits >> 24) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - Address prefix 0."]
    #[inline(always)]
    pub fn ap0(&mut self) -> AP0_W {
        AP0_W { w: self }
    }
    #[doc = "Bits 8:15 - Address prefix 1."]
    #[inline(always)]
    pub fn ap1(&mut self) -> AP1_W {
        AP1_W { w: self }
    }
    #[doc = "Bits 16:23 - Address prefix 2."]
    #[inline(always)]
    pub fn ap2(&mut self) -> AP2_W {
        AP2_W { w: self }
    }
    #[doc = "Bits 24:31 - Address prefix 3."]
    #[inline(always)]
    pub fn ap3(&mut self) -> AP3_W {
        AP3_W { w: self }
    }
}
