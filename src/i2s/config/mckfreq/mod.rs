#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::MCKFREQ {
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
#[doc = "Possible values of the field `MCKFREQ`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum MCKFREQR {
    #[doc = "32 MHz / 2 = 16.0 MHz"]
    _32MDIV2,
    #[doc = "32 MHz / 3 = 10.6666667 MHz"]
    _32MDIV3,
    #[doc = "32 MHz / 4 = 8.0 MHz"]
    _32MDIV4,
    #[doc = "32 MHz / 5 = 6.4 MHz"]
    _32MDIV5,
    #[doc = "32 MHz / 6 = 5.3333333 MHz"]
    _32MDIV6,
    #[doc = "32 MHz / 8 = 4.0 MHz"]
    _32MDIV8,
    #[doc = "32 MHz / 10 = 3.2 MHz"]
    _32MDIV10,
    #[doc = "32 MHz / 11 = 2.9090909 MHz"]
    _32MDIV11,
    #[doc = "32 MHz / 15 = 2.1333333 MHz"]
    _32MDIV15,
    #[doc = "32 MHz / 16 = 2.0 MHz"]
    _32MDIV16,
    #[doc = "32 MHz / 21 = 1.5238095"]
    _32MDIV21,
    #[doc = "32 MHz / 23 = 1.3913043 MHz"]
    _32MDIV23,
    #[doc = "32 MHz / 30 = 1.0666667 MHz"]
    _32MDIV30,
    #[doc = "32 MHz / 31 = 1.0322581 MHz"]
    _32MDIV31,
    #[doc = "32 MHz / 32 = 1.0 MHz"]
    _32MDIV32,
    #[doc = "32 MHz / 42 = 0.7619048 MHz"]
    _32MDIV42,
    #[doc = "32 MHz / 63 = 0.5079365 MHz"]
    _32MDIV63,
    #[doc = "32 MHz / 125 = 0.256 MHz"]
    _32MDIV125,
    #[doc = r" Reserved"]
    _Reserved(u32),
}
impl MCKFREQR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u32 {
        match *self {
            MCKFREQR::_32MDIV2 => 2147483648,
            MCKFREQR::_32MDIV3 => 1342177280,
            MCKFREQR::_32MDIV4 => 1073741824,
            MCKFREQR::_32MDIV5 => 805306368,
            MCKFREQR::_32MDIV6 => 671088640,
            MCKFREQR::_32MDIV8 => 536870912,
            MCKFREQR::_32MDIV10 => 402653184,
            MCKFREQR::_32MDIV11 => 369098752,
            MCKFREQR::_32MDIV15 => 285212672,
            MCKFREQR::_32MDIV16 => 268435456,
            MCKFREQR::_32MDIV21 => 201326592,
            MCKFREQR::_32MDIV23 => 184549376,
            MCKFREQR::_32MDIV30 => 142606336,
            MCKFREQR::_32MDIV31 => 138412032,
            MCKFREQR::_32MDIV32 => 134217728,
            MCKFREQR::_32MDIV42 => 100663296,
            MCKFREQR::_32MDIV63 => 68157440,
            MCKFREQR::_32MDIV125 => 34340864,
            MCKFREQR::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u32) -> MCKFREQR {
        match value {
            2147483648 => MCKFREQR::_32MDIV2,
            1342177280 => MCKFREQR::_32MDIV3,
            1073741824 => MCKFREQR::_32MDIV4,
            805306368 => MCKFREQR::_32MDIV5,
            671088640 => MCKFREQR::_32MDIV6,
            536870912 => MCKFREQR::_32MDIV8,
            402653184 => MCKFREQR::_32MDIV10,
            369098752 => MCKFREQR::_32MDIV11,
            285212672 => MCKFREQR::_32MDIV15,
            268435456 => MCKFREQR::_32MDIV16,
            201326592 => MCKFREQR::_32MDIV21,
            184549376 => MCKFREQR::_32MDIV23,
            142606336 => MCKFREQR::_32MDIV30,
            138412032 => MCKFREQR::_32MDIV31,
            134217728 => MCKFREQR::_32MDIV32,
            100663296 => MCKFREQR::_32MDIV42,
            68157440 => MCKFREQR::_32MDIV63,
            34340864 => MCKFREQR::_32MDIV125,
            i => MCKFREQR::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `_32MDIV2`"]
    #[inline]
    pub fn is_32mdiv2(&self) -> bool {
        *self == MCKFREQR::_32MDIV2
    }
    #[doc = "Checks if the value of the field is `_32MDIV3`"]
    #[inline]
    pub fn is_32mdiv3(&self) -> bool {
        *self == MCKFREQR::_32MDIV3
    }
    #[doc = "Checks if the value of the field is `_32MDIV4`"]
    #[inline]
    pub fn is_32mdiv4(&self) -> bool {
        *self == MCKFREQR::_32MDIV4
    }
    #[doc = "Checks if the value of the field is `_32MDIV5`"]
    #[inline]
    pub fn is_32mdiv5(&self) -> bool {
        *self == MCKFREQR::_32MDIV5
    }
    #[doc = "Checks if the value of the field is `_32MDIV6`"]
    #[inline]
    pub fn is_32mdiv6(&self) -> bool {
        *self == MCKFREQR::_32MDIV6
    }
    #[doc = "Checks if the value of the field is `_32MDIV8`"]
    #[inline]
    pub fn is_32mdiv8(&self) -> bool {
        *self == MCKFREQR::_32MDIV8
    }
    #[doc = "Checks if the value of the field is `_32MDIV10`"]
    #[inline]
    pub fn is_32mdiv10(&self) -> bool {
        *self == MCKFREQR::_32MDIV10
    }
    #[doc = "Checks if the value of the field is `_32MDIV11`"]
    #[inline]
    pub fn is_32mdiv11(&self) -> bool {
        *self == MCKFREQR::_32MDIV11
    }
    #[doc = "Checks if the value of the field is `_32MDIV15`"]
    #[inline]
    pub fn is_32mdiv15(&self) -> bool {
        *self == MCKFREQR::_32MDIV15
    }
    #[doc = "Checks if the value of the field is `_32MDIV16`"]
    #[inline]
    pub fn is_32mdiv16(&self) -> bool {
        *self == MCKFREQR::_32MDIV16
    }
    #[doc = "Checks if the value of the field is `_32MDIV21`"]
    #[inline]
    pub fn is_32mdiv21(&self) -> bool {
        *self == MCKFREQR::_32MDIV21
    }
    #[doc = "Checks if the value of the field is `_32MDIV23`"]
    #[inline]
    pub fn is_32mdiv23(&self) -> bool {
        *self == MCKFREQR::_32MDIV23
    }
    #[doc = "Checks if the value of the field is `_32MDIV30`"]
    #[inline]
    pub fn is_32mdiv30(&self) -> bool {
        *self == MCKFREQR::_32MDIV30
    }
    #[doc = "Checks if the value of the field is `_32MDIV31`"]
    #[inline]
    pub fn is_32mdiv31(&self) -> bool {
        *self == MCKFREQR::_32MDIV31
    }
    #[doc = "Checks if the value of the field is `_32MDIV32`"]
    #[inline]
    pub fn is_32mdiv32(&self) -> bool {
        *self == MCKFREQR::_32MDIV32
    }
    #[doc = "Checks if the value of the field is `_32MDIV42`"]
    #[inline]
    pub fn is_32mdiv42(&self) -> bool {
        *self == MCKFREQR::_32MDIV42
    }
    #[doc = "Checks if the value of the field is `_32MDIV63`"]
    #[inline]
    pub fn is_32mdiv63(&self) -> bool {
        *self == MCKFREQR::_32MDIV63
    }
    #[doc = "Checks if the value of the field is `_32MDIV125`"]
    #[inline]
    pub fn is_32mdiv125(&self) -> bool {
        *self == MCKFREQR::_32MDIV125
    }
}
#[doc = "Values that can be written to the field `MCKFREQ`"]
pub enum MCKFREQW {
    #[doc = "32 MHz / 2 = 16.0 MHz"]
    _32MDIV2,
    #[doc = "32 MHz / 3 = 10.6666667 MHz"]
    _32MDIV3,
    #[doc = "32 MHz / 4 = 8.0 MHz"]
    _32MDIV4,
    #[doc = "32 MHz / 5 = 6.4 MHz"]
    _32MDIV5,
    #[doc = "32 MHz / 6 = 5.3333333 MHz"]
    _32MDIV6,
    #[doc = "32 MHz / 8 = 4.0 MHz"]
    _32MDIV8,
    #[doc = "32 MHz / 10 = 3.2 MHz"]
    _32MDIV10,
    #[doc = "32 MHz / 11 = 2.9090909 MHz"]
    _32MDIV11,
    #[doc = "32 MHz / 15 = 2.1333333 MHz"]
    _32MDIV15,
    #[doc = "32 MHz / 16 = 2.0 MHz"]
    _32MDIV16,
    #[doc = "32 MHz / 21 = 1.5238095"]
    _32MDIV21,
    #[doc = "32 MHz / 23 = 1.3913043 MHz"]
    _32MDIV23,
    #[doc = "32 MHz / 30 = 1.0666667 MHz"]
    _32MDIV30,
    #[doc = "32 MHz / 31 = 1.0322581 MHz"]
    _32MDIV31,
    #[doc = "32 MHz / 32 = 1.0 MHz"]
    _32MDIV32,
    #[doc = "32 MHz / 42 = 0.7619048 MHz"]
    _32MDIV42,
    #[doc = "32 MHz / 63 = 0.5079365 MHz"]
    _32MDIV63,
    #[doc = "32 MHz / 125 = 0.256 MHz"]
    _32MDIV125,
}
impl MCKFREQW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u32 {
        match *self {
            MCKFREQW::_32MDIV2 => 2147483648,
            MCKFREQW::_32MDIV3 => 1342177280,
            MCKFREQW::_32MDIV4 => 1073741824,
            MCKFREQW::_32MDIV5 => 805306368,
            MCKFREQW::_32MDIV6 => 671088640,
            MCKFREQW::_32MDIV8 => 536870912,
            MCKFREQW::_32MDIV10 => 402653184,
            MCKFREQW::_32MDIV11 => 369098752,
            MCKFREQW::_32MDIV15 => 285212672,
            MCKFREQW::_32MDIV16 => 268435456,
            MCKFREQW::_32MDIV21 => 201326592,
            MCKFREQW::_32MDIV23 => 184549376,
            MCKFREQW::_32MDIV30 => 142606336,
            MCKFREQW::_32MDIV31 => 138412032,
            MCKFREQW::_32MDIV32 => 134217728,
            MCKFREQW::_32MDIV42 => 100663296,
            MCKFREQW::_32MDIV63 => 68157440,
            MCKFREQW::_32MDIV125 => 34340864,
        }
    }
}
#[doc = r" Proxy"]
pub struct _MCKFREQW<'a> {
    w: &'a mut W,
}
impl<'a> _MCKFREQW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: MCKFREQW) -> &'a mut W {
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = "32 MHz / 2 = 16.0 MHz"]
    #[inline]
    pub fn _32mdiv2(self) -> &'a mut W {
        self.variant(MCKFREQW::_32MDIV2)
    }
    #[doc = "32 MHz / 3 = 10.6666667 MHz"]
    #[inline]
    pub fn _32mdiv3(self) -> &'a mut W {
        self.variant(MCKFREQW::_32MDIV3)
    }
    #[doc = "32 MHz / 4 = 8.0 MHz"]
    #[inline]
    pub fn _32mdiv4(self) -> &'a mut W {
        self.variant(MCKFREQW::_32MDIV4)
    }
    #[doc = "32 MHz / 5 = 6.4 MHz"]
    #[inline]
    pub fn _32mdiv5(self) -> &'a mut W {
        self.variant(MCKFREQW::_32MDIV5)
    }
    #[doc = "32 MHz / 6 = 5.3333333 MHz"]
    #[inline]
    pub fn _32mdiv6(self) -> &'a mut W {
        self.variant(MCKFREQW::_32MDIV6)
    }
    #[doc = "32 MHz / 8 = 4.0 MHz"]
    #[inline]
    pub fn _32mdiv8(self) -> &'a mut W {
        self.variant(MCKFREQW::_32MDIV8)
    }
    #[doc = "32 MHz / 10 = 3.2 MHz"]
    #[inline]
    pub fn _32mdiv10(self) -> &'a mut W {
        self.variant(MCKFREQW::_32MDIV10)
    }
    #[doc = "32 MHz / 11 = 2.9090909 MHz"]
    #[inline]
    pub fn _32mdiv11(self) -> &'a mut W {
        self.variant(MCKFREQW::_32MDIV11)
    }
    #[doc = "32 MHz / 15 = 2.1333333 MHz"]
    #[inline]
    pub fn _32mdiv15(self) -> &'a mut W {
        self.variant(MCKFREQW::_32MDIV15)
    }
    #[doc = "32 MHz / 16 = 2.0 MHz"]
    #[inline]
    pub fn _32mdiv16(self) -> &'a mut W {
        self.variant(MCKFREQW::_32MDIV16)
    }
    #[doc = "32 MHz / 21 = 1.5238095"]
    #[inline]
    pub fn _32mdiv21(self) -> &'a mut W {
        self.variant(MCKFREQW::_32MDIV21)
    }
    #[doc = "32 MHz / 23 = 1.3913043 MHz"]
    #[inline]
    pub fn _32mdiv23(self) -> &'a mut W {
        self.variant(MCKFREQW::_32MDIV23)
    }
    #[doc = "32 MHz / 30 = 1.0666667 MHz"]
    #[inline]
    pub fn _32mdiv30(self) -> &'a mut W {
        self.variant(MCKFREQW::_32MDIV30)
    }
    #[doc = "32 MHz / 31 = 1.0322581 MHz"]
    #[inline]
    pub fn _32mdiv31(self) -> &'a mut W {
        self.variant(MCKFREQW::_32MDIV31)
    }
    #[doc = "32 MHz / 32 = 1.0 MHz"]
    #[inline]
    pub fn _32mdiv32(self) -> &'a mut W {
        self.variant(MCKFREQW::_32MDIV32)
    }
    #[doc = "32 MHz / 42 = 0.7619048 MHz"]
    #[inline]
    pub fn _32mdiv42(self) -> &'a mut W {
        self.variant(MCKFREQW::_32MDIV42)
    }
    #[doc = "32 MHz / 63 = 0.5079365 MHz"]
    #[inline]
    pub fn _32mdiv63(self) -> &'a mut W {
        self.variant(MCKFREQW::_32MDIV63)
    }
    #[doc = "32 MHz / 125 = 0.256 MHz"]
    #[inline]
    pub fn _32mdiv125(self) -> &'a mut W {
        self.variant(MCKFREQW::_32MDIV125)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        const MASK: u32 = 4294967295;
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
    #[doc = "Bits 0:31 - Master clock generator frequency."]
    #[inline]
    pub fn mckfreq(&self) -> MCKFREQR {
        MCKFREQR::_from({
            const MASK: u32 = 4294967295;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) as u32
        })
    }
}
impl W {
    #[doc = r" Reset value of the register"]
    #[inline]
    pub fn reset_value() -> W {
        W { bits: 536870912 }
    }
    #[doc = r" Writes raw bits to the register"]
    #[inline]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
    #[doc = "Bits 0:31 - Master clock generator frequency."]
    #[inline]
    pub fn mckfreq(&mut self) -> _MCKFREQW {
        _MCKFREQW { w: self }
    }
}
