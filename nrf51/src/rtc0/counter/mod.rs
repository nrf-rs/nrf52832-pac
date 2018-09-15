#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
impl super::COUNTER {
    #[doc = r" Reads the contents of the register"]
    #[inline]
    pub fn read(&self) -> R {
        R {
            bits: self.register.get(),
        }
    }
}
#[doc = r" Value of the field"]
pub struct COUNTERR {
    bits: u32,
}
impl COUNTERR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u32 {
        self.bits
    }
}
impl R {
    #[doc = r" Value of the register as raw bits"]
    #[inline]
    pub fn bits(&self) -> u32 {
        self.bits
    }
    #[doc = "Bits 0:23 - Counter value."]
    #[inline]
    pub fn counter(&self) -> COUNTERR {
        let bits = {
            const MASK: u32 = 16777215;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) as u32
        };
        COUNTERR { bits }
    }
}
