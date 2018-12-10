#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::CONFIG {
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
#[doc = "Possible values of the field `ADDRESS0`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ADDRESS0R {
    #[doc = "Disabled"]
    DISABLED,
    #[doc = "Enabled"]
    ENABLED,
}
impl ADDRESS0R {
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
            ADDRESS0R::DISABLED => false,
            ADDRESS0R::ENABLED => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> ADDRESS0R {
        match value {
            false => ADDRESS0R::DISABLED,
            true => ADDRESS0R::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline]
    pub fn is_disabled(&self) -> bool {
        *self == ADDRESS0R::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline]
    pub fn is_enabled(&self) -> bool {
        *self == ADDRESS0R::ENABLED
    }
}
#[doc = "Possible values of the field `ADDRESS1`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ADDRESS1R {
    #[doc = "Disabled"]
    DISABLED,
    #[doc = "Enabled"]
    ENABLED,
}
impl ADDRESS1R {
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
            ADDRESS1R::DISABLED => false,
            ADDRESS1R::ENABLED => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> ADDRESS1R {
        match value {
            false => ADDRESS1R::DISABLED,
            true => ADDRESS1R::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline]
    pub fn is_disabled(&self) -> bool {
        *self == ADDRESS1R::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline]
    pub fn is_enabled(&self) -> bool {
        *self == ADDRESS1R::ENABLED
    }
}
#[doc = "Values that can be written to the field `ADDRESS0`"]
pub enum ADDRESS0W {
    #[doc = "Disabled"]
    DISABLED,
    #[doc = "Enabled"]
    ENABLED,
}
impl ADDRESS0W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            ADDRESS0W::DISABLED => false,
            ADDRESS0W::ENABLED => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _ADDRESS0W<'a> {
    w: &'a mut W,
}
impl<'a> _ADDRESS0W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: ADDRESS0W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Disabled"]
    #[inline]
    pub fn disabled(self) -> &'a mut W {
        self.variant(ADDRESS0W::DISABLED)
    }
    #[doc = "Enabled"]
    #[inline]
    pub fn enabled(self) -> &'a mut W {
        self.variant(ADDRESS0W::ENABLED)
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
#[doc = "Values that can be written to the field `ADDRESS1`"]
pub enum ADDRESS1W {
    #[doc = "Disabled"]
    DISABLED,
    #[doc = "Enabled"]
    ENABLED,
}
impl ADDRESS1W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            ADDRESS1W::DISABLED => false,
            ADDRESS1W::ENABLED => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _ADDRESS1W<'a> {
    w: &'a mut W,
}
impl<'a> _ADDRESS1W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: ADDRESS1W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Disabled"]
    #[inline]
    pub fn disabled(self) -> &'a mut W {
        self.variant(ADDRESS1W::DISABLED)
    }
    #[doc = "Enabled"]
    #[inline]
    pub fn enabled(self) -> &'a mut W {
        self.variant(ADDRESS1W::ENABLED)
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
impl R {
    #[doc = r" Value of the register as raw bits"]
    #[inline]
    pub fn bits(&self) -> u32 {
        self.bits
    }
    #[doc = "Bit 0 - Enable or disable address matching on ADDRESS\\[0\\]"]
    #[inline]
    pub fn address0(&self) -> ADDRESS0R {
        ADDRESS0R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 1 - Enable or disable address matching on ADDRESS\\[1\\]"]
    #[inline]
    pub fn address1(&self) -> ADDRESS1R {
        ADDRESS1R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 1;
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
    #[doc = "Bit 0 - Enable or disable address matching on ADDRESS\\[0\\]"]
    #[inline]
    pub fn address0(&mut self) -> _ADDRESS0W {
        _ADDRESS0W { w: self }
    }
    #[doc = "Bit 1 - Enable or disable address matching on ADDRESS\\[1\\]"]
    #[inline]
    pub fn address1(&mut self) -> _ADDRESS1W {
        _ADDRESS1W { w: self }
    }
}
