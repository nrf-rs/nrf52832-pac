#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::RESETREAS {
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
#[doc = "Possible values of the field `RESETPIN`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RESETPINR {
    #[doc = "Reset not detected."]
    NOTDETECTED,
    #[doc = "Reset detected."]
    DETECTED,
}
impl RESETPINR {
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
            RESETPINR::NOTDETECTED => false,
            RESETPINR::DETECTED => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> RESETPINR {
        match value {
            false => RESETPINR::NOTDETECTED,
            true => RESETPINR::DETECTED,
        }
    }
    #[doc = "Checks if the value of the field is `NOTDETECTED`"]
    #[inline]
    pub fn is_not_detected(&self) -> bool {
        *self == RESETPINR::NOTDETECTED
    }
    #[doc = "Checks if the value of the field is `DETECTED`"]
    #[inline]
    pub fn is_detected(&self) -> bool {
        *self == RESETPINR::DETECTED
    }
}
#[doc = "Possible values of the field `DOG`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DOGR {
    #[doc = "Reset not detected."]
    NOTDETECTED,
    #[doc = "Reset detected."]
    DETECTED,
}
impl DOGR {
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
            DOGR::NOTDETECTED => false,
            DOGR::DETECTED => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> DOGR {
        match value {
            false => DOGR::NOTDETECTED,
            true => DOGR::DETECTED,
        }
    }
    #[doc = "Checks if the value of the field is `NOTDETECTED`"]
    #[inline]
    pub fn is_not_detected(&self) -> bool {
        *self == DOGR::NOTDETECTED
    }
    #[doc = "Checks if the value of the field is `DETECTED`"]
    #[inline]
    pub fn is_detected(&self) -> bool {
        *self == DOGR::DETECTED
    }
}
#[doc = "Possible values of the field `SREQ`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SREQR {
    #[doc = "Reset not detected."]
    NOTDETECTED,
    #[doc = "Reset detected."]
    DETECTED,
}
impl SREQR {
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
            SREQR::NOTDETECTED => false,
            SREQR::DETECTED => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> SREQR {
        match value {
            false => SREQR::NOTDETECTED,
            true => SREQR::DETECTED,
        }
    }
    #[doc = "Checks if the value of the field is `NOTDETECTED`"]
    #[inline]
    pub fn is_not_detected(&self) -> bool {
        *self == SREQR::NOTDETECTED
    }
    #[doc = "Checks if the value of the field is `DETECTED`"]
    #[inline]
    pub fn is_detected(&self) -> bool {
        *self == SREQR::DETECTED
    }
}
#[doc = "Possible values of the field `LOCKUP`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum LOCKUPR {
    #[doc = "Reset not detected."]
    NOTDETECTED,
    #[doc = "Reset detected."]
    DETECTED,
}
impl LOCKUPR {
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
            LOCKUPR::NOTDETECTED => false,
            LOCKUPR::DETECTED => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> LOCKUPR {
        match value {
            false => LOCKUPR::NOTDETECTED,
            true => LOCKUPR::DETECTED,
        }
    }
    #[doc = "Checks if the value of the field is `NOTDETECTED`"]
    #[inline]
    pub fn is_not_detected(&self) -> bool {
        *self == LOCKUPR::NOTDETECTED
    }
    #[doc = "Checks if the value of the field is `DETECTED`"]
    #[inline]
    pub fn is_detected(&self) -> bool {
        *self == LOCKUPR::DETECTED
    }
}
#[doc = "Possible values of the field `OFF`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum OFFR {
    #[doc = "Reset not detected."]
    NOTDETECTED,
    #[doc = "Reset detected."]
    DETECTED,
}
impl OFFR {
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
            OFFR::NOTDETECTED => false,
            OFFR::DETECTED => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> OFFR {
        match value {
            false => OFFR::NOTDETECTED,
            true => OFFR::DETECTED,
        }
    }
    #[doc = "Checks if the value of the field is `NOTDETECTED`"]
    #[inline]
    pub fn is_not_detected(&self) -> bool {
        *self == OFFR::NOTDETECTED
    }
    #[doc = "Checks if the value of the field is `DETECTED`"]
    #[inline]
    pub fn is_detected(&self) -> bool {
        *self == OFFR::DETECTED
    }
}
#[doc = "Possible values of the field `LPCOMP`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum LPCOMPR {
    #[doc = "Reset not detected."]
    NOTDETECTED,
    #[doc = "Reset detected."]
    DETECTED,
}
impl LPCOMPR {
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
            LPCOMPR::NOTDETECTED => false,
            LPCOMPR::DETECTED => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> LPCOMPR {
        match value {
            false => LPCOMPR::NOTDETECTED,
            true => LPCOMPR::DETECTED,
        }
    }
    #[doc = "Checks if the value of the field is `NOTDETECTED`"]
    #[inline]
    pub fn is_not_detected(&self) -> bool {
        *self == LPCOMPR::NOTDETECTED
    }
    #[doc = "Checks if the value of the field is `DETECTED`"]
    #[inline]
    pub fn is_detected(&self) -> bool {
        *self == LPCOMPR::DETECTED
    }
}
#[doc = "Possible values of the field `DIF`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DIFR {
    #[doc = "Reset not detected."]
    NOTDETECTED,
    #[doc = "Reset detected."]
    DETECTED,
}
impl DIFR {
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
            DIFR::NOTDETECTED => false,
            DIFR::DETECTED => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> DIFR {
        match value {
            false => DIFR::NOTDETECTED,
            true => DIFR::DETECTED,
        }
    }
    #[doc = "Checks if the value of the field is `NOTDETECTED`"]
    #[inline]
    pub fn is_not_detected(&self) -> bool {
        *self == DIFR::NOTDETECTED
    }
    #[doc = "Checks if the value of the field is `DETECTED`"]
    #[inline]
    pub fn is_detected(&self) -> bool {
        *self == DIFR::DETECTED
    }
}
#[doc = "Values that can be written to the field `RESETPIN`"]
pub enum RESETPINW {
    #[doc = "Reset not detected."]
    NOTDETECTED,
    #[doc = "Reset detected."]
    DETECTED,
}
impl RESETPINW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            RESETPINW::NOTDETECTED => false,
            RESETPINW::DETECTED => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _RESETPINW<'a> {
    w: &'a mut W,
}
impl<'a> _RESETPINW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: RESETPINW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Reset not detected."]
    #[inline]
    pub fn not_detected(self) -> &'a mut W {
        self.variant(RESETPINW::NOTDETECTED)
    }
    #[doc = "Reset detected."]
    #[inline]
    pub fn detected(self) -> &'a mut W {
        self.variant(RESETPINW::DETECTED)
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
#[doc = "Values that can be written to the field `DOG`"]
pub enum DOGW {
    #[doc = "Reset not detected."]
    NOTDETECTED,
    #[doc = "Reset detected."]
    DETECTED,
}
impl DOGW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            DOGW::NOTDETECTED => false,
            DOGW::DETECTED => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _DOGW<'a> {
    w: &'a mut W,
}
impl<'a> _DOGW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: DOGW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Reset not detected."]
    #[inline]
    pub fn not_detected(self) -> &'a mut W {
        self.variant(DOGW::NOTDETECTED)
    }
    #[doc = "Reset detected."]
    #[inline]
    pub fn detected(self) -> &'a mut W {
        self.variant(DOGW::DETECTED)
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
#[doc = "Values that can be written to the field `SREQ`"]
pub enum SREQW {
    #[doc = "Reset not detected."]
    NOTDETECTED,
    #[doc = "Reset detected."]
    DETECTED,
}
impl SREQW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            SREQW::NOTDETECTED => false,
            SREQW::DETECTED => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _SREQW<'a> {
    w: &'a mut W,
}
impl<'a> _SREQW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: SREQW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Reset not detected."]
    #[inline]
    pub fn not_detected(self) -> &'a mut W {
        self.variant(SREQW::NOTDETECTED)
    }
    #[doc = "Reset detected."]
    #[inline]
    pub fn detected(self) -> &'a mut W {
        self.variant(SREQW::DETECTED)
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
#[doc = "Values that can be written to the field `LOCKUP`"]
pub enum LOCKUPW {
    #[doc = "Reset not detected."]
    NOTDETECTED,
    #[doc = "Reset detected."]
    DETECTED,
}
impl LOCKUPW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            LOCKUPW::NOTDETECTED => false,
            LOCKUPW::DETECTED => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _LOCKUPW<'a> {
    w: &'a mut W,
}
impl<'a> _LOCKUPW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: LOCKUPW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Reset not detected."]
    #[inline]
    pub fn not_detected(self) -> &'a mut W {
        self.variant(LOCKUPW::NOTDETECTED)
    }
    #[doc = "Reset detected."]
    #[inline]
    pub fn detected(self) -> &'a mut W {
        self.variant(LOCKUPW::DETECTED)
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
#[doc = "Values that can be written to the field `OFF`"]
pub enum OFFW {
    #[doc = "Reset not detected."]
    NOTDETECTED,
    #[doc = "Reset detected."]
    DETECTED,
}
impl OFFW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            OFFW::NOTDETECTED => false,
            OFFW::DETECTED => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _OFFW<'a> {
    w: &'a mut W,
}
impl<'a> _OFFW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: OFFW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Reset not detected."]
    #[inline]
    pub fn not_detected(self) -> &'a mut W {
        self.variant(OFFW::NOTDETECTED)
    }
    #[doc = "Reset detected."]
    #[inline]
    pub fn detected(self) -> &'a mut W {
        self.variant(OFFW::DETECTED)
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
#[doc = "Values that can be written to the field `LPCOMP`"]
pub enum LPCOMPW {
    #[doc = "Reset not detected."]
    NOTDETECTED,
    #[doc = "Reset detected."]
    DETECTED,
}
impl LPCOMPW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            LPCOMPW::NOTDETECTED => false,
            LPCOMPW::DETECTED => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _LPCOMPW<'a> {
    w: &'a mut W,
}
impl<'a> _LPCOMPW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: LPCOMPW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Reset not detected."]
    #[inline]
    pub fn not_detected(self) -> &'a mut W {
        self.variant(LPCOMPW::NOTDETECTED)
    }
    #[doc = "Reset detected."]
    #[inline]
    pub fn detected(self) -> &'a mut W {
        self.variant(LPCOMPW::DETECTED)
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
#[doc = "Values that can be written to the field `DIF`"]
pub enum DIFW {
    #[doc = "Reset not detected."]
    NOTDETECTED,
    #[doc = "Reset detected."]
    DETECTED,
}
impl DIFW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            DIFW::NOTDETECTED => false,
            DIFW::DETECTED => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _DIFW<'a> {
    w: &'a mut W,
}
impl<'a> _DIFW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: DIFW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Reset not detected."]
    #[inline]
    pub fn not_detected(self) -> &'a mut W {
        self.variant(DIFW::NOTDETECTED)
    }
    #[doc = "Reset detected."]
    #[inline]
    pub fn detected(self) -> &'a mut W {
        self.variant(DIFW::DETECTED)
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
        const OFFSET: u8 = 18;
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
    #[doc = "Bit 0 - Reset from pin-reset detected."]
    #[inline]
    pub fn resetpin(&self) -> RESETPINR {
        RESETPINR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 1 - Reset from watchdog detected."]
    #[inline]
    pub fn dog(&self) -> DOGR {
        DOGR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 1;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 2 - Reset from AIRCR.SYSRESETREQ detected."]
    #[inline]
    pub fn sreq(&self) -> SREQR {
        SREQR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 2;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 3 - Reset from CPU lock-up detected."]
    #[inline]
    pub fn lockup(&self) -> LOCKUPR {
        LOCKUPR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 3;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 16 - Reset from wake-up from OFF mode detected by the use of DETECT signal from GPIO."]
    #[inline]
    pub fn off(&self) -> OFFR {
        OFFR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 16;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 17 - Reset from wake-up from OFF mode detected by the use of ANADETECT signal from LPCOMP."]
    #[inline]
    pub fn lpcomp(&self) -> LPCOMPR {
        LPCOMPR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 17;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 18 - Reset from wake-up from OFF mode detected by entering into debug interface mode."]
    #[inline]
    pub fn dif(&self) -> DIFR {
        DIFR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 18;
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
    #[doc = "Bit 0 - Reset from pin-reset detected."]
    #[inline]
    pub fn resetpin(&mut self) -> _RESETPINW {
        _RESETPINW { w: self }
    }
    #[doc = "Bit 1 - Reset from watchdog detected."]
    #[inline]
    pub fn dog(&mut self) -> _DOGW {
        _DOGW { w: self }
    }
    #[doc = "Bit 2 - Reset from AIRCR.SYSRESETREQ detected."]
    #[inline]
    pub fn sreq(&mut self) -> _SREQW {
        _SREQW { w: self }
    }
    #[doc = "Bit 3 - Reset from CPU lock-up detected."]
    #[inline]
    pub fn lockup(&mut self) -> _LOCKUPW {
        _LOCKUPW { w: self }
    }
    #[doc = "Bit 16 - Reset from wake-up from OFF mode detected by the use of DETECT signal from GPIO."]
    #[inline]
    pub fn off(&mut self) -> _OFFW {
        _OFFW { w: self }
    }
    #[doc = "Bit 17 - Reset from wake-up from OFF mode detected by the use of ANADETECT signal from LPCOMP."]
    #[inline]
    pub fn lpcomp(&mut self) -> _LPCOMPW {
        _LPCOMPW { w: self }
    }
    #[doc = "Bit 18 - Reset from wake-up from OFF mode detected by entering into debug interface mode."]
    #[inline]
    pub fn dif(&mut self) -> _DIFW {
        _DIFW { w: self }
    }
}
