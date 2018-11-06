#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::EXTREFSEL {
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
#[doc = "Possible values of the field `EXTREFSEL`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum EXTREFSELR {
    #[doc = "Use AIN0 as external analog reference"]
    ANALOGREFERENCE0,
    #[doc = "Use AIN1 as external analog reference"]
    ANALOGREFERENCE1,
}
impl EXTREFSELR {
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
            EXTREFSELR::ANALOGREFERENCE0 => false,
            EXTREFSELR::ANALOGREFERENCE1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> EXTREFSELR {
        match value {
            false => EXTREFSELR::ANALOGREFERENCE0,
            true => EXTREFSELR::ANALOGREFERENCE1,
        }
    }
    #[doc = "Checks if the value of the field is `ANALOGREFERENCE0`"]
    #[inline]
    pub fn is_analog_reference0(&self) -> bool {
        *self == EXTREFSELR::ANALOGREFERENCE0
    }
    #[doc = "Checks if the value of the field is `ANALOGREFERENCE1`"]
    #[inline]
    pub fn is_analog_reference1(&self) -> bool {
        *self == EXTREFSELR::ANALOGREFERENCE1
    }
}
#[doc = "Values that can be written to the field `EXTREFSEL`"]
pub enum EXTREFSELW {
    #[doc = "Use AIN0 as external analog reference"]
    ANALOGREFERENCE0,
    #[doc = "Use AIN1 as external analog reference"]
    ANALOGREFERENCE1,
}
impl EXTREFSELW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            EXTREFSELW::ANALOGREFERENCE0 => false,
            EXTREFSELW::ANALOGREFERENCE1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _EXTREFSELW<'a> {
    w: &'a mut W,
}
impl<'a> _EXTREFSELW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: EXTREFSELW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Use AIN0 as external analog reference"]
    #[inline]
    pub fn analog_reference0(self) -> &'a mut W {
        self.variant(EXTREFSELW::ANALOGREFERENCE0)
    }
    #[doc = "Use AIN1 as external analog reference"]
    #[inline]
    pub fn analog_reference1(self) -> &'a mut W {
        self.variant(EXTREFSELW::ANALOGREFERENCE1)
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
impl R {
    #[doc = r" Value of the register as raw bits"]
    #[inline]
    pub fn bits(&self) -> u32 {
        self.bits
    }
    #[doc = "Bit 0 - External analog reference select"]
    #[inline]
    pub fn extrefsel(&self) -> EXTREFSELR {
        EXTREFSELR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 0;
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
    #[doc = "Bit 0 - External analog reference select"]
    #[inline]
    pub fn extrefsel(&mut self) -> _EXTREFSELW {
        _EXTREFSELW { w: self }
    }
}
