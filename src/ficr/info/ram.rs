#[doc = "Reader of register RAM"]
pub type R = crate::R<u32, super::RAM>;
#[doc = "RAM variant\n\nValue on reset: 64"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u32)]
pub enum RAM_A {
    #[doc = "16: 16 kByte RAM"]
    K16 = 16,
    #[doc = "32: 32 kByte RAM"]
    K32 = 32,
    #[doc = "64: 64 kByte RAM"]
    K64 = 64,
    #[doc = "4294967295: Unspecified"]
    UNSPECIFIED = 4294967295,
}
impl From<RAM_A> for u32 {
    #[inline(always)]
    fn from(variant: RAM_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `RAM`"]
pub type RAM_R = crate::R<u32, RAM_A>;
impl RAM_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u32, RAM_A> {
        use crate::Variant::*;
        match self.bits {
            16 => Val(RAM_A::K16),
            32 => Val(RAM_A::K32),
            64 => Val(RAM_A::K64),
            4294967295 => Val(RAM_A::UNSPECIFIED),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `K16`"]
    #[inline(always)]
    pub fn is_k16(&self) -> bool {
        *self == RAM_A::K16
    }
    #[doc = "Checks if the value of the field is `K32`"]
    #[inline(always)]
    pub fn is_k32(&self) -> bool {
        *self == RAM_A::K32
    }
    #[doc = "Checks if the value of the field is `K64`"]
    #[inline(always)]
    pub fn is_k64(&self) -> bool {
        *self == RAM_A::K64
    }
    #[doc = "Checks if the value of the field is `UNSPECIFIED`"]
    #[inline(always)]
    pub fn is_unspecified(&self) -> bool {
        *self == RAM_A::UNSPECIFIED
    }
}
impl R {
    #[doc = "Bits 0:31 - RAM variant"]
    #[inline(always)]
    pub fn ram(&self) -> RAM_R {
        RAM_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
