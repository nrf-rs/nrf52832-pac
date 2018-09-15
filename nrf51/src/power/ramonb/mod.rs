#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::RAMONB {
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
#[doc = "Possible values of the field `ONRAM2`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ONRAM2R {
    #[doc = "RAM block 2 OFF in ON mode."]
    RAM2OFF,
    #[doc = "RAM block 2 ON in ON mode."]
    RAM2ON,
}
impl ONRAM2R {
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
            ONRAM2R::RAM2OFF => false,
            ONRAM2R::RAM2ON => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> ONRAM2R {
        match value {
            false => ONRAM2R::RAM2OFF,
            true => ONRAM2R::RAM2ON,
        }
    }
    #[doc = "Checks if the value of the field is `RAM2OFF`"]
    #[inline]
    pub fn is_ram2off(&self) -> bool {
        *self == ONRAM2R::RAM2OFF
    }
    #[doc = "Checks if the value of the field is `RAM2ON`"]
    #[inline]
    pub fn is_ram2on(&self) -> bool {
        *self == ONRAM2R::RAM2ON
    }
}
#[doc = "Possible values of the field `ONRAM3`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ONRAM3R {
    #[doc = "RAM block 33 OFF in ON mode."]
    RAM3OFF,
    #[doc = "RAM block 3 ON in ON mode."]
    RAM3ON,
}
impl ONRAM3R {
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
            ONRAM3R::RAM3OFF => false,
            ONRAM3R::RAM3ON => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> ONRAM3R {
        match value {
            false => ONRAM3R::RAM3OFF,
            true => ONRAM3R::RAM3ON,
        }
    }
    #[doc = "Checks if the value of the field is `RAM3OFF`"]
    #[inline]
    pub fn is_ram3off(&self) -> bool {
        *self == ONRAM3R::RAM3OFF
    }
    #[doc = "Checks if the value of the field is `RAM3ON`"]
    #[inline]
    pub fn is_ram3on(&self) -> bool {
        *self == ONRAM3R::RAM3ON
    }
}
#[doc = "Possible values of the field `OFFRAM2`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum OFFRAM2R {
    #[doc = "RAM block 2 OFF in OFF mode."]
    RAM2OFF,
    #[doc = "RAM block 2 ON in OFF mode."]
    RAM2ON,
}
impl OFFRAM2R {
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
            OFFRAM2R::RAM2OFF => false,
            OFFRAM2R::RAM2ON => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> OFFRAM2R {
        match value {
            false => OFFRAM2R::RAM2OFF,
            true => OFFRAM2R::RAM2ON,
        }
    }
    #[doc = "Checks if the value of the field is `RAM2OFF`"]
    #[inline]
    pub fn is_ram2off(&self) -> bool {
        *self == OFFRAM2R::RAM2OFF
    }
    #[doc = "Checks if the value of the field is `RAM2ON`"]
    #[inline]
    pub fn is_ram2on(&self) -> bool {
        *self == OFFRAM2R::RAM2ON
    }
}
#[doc = "Possible values of the field `OFFRAM3`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum OFFRAM3R {
    #[doc = "RAM block 3 OFF in OFF mode."]
    RAM3OFF,
    #[doc = "RAM block 3 ON in OFF mode."]
    RAM3ON,
}
impl OFFRAM3R {
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
            OFFRAM3R::RAM3OFF => false,
            OFFRAM3R::RAM3ON => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> OFFRAM3R {
        match value {
            false => OFFRAM3R::RAM3OFF,
            true => OFFRAM3R::RAM3ON,
        }
    }
    #[doc = "Checks if the value of the field is `RAM3OFF`"]
    #[inline]
    pub fn is_ram3off(&self) -> bool {
        *self == OFFRAM3R::RAM3OFF
    }
    #[doc = "Checks if the value of the field is `RAM3ON`"]
    #[inline]
    pub fn is_ram3on(&self) -> bool {
        *self == OFFRAM3R::RAM3ON
    }
}
#[doc = "Values that can be written to the field `ONRAM2`"]
pub enum ONRAM2W {
    #[doc = "RAM block 2 OFF in ON mode."]
    RAM2OFF,
    #[doc = "RAM block 2 ON in ON mode."]
    RAM2ON,
}
impl ONRAM2W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            ONRAM2W::RAM2OFF => false,
            ONRAM2W::RAM2ON => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _ONRAM2W<'a> {
    w: &'a mut W,
}
impl<'a> _ONRAM2W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: ONRAM2W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "RAM block 2 OFF in ON mode."]
    #[inline]
    pub fn ram2off(self) -> &'a mut W {
        self.variant(ONRAM2W::RAM2OFF)
    }
    #[doc = "RAM block 2 ON in ON mode."]
    #[inline]
    pub fn ram2on(self) -> &'a mut W {
        self.variant(ONRAM2W::RAM2ON)
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
#[doc = "Values that can be written to the field `ONRAM3`"]
pub enum ONRAM3W {
    #[doc = "RAM block 33 OFF in ON mode."]
    RAM3OFF,
    #[doc = "RAM block 3 ON in ON mode."]
    RAM3ON,
}
impl ONRAM3W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            ONRAM3W::RAM3OFF => false,
            ONRAM3W::RAM3ON => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _ONRAM3W<'a> {
    w: &'a mut W,
}
impl<'a> _ONRAM3W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: ONRAM3W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "RAM block 33 OFF in ON mode."]
    #[inline]
    pub fn ram3off(self) -> &'a mut W {
        self.variant(ONRAM3W::RAM3OFF)
    }
    #[doc = "RAM block 3 ON in ON mode."]
    #[inline]
    pub fn ram3on(self) -> &'a mut W {
        self.variant(ONRAM3W::RAM3ON)
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
#[doc = "Values that can be written to the field `OFFRAM2`"]
pub enum OFFRAM2W {
    #[doc = "RAM block 2 OFF in OFF mode."]
    RAM2OFF,
    #[doc = "RAM block 2 ON in OFF mode."]
    RAM2ON,
}
impl OFFRAM2W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            OFFRAM2W::RAM2OFF => false,
            OFFRAM2W::RAM2ON => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _OFFRAM2W<'a> {
    w: &'a mut W,
}
impl<'a> _OFFRAM2W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: OFFRAM2W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "RAM block 2 OFF in OFF mode."]
    #[inline]
    pub fn ram2off(self) -> &'a mut W {
        self.variant(OFFRAM2W::RAM2OFF)
    }
    #[doc = "RAM block 2 ON in OFF mode."]
    #[inline]
    pub fn ram2on(self) -> &'a mut W {
        self.variant(OFFRAM2W::RAM2ON)
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
#[doc = "Values that can be written to the field `OFFRAM3`"]
pub enum OFFRAM3W {
    #[doc = "RAM block 3 OFF in OFF mode."]
    RAM3OFF,
    #[doc = "RAM block 3 ON in OFF mode."]
    RAM3ON,
}
impl OFFRAM3W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            OFFRAM3W::RAM3OFF => false,
            OFFRAM3W::RAM3ON => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _OFFRAM3W<'a> {
    w: &'a mut W,
}
impl<'a> _OFFRAM3W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: OFFRAM3W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "RAM block 3 OFF in OFF mode."]
    #[inline]
    pub fn ram3off(self) -> &'a mut W {
        self.variant(OFFRAM3W::RAM3OFF)
    }
    #[doc = "RAM block 3 ON in OFF mode."]
    #[inline]
    pub fn ram3on(self) -> &'a mut W {
        self.variant(OFFRAM3W::RAM3ON)
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
impl R {
    #[doc = r" Value of the register as raw bits"]
    #[inline]
    pub fn bits(&self) -> u32 {
        self.bits
    }
    #[doc = "Bit 0 - RAM block 2 behaviour in ON mode."]
    #[inline]
    pub fn onram2(&self) -> ONRAM2R {
        ONRAM2R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 1 - RAM block 3 behaviour in ON mode."]
    #[inline]
    pub fn onram3(&self) -> ONRAM3R {
        ONRAM3R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 1;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 16 - RAM block 2 behaviour in OFF mode."]
    #[inline]
    pub fn offram2(&self) -> OFFRAM2R {
        OFFRAM2R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 16;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 17 - RAM block 3 behaviour in OFF mode."]
    #[inline]
    pub fn offram3(&self) -> OFFRAM3R {
        OFFRAM3R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 17;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
}
impl W {
    #[doc = r" Reset value of the register"]
    #[inline]
    pub fn reset_value() -> W {
        W { bits: 3 }
    }
    #[doc = r" Writes raw bits to the register"]
    #[inline]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
    #[doc = "Bit 0 - RAM block 2 behaviour in ON mode."]
    #[inline]
    pub fn onram2(&mut self) -> _ONRAM2W {
        _ONRAM2W { w: self }
    }
    #[doc = "Bit 1 - RAM block 3 behaviour in ON mode."]
    #[inline]
    pub fn onram3(&mut self) -> _ONRAM3W {
        _ONRAM3W { w: self }
    }
    #[doc = "Bit 16 - RAM block 2 behaviour in OFF mode."]
    #[inline]
    pub fn offram2(&mut self) -> _OFFRAM2W {
        _OFFRAM2W { w: self }
    }
    #[doc = "Bit 17 - RAM block 3 behaviour in OFF mode."]
    #[inline]
    pub fn offram3(&mut self) -> _OFFRAM3W {
        _OFFRAM3W { w: self }
    }
}
