#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
impl super::WLENGTHH {
    #[doc = r" Reads the contents of the register"]
    #[inline]
    pub fn read(&self) -> R {
        R {
            bits: self.register.get(),
        }
    }
}
#[doc = r" Value of the field"]
pub struct WLENGTHHR {
    bits: u8,
}
impl WLENGTHHR {
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
    #[doc = "Bits 0:7 - SETUP data, byte 7, MSB of wLength"]
    #[inline]
    pub fn wlengthh(&self) -> WLENGTHHR {
        let bits = {
            const MASK: u8 = 255;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        WLENGTHHR { bits }
    }
}
