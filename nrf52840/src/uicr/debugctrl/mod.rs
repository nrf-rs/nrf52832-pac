#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::DEBUGCTRL {
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
#[doc = "Possible values of the field `CPUNIDEN`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CPUNIDENR {
    #[doc = "Enable CPU ITM and ETM functionality (default behavior)"]
    ENABLED,
    #[doc = "Disable CPU ITM and ETM functionality"]
    DISABLED,
    #[doc = r" Reserved"]
    _Reserved(u8),
}
impl CPUNIDENR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            CPUNIDENR::ENABLED => 255,
            CPUNIDENR::DISABLED => 0,
            CPUNIDENR::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> CPUNIDENR {
        match value {
            255 => CPUNIDENR::ENABLED,
            0 => CPUNIDENR::DISABLED,
            i => CPUNIDENR::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline]
    pub fn is_enabled(&self) -> bool {
        *self == CPUNIDENR::ENABLED
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline]
    pub fn is_disabled(&self) -> bool {
        *self == CPUNIDENR::DISABLED
    }
}
#[doc = "Possible values of the field `CPUFPBEN`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CPUFPBENR {
    #[doc = "Enable CPU FPB unit (default behavior)"]
    ENABLED,
    #[doc = "Disable CPU FPB unit. Writes into the FPB registers will be ignored."]
    DISABLED,
    #[doc = r" Reserved"]
    _Reserved(u8),
}
impl CPUFPBENR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            CPUFPBENR::ENABLED => 255,
            CPUFPBENR::DISABLED => 0,
            CPUFPBENR::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> CPUFPBENR {
        match value {
            255 => CPUFPBENR::ENABLED,
            0 => CPUFPBENR::DISABLED,
            i => CPUFPBENR::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline]
    pub fn is_enabled(&self) -> bool {
        *self == CPUFPBENR::ENABLED
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline]
    pub fn is_disabled(&self) -> bool {
        *self == CPUFPBENR::DISABLED
    }
}
#[doc = "Values that can be written to the field `CPUNIDEN`"]
pub enum CPUNIDENW {
    #[doc = "Enable CPU ITM and ETM functionality (default behavior)"]
    ENABLED,
    #[doc = "Disable CPU ITM and ETM functionality"]
    DISABLED,
}
impl CPUNIDENW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            CPUNIDENW::ENABLED => 255,
            CPUNIDENW::DISABLED => 0,
        }
    }
}
#[doc = r" Proxy"]
pub struct _CPUNIDENW<'a> {
    w: &'a mut W,
}
impl<'a> _CPUNIDENW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: CPUNIDENW) -> &'a mut W {
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = "Enable CPU ITM and ETM functionality (default behavior)"]
    #[inline]
    pub fn enabled(self) -> &'a mut W {
        self.variant(CPUNIDENW::ENABLED)
    }
    #[doc = "Disable CPU ITM and ETM functionality"]
    #[inline]
    pub fn disabled(self) -> &'a mut W {
        self.variant(CPUNIDENW::DISABLED)
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
#[doc = "Values that can be written to the field `CPUFPBEN`"]
pub enum CPUFPBENW {
    #[doc = "Enable CPU FPB unit (default behavior)"]
    ENABLED,
    #[doc = "Disable CPU FPB unit. Writes into the FPB registers will be ignored."]
    DISABLED,
}
impl CPUFPBENW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            CPUFPBENW::ENABLED => 255,
            CPUFPBENW::DISABLED => 0,
        }
    }
}
#[doc = r" Proxy"]
pub struct _CPUFPBENW<'a> {
    w: &'a mut W,
}
impl<'a> _CPUFPBENW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: CPUFPBENW) -> &'a mut W {
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = "Enable CPU FPB unit (default behavior)"]
    #[inline]
    pub fn enabled(self) -> &'a mut W {
        self.variant(CPUFPBENW::ENABLED)
    }
    #[doc = "Disable CPU FPB unit. Writes into the FPB registers will be ignored."]
    #[inline]
    pub fn disabled(self) -> &'a mut W {
        self.variant(CPUFPBENW::DISABLED)
    }
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
impl R {
    #[doc = r" Value of the register as raw bits"]
    #[inline]
    pub fn bits(&self) -> u32 {
        self.bits
    }
    #[doc = "Bits 0:7 - Configure CPU non-intrusive debug features"]
    #[inline]
    pub fn cpuniden(&self) -> CPUNIDENR {
        CPUNIDENR::_from({
            const MASK: u8 = 255;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bits 8:15 - Configure CPU flash patch and breakpoint (FPB) unit behavior"]
    #[inline]
    pub fn cpufpben(&self) -> CPUFPBENR {
        CPUFPBENR::_from({
            const MASK: u8 = 255;
            const OFFSET: u8 = 8;
            ((self.bits >> OFFSET) & MASK as u32) as u8
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
    #[doc = "Bits 0:7 - Configure CPU non-intrusive debug features"]
    #[inline]
    pub fn cpuniden(&mut self) -> _CPUNIDENW {
        _CPUNIDENW { w: self }
    }
    #[doc = "Bits 8:15 - Configure CPU flash patch and breakpoint (FPB) unit behavior"]
    #[inline]
    pub fn cpufpben(&mut self) -> _CPUFPBENW {
        _CPUFPBENW { w: self }
    }
}
