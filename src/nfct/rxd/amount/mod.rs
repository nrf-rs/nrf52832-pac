#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
impl super::AMOUNT {
    #[doc = r" Reads the contents of the register"]
    #[inline]
    pub fn read(&self) -> R {
        R {
            bits: self.register.get(),
        }
    }
}
#[doc = r" Value of the field"]
pub struct RXDATABITSR {
    bits: u8,
}
impl RXDATABITSR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct RXDATABYTESR {
    bits: u16,
}
impl RXDATABYTESR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u16 {
        self.bits
    }
}
impl R {
    #[doc = r" Value of the register as raw bits"]
    #[inline]
    pub fn bits(&self) -> u32 {
        self.bits
    }
    #[doc = "Bits 0:2 - Number of bits in the last byte in the frame, if less than 8 (including CRC, but excluding parity and SoF/EoF framing)."]
    #[inline]
    pub fn rxdatabits(&self) -> RXDATABITSR {
        let bits = {
            const MASK: u8 = 7;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        RXDATABITSR { bits }
    }
    #[doc = "Bits 3:11 - Number of complete bytes received in the frame (including CRC, but excluding parity and SoF/EoF framing)"]
    #[inline]
    pub fn rxdatabytes(&self) -> RXDATABYTESR {
        let bits = {
            const MASK: u16 = 511;
            const OFFSET: u8 = 3;
            ((self.bits >> OFFSET) & MASK as u32) as u16
        };
        RXDATABYTESR { bits }
    }
}
