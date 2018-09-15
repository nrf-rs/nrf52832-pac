#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::PIN_CNF {
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
#[doc = "Possible values of the field `DIR`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DIRR {
    #[doc = "Configure pin as an input pin."]
    INPUT,
    #[doc = "Configure pin as an output pin."]
    OUTPUT,
}
impl DIRR {
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
            DIRR::INPUT => false,
            DIRR::OUTPUT => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> DIRR {
        match value {
            false => DIRR::INPUT,
            true => DIRR::OUTPUT,
        }
    }
    #[doc = "Checks if the value of the field is `INPUT`"]
    #[inline]
    pub fn is_input(&self) -> bool {
        *self == DIRR::INPUT
    }
    #[doc = "Checks if the value of the field is `OUTPUT`"]
    #[inline]
    pub fn is_output(&self) -> bool {
        *self == DIRR::OUTPUT
    }
}
#[doc = "Possible values of the field `INPUT`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum INPUTR {
    #[doc = "Connect input pin."]
    CONNECT,
    #[doc = "Disconnect input pin."]
    DISCONNECT,
}
impl INPUTR {
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
            INPUTR::CONNECT => false,
            INPUTR::DISCONNECT => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> INPUTR {
        match value {
            false => INPUTR::CONNECT,
            true => INPUTR::DISCONNECT,
        }
    }
    #[doc = "Checks if the value of the field is `CONNECT`"]
    #[inline]
    pub fn is_connect(&self) -> bool {
        *self == INPUTR::CONNECT
    }
    #[doc = "Checks if the value of the field is `DISCONNECT`"]
    #[inline]
    pub fn is_disconnect(&self) -> bool {
        *self == INPUTR::DISCONNECT
    }
}
#[doc = "Possible values of the field `PULL`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PULLR {
    #[doc = "No pull."]
    DISABLED,
    #[doc = "Pulldown on pin."]
    PULLDOWN,
    #[doc = "Pullup on pin."]
    PULLUP,
    #[doc = r" Reserved"]
    _Reserved(u8),
}
impl PULLR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            PULLR::DISABLED => 0,
            PULLR::PULLDOWN => 1,
            PULLR::PULLUP => 3,
            PULLR::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> PULLR {
        match value {
            0 => PULLR::DISABLED,
            1 => PULLR::PULLDOWN,
            3 => PULLR::PULLUP,
            i => PULLR::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline]
    pub fn is_disabled(&self) -> bool {
        *self == PULLR::DISABLED
    }
    #[doc = "Checks if the value of the field is `PULLDOWN`"]
    #[inline]
    pub fn is_pulldown(&self) -> bool {
        *self == PULLR::PULLDOWN
    }
    #[doc = "Checks if the value of the field is `PULLUP`"]
    #[inline]
    pub fn is_pullup(&self) -> bool {
        *self == PULLR::PULLUP
    }
}
#[doc = "Possible values of the field `DRIVE`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DRIVER {
    #[doc = "Standard '0', Standard '1'."]
    S0S1,
    #[doc = "High '0', Standard '1'."]
    H0S1,
    #[doc = "Standard '0', High '1'."]
    S0H1,
    #[doc = "High '0', High '1'."]
    H0H1,
    #[doc = "Disconnected '0', Standard '1'."]
    D0S1,
    #[doc = "Disconnected '0', High '1'."]
    D0H1,
    #[doc = "Standard '0', Disconnected '1'."]
    S0D1,
    #[doc = "High '0', Disconnected '1'."]
    H0D1,
}
impl DRIVER {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            DRIVER::S0S1 => 0,
            DRIVER::H0S1 => 1,
            DRIVER::S0H1 => 2,
            DRIVER::H0H1 => 3,
            DRIVER::D0S1 => 4,
            DRIVER::D0H1 => 5,
            DRIVER::S0D1 => 6,
            DRIVER::H0D1 => 7,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> DRIVER {
        match value {
            0 => DRIVER::S0S1,
            1 => DRIVER::H0S1,
            2 => DRIVER::S0H1,
            3 => DRIVER::H0H1,
            4 => DRIVER::D0S1,
            5 => DRIVER::D0H1,
            6 => DRIVER::S0D1,
            7 => DRIVER::H0D1,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `S0S1`"]
    #[inline]
    pub fn is_s0s1(&self) -> bool {
        *self == DRIVER::S0S1
    }
    #[doc = "Checks if the value of the field is `H0S1`"]
    #[inline]
    pub fn is_h0s1(&self) -> bool {
        *self == DRIVER::H0S1
    }
    #[doc = "Checks if the value of the field is `S0H1`"]
    #[inline]
    pub fn is_s0h1(&self) -> bool {
        *self == DRIVER::S0H1
    }
    #[doc = "Checks if the value of the field is `H0H1`"]
    #[inline]
    pub fn is_h0h1(&self) -> bool {
        *self == DRIVER::H0H1
    }
    #[doc = "Checks if the value of the field is `D0S1`"]
    #[inline]
    pub fn is_d0s1(&self) -> bool {
        *self == DRIVER::D0S1
    }
    #[doc = "Checks if the value of the field is `D0H1`"]
    #[inline]
    pub fn is_d0h1(&self) -> bool {
        *self == DRIVER::D0H1
    }
    #[doc = "Checks if the value of the field is `S0D1`"]
    #[inline]
    pub fn is_s0d1(&self) -> bool {
        *self == DRIVER::S0D1
    }
    #[doc = "Checks if the value of the field is `H0D1`"]
    #[inline]
    pub fn is_h0d1(&self) -> bool {
        *self == DRIVER::H0D1
    }
}
#[doc = "Possible values of the field `SENSE`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SENSER {
    #[doc = "Disabled."]
    DISABLED,
    #[doc = "Wakeup on high level."]
    HIGH,
    #[doc = "Wakeup on low level."]
    LOW,
    #[doc = r" Reserved"]
    _Reserved(u8),
}
impl SENSER {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            SENSER::DISABLED => 0,
            SENSER::HIGH => 2,
            SENSER::LOW => 3,
            SENSER::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> SENSER {
        match value {
            0 => SENSER::DISABLED,
            2 => SENSER::HIGH,
            3 => SENSER::LOW,
            i => SENSER::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline]
    pub fn is_disabled(&self) -> bool {
        *self == SENSER::DISABLED
    }
    #[doc = "Checks if the value of the field is `HIGH`"]
    #[inline]
    pub fn is_high(&self) -> bool {
        *self == SENSER::HIGH
    }
    #[doc = "Checks if the value of the field is `LOW`"]
    #[inline]
    pub fn is_low(&self) -> bool {
        *self == SENSER::LOW
    }
}
#[doc = "Values that can be written to the field `DIR`"]
pub enum DIRW {
    #[doc = "Configure pin as an input pin."]
    INPUT,
    #[doc = "Configure pin as an output pin."]
    OUTPUT,
}
impl DIRW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            DIRW::INPUT => false,
            DIRW::OUTPUT => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _DIRW<'a> {
    w: &'a mut W,
}
impl<'a> _DIRW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: DIRW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Configure pin as an input pin."]
    #[inline]
    pub fn input(self) -> &'a mut W {
        self.variant(DIRW::INPUT)
    }
    #[doc = "Configure pin as an output pin."]
    #[inline]
    pub fn output(self) -> &'a mut W {
        self.variant(DIRW::OUTPUT)
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
#[doc = "Values that can be written to the field `INPUT`"]
pub enum INPUTW {
    #[doc = "Connect input pin."]
    CONNECT,
    #[doc = "Disconnect input pin."]
    DISCONNECT,
}
impl INPUTW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            INPUTW::CONNECT => false,
            INPUTW::DISCONNECT => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _INPUTW<'a> {
    w: &'a mut W,
}
impl<'a> _INPUTW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: INPUTW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Connect input pin."]
    #[inline]
    pub fn connect(self) -> &'a mut W {
        self.variant(INPUTW::CONNECT)
    }
    #[doc = "Disconnect input pin."]
    #[inline]
    pub fn disconnect(self) -> &'a mut W {
        self.variant(INPUTW::DISCONNECT)
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
#[doc = "Values that can be written to the field `PULL`"]
pub enum PULLW {
    #[doc = "No pull."]
    DISABLED,
    #[doc = "Pulldown on pin."]
    PULLDOWN,
    #[doc = "Pullup on pin."]
    PULLUP,
}
impl PULLW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            PULLW::DISABLED => 0,
            PULLW::PULLDOWN => 1,
            PULLW::PULLUP => 3,
        }
    }
}
#[doc = r" Proxy"]
pub struct _PULLW<'a> {
    w: &'a mut W,
}
impl<'a> _PULLW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: PULLW) -> &'a mut W {
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = "No pull."]
    #[inline]
    pub fn disabled(self) -> &'a mut W {
        self.variant(PULLW::DISABLED)
    }
    #[doc = "Pulldown on pin."]
    #[inline]
    pub fn pulldown(self) -> &'a mut W {
        self.variant(PULLW::PULLDOWN)
    }
    #[doc = "Pullup on pin."]
    #[inline]
    pub fn pullup(self) -> &'a mut W {
        self.variant(PULLW::PULLUP)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 3;
        const OFFSET: u8 = 2;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `DRIVE`"]
pub enum DRIVEW {
    #[doc = "Standard '0', Standard '1'."]
    S0S1,
    #[doc = "High '0', Standard '1'."]
    H0S1,
    #[doc = "Standard '0', High '1'."]
    S0H1,
    #[doc = "High '0', High '1'."]
    H0H1,
    #[doc = "Disconnected '0', Standard '1'."]
    D0S1,
    #[doc = "Disconnected '0', High '1'."]
    D0H1,
    #[doc = "Standard '0', Disconnected '1'."]
    S0D1,
    #[doc = "High '0', Disconnected '1'."]
    H0D1,
}
impl DRIVEW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            DRIVEW::S0S1 => 0,
            DRIVEW::H0S1 => 1,
            DRIVEW::S0H1 => 2,
            DRIVEW::H0H1 => 3,
            DRIVEW::D0S1 => 4,
            DRIVEW::D0H1 => 5,
            DRIVEW::S0D1 => 6,
            DRIVEW::H0D1 => 7,
        }
    }
}
#[doc = r" Proxy"]
pub struct _DRIVEW<'a> {
    w: &'a mut W,
}
impl<'a> _DRIVEW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: DRIVEW) -> &'a mut W {
        {
            self.bits(variant._bits())
        }
    }
    #[doc = "Standard '0', Standard '1'."]
    #[inline]
    pub fn s0s1(self) -> &'a mut W {
        self.variant(DRIVEW::S0S1)
    }
    #[doc = "High '0', Standard '1'."]
    #[inline]
    pub fn h0s1(self) -> &'a mut W {
        self.variant(DRIVEW::H0S1)
    }
    #[doc = "Standard '0', High '1'."]
    #[inline]
    pub fn s0h1(self) -> &'a mut W {
        self.variant(DRIVEW::S0H1)
    }
    #[doc = "High '0', High '1'."]
    #[inline]
    pub fn h0h1(self) -> &'a mut W {
        self.variant(DRIVEW::H0H1)
    }
    #[doc = "Disconnected '0', Standard '1'."]
    #[inline]
    pub fn d0s1(self) -> &'a mut W {
        self.variant(DRIVEW::D0S1)
    }
    #[doc = "Disconnected '0', High '1'."]
    #[inline]
    pub fn d0h1(self) -> &'a mut W {
        self.variant(DRIVEW::D0H1)
    }
    #[doc = "Standard '0', Disconnected '1'."]
    #[inline]
    pub fn s0d1(self) -> &'a mut W {
        self.variant(DRIVEW::S0D1)
    }
    #[doc = "High '0', Disconnected '1'."]
    #[inline]
    pub fn h0d1(self) -> &'a mut W {
        self.variant(DRIVEW::H0D1)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 7;
        const OFFSET: u8 = 8;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `SENSE`"]
pub enum SENSEW {
    #[doc = "Disabled."]
    DISABLED,
    #[doc = "Wakeup on high level."]
    HIGH,
    #[doc = "Wakeup on low level."]
    LOW,
}
impl SENSEW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            SENSEW::DISABLED => 0,
            SENSEW::HIGH => 2,
            SENSEW::LOW => 3,
        }
    }
}
#[doc = r" Proxy"]
pub struct _SENSEW<'a> {
    w: &'a mut W,
}
impl<'a> _SENSEW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: SENSEW) -> &'a mut W {
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = "Disabled."]
    #[inline]
    pub fn disabled(self) -> &'a mut W {
        self.variant(SENSEW::DISABLED)
    }
    #[doc = "Wakeup on high level."]
    #[inline]
    pub fn high(self) -> &'a mut W {
        self.variant(SENSEW::HIGH)
    }
    #[doc = "Wakeup on low level."]
    #[inline]
    pub fn low(self) -> &'a mut W {
        self.variant(SENSEW::LOW)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 3;
        const OFFSET: u8 = 16;
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
    #[doc = "Bit 0 - Pin direction."]
    #[inline]
    pub fn dir(&self) -> DIRR {
        DIRR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 1 - Connect or disconnect input path."]
    #[inline]
    pub fn input(&self) -> INPUTR {
        INPUTR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 1;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bits 2:3 - Pull-up or -down configuration."]
    #[inline]
    pub fn pull(&self) -> PULLR {
        PULLR::_from({
            const MASK: u8 = 3;
            const OFFSET: u8 = 2;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bits 8:10 - Drive configuration."]
    #[inline]
    pub fn drive(&self) -> DRIVER {
        DRIVER::_from({
            const MASK: u8 = 7;
            const OFFSET: u8 = 8;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bits 16:17 - Pin sensing mechanism."]
    #[inline]
    pub fn sense(&self) -> SENSER {
        SENSER::_from({
            const MASK: u8 = 3;
            const OFFSET: u8 = 16;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
}
impl W {
    #[doc = r" Reset value of the register"]
    #[inline]
    pub fn reset_value() -> W {
        W { bits: 2 }
    }
    #[doc = r" Writes raw bits to the register"]
    #[inline]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
    #[doc = "Bit 0 - Pin direction."]
    #[inline]
    pub fn dir(&mut self) -> _DIRW {
        _DIRW { w: self }
    }
    #[doc = "Bit 1 - Connect or disconnect input path."]
    #[inline]
    pub fn input(&mut self) -> _INPUTW {
        _INPUTW { w: self }
    }
    #[doc = "Bits 2:3 - Pull-up or -down configuration."]
    #[inline]
    pub fn pull(&mut self) -> _PULLW {
        _PULLW { w: self }
    }
    #[doc = "Bits 8:10 - Drive configuration."]
    #[inline]
    pub fn drive(&mut self) -> _DRIVEW {
        _DRIVEW { w: self }
    }
    #[doc = "Bits 16:17 - Pin sensing mechanism."]
    #[inline]
    pub fn sense(&mut self) -> _SENSEW {
        _SENSEW { w: self }
    }
}
