#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::CINSTRCONF {
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
#[doc = r" Value of the field"]
pub struct OPCODER {
    bits: u8,
}
impl OPCODER {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = "Possible values of the field `LENGTH`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum LENGTHR {
    #[doc = "Send opcode only."]
    _1B,
    #[doc = "Send opcode, CINSTRDAT0.BYTE0."]
    _2B,
    #[doc = "Send opcode, CINSTRDAT0.BYTE0 -&gt; CINSTRDAT0.BYTE1."]
    _3B,
    #[doc = "Send opcode, CINSTRDAT0.BYTE0 -&gt; CINSTRDAT0.BYTE2."]
    _4B,
    #[doc = "Send opcode, CINSTRDAT0.BYTE0 -&gt; CINSTRDAT0.BYTE3."]
    _5B,
    #[doc = "Send opcode, CINSTRDAT0.BYTE0 -&gt; CINSTRDAT1.BYTE4."]
    _6B,
    #[doc = "Send opcode, CINSTRDAT0.BYTE0 -&gt; CINSTRDAT1.BYTE5."]
    _7B,
    #[doc = "Send opcode, CINSTRDAT0.BYTE0 -&gt; CINSTRDAT1.BYTE6."]
    _8B,
    #[doc = "Send opcode, CINSTRDAT0.BYTE0 -&gt; CINSTRDAT1.BYTE7."]
    _9B,
    #[doc = r" Reserved"]
    _Reserved(u8),
}
impl LENGTHR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            LENGTHR::_1B => 1,
            LENGTHR::_2B => 2,
            LENGTHR::_3B => 3,
            LENGTHR::_4B => 4,
            LENGTHR::_5B => 5,
            LENGTHR::_6B => 6,
            LENGTHR::_7B => 7,
            LENGTHR::_8B => 8,
            LENGTHR::_9B => 9,
            LENGTHR::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> LENGTHR {
        match value {
            1 => LENGTHR::_1B,
            2 => LENGTHR::_2B,
            3 => LENGTHR::_3B,
            4 => LENGTHR::_4B,
            5 => LENGTHR::_5B,
            6 => LENGTHR::_6B,
            7 => LENGTHR::_7B,
            8 => LENGTHR::_8B,
            9 => LENGTHR::_9B,
            i => LENGTHR::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `_1B`"]
    #[inline]
    pub fn is_1b(&self) -> bool {
        *self == LENGTHR::_1B
    }
    #[doc = "Checks if the value of the field is `_2B`"]
    #[inline]
    pub fn is_2b(&self) -> bool {
        *self == LENGTHR::_2B
    }
    #[doc = "Checks if the value of the field is `_3B`"]
    #[inline]
    pub fn is_3b(&self) -> bool {
        *self == LENGTHR::_3B
    }
    #[doc = "Checks if the value of the field is `_4B`"]
    #[inline]
    pub fn is_4b(&self) -> bool {
        *self == LENGTHR::_4B
    }
    #[doc = "Checks if the value of the field is `_5B`"]
    #[inline]
    pub fn is_5b(&self) -> bool {
        *self == LENGTHR::_5B
    }
    #[doc = "Checks if the value of the field is `_6B`"]
    #[inline]
    pub fn is_6b(&self) -> bool {
        *self == LENGTHR::_6B
    }
    #[doc = "Checks if the value of the field is `_7B`"]
    #[inline]
    pub fn is_7b(&self) -> bool {
        *self == LENGTHR::_7B
    }
    #[doc = "Checks if the value of the field is `_8B`"]
    #[inline]
    pub fn is_8b(&self) -> bool {
        *self == LENGTHR::_8B
    }
    #[doc = "Checks if the value of the field is `_9B`"]
    #[inline]
    pub fn is_9b(&self) -> bool {
        *self == LENGTHR::_9B
    }
}
#[doc = r" Value of the field"]
pub struct LIO2R {
    bits: bool,
}
impl LIO2R {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bit(&self) -> bool {
        self.bits
    }
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
}
#[doc = r" Value of the field"]
pub struct LIO3R {
    bits: bool,
}
impl LIO3R {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bit(&self) -> bool {
        self.bits
    }
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
}
#[doc = "Possible values of the field `WIPWAIT`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum WIPWAITR {
    #[doc = "No wait."]
    DISABLE,
    #[doc = "Wait."]
    ENABLE,
}
impl WIPWAITR {
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
            WIPWAITR::DISABLE => false,
            WIPWAITR::ENABLE => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> WIPWAITR {
        match value {
            false => WIPWAITR::DISABLE,
            true => WIPWAITR::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline]
    pub fn is_disable(&self) -> bool {
        *self == WIPWAITR::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline]
    pub fn is_enable(&self) -> bool {
        *self == WIPWAITR::ENABLE
    }
}
#[doc = "Possible values of the field `WREN`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum WRENR {
    #[doc = "Do not send WREN."]
    DISABLE,
    #[doc = "Send WREN."]
    ENABLE,
}
impl WRENR {
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
            WRENR::DISABLE => false,
            WRENR::ENABLE => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> WRENR {
        match value {
            false => WRENR::DISABLE,
            true => WRENR::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline]
    pub fn is_disable(&self) -> bool {
        *self == WRENR::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline]
    pub fn is_enable(&self) -> bool {
        *self == WRENR::ENABLE
    }
}
#[doc = "Possible values of the field `LFEN`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum LFENR {
    #[doc = "Long frame mode disabled"]
    DISABLE,
    #[doc = "Long frame mode enabled"]
    ENABLE,
}
impl LFENR {
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
            LFENR::DISABLE => false,
            LFENR::ENABLE => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> LFENR {
        match value {
            false => LFENR::DISABLE,
            true => LFENR::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline]
    pub fn is_disable(&self) -> bool {
        *self == LFENR::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline]
    pub fn is_enable(&self) -> bool {
        *self == LFENR::ENABLE
    }
}
#[doc = "Possible values of the field `LFSTOP`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum LFSTOPR {
    #[doc = "Stop"]
    STOP,
    #[doc = r" Reserved"]
    _Reserved(bool),
}
impl LFSTOPR {
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
            LFSTOPR::STOP => true,
            LFSTOPR::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> LFSTOPR {
        match value {
            true => LFSTOPR::STOP,
            i => LFSTOPR::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `STOP`"]
    #[inline]
    pub fn is_stop(&self) -> bool {
        *self == LFSTOPR::STOP
    }
}
#[doc = r" Proxy"]
pub struct _OPCODEW<'a> {
    w: &'a mut W,
}
impl<'a> _OPCODEW<'a> {
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 255;
        const OFFSET: u8 = 0;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `LENGTH`"]
pub enum LENGTHW {
    #[doc = "Send opcode only."]
    _1B,
    #[doc = "Send opcode, CINSTRDAT0.BYTE0."]
    _2B,
    #[doc = "Send opcode, CINSTRDAT0.BYTE0 -&gt; CINSTRDAT0.BYTE1."]
    _3B,
    #[doc = "Send opcode, CINSTRDAT0.BYTE0 -&gt; CINSTRDAT0.BYTE2."]
    _4B,
    #[doc = "Send opcode, CINSTRDAT0.BYTE0 -&gt; CINSTRDAT0.BYTE3."]
    _5B,
    #[doc = "Send opcode, CINSTRDAT0.BYTE0 -&gt; CINSTRDAT1.BYTE4."]
    _6B,
    #[doc = "Send opcode, CINSTRDAT0.BYTE0 -&gt; CINSTRDAT1.BYTE5."]
    _7B,
    #[doc = "Send opcode, CINSTRDAT0.BYTE0 -&gt; CINSTRDAT1.BYTE6."]
    _8B,
    #[doc = "Send opcode, CINSTRDAT0.BYTE0 -&gt; CINSTRDAT1.BYTE7."]
    _9B,
}
impl LENGTHW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            LENGTHW::_1B => 1,
            LENGTHW::_2B => 2,
            LENGTHW::_3B => 3,
            LENGTHW::_4B => 4,
            LENGTHW::_5B => 5,
            LENGTHW::_6B => 6,
            LENGTHW::_7B => 7,
            LENGTHW::_8B => 8,
            LENGTHW::_9B => 9,
        }
    }
}
#[doc = r" Proxy"]
pub struct _LENGTHW<'a> {
    w: &'a mut W,
}
impl<'a> _LENGTHW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: LENGTHW) -> &'a mut W {
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = "Send opcode only."]
    #[inline]
    pub fn _1b(self) -> &'a mut W {
        self.variant(LENGTHW::_1B)
    }
    #[doc = "Send opcode, CINSTRDAT0.BYTE0."]
    #[inline]
    pub fn _2b(self) -> &'a mut W {
        self.variant(LENGTHW::_2B)
    }
    #[doc = "Send opcode, CINSTRDAT0.BYTE0 -&gt; CINSTRDAT0.BYTE1."]
    #[inline]
    pub fn _3b(self) -> &'a mut W {
        self.variant(LENGTHW::_3B)
    }
    #[doc = "Send opcode, CINSTRDAT0.BYTE0 -&gt; CINSTRDAT0.BYTE2."]
    #[inline]
    pub fn _4b(self) -> &'a mut W {
        self.variant(LENGTHW::_4B)
    }
    #[doc = "Send opcode, CINSTRDAT0.BYTE0 -&gt; CINSTRDAT0.BYTE3."]
    #[inline]
    pub fn _5b(self) -> &'a mut W {
        self.variant(LENGTHW::_5B)
    }
    #[doc = "Send opcode, CINSTRDAT0.BYTE0 -&gt; CINSTRDAT1.BYTE4."]
    #[inline]
    pub fn _6b(self) -> &'a mut W {
        self.variant(LENGTHW::_6B)
    }
    #[doc = "Send opcode, CINSTRDAT0.BYTE0 -&gt; CINSTRDAT1.BYTE5."]
    #[inline]
    pub fn _7b(self) -> &'a mut W {
        self.variant(LENGTHW::_7B)
    }
    #[doc = "Send opcode, CINSTRDAT0.BYTE0 -&gt; CINSTRDAT1.BYTE6."]
    #[inline]
    pub fn _8b(self) -> &'a mut W {
        self.variant(LENGTHW::_8B)
    }
    #[doc = "Send opcode, CINSTRDAT0.BYTE0 -&gt; CINSTRDAT1.BYTE7."]
    #[inline]
    pub fn _9b(self) -> &'a mut W {
        self.variant(LENGTHW::_9B)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 15;
        const OFFSET: u8 = 8;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _LIO2W<'a> {
    w: &'a mut W,
}
impl<'a> _LIO2W<'a> {
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
        const OFFSET: u8 = 12;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _LIO3W<'a> {
    w: &'a mut W,
}
impl<'a> _LIO3W<'a> {
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
        const OFFSET: u8 = 13;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `WIPWAIT`"]
pub enum WIPWAITW {
    #[doc = "No wait."]
    DISABLE,
    #[doc = "Wait."]
    ENABLE,
}
impl WIPWAITW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            WIPWAITW::DISABLE => false,
            WIPWAITW::ENABLE => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _WIPWAITW<'a> {
    w: &'a mut W,
}
impl<'a> _WIPWAITW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: WIPWAITW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "No wait."]
    #[inline]
    pub fn disable(self) -> &'a mut W {
        self.variant(WIPWAITW::DISABLE)
    }
    #[doc = "Wait."]
    #[inline]
    pub fn enable(self) -> &'a mut W {
        self.variant(WIPWAITW::ENABLE)
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
        const OFFSET: u8 = 14;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `WREN`"]
pub enum WRENW {
    #[doc = "Do not send WREN."]
    DISABLE,
    #[doc = "Send WREN."]
    ENABLE,
}
impl WRENW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            WRENW::DISABLE => false,
            WRENW::ENABLE => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _WRENW<'a> {
    w: &'a mut W,
}
impl<'a> _WRENW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: WRENW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Do not send WREN."]
    #[inline]
    pub fn disable(self) -> &'a mut W {
        self.variant(WRENW::DISABLE)
    }
    #[doc = "Send WREN."]
    #[inline]
    pub fn enable(self) -> &'a mut W {
        self.variant(WRENW::ENABLE)
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
        const OFFSET: u8 = 15;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `LFEN`"]
pub enum LFENW {
    #[doc = "Long frame mode disabled"]
    DISABLE,
    #[doc = "Long frame mode enabled"]
    ENABLE,
}
impl LFENW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            LFENW::DISABLE => false,
            LFENW::ENABLE => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _LFENW<'a> {
    w: &'a mut W,
}
impl<'a> _LFENW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: LFENW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Long frame mode disabled"]
    #[inline]
    pub fn disable(self) -> &'a mut W {
        self.variant(LFENW::DISABLE)
    }
    #[doc = "Long frame mode enabled"]
    #[inline]
    pub fn enable(self) -> &'a mut W {
        self.variant(LFENW::ENABLE)
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
        const OFFSET: u8 = 16;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `LFSTOP`"]
pub enum LFSTOPW {
    #[doc = "Stop"]
    STOP,
}
impl LFSTOPW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            LFSTOPW::STOP => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _LFSTOPW<'a> {
    w: &'a mut W,
}
impl<'a> _LFSTOPW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: LFSTOPW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Stop"]
    #[inline]
    pub fn stop(self) -> &'a mut W {
        self.variant(LFSTOPW::STOP)
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
        const OFFSET: u8 = 17;
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
    #[doc = "Bits 0:7 - Opcode of Custom instruction."]
    #[inline]
    pub fn opcode(&self) -> OPCODER {
        let bits = {
            const MASK: u8 = 255;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        OPCODER { bits }
    }
    #[doc = "Bits 8:11 - Length of custom instruction in number of bytes."]
    #[inline]
    pub fn length(&self) -> LENGTHR {
        LENGTHR::_from({
            const MASK: u8 = 15;
            const OFFSET: u8 = 8;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bit 12 - Level of the IO2 pin (if connected) during transmission of custom instruction."]
    #[inline]
    pub fn lio2(&self) -> LIO2R {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 12;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        LIO2R { bits }
    }
    #[doc = "Bit 13 - Level of the IO3 pin (if connected) during transmission of custom instruction."]
    #[inline]
    pub fn lio3(&self) -> LIO3R {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 13;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        LIO3R { bits }
    }
    #[doc = "Bit 14 - Wait for write complete before sending command."]
    #[inline]
    pub fn wipwait(&self) -> WIPWAITR {
        WIPWAITR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 14;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 15 - Send WREN (write enable opcode 0x06) before instruction."]
    #[inline]
    pub fn wren(&self) -> WRENR {
        WRENR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 15;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 16 - Enable long frame mode. When enabled, a custom instruction transaction has to be ended by writing the LFSTOP field."]
    #[inline]
    pub fn lfen(&self) -> LFENR {
        LFENR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 16;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 17 - Stop (finalize) long frame transaction"]
    #[inline]
    pub fn lfstop(&self) -> LFSTOPR {
        LFSTOPR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 17;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
}
impl W {
    #[doc = r" Reset value of the register"]
    #[inline]
    pub fn reset_value() -> W {
        W { bits: 8192 }
    }
    #[doc = r" Writes raw bits to the register"]
    #[inline]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
    #[doc = "Bits 0:7 - Opcode of Custom instruction."]
    #[inline]
    pub fn opcode(&mut self) -> _OPCODEW {
        _OPCODEW { w: self }
    }
    #[doc = "Bits 8:11 - Length of custom instruction in number of bytes."]
    #[inline]
    pub fn length(&mut self) -> _LENGTHW {
        _LENGTHW { w: self }
    }
    #[doc = "Bit 12 - Level of the IO2 pin (if connected) during transmission of custom instruction."]
    #[inline]
    pub fn lio2(&mut self) -> _LIO2W {
        _LIO2W { w: self }
    }
    #[doc = "Bit 13 - Level of the IO3 pin (if connected) during transmission of custom instruction."]
    #[inline]
    pub fn lio3(&mut self) -> _LIO3W {
        _LIO3W { w: self }
    }
    #[doc = "Bit 14 - Wait for write complete before sending command."]
    #[inline]
    pub fn wipwait(&mut self) -> _WIPWAITW {
        _WIPWAITW { w: self }
    }
    #[doc = "Bit 15 - Send WREN (write enable opcode 0x06) before instruction."]
    #[inline]
    pub fn wren(&mut self) -> _WRENW {
        _WRENW { w: self }
    }
    #[doc = "Bit 16 - Enable long frame mode. When enabled, a custom instruction transaction has to be ended by writing the LFSTOP field."]
    #[inline]
    pub fn lfen(&mut self) -> _LFENW {
        _LFENW { w: self }
    }
    #[doc = "Bit 17 - Stop (finalize) long frame transaction"]
    #[inline]
    pub fn lfstop(&mut self) -> _LFSTOPW {
        _LFSTOPW { w: self }
    }
}
