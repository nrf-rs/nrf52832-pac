#[doc = "Reader of register PART"]
pub type R = crate::R<u32, super::PART>;
#[doc = "Part code\n\nValue on reset: 337970"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u32)]
pub enum PART_A {
    #[doc = "337970: nRF52832"]
    N52832 = 337970,
    #[doc = "4294967295: Unspecified"]
    UNSPECIFIED = 4294967295,
}
impl From<PART_A> for u32 {
    #[inline(always)]
    fn from(variant: PART_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `PART`"]
pub type PART_R = crate::R<u32, PART_A>;
impl PART_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u32, PART_A> {
        use crate::Variant::*;
        match self.bits {
            337970 => Val(PART_A::N52832),
            4294967295 => Val(PART_A::UNSPECIFIED),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `N52832`"]
    #[inline(always)]
    pub fn is_n52832(&self) -> bool {
        *self == PART_A::N52832
    }
    #[doc = "Checks if the value of the field is `UNSPECIFIED`"]
    #[inline(always)]
    pub fn is_unspecified(&self) -> bool {
        *self == PART_A::UNSPECIFIED
    }
}
impl R {
    #[doc = "Bits 0:31 - Part code"]
    #[inline(always)]
    pub fn part(&self) -> PART_R {
        PART_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
