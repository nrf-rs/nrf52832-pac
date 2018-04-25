#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::LFCLKSRC {
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
#[doc = "Possible values of the field `SRC`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SRCR {
    #[doc = "32.768 kHz RC oscillator"]
    RC,
    #[doc = "32.768 kHz crystal oscillator"]
    XTAL,
    #[doc = "32.768 kHz synthesized from HFCLK"]
    SYNTH,
    #[doc = r" Reserved"]
    _Reserved(u8),
}
impl SRCR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            SRCR::RC => 0,
            SRCR::XTAL => 1,
            SRCR::SYNTH => 2,
            SRCR::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> SRCR {
        match value {
            0 => SRCR::RC,
            1 => SRCR::XTAL,
            2 => SRCR::SYNTH,
            i => SRCR::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `RC`"]
    #[inline]
    pub fn is_rc(&self) -> bool {
        *self == SRCR::RC
    }
    #[doc = "Checks if the value of the field is `XTAL`"]
    #[inline]
    pub fn is_xtal(&self) -> bool {
        *self == SRCR::XTAL
    }
    #[doc = "Checks if the value of the field is `SYNTH`"]
    #[inline]
    pub fn is_synth(&self) -> bool {
        *self == SRCR::SYNTH
    }
}
#[doc = "Possible values of the field `BYPASS`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum BYPASSR {
    #[doc = "Disable (use with Xtal or low-swing external source)"]
    DISABLED,
    #[doc = "Enable (use with rail-to-rail external source)"]
    ENABLED,
}
impl BYPASSR {
    #[doc = r" Returns `true` if the bit is clear (0)"]
    #[inline]
    pub fn bit_is_clear(&self) -> bool {
        !self.bit()
    }
    #[doc = r" Returns `true` if the bit is set (1)"]
    #[inline]
    pub fn bit_is_set(&self) -> bool {
        self.bit()
    }
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bit(&self) -> bool {
        match *self {
            BYPASSR::DISABLED => false,
            BYPASSR::ENABLED => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> BYPASSR {
        match value {
            false => BYPASSR::DISABLED,
            true => BYPASSR::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline]
    pub fn is_disabled(&self) -> bool {
        *self == BYPASSR::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline]
    pub fn is_enabled(&self) -> bool {
        *self == BYPASSR::ENABLED
    }
}
#[doc = "Possible values of the field `EXTERNAL`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum EXTERNALR {
    #[doc = "Disable external source (use with Xtal)"]
    DISABLED,
    #[doc = "Enable use of external source instead of Xtal (SRC needs to be set to Xtal)"]
    ENABLED,
}
impl EXTERNALR {
    #[doc = r" Returns `true` if the bit is clear (0)"]
    #[inline]
    pub fn bit_is_clear(&self) -> bool {
        !self.bit()
    }
    #[doc = r" Returns `true` if the bit is set (1)"]
    #[inline]
    pub fn bit_is_set(&self) -> bool {
        self.bit()
    }
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bit(&self) -> bool {
        match *self {
            EXTERNALR::DISABLED => false,
            EXTERNALR::ENABLED => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> EXTERNALR {
        match value {
            false => EXTERNALR::DISABLED,
            true => EXTERNALR::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline]
    pub fn is_disabled(&self) -> bool {
        *self == EXTERNALR::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline]
    pub fn is_enabled(&self) -> bool {
        *self == EXTERNALR::ENABLED
    }
}
#[doc = "Values that can be written to the field `SRC`"]
pub enum SRCW {
    #[doc = "32.768 kHz RC oscillator"]
    RC,
    #[doc = "32.768 kHz crystal oscillator"]
    XTAL,
    #[doc = "32.768 kHz synthesized from HFCLK"]
    SYNTH,
}
impl SRCW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            SRCW::RC => 0,
            SRCW::XTAL => 1,
            SRCW::SYNTH => 2,
        }
    }
}
#[doc = r" Proxy"]
pub struct _SRCW<'a> {
    w: &'a mut W,
}
impl<'a> _SRCW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: SRCW) -> &'a mut W {
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = "32.768 kHz RC oscillator"]
    #[inline]
    pub fn rc(self) -> &'a mut W {
        self.variant(SRCW::RC)
    }
    #[doc = "32.768 kHz crystal oscillator"]
    #[inline]
    pub fn xtal(self) -> &'a mut W {
        self.variant(SRCW::XTAL)
    }
    #[doc = "32.768 kHz synthesized from HFCLK"]
    #[inline]
    pub fn synth(self) -> &'a mut W {
        self.variant(SRCW::SYNTH)
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
#[doc = "Values that can be written to the field `BYPASS`"]
pub enum BYPASSW {
    #[doc = "Disable (use with Xtal or low-swing external source)"]
    DISABLED,
    #[doc = "Enable (use with rail-to-rail external source)"]
    ENABLED,
}
impl BYPASSW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            BYPASSW::DISABLED => false,
            BYPASSW::ENABLED => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _BYPASSW<'a> {
    w: &'a mut W,
}
impl<'a> _BYPASSW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: BYPASSW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Disable (use with Xtal or low-swing external source)"]
    #[inline]
    pub fn disabled(self) -> &'a mut W {
        self.variant(BYPASSW::DISABLED)
    }
    #[doc = "Enable (use with rail-to-rail external source)"]
    #[inline]
    pub fn enabled(self) -> &'a mut W {
        self.variant(BYPASSW::ENABLED)
    }
    #[doc = r" Sets the field bit"]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r" Clears the field bit"]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bit(self, value: bool) -> &'a mut W {
        const MASK: bool = true;
        const OFFSET: u8 = 16;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `EXTERNAL`"]
pub enum EXTERNALW {
    #[doc = "Disable external source (use with Xtal)"]
    DISABLED,
    #[doc = "Enable use of external source instead of Xtal (SRC needs to be set to Xtal)"]
    ENABLED,
}
impl EXTERNALW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            EXTERNALW::DISABLED => false,
            EXTERNALW::ENABLED => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _EXTERNALW<'a> {
    w: &'a mut W,
}
impl<'a> _EXTERNALW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: EXTERNALW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Disable external source (use with Xtal)"]
    #[inline]
    pub fn disabled(self) -> &'a mut W {
        self.variant(EXTERNALW::DISABLED)
    }
    #[doc = "Enable use of external source instead of Xtal (SRC needs to be set to Xtal)"]
    #[inline]
    pub fn enabled(self) -> &'a mut W {
        self.variant(EXTERNALW::ENABLED)
    }
    #[doc = r" Sets the field bit"]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r" Clears the field bit"]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bit(self, value: bool) -> &'a mut W {
        const MASK: bool = true;
        const OFFSET: u8 = 17;
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
    #[doc = "Bits 0:1 - Clock source"]
    #[inline]
    pub fn src(&self) -> SRCR {
        SRCR::_from({
            const MASK: u8 = 3;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bit 16 - Enable or disable bypass of LFCLK crystal oscillator with external clock source"]
    #[inline]
    pub fn bypass(&self) -> BYPASSR {
        BYPASSR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 16;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 17 - Enable or disable external source for LFCLK"]
    #[inline]
    pub fn external(&self) -> EXTERNALR {
        EXTERNALR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 17;
            ((self.bits >> OFFSET) & MASK as u32) != 0
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
    #[doc = "Bits 0:1 - Clock source"]
    #[inline]
    pub fn src(&mut self) -> _SRCW {
        _SRCW { w: self }
    }
    #[doc = "Bit 16 - Enable or disable bypass of LFCLK crystal oscillator with external clock source"]
    #[inline]
    pub fn bypass(&mut self) -> _BYPASSW {
        _BYPASSW { w: self }
    }
    #[doc = "Bit 17 - Enable or disable external source for LFCLK"]
    #[inline]
    pub fn external(&mut self) -> _EXTERNALW {
        _EXTERNALW { w: self }
    }
}
