#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
impl super::CURRENTLOADCTRL {
    #[doc = r" Reads the contents of the register"]
    #[inline]
    pub fn read(&self) -> R {
        R {
            bits: self.register.get(),
        }
    }
}
#[doc = r" Value of the field"]
pub struct CURRENTLOADCTRLR {
    bits: u8,
}
impl CURRENTLOADCTRLR {
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
    #[doc = "Bits 0:5 - Current value driven to the NFC Load Control"]
    #[inline]
    pub fn currentloadctrl(&self) -> CURRENTLOADCTRLR {
        let bits = {
            const MASK: u8 = 63;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        CURRENTLOADCTRLR { bits }
    }
}
