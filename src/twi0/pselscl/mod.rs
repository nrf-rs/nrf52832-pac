#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::PSELSCL {
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
#[doc = "Possible values of the field `PSELSCL`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PSELSCLR {
    #[doc = "Disconnect"]
    DISCONNECTED,
    #[doc = r" Reserved"]
    _Reserved(u32),
}
impl PSELSCLR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u32 {
        match *self {
            PSELSCLR::DISCONNECTED => 4294967295,
            PSELSCLR::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u32) -> PSELSCLR {
        match value {
            4294967295 => PSELSCLR::DISCONNECTED,
            i => PSELSCLR::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `DISCONNECTED`"]
    #[inline]
    pub fn is_disconnected(&self) -> bool {
        *self == PSELSCLR::DISCONNECTED
    }
}
#[doc = "Values that can be written to the field `PSELSCL`"]
pub enum PSELSCLW {
    #[doc = "Disconnect"]
    DISCONNECTED,
}
impl PSELSCLW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u32 {
        match *self {
            PSELSCLW::DISCONNECTED => 4294967295,
        }
    }
}
#[doc = r" Proxy"]
pub struct _PSELSCLW<'a> {
    w: &'a mut W,
}
impl<'a> _PSELSCLW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: PSELSCLW) -> &'a mut W {
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = "Disconnect"]
    #[inline]
    pub fn disconnected(self) -> &'a mut W {
        self.variant(PSELSCLW::DISCONNECTED)
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
    #[doc = "Bits 0:31 - Pin number configuration for TWI SCL signal"]
    #[inline]
    pub fn pselscl(&self) -> PSELSCLR {
        PSELSCLR::_from({
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
        W { bits: 4294967295 }
    }
    #[doc = r" Writes raw bits to the register"]
    #[inline]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
    #[doc = "Bits 0:31 - Pin number configuration for TWI SCL signal"]
    #[inline]
    pub fn pselscl(&mut self) -> _PSELSCLW {
        _PSELSCLW { w: self }
    }
}
