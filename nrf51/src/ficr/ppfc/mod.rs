#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::PPFC {
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
#[doc = "Possible values of the field `PPFC`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PPFCR {
    #[doc = "Not present."]
    NOTPRESENT,
    #[doc = "Present."]
    PRESENT,
    #[doc = r" Reserved"]
    _Reserved(u8),
}
impl PPFCR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            PPFCR::NOTPRESENT => 255,
            PPFCR::PRESENT => 0,
            PPFCR::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> PPFCR {
        match value {
            255 => PPFCR::NOTPRESENT,
            0 => PPFCR::PRESENT,
            i => PPFCR::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `NOTPRESENT`"]
    #[inline]
    pub fn is_not_present(&self) -> bool {
        *self == PPFCR::NOTPRESENT
    }
    #[doc = "Checks if the value of the field is `PRESENT`"]
    #[inline]
    pub fn is_present(&self) -> bool {
        *self == PPFCR::PRESENT
    }
}
#[doc = "Values that can be written to the field `PPFC`"]
pub enum PPFCW {
    #[doc = "Not present."]
    NOTPRESENT,
    #[doc = "Present."]
    PRESENT,
}
impl PPFCW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            PPFCW::NOTPRESENT => 255,
            PPFCW::PRESENT => 0,
        }
    }
}
#[doc = r" Proxy"]
pub struct _PPFCW<'a> {
    w: &'a mut W,
}
impl<'a> _PPFCW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: PPFCW) -> &'a mut W {
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = "Not present."]
    #[inline]
    pub fn not_present(self) -> &'a mut W {
        self.variant(PPFCW::NOTPRESENT)
    }
    #[doc = "Present."]
    #[inline]
    pub fn present(self) -> &'a mut W {
        self.variant(PPFCW::PRESENT)
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
    #[doc = "Bits 0:7 - Pre-programmed factory code present."]
    #[inline]
    pub fn ppfc(&self) -> PPFCR {
        PPFCR::_from({
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
        W { bits: 0 }
    }
    #[doc = r" Writes raw bits to the register"]
    #[inline]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
    #[doc = "Bits 0:7 - Pre-programmed factory code present."]
    #[inline]
    pub fn ppfc(&mut self) -> _PPFCW {
        _PPFCW { w: self }
    }
}
