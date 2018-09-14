#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
impl super::TAGHEADER1 {
    #[doc = r" Reads the contents of the register"]
    #[inline]
    pub fn read(&self) -> R {
        R {
            bits: self.register.get(),
        }
    }
}
#[doc = r" Value of the field"]
pub struct UD4R {
    bits: u8,
}
impl UD4R {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct UD5R {
    bits: u8,
}
impl UD5R {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct UD6R {
    bits: u8,
}
impl UD6R {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct UD7R {
    bits: u8,
}
impl UD7R {
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
    #[doc = "Bits 0:7 - Unique identifier byte 4"]
    #[inline]
    pub fn ud4(&self) -> UD4R {
        let bits = {
            const MASK: u8 = 255;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        UD4R { bits }
    }
    #[doc = "Bits 8:15 - Unique identifier byte 5"]
    #[inline]
    pub fn ud5(&self) -> UD5R {
        let bits = {
            const MASK: u8 = 255;
            const OFFSET: u8 = 8;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        UD5R { bits }
    }
    #[doc = "Bits 16:23 - Unique identifier byte 6"]
    #[inline]
    pub fn ud6(&self) -> UD6R {
        let bits = {
            const MASK: u8 = 255;
            const OFFSET: u8 = 16;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        UD6R { bits }
    }
    #[doc = "Bits 24:31 - Unique identifier byte 7"]
    #[inline]
    pub fn ud7(&self) -> UD7R {
        let bits = {
            const MASK: u8 = 255;
            const OFFSET: u8 = 24;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        UD7R { bits }
    }
}
