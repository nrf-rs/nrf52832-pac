#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::MODECNF0 {
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
#[doc = "Possible values of the field `RU`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RUR {
    #[doc = "Default ramp-up time (tRXEN), compatible with firmware written for nRF51"]
    DEFAULT,
    #[doc = "Fast ramp-up (tRXEN,FAST), see electrical specification for more information"]
    FAST,
}
impl RUR {
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
            RUR::DEFAULT => false,
            RUR::FAST => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> RUR {
        match value {
            false => RUR::DEFAULT,
            true => RUR::FAST,
        }
    }
    #[doc = "Checks if the value of the field is `DEFAULT`"]
    #[inline]
    pub fn is_default(&self) -> bool {
        *self == RUR::DEFAULT
    }
    #[doc = "Checks if the value of the field is `FAST`"]
    #[inline]
    pub fn is_fast(&self) -> bool {
        *self == RUR::FAST
    }
}
#[doc = "Possible values of the field `DTX`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DTXR {
    #[doc = "Transmit '1'"]
    B1,
    #[doc = "Transmit '0'"]
    B0,
    #[doc = "Transmit center frequency"]
    CENTER,
    #[doc = r" Reserved"]
    _Reserved(u8),
}
impl DTXR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            DTXR::B1 => 0,
            DTXR::B0 => 1,
            DTXR::CENTER => 2,
            DTXR::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> DTXR {
        match value {
            0 => DTXR::B1,
            1 => DTXR::B0,
            2 => DTXR::CENTER,
            i => DTXR::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `B1`"]
    #[inline]
    pub fn is_b1(&self) -> bool {
        *self == DTXR::B1
    }
    #[doc = "Checks if the value of the field is `B0`"]
    #[inline]
    pub fn is_b0(&self) -> bool {
        *self == DTXR::B0
    }
    #[doc = "Checks if the value of the field is `CENTER`"]
    #[inline]
    pub fn is_center(&self) -> bool {
        *self == DTXR::CENTER
    }
}
#[doc = "Values that can be written to the field `RU`"]
pub enum RUW {
    #[doc = "Default ramp-up time (tRXEN), compatible with firmware written for nRF51"]
    DEFAULT,
    #[doc = "Fast ramp-up (tRXEN,FAST), see electrical specification for more information"]
    FAST,
}
impl RUW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            RUW::DEFAULT => false,
            RUW::FAST => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _RUW<'a> {
    w: &'a mut W,
}
impl<'a> _RUW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: RUW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Default ramp-up time (tRXEN), compatible with firmware written for nRF51"]
    #[inline]
    pub fn default(self) -> &'a mut W {
        self.variant(RUW::DEFAULT)
    }
    #[doc = "Fast ramp-up (tRXEN,FAST), see electrical specification for more information"]
    #[inline]
    pub fn fast(self) -> &'a mut W {
        self.variant(RUW::FAST)
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
        const OFFSET: u8 = 0;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `DTX`"]
pub enum DTXW {
    #[doc = "Transmit '1'"]
    B1,
    #[doc = "Transmit '0'"]
    B0,
    #[doc = "Transmit center frequency"]
    CENTER,
}
impl DTXW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            DTXW::B1 => 0,
            DTXW::B0 => 1,
            DTXW::CENTER => 2,
        }
    }
}
#[doc = r" Proxy"]
pub struct _DTXW<'a> {
    w: &'a mut W,
}
impl<'a> _DTXW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: DTXW) -> &'a mut W {
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = "Transmit '1'"]
    #[inline]
    pub fn b1(self) -> &'a mut W {
        self.variant(DTXW::B1)
    }
    #[doc = "Transmit '0'"]
    #[inline]
    pub fn b0(self) -> &'a mut W {
        self.variant(DTXW::B0)
    }
    #[doc = "Transmit center frequency"]
    #[inline]
    pub fn center(self) -> &'a mut W {
        self.variant(DTXW::CENTER)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 3;
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
    #[doc = "Bit 0 - Radio ramp-up time"]
    #[inline]
    pub fn ru(&self) -> RUR {
        RUR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bits 8:9 - Default TX value"]
    #[inline]
    pub fn dtx(&self) -> DTXR {
        DTXR::_from({
            const MASK: u8 = 3;
            const OFFSET: u8 = 8;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
}
impl W {
    #[doc = r" Reset value of the register"]
    #[inline]
    pub fn reset_value() -> W {
        W { bits: 512 }
    }
    #[doc = r" Writes raw bits to the register"]
    #[inline]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
    #[doc = "Bit 0 - Radio ramp-up time"]
    #[inline]
    pub fn ru(&mut self) -> _RUW {
        _RUW { w: self }
    }
    #[doc = "Bits 8:9 - Default TX value"]
    #[inline]
    pub fn dtx(&mut self) -> _DTXW {
        _DTXW { w: self }
    }
}
