#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::ISOSPLIT {
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
#[doc = "Possible values of the field `SPLIT`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SPLITR {
    #[doc = "Full buffer dedicated to either iso IN or OUT"]
    ONEDIR,
    #[doc = "Lower half for IN, upper half for OUT"]
    HALFIN,
    #[doc = r" Reserved"]
    _Reserved(u16),
}
impl SPLITR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u16 {
        match *self {
            SPLITR::ONEDIR => 0,
            SPLITR::HALFIN => 128,
            SPLITR::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u16) -> SPLITR {
        match value {
            0 => SPLITR::ONEDIR,
            128 => SPLITR::HALFIN,
            i => SPLITR::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `ONEDIR`"]
    #[inline]
    pub fn is_one_dir(&self) -> bool {
        *self == SPLITR::ONEDIR
    }
    #[doc = "Checks if the value of the field is `HALFIN`"]
    #[inline]
    pub fn is_half_in(&self) -> bool {
        *self == SPLITR::HALFIN
    }
}
#[doc = "Values that can be written to the field `SPLIT`"]
pub enum SPLITW {
    #[doc = "Full buffer dedicated to either iso IN or OUT"]
    ONEDIR,
    #[doc = "Lower half for IN, upper half for OUT"]
    HALFIN,
}
impl SPLITW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u16 {
        match *self {
            SPLITW::ONEDIR => 0,
            SPLITW::HALFIN => 128,
        }
    }
}
#[doc = r" Proxy"]
pub struct _SPLITW<'a> {
    w: &'a mut W,
}
impl<'a> _SPLITW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: SPLITW) -> &'a mut W {
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = "Full buffer dedicated to either iso IN or OUT"]
    #[inline]
    pub fn one_dir(self) -> &'a mut W {
        self.variant(SPLITW::ONEDIR)
    }
    #[doc = "Lower half for IN, upper half for OUT"]
    #[inline]
    pub fn half_in(self) -> &'a mut W {
        self.variant(SPLITW::HALFIN)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        const MASK: u16 = 65535;
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
    #[doc = "Bits 0:15 - Controls the split of ISO buffers"]
    #[inline]
    pub fn split(&self) -> SPLITR {
        SPLITR::_from({
            const MASK: u16 = 65535;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) as u16
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
    #[doc = "Bits 0:15 - Controls the split of ISO buffers"]
    #[inline]
    pub fn split(&mut self) -> _SPLITW {
        _SPLITW { w: self }
    }
}
