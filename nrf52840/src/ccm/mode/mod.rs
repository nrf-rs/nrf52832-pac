#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::MODE {
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
#[doc = "Possible values of the field `MODE`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum MODER {
    #[doc = "AES CCM packet encryption mode"]
    ENCRYPTION,
    #[doc = "AES CCM packet decryption mode"]
    DECRYPTION,
}
impl MODER {
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
            MODER::ENCRYPTION => false,
            MODER::DECRYPTION => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> MODER {
        match value {
            false => MODER::ENCRYPTION,
            true => MODER::DECRYPTION,
        }
    }
    #[doc = "Checks if the value of the field is `ENCRYPTION`"]
    #[inline]
    pub fn is_encryption(&self) -> bool {
        *self == MODER::ENCRYPTION
    }
    #[doc = "Checks if the value of the field is `DECRYPTION`"]
    #[inline]
    pub fn is_decryption(&self) -> bool {
        *self == MODER::DECRYPTION
    }
}
#[doc = "Possible values of the field `DATARATE`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DATARATER {
    #[doc = "1 Mbps"]
    _1MBIT,
    #[doc = "2 Mbps"]
    _2MBIT,
    #[doc = "125 Kbps"]
    _125KBPS,
    #[doc = "500 Kbps"]
    _500KBPS,
}
impl DATARATER {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            DATARATER::_1MBIT => 0,
            DATARATER::_2MBIT => 1,
            DATARATER::_125KBPS => 2,
            DATARATER::_500KBPS => 3,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> DATARATER {
        match value {
            0 => DATARATER::_1MBIT,
            1 => DATARATER::_2MBIT,
            2 => DATARATER::_125KBPS,
            3 => DATARATER::_500KBPS,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `_1MBIT`"]
    #[inline]
    pub fn is_1mbit(&self) -> bool {
        *self == DATARATER::_1MBIT
    }
    #[doc = "Checks if the value of the field is `_2MBIT`"]
    #[inline]
    pub fn is_2mbit(&self) -> bool {
        *self == DATARATER::_2MBIT
    }
    #[doc = "Checks if the value of the field is `_125KBPS`"]
    #[inline]
    pub fn is_125kbps(&self) -> bool {
        *self == DATARATER::_125KBPS
    }
    #[doc = "Checks if the value of the field is `_500KBPS`"]
    #[inline]
    pub fn is_500kbps(&self) -> bool {
        *self == DATARATER::_500KBPS
    }
}
#[doc = "Possible values of the field `LENGTH`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum LENGTHR {
    #[doc = "Default length. Effective length of LENGTH field in encrypted/decrypted packet is 5 bits. A key-stream for packet payloads up to 27 bytes will be generated."]
    DEFAULT,
    #[doc = "Extended length. Effective length of LENGTH field in encrypted/decrypted packet is 8 bits. A key-stream for packet payloads up to MAXPACKETSIZE bytes will be generated."]
    EXTENDED,
}
impl LENGTHR {
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
            LENGTHR::DEFAULT => false,
            LENGTHR::EXTENDED => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> LENGTHR {
        match value {
            false => LENGTHR::DEFAULT,
            true => LENGTHR::EXTENDED,
        }
    }
    #[doc = "Checks if the value of the field is `DEFAULT`"]
    #[inline]
    pub fn is_default(&self) -> bool {
        *self == LENGTHR::DEFAULT
    }
    #[doc = "Checks if the value of the field is `EXTENDED`"]
    #[inline]
    pub fn is_extended(&self) -> bool {
        *self == LENGTHR::EXTENDED
    }
}
#[doc = "Values that can be written to the field `MODE`"]
pub enum MODEW {
    #[doc = "AES CCM packet encryption mode"]
    ENCRYPTION,
    #[doc = "AES CCM packet decryption mode"]
    DECRYPTION,
}
impl MODEW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            MODEW::ENCRYPTION => false,
            MODEW::DECRYPTION => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _MODEW<'a> {
    w: &'a mut W,
}
impl<'a> _MODEW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: MODEW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "AES CCM packet encryption mode"]
    #[inline]
    pub fn encryption(self) -> &'a mut W {
        self.variant(MODEW::ENCRYPTION)
    }
    #[doc = "AES CCM packet decryption mode"]
    #[inline]
    pub fn decryption(self) -> &'a mut W {
        self.variant(MODEW::DECRYPTION)
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
#[doc = "Values that can be written to the field `DATARATE`"]
pub enum DATARATEW {
    #[doc = "1 Mbps"]
    _1MBIT,
    #[doc = "2 Mbps"]
    _2MBIT,
    #[doc = "125 Kbps"]
    _125KBPS,
    #[doc = "500 Kbps"]
    _500KBPS,
}
impl DATARATEW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            DATARATEW::_1MBIT => 0,
            DATARATEW::_2MBIT => 1,
            DATARATEW::_125KBPS => 2,
            DATARATEW::_500KBPS => 3,
        }
    }
}
#[doc = r" Proxy"]
pub struct _DATARATEW<'a> {
    w: &'a mut W,
}
impl<'a> _DATARATEW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: DATARATEW) -> &'a mut W {
        {
            self.bits(variant._bits())
        }
    }
    #[doc = "1 Mbps"]
    #[inline]
    pub fn _1mbit(self) -> &'a mut W {
        self.variant(DATARATEW::_1MBIT)
    }
    #[doc = "2 Mbps"]
    #[inline]
    pub fn _2mbit(self) -> &'a mut W {
        self.variant(DATARATEW::_2MBIT)
    }
    #[doc = "125 Kbps"]
    #[inline]
    pub fn _125kbps(self) -> &'a mut W {
        self.variant(DATARATEW::_125KBPS)
    }
    #[doc = "500 Kbps"]
    #[inline]
    pub fn _500kbps(self) -> &'a mut W {
        self.variant(DATARATEW::_500KBPS)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 3;
        const OFFSET: u8 = 16;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `LENGTH`"]
pub enum LENGTHW {
    #[doc = "Default length. Effective length of LENGTH field in encrypted/decrypted packet is 5 bits. A key-stream for packet payloads up to 27 bytes will be generated."]
    DEFAULT,
    #[doc = "Extended length. Effective length of LENGTH field in encrypted/decrypted packet is 8 bits. A key-stream for packet payloads up to MAXPACKETSIZE bytes will be generated."]
    EXTENDED,
}
impl LENGTHW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            LENGTHW::DEFAULT => false,
            LENGTHW::EXTENDED => true,
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
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Default length. Effective length of LENGTH field in encrypted/decrypted packet is 5 bits. A key-stream for packet payloads up to 27 bytes will be generated."]
    #[inline]
    pub fn default(self) -> &'a mut W {
        self.variant(LENGTHW::DEFAULT)
    }
    #[doc = "Extended length. Effective length of LENGTH field in encrypted/decrypted packet is 8 bits. A key-stream for packet payloads up to MAXPACKETSIZE bytes will be generated."]
    #[inline]
    pub fn extended(self) -> &'a mut W {
        self.variant(LENGTHW::EXTENDED)
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
impl R {
    #[doc = r" Value of the register as raw bits"]
    #[inline]
    pub fn bits(&self) -> u32 {
        self.bits
    }
    #[doc = "Bit 0 - The mode of operation to be used. The settings in this register apply whenever either the KSGEN or CRYPT tasks are triggered."]
    #[inline]
    pub fn mode(&self) -> MODER {
        MODER::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bits 16:17 - Radio data rate that the CCM shall run synchronous with"]
    #[inline]
    pub fn datarate(&self) -> DATARATER {
        DATARATER::_from({
            const MASK: u8 = 3;
            const OFFSET: u8 = 16;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bit 24 - Packet length configuration"]
    #[inline]
    pub fn length(&self) -> LENGTHR {
        LENGTHR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 24;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
}
impl W {
    #[doc = r" Reset value of the register"]
    #[inline]
    pub fn reset_value() -> W {
        W { bits: 1 }
    }
    #[doc = r" Writes raw bits to the register"]
    #[inline]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
    #[doc = "Bit 0 - The mode of operation to be used. The settings in this register apply whenever either the KSGEN or CRYPT tasks are triggered."]
    #[inline]
    pub fn mode(&mut self) -> _MODEW {
        _MODEW { w: self }
    }
    #[doc = "Bits 16:17 - Radio data rate that the CCM shall run synchronous with"]
    #[inline]
    pub fn datarate(&mut self) -> _DATARATEW {
        _DATARATEW { w: self }
    }
    #[doc = "Bit 24 - Packet length configuration"]
    #[inline]
    pub fn length(&mut self) -> _LENGTHW {
        _LENGTHW { w: self }
    }
}
