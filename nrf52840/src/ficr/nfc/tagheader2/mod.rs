#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
impl super::TAGHEADER2 {
    #[doc = r" Reads the contents of the register"]
    #[inline]
    pub fn read(&self) -> R {
        R {
            bits: self.register.get(),
        }
    }
}
#[doc = r" Value of the field"]
pub struct UD8R {
    bits: u8,
}
impl UD8R {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct UD9R {
    bits: u8,
}
impl UD9R {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct UD10R {
    bits: u8,
}
impl UD10R {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct UD11R {
    bits: u8,
}
impl UD11R {
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
    #[doc = "Bits 0:7 - Unique identifier byte 8"]
    #[inline]
    pub fn ud8(&self) -> UD8R {
        let bits = {
            const MASK: u8 = 255;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        UD8R { bits }
    }
    #[doc = "Bits 8:15 - Unique identifier byte 9"]
    #[inline]
    pub fn ud9(&self) -> UD9R {
        let bits = {
            const MASK: u8 = 255;
            const OFFSET: u8 = 8;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        UD9R { bits }
    }
    #[doc = "Bits 16:23 - Unique identifier byte 10"]
    #[inline]
    pub fn ud10(&self) -> UD10R {
        let bits = {
            const MASK: u8 = 255;
            const OFFSET: u8 = 16;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        UD10R { bits }
    }
    #[doc = "Bits 24:31 - Unique identifier byte 11"]
    #[inline]
    pub fn ud11(&self) -> UD11R {
        let bits = {
            const MASK: u8 = 255;
            const OFFSET: u8 = 24;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        UD11R { bits }
    }
}
