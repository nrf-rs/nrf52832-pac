#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
impl super::ACCDBLREAD {
    #[doc = r" Reads the contents of the register"]
    #[inline]
    pub fn read(&self) -> R {
        R {
            bits: self.register.get(),
        }
    }
}
#[doc = r" Value of the field"]
pub struct ACCDBLREADR {
    bits: u8,
}
impl ACCDBLREADR {
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
    #[doc = "Bits 0:3 - Snapshot of the ACCDBL register. This field is updated when the READCLRACC or RDCLRDBL task is triggered."]
    #[inline]
    pub fn accdblread(&self) -> ACCDBLREADR {
        let bits = {
            const MASK: u8 = 15;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        ACCDBLREADR { bits }
    }
}
