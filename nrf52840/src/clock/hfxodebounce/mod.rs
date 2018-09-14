#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::HFXODEBOUNCE {
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
#[doc = "Possible values of the field `HFXODEBOUNCE`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum HFXODEBOUNCER {
    #[doc = "256 us debounce time. Recommended for TSX-3225, FA-20H and FA-128 crystals."]
    DB256US,
    #[doc = "1024 us debounce time. Recommended for NX1612AA and NX1210AB crystals."]
    DB1024US,
    #[doc = r" Reserved"]
    _Reserved(u8),
}
impl HFXODEBOUNCER {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            HFXODEBOUNCER::DB256US => 16,
            HFXODEBOUNCER::DB1024US => 64,
            HFXODEBOUNCER::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> HFXODEBOUNCER {
        match value {
            16 => HFXODEBOUNCER::DB256US,
            64 => HFXODEBOUNCER::DB1024US,
            i => HFXODEBOUNCER::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `DB256US`"]
    #[inline]
    pub fn is_db256us(&self) -> bool {
        *self == HFXODEBOUNCER::DB256US
    }
    #[doc = "Checks if the value of the field is `DB1024US`"]
    #[inline]
    pub fn is_db1024us(&self) -> bool {
        *self == HFXODEBOUNCER::DB1024US
    }
}
#[doc = "Values that can be written to the field `HFXODEBOUNCE`"]
pub enum HFXODEBOUNCEW {
    #[doc = "256 us debounce time. Recommended for TSX-3225, FA-20H and FA-128 crystals."]
    DB256US,
    #[doc = "1024 us debounce time. Recommended for NX1612AA and NX1210AB crystals."]
    DB1024US,
}
impl HFXODEBOUNCEW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            HFXODEBOUNCEW::DB256US => 16,
            HFXODEBOUNCEW::DB1024US => 64,
        }
    }
}
#[doc = r" Proxy"]
pub struct _HFXODEBOUNCEW<'a> {
    w: &'a mut W,
}
impl<'a> _HFXODEBOUNCEW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: HFXODEBOUNCEW) -> &'a mut W {
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = "256 us debounce time. Recommended for TSX-3225, FA-20H and FA-128 crystals."]
    #[inline]
    pub fn db256us(self) -> &'a mut W {
        self.variant(HFXODEBOUNCEW::DB256US)
    }
    #[doc = "1024 us debounce time. Recommended for NX1612AA and NX1210AB crystals."]
    #[inline]
    pub fn db1024us(self) -> &'a mut W {
        self.variant(HFXODEBOUNCEW::DB1024US)
    }
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
impl R {
    #[doc = r" Value of the register as raw bits"]
    #[inline]
    pub fn bits(&self) -> u32 {
        self.bits
    }
    #[doc = "Bits 0:7 - HFXO debounce time. Debounce time = HFXODEBOUNCE * 16 us."]
    #[inline]
    pub fn hfxodebounce(&self) -> HFXODEBOUNCER {
        HFXODEBOUNCER::_from({
            const MASK: u8 = 255;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
}
impl W {
    #[doc = r" Reset value of the register"]
    #[inline]
    pub fn reset_value() -> W {
        W { bits: 16 }
    }
    #[doc = r" Writes raw bits to the register"]
    #[inline]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
    #[doc = "Bits 0:7 - HFXO debounce time. Debounce time = HFXODEBOUNCE * 16 us."]
    #[inline]
    pub fn hfxodebounce(&mut self) -> _HFXODEBOUNCEW {
        _HFXODEBOUNCEW { w: self }
    }
}
