#[doc = "Reader of register FIELDPRESENT"]
pub type R = crate::R<u32, super::FIELDPRESENT>;
#[doc = "Indicates the presence or not of a valid field. Available only in the activated state.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum FIELDPRESENT_A {
    #[doc = "0: No valid field detected"]
    NOFIELD = 0,
    #[doc = "1: Valid field detected"]
    FIELDPRESENT = 1,
}
impl From<FIELDPRESENT_A> for bool {
    #[inline(always)]
    fn from(variant: FIELDPRESENT_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `FIELDPRESENT`"]
pub type FIELDPRESENT_R = crate::R<bool, FIELDPRESENT_A>;
impl FIELDPRESENT_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> FIELDPRESENT_A {
        match self.bits {
            false => FIELDPRESENT_A::NOFIELD,
            true => FIELDPRESENT_A::FIELDPRESENT,
        }
    }
    #[doc = "Checks if the value of the field is `NOFIELD`"]
    #[inline(always)]
    pub fn is_no_field(&self) -> bool {
        *self == FIELDPRESENT_A::NOFIELD
    }
    #[doc = "Checks if the value of the field is `FIELDPRESENT`"]
    #[inline(always)]
    pub fn is_field_present(&self) -> bool {
        *self == FIELDPRESENT_A::FIELDPRESENT
    }
}
#[doc = "Indicates if the low level has locked to the field\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum LOCKDETECT_A {
    #[doc = "0: Not locked to field"]
    NOTLOCKED = 0,
    #[doc = "1: Locked to field"]
    LOCKED = 1,
}
impl From<LOCKDETECT_A> for bool {
    #[inline(always)]
    fn from(variant: LOCKDETECT_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `LOCKDETECT`"]
pub type LOCKDETECT_R = crate::R<bool, LOCKDETECT_A>;
impl LOCKDETECT_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> LOCKDETECT_A {
        match self.bits {
            false => LOCKDETECT_A::NOTLOCKED,
            true => LOCKDETECT_A::LOCKED,
        }
    }
    #[doc = "Checks if the value of the field is `NOTLOCKED`"]
    #[inline(always)]
    pub fn is_not_locked(&self) -> bool {
        *self == LOCKDETECT_A::NOTLOCKED
    }
    #[doc = "Checks if the value of the field is `LOCKED`"]
    #[inline(always)]
    pub fn is_locked(&self) -> bool {
        *self == LOCKDETECT_A::LOCKED
    }
}
impl R {
    #[doc = "Bit 0 - Indicates the presence or not of a valid field. Available only in the activated state."]
    #[inline(always)]
    pub fn fieldpresent(&self) -> FIELDPRESENT_R {
        FIELDPRESENT_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - Indicates if the low level has locked to the field"]
    #[inline(always)]
    pub fn lockdetect(&self) -> LOCKDETECT_R {
        LOCKDETECT_R::new(((self.bits >> 1) & 0x01) != 0)
    }
}
