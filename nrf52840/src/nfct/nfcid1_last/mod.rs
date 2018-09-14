#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::NFCID1_LAST {
    #[doc = r" Modifies the contents of the register"]
    #[inline]
    pub fn modify<F>(&self, f: F)
    where
        for<'w> F: FnOnce(&R, &'w mut W) -> &'w mut W,
    {
        let bits = self.register.get();
        let r = R { bits: bits };
        let mut w = W { bits: bits };
        f(&r, &mut w);
        self.register.set(w.bits);
    }
    #[doc = r" Reads the contents of the register"]
    #[inline]
    pub fn read(&self) -> R {
        R {
            bits: self.register.get(),
        }
    }
    #[doc = r" Writes to the register"]
    #[inline]
    pub fn write<F>(&self, f: F)
    where
        F: FnOnce(&mut W) -> &mut W,
    {
        let mut w = W::reset_value();
        f(&mut w);
        self.register.set(w.bits);
    }
    #[doc = r" Writes the reset value to the register"]
    #[inline]
    pub fn reset(&self) {
        self.write(|w| w)
    }
}
#[doc = r" Value of the field"]
pub struct NFCID1_ZR {
    bits: u8,
}
impl NFCID1_ZR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct NFCID1_YR {
    bits: u8,
}
impl NFCID1_YR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct NFCID1_XR {
    bits: u8,
}
impl NFCID1_XR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct NFCID1_WR {
    bits: u8,
}
impl NFCID1_WR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Proxy"]
pub struct _NFCID1_ZW<'a> {
    w: &'a mut W,
}
impl<'a> _NFCID1_ZW<'a> {
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 255;
        const OFFSET: u8 = 0;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _NFCID1_YW<'a> {
    w: &'a mut W,
}
impl<'a> _NFCID1_YW<'a> {
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 255;
        const OFFSET: u8 = 8;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _NFCID1_XW<'a> {
    w: &'a mut W,
}
impl<'a> _NFCID1_XW<'a> {
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 255;
        const OFFSET: u8 = 16;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _NFCID1_WW<'a> {
    w: &'a mut W,
}
impl<'a> _NFCID1_WW<'a> {
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 255;
        const OFFSET: u8 = 24;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
impl R {
    #[doc = r" Value of the register as raw bits"]
    #[inline]
    pub fn bits(&self) -> u32 {
        self.bits
    }
    #[doc = "Bits 0:7 - NFCID1 byte Z (very last byte sent)"]
    #[inline]
    pub fn nfcid1_z(&self) -> NFCID1_ZR {
        let bits = {
            const MASK: u8 = 255;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        NFCID1_ZR { bits }
    }
    #[doc = "Bits 8:15 - NFCID1 byte Y"]
    #[inline]
    pub fn nfcid1_y(&self) -> NFCID1_YR {
        let bits = {
            const MASK: u8 = 255;
            const OFFSET: u8 = 8;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        NFCID1_YR { bits }
    }
    #[doc = "Bits 16:23 - NFCID1 byte X"]
    #[inline]
    pub fn nfcid1_x(&self) -> NFCID1_XR {
        let bits = {
            const MASK: u8 = 255;
            const OFFSET: u8 = 16;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        NFCID1_XR { bits }
    }
    #[doc = "Bits 24:31 - NFCID1 byte W"]
    #[inline]
    pub fn nfcid1_w(&self) -> NFCID1_WR {
        let bits = {
            const MASK: u8 = 255;
            const OFFSET: u8 = 24;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        NFCID1_WR { bits }
    }
}
impl W {
    #[doc = r" Reset value of the register"]
    #[inline]
    pub fn reset_value() -> W {
        W { bits: 25443 }
    }
    #[doc = r" Writes raw bits to the register"]
    #[inline]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
    #[doc = "Bits 0:7 - NFCID1 byte Z (very last byte sent)"]
    #[inline]
    pub fn nfcid1_z(&mut self) -> _NFCID1_ZW {
        _NFCID1_ZW { w: self }
    }
    #[doc = "Bits 8:15 - NFCID1 byte Y"]
    #[inline]
    pub fn nfcid1_y(&mut self) -> _NFCID1_YW {
        _NFCID1_YW { w: self }
    }
    #[doc = "Bits 16:23 - NFCID1 byte X"]
    #[inline]
    pub fn nfcid1_x(&mut self) -> _NFCID1_XW {
        _NFCID1_XW { w: self }
    }
    #[doc = "Bits 24:31 - NFCID1 byte W"]
    #[inline]
    pub fn nfcid1_w(&mut self) -> _NFCID1_WW {
        _NFCID1_WW { w: self }
    }
}
