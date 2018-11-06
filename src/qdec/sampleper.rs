#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::SAMPLEPER {
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
#[doc = "Possible values of the field `SAMPLEPER`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SAMPLEPERR {
    #[doc = "128 us"]
    _128US,
    #[doc = "256 us"]
    _256US,
    #[doc = "512 us"]
    _512US,
    #[doc = "1024 us"]
    _1024US,
    #[doc = "2048 us"]
    _2048US,
    #[doc = "4096 us"]
    _4096US,
    #[doc = "8192 us"]
    _8192US,
    #[doc = "16384 us"]
    _16384US,
    #[doc = "32768 us"]
    _32MS,
    #[doc = "65536 us"]
    _65MS,
    #[doc = "131072 us"]
    _131MS,
    #[doc = r" Reserved"]
    _Reserved(u8),
}
impl SAMPLEPERR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            SAMPLEPERR::_128US => 0,
            SAMPLEPERR::_256US => 1,
            SAMPLEPERR::_512US => 2,
            SAMPLEPERR::_1024US => 3,
            SAMPLEPERR::_2048US => 4,
            SAMPLEPERR::_4096US => 5,
            SAMPLEPERR::_8192US => 6,
            SAMPLEPERR::_16384US => 7,
            SAMPLEPERR::_32MS => 8,
            SAMPLEPERR::_65MS => 9,
            SAMPLEPERR::_131MS => 10,
            SAMPLEPERR::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> SAMPLEPERR {
        match value {
            0 => SAMPLEPERR::_128US,
            1 => SAMPLEPERR::_256US,
            2 => SAMPLEPERR::_512US,
            3 => SAMPLEPERR::_1024US,
            4 => SAMPLEPERR::_2048US,
            5 => SAMPLEPERR::_4096US,
            6 => SAMPLEPERR::_8192US,
            7 => SAMPLEPERR::_16384US,
            8 => SAMPLEPERR::_32MS,
            9 => SAMPLEPERR::_65MS,
            10 => SAMPLEPERR::_131MS,
            i => SAMPLEPERR::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `_128US`"]
    #[inline]
    pub fn is_128us(&self) -> bool {
        *self == SAMPLEPERR::_128US
    }
    #[doc = "Checks if the value of the field is `_256US`"]
    #[inline]
    pub fn is_256us(&self) -> bool {
        *self == SAMPLEPERR::_256US
    }
    #[doc = "Checks if the value of the field is `_512US`"]
    #[inline]
    pub fn is_512us(&self) -> bool {
        *self == SAMPLEPERR::_512US
    }
    #[doc = "Checks if the value of the field is `_1024US`"]
    #[inline]
    pub fn is_1024us(&self) -> bool {
        *self == SAMPLEPERR::_1024US
    }
    #[doc = "Checks if the value of the field is `_2048US`"]
    #[inline]
    pub fn is_2048us(&self) -> bool {
        *self == SAMPLEPERR::_2048US
    }
    #[doc = "Checks if the value of the field is `_4096US`"]
    #[inline]
    pub fn is_4096us(&self) -> bool {
        *self == SAMPLEPERR::_4096US
    }
    #[doc = "Checks if the value of the field is `_8192US`"]
    #[inline]
    pub fn is_8192us(&self) -> bool {
        *self == SAMPLEPERR::_8192US
    }
    #[doc = "Checks if the value of the field is `_16384US`"]
    #[inline]
    pub fn is_16384us(&self) -> bool {
        *self == SAMPLEPERR::_16384US
    }
    #[doc = "Checks if the value of the field is `_32MS`"]
    #[inline]
    pub fn is_32ms(&self) -> bool {
        *self == SAMPLEPERR::_32MS
    }
    #[doc = "Checks if the value of the field is `_65MS`"]
    #[inline]
    pub fn is_65ms(&self) -> bool {
        *self == SAMPLEPERR::_65MS
    }
    #[doc = "Checks if the value of the field is `_131MS`"]
    #[inline]
    pub fn is_131ms(&self) -> bool {
        *self == SAMPLEPERR::_131MS
    }
}
#[doc = "Values that can be written to the field `SAMPLEPER`"]
pub enum SAMPLEPERW {
    #[doc = "128 us"]
    _128US,
    #[doc = "256 us"]
    _256US,
    #[doc = "512 us"]
    _512US,
    #[doc = "1024 us"]
    _1024US,
    #[doc = "2048 us"]
    _2048US,
    #[doc = "4096 us"]
    _4096US,
    #[doc = "8192 us"]
    _8192US,
    #[doc = "16384 us"]
    _16384US,
    #[doc = "32768 us"]
    _32MS,
    #[doc = "65536 us"]
    _65MS,
    #[doc = "131072 us"]
    _131MS,
}
impl SAMPLEPERW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            SAMPLEPERW::_128US => 0,
            SAMPLEPERW::_256US => 1,
            SAMPLEPERW::_512US => 2,
            SAMPLEPERW::_1024US => 3,
            SAMPLEPERW::_2048US => 4,
            SAMPLEPERW::_4096US => 5,
            SAMPLEPERW::_8192US => 6,
            SAMPLEPERW::_16384US => 7,
            SAMPLEPERW::_32MS => 8,
            SAMPLEPERW::_65MS => 9,
            SAMPLEPERW::_131MS => 10,
        }
    }
}
#[doc = r" Proxy"]
pub struct _SAMPLEPERW<'a> {
    w: &'a mut W,
}
impl<'a> _SAMPLEPERW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: SAMPLEPERW) -> &'a mut W {
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = "128 us"]
    #[inline]
    pub fn _128us(self) -> &'a mut W {
        self.variant(SAMPLEPERW::_128US)
    }
    #[doc = "256 us"]
    #[inline]
    pub fn _256us(self) -> &'a mut W {
        self.variant(SAMPLEPERW::_256US)
    }
    #[doc = "512 us"]
    #[inline]
    pub fn _512us(self) -> &'a mut W {
        self.variant(SAMPLEPERW::_512US)
    }
    #[doc = "1024 us"]
    #[inline]
    pub fn _1024us(self) -> &'a mut W {
        self.variant(SAMPLEPERW::_1024US)
    }
    #[doc = "2048 us"]
    #[inline]
    pub fn _2048us(self) -> &'a mut W {
        self.variant(SAMPLEPERW::_2048US)
    }
    #[doc = "4096 us"]
    #[inline]
    pub fn _4096us(self) -> &'a mut W {
        self.variant(SAMPLEPERW::_4096US)
    }
    #[doc = "8192 us"]
    #[inline]
    pub fn _8192us(self) -> &'a mut W {
        self.variant(SAMPLEPERW::_8192US)
    }
    #[doc = "16384 us"]
    #[inline]
    pub fn _16384us(self) -> &'a mut W {
        self.variant(SAMPLEPERW::_16384US)
    }
    #[doc = "32768 us"]
    #[inline]
    pub fn _32ms(self) -> &'a mut W {
        self.variant(SAMPLEPERW::_32MS)
    }
    #[doc = "65536 us"]
    #[inline]
    pub fn _65ms(self) -> &'a mut W {
        self.variant(SAMPLEPERW::_65MS)
    }
    #[doc = "131072 us"]
    #[inline]
    pub fn _131ms(self) -> &'a mut W {
        self.variant(SAMPLEPERW::_131MS)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 15;
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
    #[doc = "Bits 0:3 - Sample period. The SAMPLE register will be updated for every new sample"]
    #[inline]
    pub fn sampleper(&self) -> SAMPLEPERR {
        SAMPLEPERR::_from({
            const MASK: u8 = 15;
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
    #[doc = "Bits 0:3 - Sample period. The SAMPLE register will be updated for every new sample"]
    #[inline]
    pub fn sampleper(&mut self) -> _SAMPLEPERW {
        _SAMPLEPERW { w: self }
    }
}
