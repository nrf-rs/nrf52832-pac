#[doc = "Reader of register TAGHEADER3"]
pub type R = crate::R<u32, super::TAGHEADER3>;
#[doc = "Reader of field `UD12`"]
pub type UD12_R = crate::R<u8, u8>;
#[doc = "Reader of field `UD13`"]
pub type UD13_R = crate::R<u8, u8>;
#[doc = "Reader of field `UD14`"]
pub type UD14_R = crate::R<u8, u8>;
#[doc = "Reader of field `UD15`"]
pub type UD15_R = crate::R<u8, u8>;
impl R {
    #[doc = "Bits 0:7 - Unique identifier byte 12"]
    #[inline(always)]
    pub fn ud12(&self) -> UD12_R {
        UD12_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - Unique identifier byte 13"]
    #[inline(always)]
    pub fn ud13(&self) -> UD13_R {
        UD13_R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:23 - Unique identifier byte 14"]
    #[inline(always)]
    pub fn ud14(&self) -> UD14_R {
        UD14_R::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 24:31 - Unique identifier byte 15"]
    #[inline(always)]
    pub fn ud15(&self) -> UD15_R {
        UD15_R::new(((self.bits >> 24) & 0xff) as u8)
    }
}
