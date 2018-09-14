#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::ADDRCONF {
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
#[doc = r" Value of the field"]
pub struct BYTE0R {
    bits: u8,
}
impl BYTE0R {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct BYTE1R {
    bits: u8,
}
impl BYTE1R {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = "Possible values of the field `MODE`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum MODER {
    #[doc = "Do not send any instruction."]
    NOINSTR,
    #[doc = "Send opcode."]
    OPCODE,
    #[doc = "Send opcode, byte0."]
    OPBYTE0,
    #[doc = "Send opcode, byte0, byte1."]
    ALL,
}
impl MODER {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            MODER::NOINSTR => 0,
            MODER::OPCODE => 1,
            MODER::OPBYTE0 => 2,
            MODER::ALL => 3,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> MODER {
        match value {
            0 => MODER::NOINSTR,
            1 => MODER::OPCODE,
            2 => MODER::OPBYTE0,
            3 => MODER::ALL,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `NOINSTR`"]
    #[inline]
    pub fn is_no_instr(&self) -> bool {
        *self == MODER::NOINSTR
    }
    #[doc = "Checks if the value of the field is `OPCODE`"]
    #[inline]
    pub fn is_opcode(&self) -> bool {
        *self == MODER::OPCODE
    }
    #[doc = "Checks if the value of the field is `OPBYTE0`"]
    #[inline]
    pub fn is_op_byte0(&self) -> bool {
        *self == MODER::OPBYTE0
    }
    #[doc = "Checks if the value of the field is `ALL`"]
    #[inline]
    pub fn is_all(&self) -> bool {
        *self == MODER::ALL
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
#[doc = r" Proxy"]
pub struct _BYTE0W<'a> {
    w: &'a mut W,
}
impl<'a> _BYTE0W<'a> {
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 255;
        const OFFSET: u8 = 8;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _BYTE1W<'a> {
    w: &'a mut W,
}
impl<'a> _BYTE1W<'a> {
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 255;
        const OFFSET: u8 = 16;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `MODE`"]
pub enum MODEW {
    #[doc = "Do not send any instruction."]
    NOINSTR,
    #[doc = "Send opcode."]
    OPCODE,
    #[doc = "Send opcode, byte0."]
    OPBYTE0,
    #[doc = "Send opcode, byte0, byte1."]
    ALL,
}
impl MODEW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            MODEW::NOINSTR => 0,
            MODEW::OPCODE => 1,
            MODEW::OPBYTE0 => 2,
            MODEW::ALL => 3,
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
            self.bits(variant._bits())
        }
    }
    #[doc = "Do not send any instruction."]
    #[inline]
    pub fn no_instr(self) -> &'a mut W {
        self.variant(MODEW::NOINSTR)
    }
    #[doc = "Send opcode."]
    #[inline]
    pub fn opcode(self) -> &'a mut W {
        self.variant(MODEW::OPCODE)
    }
    #[doc = "Send opcode, byte0."]
    #[inline]
    pub fn op_byte0(self) -> &'a mut W {
        self.variant(MODEW::OPBYTE0)
    }
    #[doc = "Send opcode, byte0, byte1."]
    #[inline]
    pub fn all(self) -> &'a mut W {
        self.variant(MODEW::ALL)
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
        const OFFSET: u8 = 26;
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
        const OFFSET: u8 = 27;
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
    #[doc = "Bits 0:7 - Opcode that enters the 32-bit addressing mode."]
    #[inline]
    pub fn opcode(&self) -> OPCODER {
        let bits = {
            const MASK: u8 = 255;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        OPCODER { bits }
    }
    #[doc = "Bits 8:15 - Byte 0 following opcode."]
    #[inline]
    pub fn byte0(&self) -> BYTE0R {
        let bits = {
            const MASK: u8 = 255;
            const OFFSET: u8 = 8;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        BYTE0R { bits }
    }
    #[doc = "Bits 16:23 - Byte 1 following byte 0."]
    #[inline]
    pub fn byte1(&self) -> BYTE1R {
        let bits = {
            const MASK: u8 = 255;
            const OFFSET: u8 = 16;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        BYTE1R { bits }
    }
    #[doc = "Bits 24:25 - Extended addressing mode."]
    #[inline]
    pub fn mode(&self) -> MODER {
        MODER::_from({
            const MASK: u8 = 3;
            const OFFSET: u8 = 24;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bit 26 - Wait for write complete before sending command."]
    #[inline]
    pub fn wipwait(&self) -> WIPWAITR {
        WIPWAITR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 26;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 27 - Send WREN (write enable opcode 0x06) before instruction."]
    #[inline]
    pub fn wren(&self) -> WRENR {
        WRENR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 27;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
}
impl W {
    #[doc = r" Reset value of the register"]
    #[inline]
    pub fn reset_value() -> W {
        W { bits: 183 }
    }
    #[doc = r" Writes raw bits to the register"]
    #[inline]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
    #[doc = "Bits 0:7 - Opcode that enters the 32-bit addressing mode."]
    #[inline]
    pub fn opcode(&mut self) -> _OPCODEW {
        _OPCODEW { w: self }
    }
    #[doc = "Bits 8:15 - Byte 0 following opcode."]
    #[inline]
    pub fn byte0(&mut self) -> _BYTE0W {
        _BYTE0W { w: self }
    }
    #[doc = "Bits 16:23 - Byte 1 following byte 0."]
    #[inline]
    pub fn byte1(&mut self) -> _BYTE1W {
        _BYTE1W { w: self }
    }
    #[doc = "Bits 24:25 - Extended addressing mode."]
    #[inline]
    pub fn mode(&mut self) -> _MODEW {
        _MODEW { w: self }
    }
    #[doc = "Bit 26 - Wait for write complete before sending command."]
    #[inline]
    pub fn wipwait(&mut self) -> _WIPWAITW {
        _WIPWAITW { w: self }
    }
    #[doc = "Bit 27 - Send WREN (write enable opcode 0x06) before instruction."]
    #[inline]
    pub fn wren(&mut self) -> _WRENW {
        _WRENW { w: self }
    }
}
