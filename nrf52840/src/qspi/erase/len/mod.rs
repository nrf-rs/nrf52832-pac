#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::LEN {
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
#[doc = "Possible values of the field `LEN`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum LENR {
    #[doc = "Erase 4 kB block (flash command 0x20)"]
    _4KB,
    #[doc = "Erase 64 kB block (flash command 0xD8)"]
    _64KB,
    #[doc = "Erase all (flash command 0xC7)"]
    ALL,
    #[doc = r" Reserved"]
    _Reserved(u8),
}
impl LENR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            LENR::_4KB => 0,
            LENR::_64KB => 1,
            LENR::ALL => 2,
            LENR::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> LENR {
        match value {
            0 => LENR::_4KB,
            1 => LENR::_64KB,
            2 => LENR::ALL,
            i => LENR::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `_4KB`"]
    #[inline]
    pub fn is_4kb(&self) -> bool {
        *self == LENR::_4KB
    }
    #[doc = "Checks if the value of the field is `_64KB`"]
    #[inline]
    pub fn is_64kb(&self) -> bool {
        *self == LENR::_64KB
    }
    #[doc = "Checks if the value of the field is `ALL`"]
    #[inline]
    pub fn is_all(&self) -> bool {
        *self == LENR::ALL
    }
}
#[doc = "Values that can be written to the field `LEN`"]
pub enum LENW {
    #[doc = "Erase 4 kB block (flash command 0x20)"]
    _4KB,
    #[doc = "Erase 64 kB block (flash command 0xD8)"]
    _64KB,
    #[doc = "Erase all (flash command 0xC7)"]
    ALL,
}
impl LENW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            LENW::_4KB => 0,
            LENW::_64KB => 1,
            LENW::ALL => 2,
        }
    }
}
#[doc = r" Proxy"]
pub struct _LENW<'a> {
    w: &'a mut W,
}
impl<'a> _LENW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: LENW) -> &'a mut W {
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = "Erase 4 kB block (flash command 0x20)"]
    #[inline]
    pub fn _4kb(self) -> &'a mut W {
        self.variant(LENW::_4KB)
    }
    #[doc = "Erase 64 kB block (flash command 0xD8)"]
    #[inline]
    pub fn _64kb(self) -> &'a mut W {
        self.variant(LENW::_64KB)
    }
    #[doc = "Erase all (flash command 0xC7)"]
    #[inline]
    pub fn all(self) -> &'a mut W {
        self.variant(LENW::ALL)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 3;
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
    #[doc = "Bits 0:1 - LEN"]
    #[inline]
    pub fn len(&self) -> LENR {
        LENR::_from({
            const MASK: u8 = 3;
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
    #[doc = "Bits 0:1 - LEN"]
    #[inline]
    pub fn len(&mut self) -> _LENW {
        _LENW { w: self }
    }
}
