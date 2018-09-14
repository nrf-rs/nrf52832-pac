#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::INTENCLR {
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
#[doc = "Possible values of the field `STOPPED`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum STOPPEDR {
    #[doc = "Read: Disabled"]
    DISABLED,
    #[doc = "Read: Enabled"]
    ENABLED,
}
impl STOPPEDR {
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
            STOPPEDR::DISABLED => false,
            STOPPEDR::ENABLED => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> STOPPEDR {
        match value {
            false => STOPPEDR::DISABLED,
            true => STOPPEDR::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline]
    pub fn is_disabled(&self) -> bool {
        *self == STOPPEDR::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline]
    pub fn is_enabled(&self) -> bool {
        *self == STOPPEDR::ENABLED
    }
}
#[doc = "Possible values of the field `SEQSTARTED0`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SEQSTARTED0R {
    #[doc = "Read: Disabled"]
    DISABLED,
    #[doc = "Read: Enabled"]
    ENABLED,
}
impl SEQSTARTED0R {
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
            SEQSTARTED0R::DISABLED => false,
            SEQSTARTED0R::ENABLED => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> SEQSTARTED0R {
        match value {
            false => SEQSTARTED0R::DISABLED,
            true => SEQSTARTED0R::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline]
    pub fn is_disabled(&self) -> bool {
        *self == SEQSTARTED0R::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline]
    pub fn is_enabled(&self) -> bool {
        *self == SEQSTARTED0R::ENABLED
    }
}
#[doc = "Possible values of the field `SEQSTARTED1`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SEQSTARTED1R {
    #[doc = "Read: Disabled"]
    DISABLED,
    #[doc = "Read: Enabled"]
    ENABLED,
}
impl SEQSTARTED1R {
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
            SEQSTARTED1R::DISABLED => false,
            SEQSTARTED1R::ENABLED => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> SEQSTARTED1R {
        match value {
            false => SEQSTARTED1R::DISABLED,
            true => SEQSTARTED1R::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline]
    pub fn is_disabled(&self) -> bool {
        *self == SEQSTARTED1R::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline]
    pub fn is_enabled(&self) -> bool {
        *self == SEQSTARTED1R::ENABLED
    }
}
#[doc = "Possible values of the field `SEQEND0`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SEQEND0R {
    #[doc = "Read: Disabled"]
    DISABLED,
    #[doc = "Read: Enabled"]
    ENABLED,
}
impl SEQEND0R {
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
            SEQEND0R::DISABLED => false,
            SEQEND0R::ENABLED => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> SEQEND0R {
        match value {
            false => SEQEND0R::DISABLED,
            true => SEQEND0R::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline]
    pub fn is_disabled(&self) -> bool {
        *self == SEQEND0R::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline]
    pub fn is_enabled(&self) -> bool {
        *self == SEQEND0R::ENABLED
    }
}
#[doc = "Possible values of the field `SEQEND1`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SEQEND1R {
    #[doc = "Read: Disabled"]
    DISABLED,
    #[doc = "Read: Enabled"]
    ENABLED,
}
impl SEQEND1R {
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
            SEQEND1R::DISABLED => false,
            SEQEND1R::ENABLED => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> SEQEND1R {
        match value {
            false => SEQEND1R::DISABLED,
            true => SEQEND1R::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline]
    pub fn is_disabled(&self) -> bool {
        *self == SEQEND1R::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline]
    pub fn is_enabled(&self) -> bool {
        *self == SEQEND1R::ENABLED
    }
}
#[doc = "Possible values of the field `PWMPERIODEND`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PWMPERIODENDR {
    #[doc = "Read: Disabled"]
    DISABLED,
    #[doc = "Read: Enabled"]
    ENABLED,
}
impl PWMPERIODENDR {
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
            PWMPERIODENDR::DISABLED => false,
            PWMPERIODENDR::ENABLED => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> PWMPERIODENDR {
        match value {
            false => PWMPERIODENDR::DISABLED,
            true => PWMPERIODENDR::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline]
    pub fn is_disabled(&self) -> bool {
        *self == PWMPERIODENDR::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline]
    pub fn is_enabled(&self) -> bool {
        *self == PWMPERIODENDR::ENABLED
    }
}
#[doc = "Possible values of the field `LOOPSDONE`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum LOOPSDONER {
    #[doc = "Read: Disabled"]
    DISABLED,
    #[doc = "Read: Enabled"]
    ENABLED,
}
impl LOOPSDONER {
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
            LOOPSDONER::DISABLED => false,
            LOOPSDONER::ENABLED => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> LOOPSDONER {
        match value {
            false => LOOPSDONER::DISABLED,
            true => LOOPSDONER::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline]
    pub fn is_disabled(&self) -> bool {
        *self == LOOPSDONER::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline]
    pub fn is_enabled(&self) -> bool {
        *self == LOOPSDONER::ENABLED
    }
}
#[doc = "Values that can be written to the field `STOPPED`"]
pub enum STOPPEDW {
    #[doc = "Disable"]
    CLEAR,
}
impl STOPPEDW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            STOPPEDW::CLEAR => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _STOPPEDW<'a> {
    w: &'a mut W,
}
impl<'a> _STOPPEDW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: STOPPEDW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Disable"]
    #[inline]
    pub fn clear(self) -> &'a mut W {
        self.variant(STOPPEDW::CLEAR)
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
#[doc = "Values that can be written to the field `SEQSTARTED0`"]
pub enum SEQSTARTED0W {
    #[doc = "Disable"]
    CLEAR,
}
impl SEQSTARTED0W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            SEQSTARTED0W::CLEAR => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _SEQSTARTED0W<'a> {
    w: &'a mut W,
}
impl<'a> _SEQSTARTED0W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: SEQSTARTED0W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Disable"]
    #[inline]
    pub fn clear(self) -> &'a mut W {
        self.variant(SEQSTARTED0W::CLEAR)
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
#[doc = "Values that can be written to the field `SEQSTARTED1`"]
pub enum SEQSTARTED1W {
    #[doc = "Disable"]
    CLEAR,
}
impl SEQSTARTED1W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            SEQSTARTED1W::CLEAR => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _SEQSTARTED1W<'a> {
    w: &'a mut W,
}
impl<'a> _SEQSTARTED1W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: SEQSTARTED1W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Disable"]
    #[inline]
    pub fn clear(self) -> &'a mut W {
        self.variant(SEQSTARTED1W::CLEAR)
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
#[doc = "Values that can be written to the field `SEQEND0`"]
pub enum SEQEND0W {
    #[doc = "Disable"]
    CLEAR,
}
impl SEQEND0W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            SEQEND0W::CLEAR => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _SEQEND0W<'a> {
    w: &'a mut W,
}
impl<'a> _SEQEND0W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: SEQEND0W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Disable"]
    #[inline]
    pub fn clear(self) -> &'a mut W {
        self.variant(SEQEND0W::CLEAR)
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
#[doc = "Values that can be written to the field `SEQEND1`"]
pub enum SEQEND1W {
    #[doc = "Disable"]
    CLEAR,
}
impl SEQEND1W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            SEQEND1W::CLEAR => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _SEQEND1W<'a> {
    w: &'a mut W,
}
impl<'a> _SEQEND1W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: SEQEND1W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Disable"]
    #[inline]
    pub fn clear(self) -> &'a mut W {
        self.variant(SEQEND1W::CLEAR)
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
        const OFFSET: u8 = 5;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `PWMPERIODEND`"]
pub enum PWMPERIODENDW {
    #[doc = "Disable"]
    CLEAR,
}
impl PWMPERIODENDW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            PWMPERIODENDW::CLEAR => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _PWMPERIODENDW<'a> {
    w: &'a mut W,
}
impl<'a> _PWMPERIODENDW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: PWMPERIODENDW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Disable"]
    #[inline]
    pub fn clear(self) -> &'a mut W {
        self.variant(PWMPERIODENDW::CLEAR)
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
        const OFFSET: u8 = 6;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `LOOPSDONE`"]
pub enum LOOPSDONEW {
    #[doc = "Disable"]
    CLEAR,
}
impl LOOPSDONEW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            LOOPSDONEW::CLEAR => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _LOOPSDONEW<'a> {
    w: &'a mut W,
}
impl<'a> _LOOPSDONEW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: LOOPSDONEW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Disable"]
    #[inline]
    pub fn clear(self) -> &'a mut W {
        self.variant(LOOPSDONEW::CLEAR)
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
        const OFFSET: u8 = 7;
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
    #[doc = "Bit 1 - Write '1' to Disable interrupt for STOPPED event"]
    #[inline]
    pub fn stopped(&self) -> STOPPEDR {
        STOPPEDR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 1;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 2 - Write '1' to Disable interrupt for SEQSTARTED[0] event"]
    #[inline]
    pub fn seqstarted0(&self) -> SEQSTARTED0R {
        SEQSTARTED0R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 2;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 3 - Write '1' to Disable interrupt for SEQSTARTED[1] event"]
    #[inline]
    pub fn seqstarted1(&self) -> SEQSTARTED1R {
        SEQSTARTED1R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 3;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 4 - Write '1' to Disable interrupt for SEQEND[0] event"]
    #[inline]
    pub fn seqend0(&self) -> SEQEND0R {
        SEQEND0R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 4;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 5 - Write '1' to Disable interrupt for SEQEND[1] event"]
    #[inline]
    pub fn seqend1(&self) -> SEQEND1R {
        SEQEND1R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 5;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 6 - Write '1' to Disable interrupt for PWMPERIODEND event"]
    #[inline]
    pub fn pwmperiodend(&self) -> PWMPERIODENDR {
        PWMPERIODENDR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 6;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 7 - Write '1' to Disable interrupt for LOOPSDONE event"]
    #[inline]
    pub fn loopsdone(&self) -> LOOPSDONER {
        LOOPSDONER::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 7;
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
    #[doc = "Bit 1 - Write '1' to Disable interrupt for STOPPED event"]
    #[inline]
    pub fn stopped(&mut self) -> _STOPPEDW {
        _STOPPEDW { w: self }
    }
    #[doc = "Bit 2 - Write '1' to Disable interrupt for SEQSTARTED[0] event"]
    #[inline]
    pub fn seqstarted0(&mut self) -> _SEQSTARTED0W {
        _SEQSTARTED0W { w: self }
    }
    #[doc = "Bit 3 - Write '1' to Disable interrupt for SEQSTARTED[1] event"]
    #[inline]
    pub fn seqstarted1(&mut self) -> _SEQSTARTED1W {
        _SEQSTARTED1W { w: self }
    }
    #[doc = "Bit 4 - Write '1' to Disable interrupt for SEQEND[0] event"]
    #[inline]
    pub fn seqend0(&mut self) -> _SEQEND0W {
        _SEQEND0W { w: self }
    }
    #[doc = "Bit 5 - Write '1' to Disable interrupt for SEQEND[1] event"]
    #[inline]
    pub fn seqend1(&mut self) -> _SEQEND1W {
        _SEQEND1W { w: self }
    }
    #[doc = "Bit 6 - Write '1' to Disable interrupt for PWMPERIODEND event"]
    #[inline]
    pub fn pwmperiodend(&mut self) -> _PWMPERIODENDW {
        _PWMPERIODENDW { w: self }
    }
    #[doc = "Bit 7 - Write '1' to Disable interrupt for LOOPSDONE event"]
    #[inline]
    pub fn loopsdone(&mut self) -> _LOOPSDONEW {
        _LOOPSDONEW { w: self }
    }
}
