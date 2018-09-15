#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::ENABLE {
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
#[doc = "Possible values of the field `ENABLE`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ENABLER {
    #[doc = "CCM is disabled."]
    DISABLED,
    #[doc = "CCM is enabled."]
    ENABLED,
    #[doc = r" Reserved"]
    _Reserved(u8),
}
impl ENABLER {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            ENABLER::DISABLED => 0,
            ENABLER::ENABLED => 2,
            ENABLER::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> ENABLER {
        match value {
            0 => ENABLER::DISABLED,
            2 => ENABLER::ENABLED,
            i => ENABLER::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline]
    pub fn is_disabled(&self) -> bool {
        *self == ENABLER::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline]
    pub fn is_enabled(&self) -> bool {
        *self == ENABLER::ENABLED
    }
}
#[doc = "Values that can be written to the field `ENABLE`"]
pub enum ENABLEW {
    #[doc = "CCM is disabled."]
    DISABLED,
    #[doc = "CCM is enabled."]
    ENABLED,
}
impl ENABLEW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            ENABLEW::DISABLED => 0,
            ENABLEW::ENABLED => 2,
        }
    }
}
#[doc = r" Proxy"]
pub struct _ENABLEW<'a> {
    w: &'a mut W,
}
impl<'a> _ENABLEW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: ENABLEW) -> &'a mut W {
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = "CCM is disabled."]
    #[inline]
    pub fn disabled(self) -> &'a mut W {
        self.variant(ENABLEW::DISABLED)
    }
    #[doc = "CCM is enabled."]
    #[inline]
    pub fn enabled(self) -> &'a mut W {
        self.variant(ENABLEW::ENABLED)
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
    #[doc = "Bits 0:1 - CCM enable."]
    #[inline]
    pub fn enable(&self) -> ENABLER {
        ENABLER::_from({
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
    #[doc = "Bits 0:1 - CCM enable."]
    #[inline]
    pub fn enable(&mut self) -> _ENABLEW {
        _ENABLEW { w: self }
    }
}
