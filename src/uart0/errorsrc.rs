#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::ERRORSRC {
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
#[doc = "Possible values of the field `OVERRUN`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum OVERRUNR {
    #[doc = "Read: error not present"]
    NOTPRESENT,
    #[doc = "Read: error present"]
    PRESENT,
}
impl OVERRUNR {
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
            OVERRUNR::NOTPRESENT => false,
            OVERRUNR::PRESENT => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> OVERRUNR {
        match value {
            false => OVERRUNR::NOTPRESENT,
            true => OVERRUNR::PRESENT,
        }
    }
    #[doc = "Checks if the value of the field is `NOTPRESENT`"]
    #[inline]
    pub fn is_not_present(&self) -> bool {
        *self == OVERRUNR::NOTPRESENT
    }
    #[doc = "Checks if the value of the field is `PRESENT`"]
    #[inline]
    pub fn is_present(&self) -> bool {
        *self == OVERRUNR::PRESENT
    }
}
#[doc = "Possible values of the field `PARITY`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PARITYR {
    #[doc = "Read: error not present"]
    NOTPRESENT,
    #[doc = "Read: error present"]
    PRESENT,
}
impl PARITYR {
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
            PARITYR::NOTPRESENT => false,
            PARITYR::PRESENT => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> PARITYR {
        match value {
            false => PARITYR::NOTPRESENT,
            true => PARITYR::PRESENT,
        }
    }
    #[doc = "Checks if the value of the field is `NOTPRESENT`"]
    #[inline]
    pub fn is_not_present(&self) -> bool {
        *self == PARITYR::NOTPRESENT
    }
    #[doc = "Checks if the value of the field is `PRESENT`"]
    #[inline]
    pub fn is_present(&self) -> bool {
        *self == PARITYR::PRESENT
    }
}
#[doc = "Possible values of the field `FRAMING`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum FRAMINGR {
    #[doc = "Read: error not present"]
    NOTPRESENT,
    #[doc = "Read: error present"]
    PRESENT,
}
impl FRAMINGR {
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
            FRAMINGR::NOTPRESENT => false,
            FRAMINGR::PRESENT => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> FRAMINGR {
        match value {
            false => FRAMINGR::NOTPRESENT,
            true => FRAMINGR::PRESENT,
        }
    }
    #[doc = "Checks if the value of the field is `NOTPRESENT`"]
    #[inline]
    pub fn is_not_present(&self) -> bool {
        *self == FRAMINGR::NOTPRESENT
    }
    #[doc = "Checks if the value of the field is `PRESENT`"]
    #[inline]
    pub fn is_present(&self) -> bool {
        *self == FRAMINGR::PRESENT
    }
}
#[doc = "Possible values of the field `BREAK`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum BREAKR {
    #[doc = "Read: error not present"]
    NOTPRESENT,
    #[doc = "Read: error present"]
    PRESENT,
}
impl BREAKR {
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
            BREAKR::NOTPRESENT => false,
            BREAKR::PRESENT => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> BREAKR {
        match value {
            false => BREAKR::NOTPRESENT,
            true => BREAKR::PRESENT,
        }
    }
    #[doc = "Checks if the value of the field is `NOTPRESENT`"]
    #[inline]
    pub fn is_not_present(&self) -> bool {
        *self == BREAKR::NOTPRESENT
    }
    #[doc = "Checks if the value of the field is `PRESENT`"]
    #[inline]
    pub fn is_present(&self) -> bool {
        *self == BREAKR::PRESENT
    }
}
#[doc = "Values that can be written to the field `OVERRUN`"]
pub enum OVERRUNW {
    #[doc = "Read: error not present"]
    NOTPRESENT,
    #[doc = "Read: error present"]
    PRESENT,
}
impl OVERRUNW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            OVERRUNW::NOTPRESENT => false,
            OVERRUNW::PRESENT => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _OVERRUNW<'a> {
    w: &'a mut W,
}
impl<'a> _OVERRUNW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: OVERRUNW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Read: error not present"]
    #[inline]
    pub fn not_present(self) -> &'a mut W {
        self.variant(OVERRUNW::NOTPRESENT)
    }
    #[doc = "Read: error present"]
    #[inline]
    pub fn present(self) -> &'a mut W {
        self.variant(OVERRUNW::PRESENT)
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
#[doc = "Values that can be written to the field `PARITY`"]
pub enum PARITYW {
    #[doc = "Read: error not present"]
    NOTPRESENT,
    #[doc = "Read: error present"]
    PRESENT,
}
impl PARITYW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            PARITYW::NOTPRESENT => false,
            PARITYW::PRESENT => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _PARITYW<'a> {
    w: &'a mut W,
}
impl<'a> _PARITYW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: PARITYW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Read: error not present"]
    #[inline]
    pub fn not_present(self) -> &'a mut W {
        self.variant(PARITYW::NOTPRESENT)
    }
    #[doc = "Read: error present"]
    #[inline]
    pub fn present(self) -> &'a mut W {
        self.variant(PARITYW::PRESENT)
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
#[doc = "Values that can be written to the field `FRAMING`"]
pub enum FRAMINGW {
    #[doc = "Read: error not present"]
    NOTPRESENT,
    #[doc = "Read: error present"]
    PRESENT,
}
impl FRAMINGW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            FRAMINGW::NOTPRESENT => false,
            FRAMINGW::PRESENT => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _FRAMINGW<'a> {
    w: &'a mut W,
}
impl<'a> _FRAMINGW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: FRAMINGW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Read: error not present"]
    #[inline]
    pub fn not_present(self) -> &'a mut W {
        self.variant(FRAMINGW::NOTPRESENT)
    }
    #[doc = "Read: error present"]
    #[inline]
    pub fn present(self) -> &'a mut W {
        self.variant(FRAMINGW::PRESENT)
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
#[doc = "Values that can be written to the field `BREAK`"]
pub enum BREAKW {
    #[doc = "Read: error not present"]
    NOTPRESENT,
    #[doc = "Read: error present"]
    PRESENT,
}
impl BREAKW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            BREAKW::NOTPRESENT => false,
            BREAKW::PRESENT => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _BREAKW<'a> {
    w: &'a mut W,
}
impl<'a> _BREAKW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: BREAKW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Read: error not present"]
    #[inline]
    pub fn not_present(self) -> &'a mut W {
        self.variant(BREAKW::NOTPRESENT)
    }
    #[doc = "Read: error present"]
    #[inline]
    pub fn present(self) -> &'a mut W {
        self.variant(BREAKW::PRESENT)
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
impl R {
    #[doc = r" Value of the register as raw bits"]
    #[inline]
    pub fn bits(&self) -> u32 {
        self.bits
    }
    #[doc = "Bit 0 - Overrun error"]
    #[inline]
    pub fn overrun(&self) -> OVERRUNR {
        OVERRUNR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 1 - Parity error"]
    #[inline]
    pub fn parity(&self) -> PARITYR {
        PARITYR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 1;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 2 - Framing error occurred"]
    #[inline]
    pub fn framing(&self) -> FRAMINGR {
        FRAMINGR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 2;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 3 - Break condition"]
    #[inline]
    pub fn break_(&self) -> BREAKR {
        BREAKR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 3;
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
    #[doc = "Bit 0 - Overrun error"]
    #[inline]
    pub fn overrun(&mut self) -> _OVERRUNW {
        _OVERRUNW { w: self }
    }
    #[doc = "Bit 1 - Parity error"]
    #[inline]
    pub fn parity(&mut self) -> _PARITYW {
        _PARITYW { w: self }
    }
    #[doc = "Bit 2 - Framing error occurred"]
    #[inline]
    pub fn framing(&mut self) -> _FRAMINGW {
        _FRAMINGW { w: self }
    }
    #[doc = "Bit 3 - Break condition"]
    #[inline]
    pub fn break_(&mut self) -> _BREAKW {
        _BREAKW { w: self }
    }
}
