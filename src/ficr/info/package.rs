#[doc = "Reader of register PACKAGE"]
pub type R = crate::R<u32, super::PACKAGE>;
#[doc = "Package option\n\nValue on reset: 8192"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PACKAGE_A {
    #[doc = "8192: QFxx - 48-pin QFN"]
    QF,
    #[doc = "8193: CHxx - 7x8 WLCSP 56 balls"]
    CH,
    #[doc = "8194: CIxx - 7x8 WLCSP 56 balls"]
    CI,
    #[doc = "8197: CKxx - 7x8 WLCSP 56 balls with backside coating for light protection"]
    CK,
    #[doc = "4294967295: Unspecified"]
    UNSPECIFIED,
}
impl From<PACKAGE_A> for u32 {
    #[inline(always)]
    fn from(variant: PACKAGE_A) -> Self {
        match variant {
            PACKAGE_A::QF => 8192,
            PACKAGE_A::CH => 8193,
            PACKAGE_A::CI => 8194,
            PACKAGE_A::CK => 8197,
            PACKAGE_A::UNSPECIFIED => 4294967295,
        }
    }
}
#[doc = "Reader of field `PACKAGE`"]
pub type PACKAGE_R = crate::R<u32, PACKAGE_A>;
impl PACKAGE_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u32, PACKAGE_A> {
        use crate::Variant::*;
        match self.bits {
            8192 => Val(PACKAGE_A::QF),
            8193 => Val(PACKAGE_A::CH),
            8194 => Val(PACKAGE_A::CI),
            8197 => Val(PACKAGE_A::CK),
            4294967295 => Val(PACKAGE_A::UNSPECIFIED),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `QF`"]
    #[inline(always)]
    pub fn is_qf(&self) -> bool {
        *self == PACKAGE_A::QF
    }
    #[doc = "Checks if the value of the field is `CH`"]
    #[inline(always)]
    pub fn is_ch(&self) -> bool {
        *self == PACKAGE_A::CH
    }
    #[doc = "Checks if the value of the field is `CI`"]
    #[inline(always)]
    pub fn is_ci(&self) -> bool {
        *self == PACKAGE_A::CI
    }
    #[doc = "Checks if the value of the field is `CK`"]
    #[inline(always)]
    pub fn is_ck(&self) -> bool {
        *self == PACKAGE_A::CK
    }
    #[doc = "Checks if the value of the field is `UNSPECIFIED`"]
    #[inline(always)]
    pub fn is_unspecified(&self) -> bool {
        *self == PACKAGE_A::UNSPECIFIED
    }
}
impl R {
    #[doc = "Bits 0:31 - Package option"]
    #[inline(always)]
    pub fn package(&self) -> PACKAGE_R {
        PACKAGE_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
