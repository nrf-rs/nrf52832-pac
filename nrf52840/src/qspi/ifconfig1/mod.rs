#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::IFCONFIG1 {
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
pub struct SCKDELAYR {
    bits: u8,
}
impl SCKDELAYR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = "Possible values of the field `DPMEN`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DPMENR {
    #[doc = "Exit DPM."]
    EXIT,
    #[doc = "Enter DPM."]
    ENTER,
}
impl DPMENR {
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
            DPMENR::EXIT => false,
            DPMENR::ENTER => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> DPMENR {
        match value {
            false => DPMENR::EXIT,
            true => DPMENR::ENTER,
        }
    }
    #[doc = "Checks if the value of the field is `EXIT`"]
    #[inline]
    pub fn is_exit(&self) -> bool {
        *self == DPMENR::EXIT
    }
    #[doc = "Checks if the value of the field is `ENTER`"]
    #[inline]
    pub fn is_enter(&self) -> bool {
        *self == DPMENR::ENTER
    }
}
#[doc = "Possible values of the field `SPIMODE`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SPIMODER {
    #[doc = "Mode 0: Data are captured on the clock rising edge and data is output on a falling edge. Base level of clock is 0 (CPOL=0, CPHA=0)."]
    MODE0,
    #[doc = "Mode 3: Data are captured on the clock falling edge and data is output on a rising edge. Base level of clock is 1 (CPOL=1, CPHA=1)."]
    MODE3,
}
impl SPIMODER {
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
            SPIMODER::MODE0 => false,
            SPIMODER::MODE3 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> SPIMODER {
        match value {
            false => SPIMODER::MODE0,
            true => SPIMODER::MODE3,
        }
    }
    #[doc = "Checks if the value of the field is `MODE0`"]
    #[inline]
    pub fn is_mode0(&self) -> bool {
        *self == SPIMODER::MODE0
    }
    #[doc = "Checks if the value of the field is `MODE3`"]
    #[inline]
    pub fn is_mode3(&self) -> bool {
        *self == SPIMODER::MODE3
    }
}
#[doc = r" Value of the field"]
pub struct SCKFREQR {
    bits: u8,
}
impl SCKFREQR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Proxy"]
pub struct _SCKDELAYW<'a> {
    w: &'a mut W,
}
impl<'a> _SCKDELAYW<'a> {
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
#[doc = "Values that can be written to the field `DPMEN`"]
pub enum DPMENW {
    #[doc = "Exit DPM."]
    EXIT,
    #[doc = "Enter DPM."]
    ENTER,
}
impl DPMENW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            DPMENW::EXIT => false,
            DPMENW::ENTER => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _DPMENW<'a> {
    w: &'a mut W,
}
impl<'a> _DPMENW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: DPMENW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Exit DPM."]
    #[inline]
    pub fn exit(self) -> &'a mut W {
        self.variant(DPMENW::EXIT)
    }
    #[doc = "Enter DPM."]
    #[inline]
    pub fn enter(self) -> &'a mut W {
        self.variant(DPMENW::ENTER)
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
        const OFFSET: u8 = 24;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `SPIMODE`"]
pub enum SPIMODEW {
    #[doc = "Mode 0: Data are captured on the clock rising edge and data is output on a falling edge. Base level of clock is 0 (CPOL=0, CPHA=0)."]
    MODE0,
    #[doc = "Mode 3: Data are captured on the clock falling edge and data is output on a rising edge. Base level of clock is 1 (CPOL=1, CPHA=1)."]
    MODE3,
}
impl SPIMODEW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            SPIMODEW::MODE0 => false,
            SPIMODEW::MODE3 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _SPIMODEW<'a> {
    w: &'a mut W,
}
impl<'a> _SPIMODEW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: SPIMODEW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Mode 0: Data are captured on the clock rising edge and data is output on a falling edge. Base level of clock is 0 (CPOL=0, CPHA=0)."]
    #[inline]
    pub fn mode0(self) -> &'a mut W {
        self.variant(SPIMODEW::MODE0)
    }
    #[doc = "Mode 3: Data are captured on the clock falling edge and data is output on a rising edge. Base level of clock is 1 (CPOL=1, CPHA=1)."]
    #[inline]
    pub fn mode3(self) -> &'a mut W {
        self.variant(SPIMODEW::MODE3)
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
        const OFFSET: u8 = 25;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _SCKFREQW<'a> {
    w: &'a mut W,
}
impl<'a> _SCKFREQW<'a> {
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 15;
        const OFFSET: u8 = 28;
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
    #[doc = "Bits 0:7 - Minimum amount of time that the CSN pin must stay high before it can go low again. Value is specified in number of 16 MHz periods (62.5 ns)."]
    #[inline]
    pub fn sckdelay(&self) -> SCKDELAYR {
        let bits = {
            const MASK: u8 = 255;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        SCKDELAYR { bits }
    }
    #[doc = "Bit 24 - Enter/exit deep power-down mode (DPM) for external flash memory."]
    #[inline]
    pub fn dpmen(&self) -> DPMENR {
        DPMENR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 24;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 25 - Select SPI mode."]
    #[inline]
    pub fn spimode(&self) -> SPIMODER {
        SPIMODER::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 25;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bits 28:31 - SCK frequency is given as 32 MHz / (SCKFREQ + 1)."]
    #[inline]
    pub fn sckfreq(&self) -> SCKFREQR {
        let bits = {
            const MASK: u8 = 15;
            const OFFSET: u8 = 28;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        SCKFREQR { bits }
    }
}
impl W {
    #[doc = r" Reset value of the register"]
    #[inline]
    pub fn reset_value() -> W {
        W { bits: 263296 }
    }
    #[doc = r" Writes raw bits to the register"]
    #[inline]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
    #[doc = "Bits 0:7 - Minimum amount of time that the CSN pin must stay high before it can go low again. Value is specified in number of 16 MHz periods (62.5 ns)."]
    #[inline]
    pub fn sckdelay(&mut self) -> _SCKDELAYW {
        _SCKDELAYW { w: self }
    }
    #[doc = "Bit 24 - Enter/exit deep power-down mode (DPM) for external flash memory."]
    #[inline]
    pub fn dpmen(&mut self) -> _DPMENW {
        _DPMENW { w: self }
    }
    #[doc = "Bit 25 - Select SPI mode."]
    #[inline]
    pub fn spimode(&mut self) -> _SPIMODEW {
        _SPIMODEW { w: self }
    }
    #[doc = "Bits 28:31 - SCK frequency is given as 32 MHz / (SCKFREQ + 1)."]
    #[inline]
    pub fn sckfreq(&mut self) -> _SCKFREQW {
        _SCKFREQW { w: self }
    }
}
