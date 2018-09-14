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
    #[doc = "Error did not occur"]
    NOTRECEIVED,
    #[doc = "Error occurred"]
    RECEIVED,
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
            OVERRUNR::NOTRECEIVED => false,
            OVERRUNR::RECEIVED => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> OVERRUNR {
        match value {
            false => OVERRUNR::NOTRECEIVED,
            true => OVERRUNR::RECEIVED,
        }
    }
    #[doc = "Checks if the value of the field is `NOTRECEIVED`"]
    #[inline]
    pub fn is_not_received(&self) -> bool {
        *self == OVERRUNR::NOTRECEIVED
    }
    #[doc = "Checks if the value of the field is `RECEIVED`"]
    #[inline]
    pub fn is_received(&self) -> bool {
        *self == OVERRUNR::RECEIVED
    }
}
#[doc = "Possible values of the field `ANACK`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ANACKR {
    #[doc = "Error did not occur"]
    NOTRECEIVED,
    #[doc = "Error occurred"]
    RECEIVED,
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
            ANACKR::NOTRECEIVED => false,
            ANACKR::RECEIVED => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> ANACKR {
        match value {
            false => ANACKR::NOTRECEIVED,
            true => ANACKR::RECEIVED,
        }
    }
    #[doc = "Checks if the value of the field is `NOTRECEIVED`"]
    #[inline]
    pub fn is_not_received(&self) -> bool {
        *self == ANACKR::NOTRECEIVED
    }
    #[doc = "Checks if the value of the field is `RECEIVED`"]
    #[inline]
    pub fn is_received(&self) -> bool {
        *self == ANACKR::RECEIVED
    }
}
#[doc = "Possible values of the field `DNACK`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DNACKR {
    #[doc = "Error did not occur"]
    NOTRECEIVED,
    #[doc = "Error occurred"]
    RECEIVED,
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
            DNACKR::NOTRECEIVED => false,
            DNACKR::RECEIVED => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> DNACKR {
        match value {
            false => DNACKR::NOTRECEIVED,
            true => DNACKR::RECEIVED,
        }
    }
    #[doc = "Checks if the value of the field is `NOTRECEIVED`"]
    #[inline]
    pub fn is_not_received(&self) -> bool {
        *self == DNACKR::NOTRECEIVED
    }
    #[doc = "Checks if the value of the field is `RECEIVED`"]
    #[inline]
    pub fn is_received(&self) -> bool {
        *self == DNACKR::RECEIVED
    }
}
#[doc = "Values that can be written to the field `OVERRUN`"]
pub enum OVERRUNW {
    #[doc = "Error did not occur"]
    NOTRECEIVED,
    #[doc = "Error occurred"]
    RECEIVED,
}
impl OVERRUNW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            OVERRUNW::NOTRECEIVED => false,
            OVERRUNW::RECEIVED => true,
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
    #[doc = "Error did not occur"]
    #[inline]
    pub fn not_received(self) -> &'a mut W {
        self.variant(OVERRUNW::NOTRECEIVED)
    }
    #[doc = "Error occurred"]
    #[inline]
    pub fn received(self) -> &'a mut W {
        self.variant(OVERRUNW::RECEIVED)
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
    #[doc = "Error did not occur"]
    NOTRECEIVED,
    #[doc = "Error occurred"]
    RECEIVED,
}
impl ANACKW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            ANACKW::NOTRECEIVED => false,
            ANACKW::RECEIVED => true,
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
    #[doc = "Error did not occur"]
    #[inline]
    pub fn not_received(self) -> &'a mut W {
        self.variant(ANACKW::NOTRECEIVED)
    }
    #[doc = "Error occurred"]
    #[inline]
    pub fn received(self) -> &'a mut W {
        self.variant(ANACKW::RECEIVED)
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
    #[doc = "Error did not occur"]
    NOTRECEIVED,
    #[doc = "Error occurred"]
    RECEIVED,
}
impl DNACKW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            DNACKW::NOTRECEIVED => false,
            DNACKW::RECEIVED => true,
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
    #[doc = "Error did not occur"]
    #[inline]
    pub fn not_received(self) -> &'a mut W {
        self.variant(DNACKW::NOTRECEIVED)
    }
    #[doc = "Error occurred"]
    #[inline]
    pub fn received(self) -> &'a mut W {
        self.variant(DNACKW::RECEIVED)
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
