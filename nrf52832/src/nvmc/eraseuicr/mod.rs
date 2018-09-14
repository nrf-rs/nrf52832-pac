#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::ERASEUICR {
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
#[doc = "Possible values of the field `ERASEUICR`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ERASEUICRR {
    #[doc = "No operation"]
    NOOPERATION,
    #[doc = "Start erase of UICR"]
    ERASE,
}
impl ERASEUICRR {
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
            ERASEUICRR::NOOPERATION => false,
            ERASEUICRR::ERASE => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> ERASEUICRR {
        match value {
            false => ERASEUICRR::NOOPERATION,
            true => ERASEUICRR::ERASE,
        }
    }
    #[doc = "Checks if the value of the field is `NOOPERATION`"]
    #[inline]
    pub fn is_no_operation(&self) -> bool {
        *self == ERASEUICRR::NOOPERATION
    }
    #[doc = "Checks if the value of the field is `ERASE`"]
    #[inline]
    pub fn is_erase(&self) -> bool {
        *self == ERASEUICRR::ERASE
    }
}
#[doc = "Values that can be written to the field `ERASEUICR`"]
pub enum ERASEUICRW {
    #[doc = "No operation"]
    NOOPERATION,
    #[doc = "Start erase of UICR"]
    ERASE,
}
impl ERASEUICRW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            ERASEUICRW::NOOPERATION => false,
            ERASEUICRW::ERASE => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _ERASEUICRW<'a> {
    w: &'a mut W,
}
impl<'a> _ERASEUICRW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: ERASEUICRW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "No operation"]
    #[inline]
    pub fn no_operation(self) -> &'a mut W {
        self.variant(ERASEUICRW::NOOPERATION)
    }
    #[doc = "Start erase of UICR"]
    #[inline]
    pub fn erase(self) -> &'a mut W {
        self.variant(ERASEUICRW::ERASE)
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
    #[doc = "Bit 0 - Register starting erase of all User Information Configuration Registers. Note that code erase has to be enabled by CONFIG.EEN before the UICR can be erased."]
    #[inline]
    pub fn eraseuicr(&self) -> ERASEUICRR {
        ERASEUICRR::_from({
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
    #[doc = "Bit 0 - Register starting erase of all User Information Configuration Registers. Note that code erase has to be enabled by CONFIG.EEN before the UICR can be erased."]
    #[inline]
    pub fn eraseuicr(&mut self) -> _ERASEUICRW {
        _ERASEUICRW { w: self }
    }
}
