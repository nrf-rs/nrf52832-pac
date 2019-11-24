#[doc = "Reader of register TAGHEADER1"]
pub type R = crate::R<u32, super::TAGHEADER1>;
#[doc = "Reader of field `UD4`"]
pub type UD4_R = crate::R<u8, u8>;
#[doc = "Reader of field `UD5`"]
pub type UD5_R = crate::R<u8, u8>;
#[doc = "Reader of field `UD6`"]
pub type UD6_R = crate::R<u8, u8>;
#[doc = "Reader of field `UD7`"]
pub type UD7_R = crate::R<u8, u8>;
impl R {
    #[doc = "Bits 0:7 - Unique identifier byte 4"]
    #[inline(always)]
    pub fn ud4(&self) -> UD4_R {
        UD4_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - Unique identifier byte 5"]
    #[inline(always)]
    pub fn ud5(&self) -> UD5_R {
        UD5_R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:23 - Unique identifier byte 6"]
    #[inline(always)]
    pub fn ud6(&self) -> UD6_R {
        UD6_R::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 24:31 - Unique identifier byte 7"]
    #[inline(always)]
    pub fn ud7(&self) -> UD7_R {
        UD7_R::new(((self.bits >> 24) & 0xff) as u8)
    }
}
