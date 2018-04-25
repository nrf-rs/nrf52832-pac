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
#[doc = "Possible values of the field `READY_START`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum READY_STARTR {
    #[doc = "Disable shortcut"]
    DISABLED,
    #[doc = "Enable shortcut"]
    ENABLED,
}
impl READY_STARTR {
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
            READY_STARTR::DISABLED => false,
            READY_STARTR::ENABLED => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> READY_STARTR {
        match value {
            false => READY_STARTR::DISABLED,
            true => READY_STARTR::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline]
    pub fn is_disabled(&self) -> bool {
        *self == READY_STARTR::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline]
    pub fn is_enabled(&self) -> bool {
        *self == READY_STARTR::ENABLED
    }
}
#[doc = "Possible values of the field `END_DISABLE`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum END_DISABLER {
    #[doc = "Disable shortcut"]
    DISABLED,
    #[doc = "Enable shortcut"]
    ENABLED,
}
impl END_DISABLER {
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
            END_DISABLER::DISABLED => false,
            END_DISABLER::ENABLED => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> END_DISABLER {
        match value {
            false => END_DISABLER::DISABLED,
            true => END_DISABLER::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline]
    pub fn is_disabled(&self) -> bool {
        *self == END_DISABLER::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline]
    pub fn is_enabled(&self) -> bool {
        *self == END_DISABLER::ENABLED
    }
}
#[doc = "Possible values of the field `DISABLED_TXEN`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DISABLED_TXENR {
    #[doc = "Disable shortcut"]
    DISABLED,
    #[doc = "Enable shortcut"]
    ENABLED,
}
impl DISABLED_TXENR {
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
            DISABLED_TXENR::DISABLED => false,
            DISABLED_TXENR::ENABLED => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> DISABLED_TXENR {
        match value {
            false => DISABLED_TXENR::DISABLED,
            true => DISABLED_TXENR::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline]
    pub fn is_disabled(&self) -> bool {
        *self == DISABLED_TXENR::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline]
    pub fn is_enabled(&self) -> bool {
        *self == DISABLED_TXENR::ENABLED
    }
}
#[doc = "Possible values of the field `DISABLED_RXEN`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DISABLED_RXENR {
    #[doc = "Disable shortcut"]
    DISABLED,
    #[doc = "Enable shortcut"]
    ENABLED,
}
impl DISABLED_RXENR {
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
            DISABLED_RXENR::DISABLED => false,
            DISABLED_RXENR::ENABLED => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> DISABLED_RXENR {
        match value {
            false => DISABLED_RXENR::DISABLED,
            true => DISABLED_RXENR::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline]
    pub fn is_disabled(&self) -> bool {
        *self == DISABLED_RXENR::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline]
    pub fn is_enabled(&self) -> bool {
        *self == DISABLED_RXENR::ENABLED
    }
}
#[doc = "Possible values of the field `ADDRESS_RSSISTART`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ADDRESS_RSSISTARTR {
    #[doc = "Disable shortcut"]
    DISABLED,
    #[doc = "Enable shortcut"]
    ENABLED,
}
impl ADDRESS_RSSISTARTR {
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
            ADDRESS_RSSISTARTR::DISABLED => false,
            ADDRESS_RSSISTARTR::ENABLED => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> ADDRESS_RSSISTARTR {
        match value {
            false => ADDRESS_RSSISTARTR::DISABLED,
            true => ADDRESS_RSSISTARTR::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline]
    pub fn is_disabled(&self) -> bool {
        *self == ADDRESS_RSSISTARTR::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline]
    pub fn is_enabled(&self) -> bool {
        *self == ADDRESS_RSSISTARTR::ENABLED
    }
}
#[doc = "Possible values of the field `END_START`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum END_STARTR {
    #[doc = "Disable shortcut"]
    DISABLED,
    #[doc = "Enable shortcut"]
    ENABLED,
}
impl END_STARTR {
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
            END_STARTR::DISABLED => false,
            END_STARTR::ENABLED => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> END_STARTR {
        match value {
            false => END_STARTR::DISABLED,
            true => END_STARTR::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline]
    pub fn is_disabled(&self) -> bool {
        *self == END_STARTR::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline]
    pub fn is_enabled(&self) -> bool {
        *self == END_STARTR::ENABLED
    }
}
#[doc = "Possible values of the field `ADDRESS_BCSTART`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ADDRESS_BCSTARTR {
    #[doc = "Disable shortcut"]
    DISABLED,
    #[doc = "Enable shortcut"]
    ENABLED,
}
impl ADDRESS_BCSTARTR {
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
            ADDRESS_BCSTARTR::DISABLED => false,
            ADDRESS_BCSTARTR::ENABLED => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> ADDRESS_BCSTARTR {
        match value {
            false => ADDRESS_BCSTARTR::DISABLED,
            true => ADDRESS_BCSTARTR::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline]
    pub fn is_disabled(&self) -> bool {
        *self == ADDRESS_BCSTARTR::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline]
    pub fn is_enabled(&self) -> bool {
        *self == ADDRESS_BCSTARTR::ENABLED
    }
}
#[doc = "Possible values of the field `DISABLED_RSSISTOP`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DISABLED_RSSISTOPR {
    #[doc = "Disable shortcut"]
    DISABLED,
    #[doc = "Enable shortcut"]
    ENABLED,
}
impl DISABLED_RSSISTOPR {
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
            DISABLED_RSSISTOPR::DISABLED => false,
            DISABLED_RSSISTOPR::ENABLED => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> DISABLED_RSSISTOPR {
        match value {
            false => DISABLED_RSSISTOPR::DISABLED,
            true => DISABLED_RSSISTOPR::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline]
    pub fn is_disabled(&self) -> bool {
        *self == DISABLED_RSSISTOPR::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline]
    pub fn is_enabled(&self) -> bool {
        *self == DISABLED_RSSISTOPR::ENABLED
    }
}
#[doc = "Values that can be written to the field `READY_START`"]
pub enum READY_STARTW {
    #[doc = "Disable shortcut"]
    DISABLED,
    #[doc = "Enable shortcut"]
    ENABLED,
}
impl READY_STARTW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            READY_STARTW::DISABLED => false,
            READY_STARTW::ENABLED => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _READY_STARTW<'a> {
    w: &'a mut W,
}
impl<'a> _READY_STARTW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: READY_STARTW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Disable shortcut"]
    #[inline]
    pub fn disabled(self) -> &'a mut W {
        self.variant(READY_STARTW::DISABLED)
    }
    #[doc = "Enable shortcut"]
    #[inline]
    pub fn enabled(self) -> &'a mut W {
        self.variant(READY_STARTW::ENABLED)
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
#[doc = "Values that can be written to the field `END_DISABLE`"]
pub enum END_DISABLEW {
    #[doc = "Disable shortcut"]
    DISABLED,
    #[doc = "Enable shortcut"]
    ENABLED,
}
impl END_DISABLEW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            END_DISABLEW::DISABLED => false,
            END_DISABLEW::ENABLED => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _END_DISABLEW<'a> {
    w: &'a mut W,
}
impl<'a> _END_DISABLEW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: END_DISABLEW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Disable shortcut"]
    #[inline]
    pub fn disabled(self) -> &'a mut W {
        self.variant(END_DISABLEW::DISABLED)
    }
    #[doc = "Enable shortcut"]
    #[inline]
    pub fn enabled(self) -> &'a mut W {
        self.variant(END_DISABLEW::ENABLED)
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
#[doc = "Values that can be written to the field `DISABLED_TXEN`"]
pub enum DISABLED_TXENW {
    #[doc = "Disable shortcut"]
    DISABLED,
    #[doc = "Enable shortcut"]
    ENABLED,
}
impl DISABLED_TXENW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            DISABLED_TXENW::DISABLED => false,
            DISABLED_TXENW::ENABLED => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _DISABLED_TXENW<'a> {
    w: &'a mut W,
}
impl<'a> _DISABLED_TXENW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: DISABLED_TXENW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Disable shortcut"]
    #[inline]
    pub fn disabled(self) -> &'a mut W {
        self.variant(DISABLED_TXENW::DISABLED)
    }
    #[doc = "Enable shortcut"]
    #[inline]
    pub fn enabled(self) -> &'a mut W {
        self.variant(DISABLED_TXENW::ENABLED)
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
#[doc = "Values that can be written to the field `DISABLED_RXEN`"]
pub enum DISABLED_RXENW {
    #[doc = "Disable shortcut"]
    DISABLED,
    #[doc = "Enable shortcut"]
    ENABLED,
}
impl DISABLED_RXENW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            DISABLED_RXENW::DISABLED => false,
            DISABLED_RXENW::ENABLED => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _DISABLED_RXENW<'a> {
    w: &'a mut W,
}
impl<'a> _DISABLED_RXENW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: DISABLED_RXENW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Disable shortcut"]
    #[inline]
    pub fn disabled(self) -> &'a mut W {
        self.variant(DISABLED_RXENW::DISABLED)
    }
    #[doc = "Enable shortcut"]
    #[inline]
    pub fn enabled(self) -> &'a mut W {
        self.variant(DISABLED_RXENW::ENABLED)
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
#[doc = "Values that can be written to the field `ADDRESS_RSSISTART`"]
pub enum ADDRESS_RSSISTARTW {
    #[doc = "Disable shortcut"]
    DISABLED,
    #[doc = "Enable shortcut"]
    ENABLED,
}
impl ADDRESS_RSSISTARTW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            ADDRESS_RSSISTARTW::DISABLED => false,
            ADDRESS_RSSISTARTW::ENABLED => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _ADDRESS_RSSISTARTW<'a> {
    w: &'a mut W,
}
impl<'a> _ADDRESS_RSSISTARTW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: ADDRESS_RSSISTARTW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Disable shortcut"]
    #[inline]
    pub fn disabled(self) -> &'a mut W {
        self.variant(ADDRESS_RSSISTARTW::DISABLED)
    }
    #[doc = "Enable shortcut"]
    #[inline]
    pub fn enabled(self) -> &'a mut W {
        self.variant(ADDRESS_RSSISTARTW::ENABLED)
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
#[doc = "Values that can be written to the field `END_START`"]
pub enum END_STARTW {
    #[doc = "Disable shortcut"]
    DISABLED,
    #[doc = "Enable shortcut"]
    ENABLED,
}
impl END_STARTW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            END_STARTW::DISABLED => false,
            END_STARTW::ENABLED => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _END_STARTW<'a> {
    w: &'a mut W,
}
impl<'a> _END_STARTW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: END_STARTW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Disable shortcut"]
    #[inline]
    pub fn disabled(self) -> &'a mut W {
        self.variant(END_STARTW::DISABLED)
    }
    #[doc = "Enable shortcut"]
    #[inline]
    pub fn enabled(self) -> &'a mut W {
        self.variant(END_STARTW::ENABLED)
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
#[doc = "Values that can be written to the field `ADDRESS_BCSTART`"]
pub enum ADDRESS_BCSTARTW {
    #[doc = "Disable shortcut"]
    DISABLED,
    #[doc = "Enable shortcut"]
    ENABLED,
}
impl ADDRESS_BCSTARTW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            ADDRESS_BCSTARTW::DISABLED => false,
            ADDRESS_BCSTARTW::ENABLED => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _ADDRESS_BCSTARTW<'a> {
    w: &'a mut W,
}
impl<'a> _ADDRESS_BCSTARTW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: ADDRESS_BCSTARTW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Disable shortcut"]
    #[inline]
    pub fn disabled(self) -> &'a mut W {
        self.variant(ADDRESS_BCSTARTW::DISABLED)
    }
    #[doc = "Enable shortcut"]
    #[inline]
    pub fn enabled(self) -> &'a mut W {
        self.variant(ADDRESS_BCSTARTW::ENABLED)
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
#[doc = "Values that can be written to the field `DISABLED_RSSISTOP`"]
pub enum DISABLED_RSSISTOPW {
    #[doc = "Disable shortcut"]
    DISABLED,
    #[doc = "Enable shortcut"]
    ENABLED,
}
impl DISABLED_RSSISTOPW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            DISABLED_RSSISTOPW::DISABLED => false,
            DISABLED_RSSISTOPW::ENABLED => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _DISABLED_RSSISTOPW<'a> {
    w: &'a mut W,
}
impl<'a> _DISABLED_RSSISTOPW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: DISABLED_RSSISTOPW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Disable shortcut"]
    #[inline]
    pub fn disabled(self) -> &'a mut W {
        self.variant(DISABLED_RSSISTOPW::DISABLED)
    }
    #[doc = "Enable shortcut"]
    #[inline]
    pub fn enabled(self) -> &'a mut W {
        self.variant(DISABLED_RSSISTOPW::ENABLED)
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
    #[doc = "Bit 0 - Shortcut between READY event and START task"]
    #[inline]
    pub fn ready_start(&self) -> READY_STARTR {
        READY_STARTR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 1 - Shortcut between END event and DISABLE task"]
    #[inline]
    pub fn end_disable(&self) -> END_DISABLER {
        END_DISABLER::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 1;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 2 - Shortcut between DISABLED event and TXEN task"]
    #[inline]
    pub fn disabled_txen(&self) -> DISABLED_TXENR {
        DISABLED_TXENR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 2;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 3 - Shortcut between DISABLED event and RXEN task"]
    #[inline]
    pub fn disabled_rxen(&self) -> DISABLED_RXENR {
        DISABLED_RXENR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 3;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 4 - Shortcut between ADDRESS event and RSSISTART task"]
    #[inline]
    pub fn address_rssistart(&self) -> ADDRESS_RSSISTARTR {
        ADDRESS_RSSISTARTR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 4;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 5 - Shortcut between END event and START task"]
    #[inline]
    pub fn end_start(&self) -> END_STARTR {
        END_STARTR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 5;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 6 - Shortcut between ADDRESS event and BCSTART task"]
    #[inline]
    pub fn address_bcstart(&self) -> ADDRESS_BCSTARTR {
        ADDRESS_BCSTARTR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 6;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 8 - Shortcut between DISABLED event and RSSISTOP task"]
    #[inline]
    pub fn disabled_rssistop(&self) -> DISABLED_RSSISTOPR {
        DISABLED_RSSISTOPR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 8;
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
    #[doc = "Bit 0 - Shortcut between READY event and START task"]
    #[inline]
    pub fn ready_start(&mut self) -> _READY_STARTW {
        _READY_STARTW { w: self }
    }
    #[doc = "Bit 1 - Shortcut between END event and DISABLE task"]
    #[inline]
    pub fn end_disable(&mut self) -> _END_DISABLEW {
        _END_DISABLEW { w: self }
    }
    #[doc = "Bit 2 - Shortcut between DISABLED event and TXEN task"]
    #[inline]
    pub fn disabled_txen(&mut self) -> _DISABLED_TXENW {
        _DISABLED_TXENW { w: self }
    }
    #[doc = "Bit 3 - Shortcut between DISABLED event and RXEN task"]
    #[inline]
    pub fn disabled_rxen(&mut self) -> _DISABLED_RXENW {
        _DISABLED_RXENW { w: self }
    }
    #[doc = "Bit 4 - Shortcut between ADDRESS event and RSSISTART task"]
    #[inline]
    pub fn address_rssistart(&mut self) -> _ADDRESS_RSSISTARTW {
        _ADDRESS_RSSISTARTW { w: self }
    }
    #[doc = "Bit 5 - Shortcut between END event and START task"]
    #[inline]
    pub fn end_start(&mut self) -> _END_STARTW {
        _END_STARTW { w: self }
    }
    #[doc = "Bit 6 - Shortcut between ADDRESS event and BCSTART task"]
    #[inline]
    pub fn address_bcstart(&mut self) -> _ADDRESS_BCSTARTW {
        _ADDRESS_BCSTARTW { w: self }
    }
    #[doc = "Bit 8 - Shortcut between DISABLED event and RSSISTOP task"]
    #[inline]
    pub fn disabled_rssistop(&mut self) -> _DISABLED_RSSISTOPW {
        _DISABLED_RSSISTOPW { w: self }
    }
}
