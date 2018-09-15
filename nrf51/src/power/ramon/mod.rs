#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::RAMON {
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
#[doc = "Possible values of the field `ONRAM0`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ONRAM0R {
    #[doc = "RAM block 0 OFF in ON mode."]
    RAM0OFF,
    #[doc = "RAM block 0 ON in ON mode."]
    RAM0ON,
}
impl ONRAM0R {
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
            ONRAM0R::RAM0OFF => false,
            ONRAM0R::RAM0ON => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> ONRAM0R {
        match value {
            false => ONRAM0R::RAM0OFF,
            true => ONRAM0R::RAM0ON,
        }
    }
    #[doc = "Checks if the value of the field is `RAM0OFF`"]
    #[inline]
    pub fn is_ram0off(&self) -> bool {
        *self == ONRAM0R::RAM0OFF
    }
    #[doc = "Checks if the value of the field is `RAM0ON`"]
    #[inline]
    pub fn is_ram0on(&self) -> bool {
        *self == ONRAM0R::RAM0ON
    }
}
#[doc = "Possible values of the field `ONRAM1`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ONRAM1R {
    #[doc = "RAM block 1 OFF in ON mode."]
    RAM1OFF,
    #[doc = "RAM block 1 ON in ON mode."]
    RAM1ON,
}
impl ONRAM1R {
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
            ONRAM1R::RAM1OFF => false,
            ONRAM1R::RAM1ON => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> ONRAM1R {
        match value {
            false => ONRAM1R::RAM1OFF,
            true => ONRAM1R::RAM1ON,
        }
    }
    #[doc = "Checks if the value of the field is `RAM1OFF`"]
    #[inline]
    pub fn is_ram1off(&self) -> bool {
        *self == ONRAM1R::RAM1OFF
    }
    #[doc = "Checks if the value of the field is `RAM1ON`"]
    #[inline]
    pub fn is_ram1on(&self) -> bool {
        *self == ONRAM1R::RAM1ON
    }
}
#[doc = "Possible values of the field `OFFRAM0`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum OFFRAM0R {
    #[doc = "RAM block 0 OFF in OFF mode."]
    RAM0OFF,
    #[doc = "RAM block 0 ON in OFF mode."]
    RAM0ON,
}
impl OFFRAM0R {
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
            OFFRAM0R::RAM0OFF => false,
            OFFRAM0R::RAM0ON => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> OFFRAM0R {
        match value {
            false => OFFRAM0R::RAM0OFF,
            true => OFFRAM0R::RAM0ON,
        }
    }
    #[doc = "Checks if the value of the field is `RAM0OFF`"]
    #[inline]
    pub fn is_ram0off(&self) -> bool {
        *self == OFFRAM0R::RAM0OFF
    }
    #[doc = "Checks if the value of the field is `RAM0ON`"]
    #[inline]
    pub fn is_ram0on(&self) -> bool {
        *self == OFFRAM0R::RAM0ON
    }
}
#[doc = "Possible values of the field `OFFRAM1`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum OFFRAM1R {
    #[doc = "RAM block 1 OFF in OFF mode."]
    RAM1OFF,
    #[doc = "RAM block 1 ON in OFF mode."]
    RAM1ON,
}
impl OFFRAM1R {
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
            OFFRAM1R::RAM1OFF => false,
            OFFRAM1R::RAM1ON => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> OFFRAM1R {
        match value {
            false => OFFRAM1R::RAM1OFF,
            true => OFFRAM1R::RAM1ON,
        }
    }
    #[doc = "Checks if the value of the field is `RAM1OFF`"]
    #[inline]
    pub fn is_ram1off(&self) -> bool {
        *self == OFFRAM1R::RAM1OFF
    }
    #[doc = "Checks if the value of the field is `RAM1ON`"]
    #[inline]
    pub fn is_ram1on(&self) -> bool {
        *self == OFFRAM1R::RAM1ON
    }
}
#[doc = "Values that can be written to the field `ONRAM0`"]
pub enum ONRAM0W {
    #[doc = "RAM block 0 OFF in ON mode."]
    RAM0OFF,
    #[doc = "RAM block 0 ON in ON mode."]
    RAM0ON,
}
impl ONRAM0W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            ONRAM0W::RAM0OFF => false,
            ONRAM0W::RAM0ON => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _ONRAM0W<'a> {
    w: &'a mut W,
}
impl<'a> _ONRAM0W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: ONRAM0W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "RAM block 0 OFF in ON mode."]
    #[inline]
    pub fn ram0off(self) -> &'a mut W {
        self.variant(ONRAM0W::RAM0OFF)
    }
    #[doc = "RAM block 0 ON in ON mode."]
    #[inline]
    pub fn ram0on(self) -> &'a mut W {
        self.variant(ONRAM0W::RAM0ON)
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
#[doc = "Values that can be written to the field `ONRAM1`"]
pub enum ONRAM1W {
    #[doc = "RAM block 1 OFF in ON mode."]
    RAM1OFF,
    #[doc = "RAM block 1 ON in ON mode."]
    RAM1ON,
}
impl ONRAM1W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            ONRAM1W::RAM1OFF => false,
            ONRAM1W::RAM1ON => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _ONRAM1W<'a> {
    w: &'a mut W,
}
impl<'a> _ONRAM1W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: ONRAM1W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "RAM block 1 OFF in ON mode."]
    #[inline]
    pub fn ram1off(self) -> &'a mut W {
        self.variant(ONRAM1W::RAM1OFF)
    }
    #[doc = "RAM block 1 ON in ON mode."]
    #[inline]
    pub fn ram1on(self) -> &'a mut W {
        self.variant(ONRAM1W::RAM1ON)
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
#[doc = "Values that can be written to the field `OFFRAM0`"]
pub enum OFFRAM0W {
    #[doc = "RAM block 0 OFF in OFF mode."]
    RAM0OFF,
    #[doc = "RAM block 0 ON in OFF mode."]
    RAM0ON,
}
impl OFFRAM0W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            OFFRAM0W::RAM0OFF => false,
            OFFRAM0W::RAM0ON => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _OFFRAM0W<'a> {
    w: &'a mut W,
}
impl<'a> _OFFRAM0W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: OFFRAM0W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "RAM block 0 OFF in OFF mode."]
    #[inline]
    pub fn ram0off(self) -> &'a mut W {
        self.variant(OFFRAM0W::RAM0OFF)
    }
    #[doc = "RAM block 0 ON in OFF mode."]
    #[inline]
    pub fn ram0on(self) -> &'a mut W {
        self.variant(OFFRAM0W::RAM0ON)
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
#[doc = "Values that can be written to the field `OFFRAM1`"]
pub enum OFFRAM1W {
    #[doc = "RAM block 1 OFF in OFF mode."]
    RAM1OFF,
    #[doc = "RAM block 1 ON in OFF mode."]
    RAM1ON,
}
impl OFFRAM1W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            OFFRAM1W::RAM1OFF => false,
            OFFRAM1W::RAM1ON => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _OFFRAM1W<'a> {
    w: &'a mut W,
}
impl<'a> _OFFRAM1W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: OFFRAM1W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "RAM block 1 OFF in OFF mode."]
    #[inline]
    pub fn ram1off(self) -> &'a mut W {
        self.variant(OFFRAM1W::RAM1OFF)
    }
    #[doc = "RAM block 1 ON in OFF mode."]
    #[inline]
    pub fn ram1on(self) -> &'a mut W {
        self.variant(OFFRAM1W::RAM1ON)
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
    #[doc = "Bit 0 - RAM block 0 behaviour in ON mode."]
    #[inline]
    pub fn onram0(&self) -> ONRAM0R {
        ONRAM0R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 1 - RAM block 1 behaviour in ON mode."]
    #[inline]
    pub fn onram1(&self) -> ONRAM1R {
        ONRAM1R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 1;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 16 - RAM block 0 behaviour in OFF mode."]
    #[inline]
    pub fn offram0(&self) -> OFFRAM0R {
        OFFRAM0R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 16;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 17 - RAM block 1 behaviour in OFF mode."]
    #[inline]
    pub fn offram1(&self) -> OFFRAM1R {
        OFFRAM1R::_from({
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
    #[doc = "Bit 0 - RAM block 0 behaviour in ON mode."]
    #[inline]
    pub fn onram0(&mut self) -> _ONRAM0W {
        _ONRAM0W { w: self }
    }
    #[doc = "Bit 1 - RAM block 1 behaviour in ON mode."]
    #[inline]
    pub fn onram1(&mut self) -> _ONRAM1W {
        _ONRAM1W { w: self }
    }
    #[doc = "Bit 16 - RAM block 0 behaviour in OFF mode."]
    #[inline]
    pub fn offram0(&mut self) -> _OFFRAM0W {
        _OFFRAM0W { w: self }
    }
    #[doc = "Bit 17 - RAM block 1 behaviour in OFF mode."]
    #[inline]
    pub fn offram1(&mut self) -> _OFFRAM1W {
        _OFFRAM1W { w: self }
    }
}
