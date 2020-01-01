#[doc = "Reader of register DEVICEADDRTYPE"]
pub type R = crate::R<u32, super::DEVICEADDRTYPE>;
#[doc = "Device address type\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DEVICEADDRTYPE_A {
    #[doc = "0: Public address"]
    PUBLIC = 0,
    #[doc = "1: Random address"]
    RANDOM = 1,
}
impl From<DEVICEADDRTYPE_A> for bool {
    #[inline(always)]
    fn from(variant: DEVICEADDRTYPE_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `DEVICEADDRTYPE`"]
pub type DEVICEADDRTYPE_R = crate::R<bool, DEVICEADDRTYPE_A>;
impl DEVICEADDRTYPE_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DEVICEADDRTYPE_A {
        match self.bits {
            false => DEVICEADDRTYPE_A::PUBLIC,
            true => DEVICEADDRTYPE_A::RANDOM,
        }
    }
    #[doc = "Checks if the value of the field is `PUBLIC`"]
    #[inline(always)]
    pub fn is_public(&self) -> bool {
        *self == DEVICEADDRTYPE_A::PUBLIC
    }
    #[doc = "Checks if the value of the field is `RANDOM`"]
    #[inline(always)]
    pub fn is_random(&self) -> bool {
        *self == DEVICEADDRTYPE_A::RANDOM
    }
}
impl R {
    #[doc = "Bit 0 - Device address type"]
    #[inline(always)]
    pub fn deviceaddrtype(&self) -> DEVICEADDRTYPE_R {
        DEVICEADDRTYPE_R::new((self.bits & 0x01) != 0)
    }
}
