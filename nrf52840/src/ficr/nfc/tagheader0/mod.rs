#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
impl super::TAGHEADER0 {
    #[doc = r" Reads the contents of the register"]
    #[inline]
    pub fn read(&self) -> R {
        R {
            bits: self.register.get(),
        }
    }
}
#[doc = r" Value of the field"]
pub struct MFGIDR {
    bits: u8,
}
impl MFGIDR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct UD1R {
    bits: u8,
}
impl UD1R {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct UD2R {
    bits: u8,
}
impl UD2R {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct UD3R {
    bits: u8,
}
impl UD3R {
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
    #[doc = "Bits 0:7 - Default Manufacturer ID: Nordic Semiconductor ASA has ICM 0x5F"]
    #[inline]
    pub fn mfgid(&self) -> MFGIDR {
        let bits = {
            const MASK: u8 = 255;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        MFGIDR { bits }
    }
    #[doc = "Bits 8:15 - Unique identifier byte 1"]
    #[inline]
    pub fn ud1(&self) -> UD1R {
        let bits = {
            const MASK: u8 = 255;
            const OFFSET: u8 = 8;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        UD1R { bits }
    }
    #[doc = "Bits 16:23 - Unique identifier byte 2"]
    #[inline]
    pub fn ud2(&self) -> UD2R {
        let bits = {
            const MASK: u8 = 255;
            const OFFSET: u8 = 16;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        UD2R { bits }
    }
    #[doc = "Bits 24:31 - Unique identifier byte 3"]
    #[inline]
    pub fn ud3(&self) -> UD3R {
        let bits = {
            const MASK: u8 = 255;
            const OFFSET: u8 = 24;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        UD3R { bits }
    }
}
