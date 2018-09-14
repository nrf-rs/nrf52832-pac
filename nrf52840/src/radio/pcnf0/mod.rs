#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::PCNF0 {
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
pub struct LFLENR {
    bits: u8,
}
impl LFLENR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct S0LENR {
    bits: bool,
}
impl S0LENR {
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
pub struct S1LENR {
    bits: u8,
}
impl S1LENR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = "Possible values of the field `S1INCL`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum S1INCLR {
    #[doc = "Include S1 field in RAM only if S1LEN &gt; 0"]
    AUTOMATIC,
    #[doc = "Always include S1 field in RAM independent of S1LEN"]
    INCLUDE,
}
impl S1INCLR {
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
            S1INCLR::AUTOMATIC => false,
            S1INCLR::INCLUDE => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> S1INCLR {
        match value {
            false => S1INCLR::AUTOMATIC,
            true => S1INCLR::INCLUDE,
        }
    }
    #[doc = "Checks if the value of the field is `AUTOMATIC`"]
    #[inline]
    pub fn is_automatic(&self) -> bool {
        *self == S1INCLR::AUTOMATIC
    }
    #[doc = "Checks if the value of the field is `INCLUDE`"]
    #[inline]
    pub fn is_include(&self) -> bool {
        *self == S1INCLR::INCLUDE
    }
}
#[doc = r" Value of the field"]
pub struct CILENR {
    bits: u8,
}
impl CILENR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = "Possible values of the field `PLEN`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PLENR {
    #[doc = "8-bit preamble"]
    _8BIT,
    #[doc = "16-bit preamble"]
    _16BIT,
    #[doc = "32-bit zero preamble - used for IEEE 802.15.4"]
    _32BITZERO,
    #[doc = "Preamble - used for BLE long range"]
    LONGRANGE,
}
impl PLENR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            PLENR::_8BIT => 0,
            PLENR::_16BIT => 1,
            PLENR::_32BITZERO => 2,
            PLENR::LONGRANGE => 3,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> PLENR {
        match value {
            0 => PLENR::_8BIT,
            1 => PLENR::_16BIT,
            2 => PLENR::_32BITZERO,
            3 => PLENR::LONGRANGE,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `_8BIT`"]
    #[inline]
    pub fn is_8bit(&self) -> bool {
        *self == PLENR::_8BIT
    }
    #[doc = "Checks if the value of the field is `_16BIT`"]
    #[inline]
    pub fn is_16bit(&self) -> bool {
        *self == PLENR::_16BIT
    }
    #[doc = "Checks if the value of the field is `_32BITZERO`"]
    #[inline]
    pub fn is_32bit_zero(&self) -> bool {
        *self == PLENR::_32BITZERO
    }
    #[doc = "Checks if the value of the field is `LONGRANGE`"]
    #[inline]
    pub fn is_long_range(&self) -> bool {
        *self == PLENR::LONGRANGE
    }
}
#[doc = "Possible values of the field `CRCINC`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CRCINCR {
    #[doc = "LENGTH does not contain CRC"]
    EXCLUDE,
    #[doc = "LENGTH includes CRC"]
    INCLUDE,
}
impl CRCINCR {
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
            CRCINCR::EXCLUDE => false,
            CRCINCR::INCLUDE => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> CRCINCR {
        match value {
            false => CRCINCR::EXCLUDE,
            true => CRCINCR::INCLUDE,
        }
    }
    #[doc = "Checks if the value of the field is `EXCLUDE`"]
    #[inline]
    pub fn is_exclude(&self) -> bool {
        *self == CRCINCR::EXCLUDE
    }
    #[doc = "Checks if the value of the field is `INCLUDE`"]
    #[inline]
    pub fn is_include(&self) -> bool {
        *self == CRCINCR::INCLUDE
    }
}
#[doc = r" Value of the field"]
pub struct TERMLENR {
    bits: u8,
}
impl TERMLENR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Proxy"]
pub struct _LFLENW<'a> {
    w: &'a mut W,
}
impl<'a> _LFLENW<'a> {
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 15;
        const OFFSET: u8 = 0;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _S0LENW<'a> {
    w: &'a mut W,
}
impl<'a> _S0LENW<'a> {
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
#[doc = r" Proxy"]
pub struct _S1LENW<'a> {
    w: &'a mut W,
}
impl<'a> _S1LENW<'a> {
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 15;
        const OFFSET: u8 = 16;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `S1INCL`"]
pub enum S1INCLW {
    #[doc = "Include S1 field in RAM only if S1LEN &gt; 0"]
    AUTOMATIC,
    #[doc = "Always include S1 field in RAM independent of S1LEN"]
    INCLUDE,
}
impl S1INCLW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            S1INCLW::AUTOMATIC => false,
            S1INCLW::INCLUDE => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _S1INCLW<'a> {
    w: &'a mut W,
}
impl<'a> _S1INCLW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: S1INCLW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Include S1 field in RAM only if S1LEN &gt; 0"]
    #[inline]
    pub fn automatic(self) -> &'a mut W {
        self.variant(S1INCLW::AUTOMATIC)
    }
    #[doc = "Always include S1 field in RAM independent of S1LEN"]
    #[inline]
    pub fn include(self) -> &'a mut W {
        self.variant(S1INCLW::INCLUDE)
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
        const OFFSET: u8 = 20;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _CILENW<'a> {
    w: &'a mut W,
}
impl<'a> _CILENW<'a> {
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 3;
        const OFFSET: u8 = 22;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `PLEN`"]
pub enum PLENW {
    #[doc = "8-bit preamble"]
    _8BIT,
    #[doc = "16-bit preamble"]
    _16BIT,
    #[doc = "32-bit zero preamble - used for IEEE 802.15.4"]
    _32BITZERO,
    #[doc = "Preamble - used for BLE long range"]
    LONGRANGE,
}
impl PLENW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            PLENW::_8BIT => 0,
            PLENW::_16BIT => 1,
            PLENW::_32BITZERO => 2,
            PLENW::LONGRANGE => 3,
        }
    }
}
#[doc = r" Proxy"]
pub struct _PLENW<'a> {
    w: &'a mut W,
}
impl<'a> _PLENW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: PLENW) -> &'a mut W {
        {
            self.bits(variant._bits())
        }
    }
    #[doc = "8-bit preamble"]
    #[inline]
    pub fn _8bit(self) -> &'a mut W {
        self.variant(PLENW::_8BIT)
    }
    #[doc = "16-bit preamble"]
    #[inline]
    pub fn _16bit(self) -> &'a mut W {
        self.variant(PLENW::_16BIT)
    }
    #[doc = "32-bit zero preamble - used for IEEE 802.15.4"]
    #[inline]
    pub fn _32bit_zero(self) -> &'a mut W {
        self.variant(PLENW::_32BITZERO)
    }
    #[doc = "Preamble - used for BLE long range"]
    #[inline]
    pub fn long_range(self) -> &'a mut W {
        self.variant(PLENW::LONGRANGE)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 3;
        const OFFSET: u8 = 24;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `CRCINC`"]
pub enum CRCINCW {
    #[doc = "LENGTH does not contain CRC"]
    EXCLUDE,
    #[doc = "LENGTH includes CRC"]
    INCLUDE,
}
impl CRCINCW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            CRCINCW::EXCLUDE => false,
            CRCINCW::INCLUDE => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _CRCINCW<'a> {
    w: &'a mut W,
}
impl<'a> _CRCINCW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: CRCINCW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "LENGTH does not contain CRC"]
    #[inline]
    pub fn exclude(self) -> &'a mut W {
        self.variant(CRCINCW::EXCLUDE)
    }
    #[doc = "LENGTH includes CRC"]
    #[inline]
    pub fn include(self) -> &'a mut W {
        self.variant(CRCINCW::INCLUDE)
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
        const OFFSET: u8 = 26;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _TERMLENW<'a> {
    w: &'a mut W,
}
impl<'a> _TERMLENW<'a> {
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 3;
        const OFFSET: u8 = 29;
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
    #[doc = "Bits 0:3 - Length on air of LENGTH field in number of bits."]
    #[inline]
    pub fn lflen(&self) -> LFLENR {
        let bits = {
            const MASK: u8 = 15;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        LFLENR { bits }
    }
    #[doc = "Bit 8 - Length on air of S0 field in number of bytes."]
    #[inline]
    pub fn s0len(&self) -> S0LENR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 8;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        S0LENR { bits }
    }
    #[doc = "Bits 16:19 - Length on air of S1 field in number of bits."]
    #[inline]
    pub fn s1len(&self) -> S1LENR {
        let bits = {
            const MASK: u8 = 15;
            const OFFSET: u8 = 16;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        S1LENR { bits }
    }
    #[doc = "Bit 20 - Include or exclude S1 field in RAM"]
    #[inline]
    pub fn s1incl(&self) -> S1INCLR {
        S1INCLR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 20;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bits 22:23 - Length of code indicator - long range"]
    #[inline]
    pub fn cilen(&self) -> CILENR {
        let bits = {
            const MASK: u8 = 3;
            const OFFSET: u8 = 22;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        CILENR { bits }
    }
    #[doc = "Bits 24:25 - Length of preamble on air. Decision point: TASKS_START task"]
    #[inline]
    pub fn plen(&self) -> PLENR {
        PLENR::_from({
            const MASK: u8 = 3;
            const OFFSET: u8 = 24;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bit 26 - Indicates if LENGTH field contains CRC or not"]
    #[inline]
    pub fn crcinc(&self) -> CRCINCR {
        CRCINCR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 26;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bits 29:30 - Length of TERM field in Long Range operation"]
    #[inline]
    pub fn termlen(&self) -> TERMLENR {
        let bits = {
            const MASK: u8 = 3;
            const OFFSET: u8 = 29;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        TERMLENR { bits }
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
    #[doc = "Bits 0:3 - Length on air of LENGTH field in number of bits."]
    #[inline]
    pub fn lflen(&mut self) -> _LFLENW {
        _LFLENW { w: self }
    }
    #[doc = "Bit 8 - Length on air of S0 field in number of bytes."]
    #[inline]
    pub fn s0len(&mut self) -> _S0LENW {
        _S0LENW { w: self }
    }
    #[doc = "Bits 16:19 - Length on air of S1 field in number of bits."]
    #[inline]
    pub fn s1len(&mut self) -> _S1LENW {
        _S1LENW { w: self }
    }
    #[doc = "Bit 20 - Include or exclude S1 field in RAM"]
    #[inline]
    pub fn s1incl(&mut self) -> _S1INCLW {
        _S1INCLW { w: self }
    }
    #[doc = "Bits 22:23 - Length of code indicator - long range"]
    #[inline]
    pub fn cilen(&mut self) -> _CILENW {
        _CILENW { w: self }
    }
    #[doc = "Bits 24:25 - Length of preamble on air. Decision point: TASKS_START task"]
    #[inline]
    pub fn plen(&mut self) -> _PLENW {
        _PLENW { w: self }
    }
    #[doc = "Bit 26 - Indicates if LENGTH field contains CRC or not"]
    #[inline]
    pub fn crcinc(&mut self) -> _CRCINCW {
        _CRCINCW { w: self }
    }
    #[doc = "Bits 29:30 - Length of TERM field in Long Range operation"]
    #[inline]
    pub fn termlen(&mut self) -> _TERMLENW {
        _TERMLENW { w: self }
    }
}
