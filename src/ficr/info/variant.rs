#[doc = "Reader of register VARIANT"]
pub type R = crate::R<u32, super::VARIANT>;
#[doc = "Part Variant, Hardware version and Production configuration, encoded as ASCII\n\nValue on reset: 1094795586"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u32)]
pub enum VARIANT_A {
    #[doc = "1094795585: AAAA"]
    AAAA = 1094795585,
    #[doc = "1094795586: AAAB"]
    AAAB = 1094795586,
    #[doc = "1094795841: AABA"]
    AABA = 1094795841,
    #[doc = "1094795842: AABB"]
    AABB = 1094795842,
    #[doc = "1094795824: AAB0"]
    AAB0 = 1094795824,
    #[doc = "1094796592: AAE0"]
    AAE0 = 1094796592,
    #[doc = "4294967295: Unspecified"]
    UNSPECIFIED = 4294967295,
}
impl From<VARIANT_A> for u32 {
    #[inline(always)]
    fn from(variant: VARIANT_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `VARIANT`"]
pub type VARIANT_R = crate::R<u32, VARIANT_A>;
impl VARIANT_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u32, VARIANT_A> {
        use crate::Variant::*;
        match self.bits {
            1094795585 => Val(VARIANT_A::AAAA),
            1094795586 => Val(VARIANT_A::AAAB),
            1094795841 => Val(VARIANT_A::AABA),
            1094795842 => Val(VARIANT_A::AABB),
            1094795824 => Val(VARIANT_A::AAB0),
            1094796592 => Val(VARIANT_A::AAE0),
            4294967295 => Val(VARIANT_A::UNSPECIFIED),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `AAAA`"]
    #[inline(always)]
    pub fn is_aaaa(&self) -> bool {
        *self == VARIANT_A::AAAA
    }
    #[doc = "Checks if the value of the field is `AAAB`"]
    #[inline(always)]
    pub fn is_aaab(&self) -> bool {
        *self == VARIANT_A::AAAB
    }
    #[doc = "Checks if the value of the field is `AABA`"]
    #[inline(always)]
    pub fn is_aaba(&self) -> bool {
        *self == VARIANT_A::AABA
    }
    #[doc = "Checks if the value of the field is `AABB`"]
    #[inline(always)]
    pub fn is_aabb(&self) -> bool {
        *self == VARIANT_A::AABB
    }
    #[doc = "Checks if the value of the field is `AAB0`"]
    #[inline(always)]
    pub fn is_aab0(&self) -> bool {
        *self == VARIANT_A::AAB0
    }
    #[doc = "Checks if the value of the field is `AAE0`"]
    #[inline(always)]
    pub fn is_aae0(&self) -> bool {
        *self == VARIANT_A::AAE0
    }
    #[doc = "Checks if the value of the field is `UNSPECIFIED`"]
    #[inline(always)]
    pub fn is_unspecified(&self) -> bool {
        *self == VARIANT_A::UNSPECIFIED
    }
}
impl R {
    #[doc = "Bits 0:31 - Part Variant, Hardware version and Production configuration, encoded as ASCII"]
    #[inline(always)]
    pub fn variant(&self) -> VARIANT_R {
        VARIANT_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
