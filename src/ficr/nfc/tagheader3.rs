#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
impl super::TAGHEADER3 {
    #[doc = r" Reads the contents of the register"]
    #[inline]
    pub fn read(&self) -> R {
        R {
            bits: self.register.get(),
        }
    }
}
#[doc = r" Value of the field"]
pub struct UD12R {
    bits: u8,
}
impl UD12R {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct UD13R {
    bits: u8,
}
impl UD13R {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct UD14R {
    bits: u8,
}
impl UD14R {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct UD15R {
    bits: u8,
}
impl UD15R {
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
    #[doc = "Bits 0:7 - Unique identifier byte 12"]
    #[inline]
    pub fn ud12(&self) -> UD12R {
        let bits = {
            const MASK: u8 = 255;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        UD12R { bits }
    }
    #[doc = "Bits 8:15 - Unique identifier byte 13"]
    #[inline]
    pub fn ud13(&self) -> UD13R {
        let bits = {
            const MASK: u8 = 255;
            const OFFSET: u8 = 8;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        UD13R { bits }
    }
    #[doc = "Bits 16:23 - Unique identifier byte 14"]
    #[inline]
    pub fn ud14(&self) -> UD14R {
        let bits = {
            const MASK: u8 = 255;
            const OFFSET: u8 = 16;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        UD14R { bits }
    }
    #[doc = "Bits 24:31 - Unique identifier byte 15"]
    #[inline]
    pub fn ud15(&self) -> UD15R {
        let bits = {
            const MASK: u8 = 255;
            const OFFSET: u8 = 24;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        UD15R { bits }
    }
}
