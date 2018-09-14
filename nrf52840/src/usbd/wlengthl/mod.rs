#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
impl super::WLENGTHL {
    #[doc = r" Reads the contents of the register"]
    #[inline]
    pub fn read(&self) -> R {
        R {
            bits: self.register.get(),
        }
    }
}
#[doc = r" Value of the field"]
pub struct WLENGTHLR {
    bits: u8,
}
impl WLENGTHLR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
impl R {
    #[doc = r" Value of the register as raw bits"]
    #[inline]
    pub fn bits(&self) -> u32 {
        self.bits
    }
    #[doc = "Bits 0:7 - SETUP data, byte 6, LSB of wLength"]
    #[inline]
    pub fn wlengthl(&self) -> WLENGTHLR {
        let bits = {
            const MASK: u8 = 255;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        WLENGTHLR { bits }
    }
}
