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
    #[doc = "Read: no overrun occured"]
    NOTPRESENT,
    #[doc = "Read: overrun occured"]
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
#[doc = "Possible values of the field `ANACK`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ANACKR {
    #[doc = "Read: error not present"]
    NOTPRESENT,
    #[doc = "Read: error present"]
    PRESENT,
}
impl ANACKR {
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
            ANACKR::NOTPRESENT => false,
            ANACKR::PRESENT => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> ANACKR {
        match value {
            false => ANACKR::NOTPRESENT,
            true => ANACKR::PRESENT,
        }
    }
    #[doc = "Checks if the value of the field is `NOTPRESENT`"]
    #[inline]
    pub fn is_not_present(&self) -> bool {
        *self == ANACKR::NOTPRESENT
    }
    #[doc = "Checks if the value of the field is `PRESENT`"]
    #[inline]
    pub fn is_present(&self) -> bool {
        *self == ANACKR::PRESENT
    }
}
#[doc = "Possible values of the field `DNACK`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DNACKR {
    #[doc = "Read: error not present"]
    NOTPRESENT,
    #[doc = "Read: error present"]
    PRESENT,
}
impl DNACKR {
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
            DNACKR::NOTPRESENT => false,
            DNACKR::PRESENT => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> DNACKR {
        match value {
            false => DNACKR::NOTPRESENT,
            true => DNACKR::PRESENT,
        }
    }
    #[doc = "Checks if the value of the field is `NOTPRESENT`"]
    #[inline]
    pub fn is_not_present(&self) -> bool {
        *self == DNACKR::NOTPRESENT
    }
    #[doc = "Checks if the value of the field is `PRESENT`"]
    #[inline]
    pub fn is_present(&self) -> bool {
        *self == DNACKR::PRESENT
    }
}
#[doc = "Values that can be written to the field `OVERRUN`"]
pub enum OVERRUNW {
    #[doc = "Read: no overrun occured"]
    NOTPRESENT,
    #[doc = "Read: overrun occured"]
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
    #[doc = "Read: no overrun occured"]
    #[inline]
    pub fn not_present(self) -> &'a mut W {
        self.variant(OVERRUNW::NOTPRESENT)
    }
    #[doc = "Read: overrun occured"]
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
#[doc = "Values that can be written to the field `ANACK`"]
pub enum ANACKW {
    #[doc = "Read: error not present"]
    NOTPRESENT,
    #[doc = "Read: error present"]
    PRESENT,
}
impl ANACKW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            ANACKW::NOTPRESENT => false,
            ANACKW::PRESENT => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _ANACKW<'a> {
    w: &'a mut W,
}
impl<'a> _ANACKW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: ANACKW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Read: error not present"]
    #[inline]
    pub fn not_present(self) -> &'a mut W {
        self.variant(ANACKW::NOTPRESENT)
    }
    #[doc = "Read: error present"]
    #[inline]
    pub fn present(self) -> &'a mut W {
        self.variant(ANACKW::PRESENT)
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
#[doc = "Values that can be written to the field `DNACK`"]
pub enum DNACKW {
    #[doc = "Read: error not present"]
    NOTPRESENT,
    #[doc = "Read: error present"]
    PRESENT,
}
impl DNACKW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            DNACKW::NOTPRESENT => false,
            DNACKW::PRESENT => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _DNACKW<'a> {
    w: &'a mut W,
}
impl<'a> _DNACKW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: DNACKW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Read: error not present"]
    #[inline]
    pub fn not_present(self) -> &'a mut W {
        self.variant(DNACKW::NOTPRESENT)
    }
    #[doc = "Read: error present"]
    #[inline]
    pub fn present(self) -> &'a mut W {
        self.variant(DNACKW::PRESENT)
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
    #[doc = "Bit 1 - NACK received after sending the address (write '1' to clear)"]
    #[inline]
    pub fn anack(&self) -> ANACKR {
        ANACKR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 1;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 2 - NACK received after sending a data byte (write '1' to clear)"]
    #[inline]
    pub fn dnack(&self) -> DNACKR {
        DNACKR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 2;
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
    #[doc = "Bit 1 - NACK received after sending the address (write '1' to clear)"]
    #[inline]
    pub fn anack(&mut self) -> _ANACKW {
        _ANACKW { w: self }
    }
    #[doc = "Bit 2 - NACK received after sending a data byte (write '1' to clear)"]
    #[inline]
    pub fn dnack(&mut self) -> _DNACKW {
        _DNACKW { w: self }
    }
}
