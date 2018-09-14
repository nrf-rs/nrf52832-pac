#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::LOWPOWER {
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
#[doc = "Possible values of the field `LOWPOWER`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum LOWPOWERR {
    #[doc = "Software must write this value to exit low power mode and before performing a remote wake-up"]
    FORCENORMAL,
    #[doc = "Software must write this value to enter low power mode after DMA and software have finished interacting with the USB peripheral"]
    LOWPOWER,
}
impl LOWPOWERR {
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
            LOWPOWERR::FORCENORMAL => false,
            LOWPOWERR::LOWPOWER => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> LOWPOWERR {
        match value {
            false => LOWPOWERR::FORCENORMAL,
            true => LOWPOWERR::LOWPOWER,
        }
    }
    #[doc = "Checks if the value of the field is `FORCENORMAL`"]
    #[inline]
    pub fn is_force_normal(&self) -> bool {
        *self == LOWPOWERR::FORCENORMAL
    }
    #[doc = "Checks if the value of the field is `LOWPOWER`"]
    #[inline]
    pub fn is_low_power(&self) -> bool {
        *self == LOWPOWERR::LOWPOWER
    }
}
#[doc = "Values that can be written to the field `LOWPOWER`"]
pub enum LOWPOWERW {
    #[doc = "Software must write this value to exit low power mode and before performing a remote wake-up"]
    FORCENORMAL,
    #[doc = "Software must write this value to enter low power mode after DMA and software have finished interacting with the USB peripheral"]
    LOWPOWER,
}
impl LOWPOWERW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            LOWPOWERW::FORCENORMAL => false,
            LOWPOWERW::LOWPOWER => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _LOWPOWERW<'a> {
    w: &'a mut W,
}
impl<'a> _LOWPOWERW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: LOWPOWERW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Software must write this value to exit low power mode and before performing a remote wake-up"]
    #[inline]
    pub fn force_normal(self) -> &'a mut W {
        self.variant(LOWPOWERW::FORCENORMAL)
    }
    #[doc = "Software must write this value to enter low power mode after DMA and software have finished interacting with the USB peripheral"]
    #[inline]
    pub fn low_power(self) -> &'a mut W {
        self.variant(LOWPOWERW::LOWPOWER)
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
    #[doc = "Bit 0 - Controls USBD peripheral low-power mode during USB suspend"]
    #[inline]
    pub fn lowpower(&self) -> LOWPOWERR {
        LOWPOWERR::_from({
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
    #[doc = "Bit 0 - Controls USBD peripheral low-power mode during USB suspend"]
    #[inline]
    pub fn lowpower(&mut self) -> _LOWPOWERW {
        _LOWPOWERW { w: self }
    }
}
