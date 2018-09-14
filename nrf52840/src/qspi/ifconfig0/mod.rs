#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::IFCONFIG0 {
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
#[doc = "Possible values of the field `READOC`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum READOCR {
    #[doc = "Single data line SPI. FAST_READ (opcode 0x0B)."]
    FASTREAD,
    #[doc = "Dual data line SPI. READ2O (opcode 0x3B)."]
    READ2O,
    #[doc = "Dual data line SPI. READ2IO (opcode 0xBB)."]
    READ2IO,
    #[doc = "Quad data line SPI. READ4O (opcode 0x6B)."]
    READ4O,
    #[doc = "Quad data line SPI. READ4IO (opcode 0xEB)."]
    READ4IO,
    #[doc = r" Reserved"]
    _Reserved(u8),
}
impl READOCR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            READOCR::FASTREAD => 0,
            READOCR::READ2O => 1,
            READOCR::READ2IO => 2,
            READOCR::READ4O => 3,
            READOCR::READ4IO => 4,
            READOCR::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> READOCR {
        match value {
            0 => READOCR::FASTREAD,
            1 => READOCR::READ2O,
            2 => READOCR::READ2IO,
            3 => READOCR::READ4O,
            4 => READOCR::READ4IO,
            i => READOCR::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `FASTREAD`"]
    #[inline]
    pub fn is_fastread(&self) -> bool {
        *self == READOCR::FASTREAD
    }
    #[doc = "Checks if the value of the field is `READ2O`"]
    #[inline]
    pub fn is_read2o(&self) -> bool {
        *self == READOCR::READ2O
    }
    #[doc = "Checks if the value of the field is `READ2IO`"]
    #[inline]
    pub fn is_read2io(&self) -> bool {
        *self == READOCR::READ2IO
    }
    #[doc = "Checks if the value of the field is `READ4O`"]
    #[inline]
    pub fn is_read4o(&self) -> bool {
        *self == READOCR::READ4O
    }
    #[doc = "Checks if the value of the field is `READ4IO`"]
    #[inline]
    pub fn is_read4io(&self) -> bool {
        *self == READOCR::READ4IO
    }
}
#[doc = "Possible values of the field `WRITEOC`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum WRITEOCR {
    #[doc = "Single data line SPI. PP (opcode 0x02)."]
    PP,
    #[doc = "Dual data line SPI. PP2O (opcode 0xA2)."]
    PP2O,
    #[doc = "Quad data line SPI. PP4O (opcode 0x32)."]
    PP4O,
    #[doc = "Quad data line SPI. PP4IO (opcode 0x38)."]
    PP4IO,
    #[doc = r" Reserved"]
    _Reserved(u8),
}
impl WRITEOCR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            WRITEOCR::PP => 0,
            WRITEOCR::PP2O => 1,
            WRITEOCR::PP4O => 2,
            WRITEOCR::PP4IO => 3,
            WRITEOCR::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> WRITEOCR {
        match value {
            0 => WRITEOCR::PP,
            1 => WRITEOCR::PP2O,
            2 => WRITEOCR::PP4O,
            3 => WRITEOCR::PP4IO,
            i => WRITEOCR::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `PP`"]
    #[inline]
    pub fn is_pp(&self) -> bool {
        *self == WRITEOCR::PP
    }
    #[doc = "Checks if the value of the field is `PP2O`"]
    #[inline]
    pub fn is_pp2o(&self) -> bool {
        *self == WRITEOCR::PP2O
    }
    #[doc = "Checks if the value of the field is `PP4O`"]
    #[inline]
    pub fn is_pp4o(&self) -> bool {
        *self == WRITEOCR::PP4O
    }
    #[doc = "Checks if the value of the field is `PP4IO`"]
    #[inline]
    pub fn is_pp4io(&self) -> bool {
        *self == WRITEOCR::PP4IO
    }
}
#[doc = "Possible values of the field `ADDRMODE`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ADDRMODER {
    #[doc = "24-bit addressing."]
    _24BIT,
    #[doc = "32-bit addressing."]
    _32BIT,
}
impl ADDRMODER {
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
            ADDRMODER::_24BIT => false,
            ADDRMODER::_32BIT => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> ADDRMODER {
        match value {
            false => ADDRMODER::_24BIT,
            true => ADDRMODER::_32BIT,
        }
    }
    #[doc = "Checks if the value of the field is `_24BIT`"]
    #[inline]
    pub fn is_24bit(&self) -> bool {
        *self == ADDRMODER::_24BIT
    }
    #[doc = "Checks if the value of the field is `_32BIT`"]
    #[inline]
    pub fn is_32bit(&self) -> bool {
        *self == ADDRMODER::_32BIT
    }
}
#[doc = "Possible values of the field `DPMENABLE`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DPMENABLER {
    #[doc = "Disable DPM feature."]
    DISABLE,
    #[doc = "Enable DPM feature."]
    ENABLE,
}
impl DPMENABLER {
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
            DPMENABLER::DISABLE => false,
            DPMENABLER::ENABLE => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> DPMENABLER {
        match value {
            false => DPMENABLER::DISABLE,
            true => DPMENABLER::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline]
    pub fn is_disable(&self) -> bool {
        *self == DPMENABLER::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline]
    pub fn is_enable(&self) -> bool {
        *self == DPMENABLER::ENABLE
    }
}
#[doc = "Possible values of the field `PPSIZE`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PPSIZER {
    #[doc = "256 bytes."]
    _256BYTES,
    #[doc = "512 bytes."]
    _512BYTES,
}
impl PPSIZER {
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
            PPSIZER::_256BYTES => false,
            PPSIZER::_512BYTES => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> PPSIZER {
        match value {
            false => PPSIZER::_256BYTES,
            true => PPSIZER::_512BYTES,
        }
    }
    #[doc = "Checks if the value of the field is `_256BYTES`"]
    #[inline]
    pub fn is_256bytes(&self) -> bool {
        *self == PPSIZER::_256BYTES
    }
    #[doc = "Checks if the value of the field is `_512BYTES`"]
    #[inline]
    pub fn is_512bytes(&self) -> bool {
        *self == PPSIZER::_512BYTES
    }
}
#[doc = "Values that can be written to the field `READOC`"]
pub enum READOCW {
    #[doc = "Single data line SPI. FAST_READ (opcode 0x0B)."]
    FASTREAD,
    #[doc = "Dual data line SPI. READ2O (opcode 0x3B)."]
    READ2O,
    #[doc = "Dual data line SPI. READ2IO (opcode 0xBB)."]
    READ2IO,
    #[doc = "Quad data line SPI. READ4O (opcode 0x6B)."]
    READ4O,
    #[doc = "Quad data line SPI. READ4IO (opcode 0xEB)."]
    READ4IO,
}
impl READOCW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            READOCW::FASTREAD => 0,
            READOCW::READ2O => 1,
            READOCW::READ2IO => 2,
            READOCW::READ4O => 3,
            READOCW::READ4IO => 4,
        }
    }
}
#[doc = r" Proxy"]
pub struct _READOCW<'a> {
    w: &'a mut W,
}
impl<'a> _READOCW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: READOCW) -> &'a mut W {
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = "Single data line SPI. FAST_READ (opcode 0x0B)."]
    #[inline]
    pub fn fastread(self) -> &'a mut W {
        self.variant(READOCW::FASTREAD)
    }
    #[doc = "Dual data line SPI. READ2O (opcode 0x3B)."]
    #[inline]
    pub fn read2o(self) -> &'a mut W {
        self.variant(READOCW::READ2O)
    }
    #[doc = "Dual data line SPI. READ2IO (opcode 0xBB)."]
    #[inline]
    pub fn read2io(self) -> &'a mut W {
        self.variant(READOCW::READ2IO)
    }
    #[doc = "Quad data line SPI. READ4O (opcode 0x6B)."]
    #[inline]
    pub fn read4o(self) -> &'a mut W {
        self.variant(READOCW::READ4O)
    }
    #[doc = "Quad data line SPI. READ4IO (opcode 0xEB)."]
    #[inline]
    pub fn read4io(self) -> &'a mut W {
        self.variant(READOCW::READ4IO)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 7;
        const OFFSET: u8 = 0;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `WRITEOC`"]
pub enum WRITEOCW {
    #[doc = "Single data line SPI. PP (opcode 0x02)."]
    PP,
    #[doc = "Dual data line SPI. PP2O (opcode 0xA2)."]
    PP2O,
    #[doc = "Quad data line SPI. PP4O (opcode 0x32)."]
    PP4O,
    #[doc = "Quad data line SPI. PP4IO (opcode 0x38)."]
    PP4IO,
}
impl WRITEOCW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            WRITEOCW::PP => 0,
            WRITEOCW::PP2O => 1,
            WRITEOCW::PP4O => 2,
            WRITEOCW::PP4IO => 3,
        }
    }
}
#[doc = r" Proxy"]
pub struct _WRITEOCW<'a> {
    w: &'a mut W,
}
impl<'a> _WRITEOCW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: WRITEOCW) -> &'a mut W {
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = "Single data line SPI. PP (opcode 0x02)."]
    #[inline]
    pub fn pp(self) -> &'a mut W {
        self.variant(WRITEOCW::PP)
    }
    #[doc = "Dual data line SPI. PP2O (opcode 0xA2)."]
    #[inline]
    pub fn pp2o(self) -> &'a mut W {
        self.variant(WRITEOCW::PP2O)
    }
    #[doc = "Quad data line SPI. PP4O (opcode 0x32)."]
    #[inline]
    pub fn pp4o(self) -> &'a mut W {
        self.variant(WRITEOCW::PP4O)
    }
    #[doc = "Quad data line SPI. PP4IO (opcode 0x38)."]
    #[inline]
    pub fn pp4io(self) -> &'a mut W {
        self.variant(WRITEOCW::PP4IO)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 7;
        const OFFSET: u8 = 3;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `ADDRMODE`"]
pub enum ADDRMODEW {
    #[doc = "24-bit addressing."]
    _24BIT,
    #[doc = "32-bit addressing."]
    _32BIT,
}
impl ADDRMODEW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            ADDRMODEW::_24BIT => false,
            ADDRMODEW::_32BIT => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _ADDRMODEW<'a> {
    w: &'a mut W,
}
impl<'a> _ADDRMODEW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: ADDRMODEW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "24-bit addressing."]
    #[inline]
    pub fn _24bit(self) -> &'a mut W {
        self.variant(ADDRMODEW::_24BIT)
    }
    #[doc = "32-bit addressing."]
    #[inline]
    pub fn _32bit(self) -> &'a mut W {
        self.variant(ADDRMODEW::_32BIT)
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
#[doc = "Values that can be written to the field `DPMENABLE`"]
pub enum DPMENABLEW {
    #[doc = "Disable DPM feature."]
    DISABLE,
    #[doc = "Enable DPM feature."]
    ENABLE,
}
impl DPMENABLEW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            DPMENABLEW::DISABLE => false,
            DPMENABLEW::ENABLE => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _DPMENABLEW<'a> {
    w: &'a mut W,
}
impl<'a> _DPMENABLEW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: DPMENABLEW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Disable DPM feature."]
    #[inline]
    pub fn disable(self) -> &'a mut W {
        self.variant(DPMENABLEW::DISABLE)
    }
    #[doc = "Enable DPM feature."]
    #[inline]
    pub fn enable(self) -> &'a mut W {
        self.variant(DPMENABLEW::ENABLE)
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
#[doc = "Values that can be written to the field `PPSIZE`"]
pub enum PPSIZEW {
    #[doc = "256 bytes."]
    _256BYTES,
    #[doc = "512 bytes."]
    _512BYTES,
}
impl PPSIZEW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            PPSIZEW::_256BYTES => false,
            PPSIZEW::_512BYTES => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _PPSIZEW<'a> {
    w: &'a mut W,
}
impl<'a> _PPSIZEW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: PPSIZEW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "256 bytes."]
    #[inline]
    pub fn _256bytes(self) -> &'a mut W {
        self.variant(PPSIZEW::_256BYTES)
    }
    #[doc = "512 bytes."]
    #[inline]
    pub fn _512bytes(self) -> &'a mut W {
        self.variant(PPSIZEW::_512BYTES)
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
        const OFFSET: u8 = 12;
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
    #[doc = "Bits 0:2 - Configure number of data lines and opcode used for reading."]
    #[inline]
    pub fn readoc(&self) -> READOCR {
        READOCR::_from({
            const MASK: u8 = 7;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bits 3:5 - Configure number of data lines and opcode used for writing."]
    #[inline]
    pub fn writeoc(&self) -> WRITEOCR {
        WRITEOCR::_from({
            const MASK: u8 = 7;
            const OFFSET: u8 = 3;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bit 6 - Addressing mode."]
    #[inline]
    pub fn addrmode(&self) -> ADDRMODER {
        ADDRMODER::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 6;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 7 - Enable deep power-down mode (DPM) feature."]
    #[inline]
    pub fn dpmenable(&self) -> DPMENABLER {
        DPMENABLER::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 7;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 12 - Page size for commands PP, PP2O, PP4O and PP4IO."]
    #[inline]
    pub fn ppsize(&self) -> PPSIZER {
        PPSIZER::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 12;
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
    #[doc = "Bits 0:2 - Configure number of data lines and opcode used for reading."]
    #[inline]
    pub fn readoc(&mut self) -> _READOCW {
        _READOCW { w: self }
    }
    #[doc = "Bits 3:5 - Configure number of data lines and opcode used for writing."]
    #[inline]
    pub fn writeoc(&mut self) -> _WRITEOCW {
        _WRITEOCW { w: self }
    }
    #[doc = "Bit 6 - Addressing mode."]
    #[inline]
    pub fn addrmode(&mut self) -> _ADDRMODEW {
        _ADDRMODEW { w: self }
    }
    #[doc = "Bit 7 - Enable deep power-down mode (DPM) feature."]
    #[inline]
    pub fn dpmenable(&mut self) -> _DPMENABLEW {
        _DPMENABLEW { w: self }
    }
    #[doc = "Bit 12 - Page size for commands PP, PP2O, PP4O and PP4IO."]
    #[inline]
    pub fn ppsize(&mut self) -> _PPSIZEW {
        _PPSIZEW { w: self }
    }
}
