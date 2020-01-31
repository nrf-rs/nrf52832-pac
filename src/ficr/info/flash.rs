#[doc = "Reader of register FLASH"]
pub type R = crate::R<u32, super::FLASH>;
#[doc = "Flash variant\n\nValue on reset: 512"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u32)]
pub enum FLASH_A {
    #[doc = "128: 128 kByte FLASH"]
    K128 = 128,
    #[doc = "256: 256 kByte FLASH"]
    K256 = 256,
    #[doc = "512: 512 kByte FLASH"]
    K512 = 512,
    #[doc = "4294967295: Unspecified"]
    UNSPECIFIED = 4294967295,
}
impl From<FLASH_A> for u32 {
    #[inline(always)]
    fn from(variant: FLASH_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `FLASH`"]
pub type FLASH_R = crate::R<u32, FLASH_A>;
impl FLASH_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u32, FLASH_A> {
        use crate::Variant::*;
        match self.bits {
            128 => Val(FLASH_A::K128),
            256 => Val(FLASH_A::K256),
            512 => Val(FLASH_A::K512),
            4294967295 => Val(FLASH_A::UNSPECIFIED),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `K128`"]
    #[inline(always)]
    pub fn is_k128(&self) -> bool {
        *self == FLASH_A::K128
    }
    #[doc = "Checks if the value of the field is `K256`"]
    #[inline(always)]
    pub fn is_k256(&self) -> bool {
        *self == FLASH_A::K256
    }
    #[doc = "Checks if the value of the field is `K512`"]
    #[inline(always)]
    pub fn is_k512(&self) -> bool {
        *self == FLASH_A::K512
    }
    #[doc = "Checks if the value of the field is `UNSPECIFIED`"]
    #[inline(always)]
    pub fn is_unspecified(&self) -> bool {
        *self == FLASH_A::UNSPECIFIED
    }
}
impl R {
    #[doc = "Bits 0:31 - Flash variant"]
    #[inline(always)]
    pub fn flash(&self) -> FLASH_R {
        FLASH_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
