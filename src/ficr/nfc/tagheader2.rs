#[doc = "Reader of register TAGHEADER2"]
pub type R = crate::R<u32, super::TAGHEADER2>;
#[doc = "Reader of field `UD8`"]
pub type UD8_R = crate::R<u8, u8>;
#[doc = "Reader of field `UD9`"]
pub type UD9_R = crate::R<u8, u8>;
#[doc = "Reader of field `UD10`"]
pub type UD10_R = crate::R<u8, u8>;
#[doc = "Reader of field `UD11`"]
pub type UD11_R = crate::R<u8, u8>;
impl R {
    #[doc = "Bits 0:7 - Unique identifier byte 8"]
    #[inline(always)]
    pub fn ud8(&self) -> UD8_R {
        UD8_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - Unique identifier byte 9"]
    #[inline(always)]
    pub fn ud9(&self) -> UD9_R {
        UD9_R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:23 - Unique identifier byte 10"]
    #[inline(always)]
    pub fn ud10(&self) -> UD10_R {
        UD10_R::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 24:31 - Unique identifier byte 11"]
    #[inline(always)]
    pub fn ud11(&self) -> UD11_R {
        UD11_R::new(((self.bits >> 24) & 0xff) as u8)
    }
}
