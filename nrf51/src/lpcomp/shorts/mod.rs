#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::SHORTS {
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
#[doc = "Possible values of the field `READY_SAMPLE`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum READY_SAMPLER {
    #[doc = "Shortcut disabled."]
    DISABLED,
    #[doc = "Shortcut enabled."]
    ENABLED,
}
impl READY_SAMPLER {
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
            READY_SAMPLER::DISABLED => false,
            READY_SAMPLER::ENABLED => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> READY_SAMPLER {
        match value {
            false => READY_SAMPLER::DISABLED,
            true => READY_SAMPLER::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline]
    pub fn is_disabled(&self) -> bool {
        *self == READY_SAMPLER::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline]
    pub fn is_enabled(&self) -> bool {
        *self == READY_SAMPLER::ENABLED
    }
}
#[doc = "Possible values of the field `READY_STOP`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum READY_STOPR {
    #[doc = "Shortcut disabled."]
    DISABLED,
    #[doc = "Shortcut enabled."]
    ENABLED,
}
impl READY_STOPR {
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
            READY_STOPR::DISABLED => false,
            READY_STOPR::ENABLED => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> READY_STOPR {
        match value {
            false => READY_STOPR::DISABLED,
            true => READY_STOPR::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline]
    pub fn is_disabled(&self) -> bool {
        *self == READY_STOPR::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline]
    pub fn is_enabled(&self) -> bool {
        *self == READY_STOPR::ENABLED
    }
}
#[doc = "Possible values of the field `DOWN_STOP`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DOWN_STOPR {
    #[doc = "Shortcut disabled."]
    DISABLED,
    #[doc = "Shortcut enabled."]
    ENABLED,
}
impl DOWN_STOPR {
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
            DOWN_STOPR::DISABLED => false,
            DOWN_STOPR::ENABLED => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> DOWN_STOPR {
        match value {
            false => DOWN_STOPR::DISABLED,
            true => DOWN_STOPR::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline]
    pub fn is_disabled(&self) -> bool {
        *self == DOWN_STOPR::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline]
    pub fn is_enabled(&self) -> bool {
        *self == DOWN_STOPR::ENABLED
    }
}
#[doc = "Possible values of the field `UP_STOP`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum UP_STOPR {
    #[doc = "Shortcut disabled."]
    DISABLED,
    #[doc = "Shortcut enabled."]
    ENABLED,
}
impl UP_STOPR {
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
            UP_STOPR::DISABLED => false,
            UP_STOPR::ENABLED => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> UP_STOPR {
        match value {
            false => UP_STOPR::DISABLED,
            true => UP_STOPR::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline]
    pub fn is_disabled(&self) -> bool {
        *self == UP_STOPR::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline]
    pub fn is_enabled(&self) -> bool {
        *self == UP_STOPR::ENABLED
    }
}
#[doc = "Possible values of the field `CROSS_STOP`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CROSS_STOPR {
    #[doc = "Shortcut disabled."]
    DISABLED,
    #[doc = "Shortcut enabled."]
    ENABLED,
}
impl CROSS_STOPR {
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
            CROSS_STOPR::DISABLED => false,
            CROSS_STOPR::ENABLED => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> CROSS_STOPR {
        match value {
            false => CROSS_STOPR::DISABLED,
            true => CROSS_STOPR::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline]
    pub fn is_disabled(&self) -> bool {
        *self == CROSS_STOPR::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline]
    pub fn is_enabled(&self) -> bool {
        *self == CROSS_STOPR::ENABLED
    }
}
#[doc = "Values that can be written to the field `READY_SAMPLE`"]
pub enum READY_SAMPLEW {
    #[doc = "Shortcut disabled."]
    DISABLED,
    #[doc = "Shortcut enabled."]
    ENABLED,
}
impl READY_SAMPLEW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            READY_SAMPLEW::DISABLED => false,
            READY_SAMPLEW::ENABLED => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _READY_SAMPLEW<'a> {
    w: &'a mut W,
}
impl<'a> _READY_SAMPLEW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: READY_SAMPLEW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Shortcut disabled."]
    #[inline]
    pub fn disabled(self) -> &'a mut W {
        self.variant(READY_SAMPLEW::DISABLED)
    }
    #[doc = "Shortcut enabled."]
    #[inline]
    pub fn enabled(self) -> &'a mut W {
        self.variant(READY_SAMPLEW::ENABLED)
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
#[doc = "Values that can be written to the field `READY_STOP`"]
pub enum READY_STOPW {
    #[doc = "Shortcut disabled."]
    DISABLED,
    #[doc = "Shortcut enabled."]
    ENABLED,
}
impl READY_STOPW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            READY_STOPW::DISABLED => false,
            READY_STOPW::ENABLED => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _READY_STOPW<'a> {
    w: &'a mut W,
}
impl<'a> _READY_STOPW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: READY_STOPW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Shortcut disabled."]
    #[inline]
    pub fn disabled(self) -> &'a mut W {
        self.variant(READY_STOPW::DISABLED)
    }
    #[doc = "Shortcut enabled."]
    #[inline]
    pub fn enabled(self) -> &'a mut W {
        self.variant(READY_STOPW::ENABLED)
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
#[doc = "Values that can be written to the field `DOWN_STOP`"]
pub enum DOWN_STOPW {
    #[doc = "Shortcut disabled."]
    DISABLED,
    #[doc = "Shortcut enabled."]
    ENABLED,
}
impl DOWN_STOPW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            DOWN_STOPW::DISABLED => false,
            DOWN_STOPW::ENABLED => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _DOWN_STOPW<'a> {
    w: &'a mut W,
}
impl<'a> _DOWN_STOPW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: DOWN_STOPW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Shortcut disabled."]
    #[inline]
    pub fn disabled(self) -> &'a mut W {
        self.variant(DOWN_STOPW::DISABLED)
    }
    #[doc = "Shortcut enabled."]
    #[inline]
    pub fn enabled(self) -> &'a mut W {
        self.variant(DOWN_STOPW::ENABLED)
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
        const OFFSET: u8 = 2;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `UP_STOP`"]
pub enum UP_STOPW {
    #[doc = "Shortcut disabled."]
    DISABLED,
    #[doc = "Shortcut enabled."]
    ENABLED,
}
impl UP_STOPW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            UP_STOPW::DISABLED => false,
            UP_STOPW::ENABLED => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _UP_STOPW<'a> {
    w: &'a mut W,
}
impl<'a> _UP_STOPW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: UP_STOPW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Shortcut disabled."]
    #[inline]
    pub fn disabled(self) -> &'a mut W {
        self.variant(UP_STOPW::DISABLED)
    }
    #[doc = "Shortcut enabled."]
    #[inline]
    pub fn enabled(self) -> &'a mut W {
        self.variant(UP_STOPW::ENABLED)
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
        const OFFSET: u8 = 3;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `CROSS_STOP`"]
pub enum CROSS_STOPW {
    #[doc = "Shortcut disabled."]
    DISABLED,
    #[doc = "Shortcut enabled."]
    ENABLED,
}
impl CROSS_STOPW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            CROSS_STOPW::DISABLED => false,
            CROSS_STOPW::ENABLED => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _CROSS_STOPW<'a> {
    w: &'a mut W,
}
impl<'a> _CROSS_STOPW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: CROSS_STOPW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Shortcut disabled."]
    #[inline]
    pub fn disabled(self) -> &'a mut W {
        self.variant(CROSS_STOPW::DISABLED)
    }
    #[doc = "Shortcut enabled."]
    #[inline]
    pub fn enabled(self) -> &'a mut W {
        self.variant(CROSS_STOPW::ENABLED)
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
        const OFFSET: u8 = 4;
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
    #[doc = "Bit 0 - Shortcut between READY event and SAMPLE task."]
    #[inline]
    pub fn ready_sample(&self) -> READY_SAMPLER {
        READY_SAMPLER::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 1 - Shortcut between RADY event and STOP task."]
    #[inline]
    pub fn ready_stop(&self) -> READY_STOPR {
        READY_STOPR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 1;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 2 - Shortcut between DOWN event and STOP task."]
    #[inline]
    pub fn down_stop(&self) -> DOWN_STOPR {
        DOWN_STOPR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 2;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 3 - Shortcut between UP event and STOP task."]
    #[inline]
    pub fn up_stop(&self) -> UP_STOPR {
        UP_STOPR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 3;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 4 - Shortcut between CROSS event and STOP task."]
    #[inline]
    pub fn cross_stop(&self) -> CROSS_STOPR {
        CROSS_STOPR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 4;
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
    #[doc = "Bit 0 - Shortcut between READY event and SAMPLE task."]
    #[inline]
    pub fn ready_sample(&mut self) -> _READY_SAMPLEW {
        _READY_SAMPLEW { w: self }
    }
    #[doc = "Bit 1 - Shortcut between RADY event and STOP task."]
    #[inline]
    pub fn ready_stop(&mut self) -> _READY_STOPW {
        _READY_STOPW { w: self }
    }
    #[doc = "Bit 2 - Shortcut between DOWN event and STOP task."]
    #[inline]
    pub fn down_stop(&mut self) -> _DOWN_STOPW {
        _DOWN_STOPW { w: self }
    }
    #[doc = "Bit 3 - Shortcut between UP event and STOP task."]
    #[inline]
    pub fn up_stop(&mut self) -> _UP_STOPW {
        _UP_STOPW { w: self }
    }
    #[doc = "Bit 4 - Shortcut between CROSS event and STOP task."]
    #[inline]
    pub fn cross_stop(&mut self) -> _CROSS_STOPW {
        _CROSS_STOPW { w: self }
    }
}
