#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::TEST {
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
#[doc = "Possible values of the field `CONSTCARRIER`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CONSTCARRIERR {
    #[doc = "Constant carrier disabled."]
    DISABLED,
    #[doc = "Constant carrier enabled."]
    ENABLED,
}
impl CONSTCARRIERR {
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
            CONSTCARRIERR::DISABLED => false,
            CONSTCARRIERR::ENABLED => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> CONSTCARRIERR {
        match value {
            false => CONSTCARRIERR::DISABLED,
            true => CONSTCARRIERR::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline]
    pub fn is_disabled(&self) -> bool {
        *self == CONSTCARRIERR::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline]
    pub fn is_enabled(&self) -> bool {
        *self == CONSTCARRIERR::ENABLED
    }
}
#[doc = "Possible values of the field `PLLLOCK`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PLLLOCKR {
    #[doc = "PLL lock disabled."]
    DISABLED,
    #[doc = "PLL lock enabled."]
    ENABLED,
}
impl PLLLOCKR {
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
            PLLLOCKR::DISABLED => false,
            PLLLOCKR::ENABLED => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> PLLLOCKR {
        match value {
            false => PLLLOCKR::DISABLED,
            true => PLLLOCKR::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline]
    pub fn is_disabled(&self) -> bool {
        *self == PLLLOCKR::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline]
    pub fn is_enabled(&self) -> bool {
        *self == PLLLOCKR::ENABLED
    }
}
#[doc = "Values that can be written to the field `CONSTCARRIER`"]
pub enum CONSTCARRIERW {
    #[doc = "Constant carrier disabled."]
    DISABLED,
    #[doc = "Constant carrier enabled."]
    ENABLED,
}
impl CONSTCARRIERW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            CONSTCARRIERW::DISABLED => false,
            CONSTCARRIERW::ENABLED => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _CONSTCARRIERW<'a> {
    w: &'a mut W,
}
impl<'a> _CONSTCARRIERW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: CONSTCARRIERW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Constant carrier disabled."]
    #[inline]
    pub fn disabled(self) -> &'a mut W {
        self.variant(CONSTCARRIERW::DISABLED)
    }
    #[doc = "Constant carrier enabled."]
    #[inline]
    pub fn enabled(self) -> &'a mut W {
        self.variant(CONSTCARRIERW::ENABLED)
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
#[doc = "Values that can be written to the field `PLLLOCK`"]
pub enum PLLLOCKW {
    #[doc = "PLL lock disabled."]
    DISABLED,
    #[doc = "PLL lock enabled."]
    ENABLED,
}
impl PLLLOCKW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            PLLLOCKW::DISABLED => false,
            PLLLOCKW::ENABLED => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _PLLLOCKW<'a> {
    w: &'a mut W,
}
impl<'a> _PLLLOCKW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: PLLLOCKW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "PLL lock disabled."]
    #[inline]
    pub fn disabled(self) -> &'a mut W {
        self.variant(PLLLOCKW::DISABLED)
    }
    #[doc = "PLL lock enabled."]
    #[inline]
    pub fn enabled(self) -> &'a mut W {
        self.variant(PLLLOCKW::ENABLED)
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
        const OFFSET: u8 = 1;
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
    #[doc = "Bit 0 - Constant carrier. Decision point: TXEN task."]
    #[inline]
    pub fn constcarrier(&self) -> CONSTCARRIERR {
        CONSTCARRIERR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 1 - PLL lock. Decision point: TXEN or RXEN task."]
    #[inline]
    pub fn plllock(&self) -> PLLLOCKR {
        PLLLOCKR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 1;
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
    #[doc = "Bit 0 - Constant carrier. Decision point: TXEN task."]
    #[inline]
    pub fn constcarrier(&mut self) -> _CONSTCARRIERW {
        _CONSTCARRIERW { w: self }
    }
    #[doc = "Bit 1 - PLL lock. Decision point: TXEN or RXEN task."]
    #[inline]
    pub fn plllock(&mut self) -> _PLLLOCKW {
        _PLLLOCKW { w: self }
    }
}
