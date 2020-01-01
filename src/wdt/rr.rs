#[doc = "Writer for register RR[%s]"]
pub type W = crate::W<u32, super::RR>;
#[doc = "Register RR[%s]
`reset()`'s with value 0"]
impl crate::ResetValue for super::RR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reload request register\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u32)]
pub enum RR_AW {
    #[doc = "1850885685: Value to request a reload of the watchdog timer"]
    RELOAD = 1850885685,
}
impl From<RR_AW> for u32 {
    #[inline(always)]
    fn from(variant: RR_AW) -> Self {
        variant as _
    }
}
#[doc = "Write proxy for field `RR`"]
pub struct RR_W<'a> {
    w: &'a mut W,
}
impl<'a> RR_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: RR_AW) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "Value to request a reload of the watchdog timer"]
    #[inline(always)]
    pub fn reload(self) -> &'a mut W {
        self.variant(RR_AW::RELOAD)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff_ffff) | ((value as u32) & 0xffff_ffff);
        self.w
    }
}
impl W {
    #[doc = "Bits 0:31 - Reload request register"]
    #[inline(always)]
    pub fn rr(&mut self) -> RR_W {
        RR_W { w: self }
    }
}
