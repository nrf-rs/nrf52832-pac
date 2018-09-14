#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::HOST_IOT_KPRTL_LOCK {
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
#[doc = "Possible values of the field `HOST_IOT_KPRTL_LOCK`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum HOST_IOT_KPRTL_LOCKR {
    #[doc = "K_PRTL can be selected for use from register HOST_CRYPTOKEY_SEL"]
    DISABLED,
    #[doc = "K_PRTL has been locked until next power-on reset (POR). If K_PRTL is selected anyway, a zeroed key will be used instead."]
    ENABLED,
}
impl HOST_IOT_KPRTL_LOCKR {
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
            HOST_IOT_KPRTL_LOCKR::DISABLED => false,
            HOST_IOT_KPRTL_LOCKR::ENABLED => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> HOST_IOT_KPRTL_LOCKR {
        match value {
            false => HOST_IOT_KPRTL_LOCKR::DISABLED,
            true => HOST_IOT_KPRTL_LOCKR::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline]
    pub fn is_disabled(&self) -> bool {
        *self == HOST_IOT_KPRTL_LOCKR::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline]
    pub fn is_enabled(&self) -> bool {
        *self == HOST_IOT_KPRTL_LOCKR::ENABLED
    }
}
#[doc = "Values that can be written to the field `HOST_IOT_KPRTL_LOCK`"]
pub enum HOST_IOT_KPRTL_LOCKW {
    #[doc = "K_PRTL can be selected for use from register HOST_CRYPTOKEY_SEL"]
    DISABLED,
    #[doc = "K_PRTL has been locked until next power-on reset (POR). If K_PRTL is selected anyway, a zeroed key will be used instead."]
    ENABLED,
}
impl HOST_IOT_KPRTL_LOCKW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            HOST_IOT_KPRTL_LOCKW::DISABLED => false,
            HOST_IOT_KPRTL_LOCKW::ENABLED => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _HOST_IOT_KPRTL_LOCKW<'a> {
    w: &'a mut W,
}
impl<'a> _HOST_IOT_KPRTL_LOCKW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: HOST_IOT_KPRTL_LOCKW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "K_PRTL can be selected for use from register HOST_CRYPTOKEY_SEL"]
    #[inline]
    pub fn disabled(self) -> &'a mut W {
        self.variant(HOST_IOT_KPRTL_LOCKW::DISABLED)
    }
    #[doc = "K_PRTL has been locked until next power-on reset (POR). If K_PRTL is selected anyway, a zeroed key will be used instead."]
    #[inline]
    pub fn enabled(self) -> &'a mut W {
        self.variant(HOST_IOT_KPRTL_LOCKW::ENABLED)
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
    #[doc = "Bit 0 - This register is the K_PRTL lock register. When this register is set, K_PRTL can not be used and a zeroed key will be used instead. The value of this register is saved in the CRYPTOCELL AO power domain."]
    #[inline]
    pub fn host_iot_kprtl_lock(&self) -> HOST_IOT_KPRTL_LOCKR {
        HOST_IOT_KPRTL_LOCKR::_from({
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
    #[doc = "Bit 0 - This register is the K_PRTL lock register. When this register is set, K_PRTL can not be used and a zeroed key will be used instead. The value of this register is saved in the CRYPTOCELL AO power domain."]
    #[inline]
    pub fn host_iot_kprtl_lock(&mut self) -> _HOST_IOT_KPRTL_LOCKW {
        _HOST_IOT_KPRTL_LOCKW { w: self }
    }
}
