#[doc = "Reader of register TAGHEADER0"]
pub type R = crate::R<u32, super::TAGHEADER0>;
#[doc = "Reader of field `MFGID`"]
pub type MFGID_R = crate::R<u8, u8>;
#[doc = "Reader of field `UD1`"]
pub type UD1_R = crate::R<u8, u8>;
#[doc = "Reader of field `UD2`"]
pub type UD2_R = crate::R<u8, u8>;
#[doc = "Reader of field `UD3`"]
pub type UD3_R = crate::R<u8, u8>;
impl R {
    #[doc = "Bits 0:7 - Default Manufacturer ID: Nordic Semiconductor ASA has ICM 0x5F"]
    #[inline(always)]
    pub fn mfgid(&self) -> MFGID_R {
        MFGID_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - Unique identifier byte 1"]
    #[inline(always)]
    pub fn ud1(&self) -> UD1_R {
        UD1_R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:23 - Unique identifier byte 2"]
    #[inline(always)]
    pub fn ud2(&self) -> UD2_R {
        UD2_R::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 24:31 - Unique identifier byte 3"]
    #[inline(always)]
    pub fn ud3(&self) -> UD3_R {
        UD3_R::new(((self.bits >> 24) & 0xff) as u8)
    }
}
