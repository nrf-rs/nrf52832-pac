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
#[doc = "Possible values of the field `SEQEND0_STOP`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SEQEND0_STOPR {
    #[doc = "Disable shortcut"]
    DISABLED,
    #[doc = "Enable shortcut"]
    ENABLED,
}
impl SEQEND0_STOPR {
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
            SEQEND0_STOPR::DISABLED => false,
            SEQEND0_STOPR::ENABLED => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> SEQEND0_STOPR {
        match value {
            false => SEQEND0_STOPR::DISABLED,
            true => SEQEND0_STOPR::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline]
    pub fn is_disabled(&self) -> bool {
        *self == SEQEND0_STOPR::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline]
    pub fn is_enabled(&self) -> bool {
        *self == SEQEND0_STOPR::ENABLED
    }
}
#[doc = "Possible values of the field `SEQEND1_STOP`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SEQEND1_STOPR {
    #[doc = "Disable shortcut"]
    DISABLED,
    #[doc = "Enable shortcut"]
    ENABLED,
}
impl SEQEND1_STOPR {
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
            SEQEND1_STOPR::DISABLED => false,
            SEQEND1_STOPR::ENABLED => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> SEQEND1_STOPR {
        match value {
            false => SEQEND1_STOPR::DISABLED,
            true => SEQEND1_STOPR::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline]
    pub fn is_disabled(&self) -> bool {
        *self == SEQEND1_STOPR::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline]
    pub fn is_enabled(&self) -> bool {
        *self == SEQEND1_STOPR::ENABLED
    }
}
#[doc = "Possible values of the field `LOOPSDONE_SEQSTART0`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum LOOPSDONE_SEQSTART0R {
    #[doc = "Disable shortcut"]
    DISABLED,
    #[doc = "Enable shortcut"]
    ENABLED,
}
impl LOOPSDONE_SEQSTART0R {
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
            LOOPSDONE_SEQSTART0R::DISABLED => false,
            LOOPSDONE_SEQSTART0R::ENABLED => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> LOOPSDONE_SEQSTART0R {
        match value {
            false => LOOPSDONE_SEQSTART0R::DISABLED,
            true => LOOPSDONE_SEQSTART0R::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline]
    pub fn is_disabled(&self) -> bool {
        *self == LOOPSDONE_SEQSTART0R::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline]
    pub fn is_enabled(&self) -> bool {
        *self == LOOPSDONE_SEQSTART0R::ENABLED
    }
}
#[doc = "Possible values of the field `LOOPSDONE_SEQSTART1`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum LOOPSDONE_SEQSTART1R {
    #[doc = "Disable shortcut"]
    DISABLED,
    #[doc = "Enable shortcut"]
    ENABLED,
}
impl LOOPSDONE_SEQSTART1R {
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
            LOOPSDONE_SEQSTART1R::DISABLED => false,
            LOOPSDONE_SEQSTART1R::ENABLED => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> LOOPSDONE_SEQSTART1R {
        match value {
            false => LOOPSDONE_SEQSTART1R::DISABLED,
            true => LOOPSDONE_SEQSTART1R::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline]
    pub fn is_disabled(&self) -> bool {
        *self == LOOPSDONE_SEQSTART1R::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline]
    pub fn is_enabled(&self) -> bool {
        *self == LOOPSDONE_SEQSTART1R::ENABLED
    }
}
#[doc = "Possible values of the field `LOOPSDONE_STOP`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum LOOPSDONE_STOPR {
    #[doc = "Disable shortcut"]
    DISABLED,
    #[doc = "Enable shortcut"]
    ENABLED,
}
impl LOOPSDONE_STOPR {
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
            LOOPSDONE_STOPR::DISABLED => false,
            LOOPSDONE_STOPR::ENABLED => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> LOOPSDONE_STOPR {
        match value {
            false => LOOPSDONE_STOPR::DISABLED,
            true => LOOPSDONE_STOPR::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline]
    pub fn is_disabled(&self) -> bool {
        *self == LOOPSDONE_STOPR::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline]
    pub fn is_enabled(&self) -> bool {
        *self == LOOPSDONE_STOPR::ENABLED
    }
}
#[doc = "Values that can be written to the field `SEQEND0_STOP`"]
pub enum SEQEND0_STOPW {
    #[doc = "Disable shortcut"]
    DISABLED,
    #[doc = "Enable shortcut"]
    ENABLED,
}
impl SEQEND0_STOPW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            SEQEND0_STOPW::DISABLED => false,
            SEQEND0_STOPW::ENABLED => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _SEQEND0_STOPW<'a> {
    w: &'a mut W,
}
impl<'a> _SEQEND0_STOPW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: SEQEND0_STOPW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Disable shortcut"]
    #[inline]
    pub fn disabled(self) -> &'a mut W {
        self.variant(SEQEND0_STOPW::DISABLED)
    }
    #[doc = "Enable shortcut"]
    #[inline]
    pub fn enabled(self) -> &'a mut W {
        self.variant(SEQEND0_STOPW::ENABLED)
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
#[doc = "Values that can be written to the field `SEQEND1_STOP`"]
pub enum SEQEND1_STOPW {
    #[doc = "Disable shortcut"]
    DISABLED,
    #[doc = "Enable shortcut"]
    ENABLED,
}
impl SEQEND1_STOPW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            SEQEND1_STOPW::DISABLED => false,
            SEQEND1_STOPW::ENABLED => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _SEQEND1_STOPW<'a> {
    w: &'a mut W,
}
impl<'a> _SEQEND1_STOPW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: SEQEND1_STOPW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Disable shortcut"]
    #[inline]
    pub fn disabled(self) -> &'a mut W {
        self.variant(SEQEND1_STOPW::DISABLED)
    }
    #[doc = "Enable shortcut"]
    #[inline]
    pub fn enabled(self) -> &'a mut W {
        self.variant(SEQEND1_STOPW::ENABLED)
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
#[doc = "Values that can be written to the field `LOOPSDONE_SEQSTART0`"]
pub enum LOOPSDONE_SEQSTART0W {
    #[doc = "Disable shortcut"]
    DISABLED,
    #[doc = "Enable shortcut"]
    ENABLED,
}
impl LOOPSDONE_SEQSTART0W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            LOOPSDONE_SEQSTART0W::DISABLED => false,
            LOOPSDONE_SEQSTART0W::ENABLED => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _LOOPSDONE_SEQSTART0W<'a> {
    w: &'a mut W,
}
impl<'a> _LOOPSDONE_SEQSTART0W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: LOOPSDONE_SEQSTART0W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Disable shortcut"]
    #[inline]
    pub fn disabled(self) -> &'a mut W {
        self.variant(LOOPSDONE_SEQSTART0W::DISABLED)
    }
    #[doc = "Enable shortcut"]
    #[inline]
    pub fn enabled(self) -> &'a mut W {
        self.variant(LOOPSDONE_SEQSTART0W::ENABLED)
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
#[doc = "Values that can be written to the field `LOOPSDONE_SEQSTART1`"]
pub enum LOOPSDONE_SEQSTART1W {
    #[doc = "Disable shortcut"]
    DISABLED,
    #[doc = "Enable shortcut"]
    ENABLED,
}
impl LOOPSDONE_SEQSTART1W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            LOOPSDONE_SEQSTART1W::DISABLED => false,
            LOOPSDONE_SEQSTART1W::ENABLED => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _LOOPSDONE_SEQSTART1W<'a> {
    w: &'a mut W,
}
impl<'a> _LOOPSDONE_SEQSTART1W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: LOOPSDONE_SEQSTART1W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Disable shortcut"]
    #[inline]
    pub fn disabled(self) -> &'a mut W {
        self.variant(LOOPSDONE_SEQSTART1W::DISABLED)
    }
    #[doc = "Enable shortcut"]
    #[inline]
    pub fn enabled(self) -> &'a mut W {
        self.variant(LOOPSDONE_SEQSTART1W::ENABLED)
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
#[doc = "Values that can be written to the field `LOOPSDONE_STOP`"]
pub enum LOOPSDONE_STOPW {
    #[doc = "Disable shortcut"]
    DISABLED,
    #[doc = "Enable shortcut"]
    ENABLED,
}
impl LOOPSDONE_STOPW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            LOOPSDONE_STOPW::DISABLED => false,
            LOOPSDONE_STOPW::ENABLED => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _LOOPSDONE_STOPW<'a> {
    w: &'a mut W,
}
impl<'a> _LOOPSDONE_STOPW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: LOOPSDONE_STOPW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Disable shortcut"]
    #[inline]
    pub fn disabled(self) -> &'a mut W {
        self.variant(LOOPSDONE_STOPW::DISABLED)
    }
    #[doc = "Enable shortcut"]
    #[inline]
    pub fn enabled(self) -> &'a mut W {
        self.variant(LOOPSDONE_STOPW::ENABLED)
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
    #[doc = "Bit 0 - Shortcut between SEQEND[0] event and STOP task"]
    #[inline]
    pub fn seqend0_stop(&self) -> SEQEND0_STOPR {
        SEQEND0_STOPR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 1 - Shortcut between SEQEND[1] event and STOP task"]
    #[inline]
    pub fn seqend1_stop(&self) -> SEQEND1_STOPR {
        SEQEND1_STOPR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 1;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 2 - Shortcut between LOOPSDONE event and SEQSTART[0] task"]
    #[inline]
    pub fn loopsdone_seqstart0(&self) -> LOOPSDONE_SEQSTART0R {
        LOOPSDONE_SEQSTART0R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 2;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 3 - Shortcut between LOOPSDONE event and SEQSTART[1] task"]
    #[inline]
    pub fn loopsdone_seqstart1(&self) -> LOOPSDONE_SEQSTART1R {
        LOOPSDONE_SEQSTART1R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 3;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 4 - Shortcut between LOOPSDONE event and STOP task"]
    #[inline]
    pub fn loopsdone_stop(&self) -> LOOPSDONE_STOPR {
        LOOPSDONE_STOPR::_from({
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
    #[doc = "Bit 0 - Shortcut between SEQEND[0] event and STOP task"]
    #[inline]
    pub fn seqend0_stop(&mut self) -> _SEQEND0_STOPW {
        _SEQEND0_STOPW { w: self }
    }
    #[doc = "Bit 1 - Shortcut between SEQEND[1] event and STOP task"]
    #[inline]
    pub fn seqend1_stop(&mut self) -> _SEQEND1_STOPW {
        _SEQEND1_STOPW { w: self }
    }
    #[doc = "Bit 2 - Shortcut between LOOPSDONE event and SEQSTART[0] task"]
    #[inline]
    pub fn loopsdone_seqstart0(&mut self) -> _LOOPSDONE_SEQSTART0W {
        _LOOPSDONE_SEQSTART0W { w: self }
    }
    #[doc = "Bit 3 - Shortcut between LOOPSDONE event and SEQSTART[1] task"]
    #[inline]
    pub fn loopsdone_seqstart1(&mut self) -> _LOOPSDONE_SEQSTART1W {
        _LOOPSDONE_SEQSTART1W { w: self }
    }
    #[doc = "Bit 4 - Shortcut between LOOPSDONE event and STOP task"]
    #[inline]
    pub fn loopsdone_stop(&mut self) -> _LOOPSDONE_STOPW {
        _LOOPSDONE_STOPW { w: self }
    }
}
