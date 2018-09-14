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
#[doc = "Possible values of the field `EP0DATADONE_STARTEPIN0`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum EP0DATADONE_STARTEPIN0R {
    #[doc = "Disable shortcut"]
    DISABLED,
    #[doc = "Enable shortcut"]
    ENABLED,
}
impl EP0DATADONE_STARTEPIN0R {
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
            EP0DATADONE_STARTEPIN0R::DISABLED => false,
            EP0DATADONE_STARTEPIN0R::ENABLED => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> EP0DATADONE_STARTEPIN0R {
        match value {
            false => EP0DATADONE_STARTEPIN0R::DISABLED,
            true => EP0DATADONE_STARTEPIN0R::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline]
    pub fn is_disabled(&self) -> bool {
        *self == EP0DATADONE_STARTEPIN0R::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline]
    pub fn is_enabled(&self) -> bool {
        *self == EP0DATADONE_STARTEPIN0R::ENABLED
    }
}
#[doc = "Possible values of the field `EP0DATADONE_STARTEPOUT0`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum EP0DATADONE_STARTEPOUT0R {
    #[doc = "Disable shortcut"]
    DISABLED,
    #[doc = "Enable shortcut"]
    ENABLED,
}
impl EP0DATADONE_STARTEPOUT0R {
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
            EP0DATADONE_STARTEPOUT0R::DISABLED => false,
            EP0DATADONE_STARTEPOUT0R::ENABLED => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> EP0DATADONE_STARTEPOUT0R {
        match value {
            false => EP0DATADONE_STARTEPOUT0R::DISABLED,
            true => EP0DATADONE_STARTEPOUT0R::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline]
    pub fn is_disabled(&self) -> bool {
        *self == EP0DATADONE_STARTEPOUT0R::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline]
    pub fn is_enabled(&self) -> bool {
        *self == EP0DATADONE_STARTEPOUT0R::ENABLED
    }
}
#[doc = "Possible values of the field `EP0DATADONE_EP0STATUS`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum EP0DATADONE_EP0STATUSR {
    #[doc = "Disable shortcut"]
    DISABLED,
    #[doc = "Enable shortcut"]
    ENABLED,
}
impl EP0DATADONE_EP0STATUSR {
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
            EP0DATADONE_EP0STATUSR::DISABLED => false,
            EP0DATADONE_EP0STATUSR::ENABLED => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> EP0DATADONE_EP0STATUSR {
        match value {
            false => EP0DATADONE_EP0STATUSR::DISABLED,
            true => EP0DATADONE_EP0STATUSR::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline]
    pub fn is_disabled(&self) -> bool {
        *self == EP0DATADONE_EP0STATUSR::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline]
    pub fn is_enabled(&self) -> bool {
        *self == EP0DATADONE_EP0STATUSR::ENABLED
    }
}
#[doc = "Possible values of the field `ENDEPOUT0_EP0STATUS`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ENDEPOUT0_EP0STATUSR {
    #[doc = "Disable shortcut"]
    DISABLED,
    #[doc = "Enable shortcut"]
    ENABLED,
}
impl ENDEPOUT0_EP0STATUSR {
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
            ENDEPOUT0_EP0STATUSR::DISABLED => false,
            ENDEPOUT0_EP0STATUSR::ENABLED => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> ENDEPOUT0_EP0STATUSR {
        match value {
            false => ENDEPOUT0_EP0STATUSR::DISABLED,
            true => ENDEPOUT0_EP0STATUSR::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline]
    pub fn is_disabled(&self) -> bool {
        *self == ENDEPOUT0_EP0STATUSR::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline]
    pub fn is_enabled(&self) -> bool {
        *self == ENDEPOUT0_EP0STATUSR::ENABLED
    }
}
#[doc = "Possible values of the field `ENDEPOUT0_EP0RCVOUT`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ENDEPOUT0_EP0RCVOUTR {
    #[doc = "Disable shortcut"]
    DISABLED,
    #[doc = "Enable shortcut"]
    ENABLED,
}
impl ENDEPOUT0_EP0RCVOUTR {
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
            ENDEPOUT0_EP0RCVOUTR::DISABLED => false,
            ENDEPOUT0_EP0RCVOUTR::ENABLED => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> ENDEPOUT0_EP0RCVOUTR {
        match value {
            false => ENDEPOUT0_EP0RCVOUTR::DISABLED,
            true => ENDEPOUT0_EP0RCVOUTR::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline]
    pub fn is_disabled(&self) -> bool {
        *self == ENDEPOUT0_EP0RCVOUTR::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline]
    pub fn is_enabled(&self) -> bool {
        *self == ENDEPOUT0_EP0RCVOUTR::ENABLED
    }
}
#[doc = "Values that can be written to the field `EP0DATADONE_STARTEPIN0`"]
pub enum EP0DATADONE_STARTEPIN0W {
    #[doc = "Disable shortcut"]
    DISABLED,
    #[doc = "Enable shortcut"]
    ENABLED,
}
impl EP0DATADONE_STARTEPIN0W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            EP0DATADONE_STARTEPIN0W::DISABLED => false,
            EP0DATADONE_STARTEPIN0W::ENABLED => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _EP0DATADONE_STARTEPIN0W<'a> {
    w: &'a mut W,
}
impl<'a> _EP0DATADONE_STARTEPIN0W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: EP0DATADONE_STARTEPIN0W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Disable shortcut"]
    #[inline]
    pub fn disabled(self) -> &'a mut W {
        self.variant(EP0DATADONE_STARTEPIN0W::DISABLED)
    }
    #[doc = "Enable shortcut"]
    #[inline]
    pub fn enabled(self) -> &'a mut W {
        self.variant(EP0DATADONE_STARTEPIN0W::ENABLED)
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
#[doc = "Values that can be written to the field `EP0DATADONE_STARTEPOUT0`"]
pub enum EP0DATADONE_STARTEPOUT0W {
    #[doc = "Disable shortcut"]
    DISABLED,
    #[doc = "Enable shortcut"]
    ENABLED,
}
impl EP0DATADONE_STARTEPOUT0W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            EP0DATADONE_STARTEPOUT0W::DISABLED => false,
            EP0DATADONE_STARTEPOUT0W::ENABLED => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _EP0DATADONE_STARTEPOUT0W<'a> {
    w: &'a mut W,
}
impl<'a> _EP0DATADONE_STARTEPOUT0W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: EP0DATADONE_STARTEPOUT0W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Disable shortcut"]
    #[inline]
    pub fn disabled(self) -> &'a mut W {
        self.variant(EP0DATADONE_STARTEPOUT0W::DISABLED)
    }
    #[doc = "Enable shortcut"]
    #[inline]
    pub fn enabled(self) -> &'a mut W {
        self.variant(EP0DATADONE_STARTEPOUT0W::ENABLED)
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
#[doc = "Values that can be written to the field `EP0DATADONE_EP0STATUS`"]
pub enum EP0DATADONE_EP0STATUSW {
    #[doc = "Disable shortcut"]
    DISABLED,
    #[doc = "Enable shortcut"]
    ENABLED,
}
impl EP0DATADONE_EP0STATUSW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            EP0DATADONE_EP0STATUSW::DISABLED => false,
            EP0DATADONE_EP0STATUSW::ENABLED => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _EP0DATADONE_EP0STATUSW<'a> {
    w: &'a mut W,
}
impl<'a> _EP0DATADONE_EP0STATUSW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: EP0DATADONE_EP0STATUSW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Disable shortcut"]
    #[inline]
    pub fn disabled(self) -> &'a mut W {
        self.variant(EP0DATADONE_EP0STATUSW::DISABLED)
    }
    #[doc = "Enable shortcut"]
    #[inline]
    pub fn enabled(self) -> &'a mut W {
        self.variant(EP0DATADONE_EP0STATUSW::ENABLED)
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
#[doc = "Values that can be written to the field `ENDEPOUT0_EP0STATUS`"]
pub enum ENDEPOUT0_EP0STATUSW {
    #[doc = "Disable shortcut"]
    DISABLED,
    #[doc = "Enable shortcut"]
    ENABLED,
}
impl ENDEPOUT0_EP0STATUSW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            ENDEPOUT0_EP0STATUSW::DISABLED => false,
            ENDEPOUT0_EP0STATUSW::ENABLED => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _ENDEPOUT0_EP0STATUSW<'a> {
    w: &'a mut W,
}
impl<'a> _ENDEPOUT0_EP0STATUSW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: ENDEPOUT0_EP0STATUSW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Disable shortcut"]
    #[inline]
    pub fn disabled(self) -> &'a mut W {
        self.variant(ENDEPOUT0_EP0STATUSW::DISABLED)
    }
    #[doc = "Enable shortcut"]
    #[inline]
    pub fn enabled(self) -> &'a mut W {
        self.variant(ENDEPOUT0_EP0STATUSW::ENABLED)
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
#[doc = "Values that can be written to the field `ENDEPOUT0_EP0RCVOUT`"]
pub enum ENDEPOUT0_EP0RCVOUTW {
    #[doc = "Disable shortcut"]
    DISABLED,
    #[doc = "Enable shortcut"]
    ENABLED,
}
impl ENDEPOUT0_EP0RCVOUTW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            ENDEPOUT0_EP0RCVOUTW::DISABLED => false,
            ENDEPOUT0_EP0RCVOUTW::ENABLED => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _ENDEPOUT0_EP0RCVOUTW<'a> {
    w: &'a mut W,
}
impl<'a> _ENDEPOUT0_EP0RCVOUTW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: ENDEPOUT0_EP0RCVOUTW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Disable shortcut"]
    #[inline]
    pub fn disabled(self) -> &'a mut W {
        self.variant(ENDEPOUT0_EP0RCVOUTW::DISABLED)
    }
    #[doc = "Enable shortcut"]
    #[inline]
    pub fn enabled(self) -> &'a mut W {
        self.variant(ENDEPOUT0_EP0RCVOUTW::ENABLED)
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
    #[doc = "Bit 0 - Shortcut between EP0DATADONE event and STARTEPIN[0] task"]
    #[inline]
    pub fn ep0datadone_startepin0(&self) -> EP0DATADONE_STARTEPIN0R {
        EP0DATADONE_STARTEPIN0R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 1 - Shortcut between EP0DATADONE event and STARTEPOUT[0] task"]
    #[inline]
    pub fn ep0datadone_startepout0(&self) -> EP0DATADONE_STARTEPOUT0R {
        EP0DATADONE_STARTEPOUT0R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 1;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 2 - Shortcut between EP0DATADONE event and EP0STATUS task"]
    #[inline]
    pub fn ep0datadone_ep0status(&self) -> EP0DATADONE_EP0STATUSR {
        EP0DATADONE_EP0STATUSR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 2;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 3 - Shortcut between ENDEPOUT[0] event and EP0STATUS task"]
    #[inline]
    pub fn endepout0_ep0status(&self) -> ENDEPOUT0_EP0STATUSR {
        ENDEPOUT0_EP0STATUSR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 3;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 4 - Shortcut between ENDEPOUT[0] event and EP0RCVOUT task"]
    #[inline]
    pub fn endepout0_ep0rcvout(&self) -> ENDEPOUT0_EP0RCVOUTR {
        ENDEPOUT0_EP0RCVOUTR::_from({
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
    #[doc = "Bit 0 - Shortcut between EP0DATADONE event and STARTEPIN[0] task"]
    #[inline]
    pub fn ep0datadone_startepin0(&mut self) -> _EP0DATADONE_STARTEPIN0W {
        _EP0DATADONE_STARTEPIN0W { w: self }
    }
    #[doc = "Bit 1 - Shortcut between EP0DATADONE event and STARTEPOUT[0] task"]
    #[inline]
    pub fn ep0datadone_startepout0(&mut self) -> _EP0DATADONE_STARTEPOUT0W {
        _EP0DATADONE_STARTEPOUT0W { w: self }
    }
    #[doc = "Bit 2 - Shortcut between EP0DATADONE event and EP0STATUS task"]
    #[inline]
    pub fn ep0datadone_ep0status(&mut self) -> _EP0DATADONE_EP0STATUSW {
        _EP0DATADONE_EP0STATUSW { w: self }
    }
    #[doc = "Bit 3 - Shortcut between ENDEPOUT[0] event and EP0STATUS task"]
    #[inline]
    pub fn endepout0_ep0status(&mut self) -> _ENDEPOUT0_EP0STATUSW {
        _ENDEPOUT0_EP0STATUSW { w: self }
    }
    #[doc = "Bit 4 - Shortcut between ENDEPOUT[0] event and EP0RCVOUT task"]
    #[inline]
    pub fn endepout0_ep0rcvout(&mut self) -> _ENDEPOUT0_EP0RCVOUTW {
        _ENDEPOUT0_EP0RCVOUTW { w: self }
    }
}
