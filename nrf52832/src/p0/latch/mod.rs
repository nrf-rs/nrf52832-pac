#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::LATCH {
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
#[doc = "Possible values of the field `PIN0`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PIN0R {
    #[doc = "Criteria has not been met"]
    NOTLATCHED,
    #[doc = "Criteria has been met"]
    LATCHED,
}
impl PIN0R {
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
            PIN0R::NOTLATCHED => false,
            PIN0R::LATCHED => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> PIN0R {
        match value {
            false => PIN0R::NOTLATCHED,
            true => PIN0R::LATCHED,
        }
    }
    #[doc = "Checks if the value of the field is `NOTLATCHED`"]
    #[inline]
    pub fn is_not_latched(&self) -> bool {
        *self == PIN0R::NOTLATCHED
    }
    #[doc = "Checks if the value of the field is `LATCHED`"]
    #[inline]
    pub fn is_latched(&self) -> bool {
        *self == PIN0R::LATCHED
    }
}
#[doc = "Possible values of the field `PIN1`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PIN1R {
    #[doc = "Criteria has not been met"]
    NOTLATCHED,
    #[doc = "Criteria has been met"]
    LATCHED,
}
impl PIN1R {
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
            PIN1R::NOTLATCHED => false,
            PIN1R::LATCHED => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> PIN1R {
        match value {
            false => PIN1R::NOTLATCHED,
            true => PIN1R::LATCHED,
        }
    }
    #[doc = "Checks if the value of the field is `NOTLATCHED`"]
    #[inline]
    pub fn is_not_latched(&self) -> bool {
        *self == PIN1R::NOTLATCHED
    }
    #[doc = "Checks if the value of the field is `LATCHED`"]
    #[inline]
    pub fn is_latched(&self) -> bool {
        *self == PIN1R::LATCHED
    }
}
#[doc = "Possible values of the field `PIN2`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PIN2R {
    #[doc = "Criteria has not been met"]
    NOTLATCHED,
    #[doc = "Criteria has been met"]
    LATCHED,
}
impl PIN2R {
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
            PIN2R::NOTLATCHED => false,
            PIN2R::LATCHED => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> PIN2R {
        match value {
            false => PIN2R::NOTLATCHED,
            true => PIN2R::LATCHED,
        }
    }
    #[doc = "Checks if the value of the field is `NOTLATCHED`"]
    #[inline]
    pub fn is_not_latched(&self) -> bool {
        *self == PIN2R::NOTLATCHED
    }
    #[doc = "Checks if the value of the field is `LATCHED`"]
    #[inline]
    pub fn is_latched(&self) -> bool {
        *self == PIN2R::LATCHED
    }
}
#[doc = "Possible values of the field `PIN3`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PIN3R {
    #[doc = "Criteria has not been met"]
    NOTLATCHED,
    #[doc = "Criteria has been met"]
    LATCHED,
}
impl PIN3R {
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
            PIN3R::NOTLATCHED => false,
            PIN3R::LATCHED => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> PIN3R {
        match value {
            false => PIN3R::NOTLATCHED,
            true => PIN3R::LATCHED,
        }
    }
    #[doc = "Checks if the value of the field is `NOTLATCHED`"]
    #[inline]
    pub fn is_not_latched(&self) -> bool {
        *self == PIN3R::NOTLATCHED
    }
    #[doc = "Checks if the value of the field is `LATCHED`"]
    #[inline]
    pub fn is_latched(&self) -> bool {
        *self == PIN3R::LATCHED
    }
}
#[doc = "Possible values of the field `PIN4`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PIN4R {
    #[doc = "Criteria has not been met"]
    NOTLATCHED,
    #[doc = "Criteria has been met"]
    LATCHED,
}
impl PIN4R {
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
            PIN4R::NOTLATCHED => false,
            PIN4R::LATCHED => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> PIN4R {
        match value {
            false => PIN4R::NOTLATCHED,
            true => PIN4R::LATCHED,
        }
    }
    #[doc = "Checks if the value of the field is `NOTLATCHED`"]
    #[inline]
    pub fn is_not_latched(&self) -> bool {
        *self == PIN4R::NOTLATCHED
    }
    #[doc = "Checks if the value of the field is `LATCHED`"]
    #[inline]
    pub fn is_latched(&self) -> bool {
        *self == PIN4R::LATCHED
    }
}
#[doc = "Possible values of the field `PIN5`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PIN5R {
    #[doc = "Criteria has not been met"]
    NOTLATCHED,
    #[doc = "Criteria has been met"]
    LATCHED,
}
impl PIN5R {
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
            PIN5R::NOTLATCHED => false,
            PIN5R::LATCHED => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> PIN5R {
        match value {
            false => PIN5R::NOTLATCHED,
            true => PIN5R::LATCHED,
        }
    }
    #[doc = "Checks if the value of the field is `NOTLATCHED`"]
    #[inline]
    pub fn is_not_latched(&self) -> bool {
        *self == PIN5R::NOTLATCHED
    }
    #[doc = "Checks if the value of the field is `LATCHED`"]
    #[inline]
    pub fn is_latched(&self) -> bool {
        *self == PIN5R::LATCHED
    }
}
#[doc = "Possible values of the field `PIN6`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PIN6R {
    #[doc = "Criteria has not been met"]
    NOTLATCHED,
    #[doc = "Criteria has been met"]
    LATCHED,
}
impl PIN6R {
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
            PIN6R::NOTLATCHED => false,
            PIN6R::LATCHED => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> PIN6R {
        match value {
            false => PIN6R::NOTLATCHED,
            true => PIN6R::LATCHED,
        }
    }
    #[doc = "Checks if the value of the field is `NOTLATCHED`"]
    #[inline]
    pub fn is_not_latched(&self) -> bool {
        *self == PIN6R::NOTLATCHED
    }
    #[doc = "Checks if the value of the field is `LATCHED`"]
    #[inline]
    pub fn is_latched(&self) -> bool {
        *self == PIN6R::LATCHED
    }
}
#[doc = "Possible values of the field `PIN7`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PIN7R {
    #[doc = "Criteria has not been met"]
    NOTLATCHED,
    #[doc = "Criteria has been met"]
    LATCHED,
}
impl PIN7R {
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
            PIN7R::NOTLATCHED => false,
            PIN7R::LATCHED => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> PIN7R {
        match value {
            false => PIN7R::NOTLATCHED,
            true => PIN7R::LATCHED,
        }
    }
    #[doc = "Checks if the value of the field is `NOTLATCHED`"]
    #[inline]
    pub fn is_not_latched(&self) -> bool {
        *self == PIN7R::NOTLATCHED
    }
    #[doc = "Checks if the value of the field is `LATCHED`"]
    #[inline]
    pub fn is_latched(&self) -> bool {
        *self == PIN7R::LATCHED
    }
}
#[doc = "Possible values of the field `PIN8`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PIN8R {
    #[doc = "Criteria has not been met"]
    NOTLATCHED,
    #[doc = "Criteria has been met"]
    LATCHED,
}
impl PIN8R {
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
            PIN8R::NOTLATCHED => false,
            PIN8R::LATCHED => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> PIN8R {
        match value {
            false => PIN8R::NOTLATCHED,
            true => PIN8R::LATCHED,
        }
    }
    #[doc = "Checks if the value of the field is `NOTLATCHED`"]
    #[inline]
    pub fn is_not_latched(&self) -> bool {
        *self == PIN8R::NOTLATCHED
    }
    #[doc = "Checks if the value of the field is `LATCHED`"]
    #[inline]
    pub fn is_latched(&self) -> bool {
        *self == PIN8R::LATCHED
    }
}
#[doc = "Possible values of the field `PIN9`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PIN9R {
    #[doc = "Criteria has not been met"]
    NOTLATCHED,
    #[doc = "Criteria has been met"]
    LATCHED,
}
impl PIN9R {
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
            PIN9R::NOTLATCHED => false,
            PIN9R::LATCHED => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> PIN9R {
        match value {
            false => PIN9R::NOTLATCHED,
            true => PIN9R::LATCHED,
        }
    }
    #[doc = "Checks if the value of the field is `NOTLATCHED`"]
    #[inline]
    pub fn is_not_latched(&self) -> bool {
        *self == PIN9R::NOTLATCHED
    }
    #[doc = "Checks if the value of the field is `LATCHED`"]
    #[inline]
    pub fn is_latched(&self) -> bool {
        *self == PIN9R::LATCHED
    }
}
#[doc = "Possible values of the field `PIN10`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PIN10R {
    #[doc = "Criteria has not been met"]
    NOTLATCHED,
    #[doc = "Criteria has been met"]
    LATCHED,
}
impl PIN10R {
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
            PIN10R::NOTLATCHED => false,
            PIN10R::LATCHED => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> PIN10R {
        match value {
            false => PIN10R::NOTLATCHED,
            true => PIN10R::LATCHED,
        }
    }
    #[doc = "Checks if the value of the field is `NOTLATCHED`"]
    #[inline]
    pub fn is_not_latched(&self) -> bool {
        *self == PIN10R::NOTLATCHED
    }
    #[doc = "Checks if the value of the field is `LATCHED`"]
    #[inline]
    pub fn is_latched(&self) -> bool {
        *self == PIN10R::LATCHED
    }
}
#[doc = "Possible values of the field `PIN11`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PIN11R {
    #[doc = "Criteria has not been met"]
    NOTLATCHED,
    #[doc = "Criteria has been met"]
    LATCHED,
}
impl PIN11R {
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
            PIN11R::NOTLATCHED => false,
            PIN11R::LATCHED => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> PIN11R {
        match value {
            false => PIN11R::NOTLATCHED,
            true => PIN11R::LATCHED,
        }
    }
    #[doc = "Checks if the value of the field is `NOTLATCHED`"]
    #[inline]
    pub fn is_not_latched(&self) -> bool {
        *self == PIN11R::NOTLATCHED
    }
    #[doc = "Checks if the value of the field is `LATCHED`"]
    #[inline]
    pub fn is_latched(&self) -> bool {
        *self == PIN11R::LATCHED
    }
}
#[doc = "Possible values of the field `PIN12`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PIN12R {
    #[doc = "Criteria has not been met"]
    NOTLATCHED,
    #[doc = "Criteria has been met"]
    LATCHED,
}
impl PIN12R {
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
            PIN12R::NOTLATCHED => false,
            PIN12R::LATCHED => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> PIN12R {
        match value {
            false => PIN12R::NOTLATCHED,
            true => PIN12R::LATCHED,
        }
    }
    #[doc = "Checks if the value of the field is `NOTLATCHED`"]
    #[inline]
    pub fn is_not_latched(&self) -> bool {
        *self == PIN12R::NOTLATCHED
    }
    #[doc = "Checks if the value of the field is `LATCHED`"]
    #[inline]
    pub fn is_latched(&self) -> bool {
        *self == PIN12R::LATCHED
    }
}
#[doc = "Possible values of the field `PIN13`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PIN13R {
    #[doc = "Criteria has not been met"]
    NOTLATCHED,
    #[doc = "Criteria has been met"]
    LATCHED,
}
impl PIN13R {
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
            PIN13R::NOTLATCHED => false,
            PIN13R::LATCHED => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> PIN13R {
        match value {
            false => PIN13R::NOTLATCHED,
            true => PIN13R::LATCHED,
        }
    }
    #[doc = "Checks if the value of the field is `NOTLATCHED`"]
    #[inline]
    pub fn is_not_latched(&self) -> bool {
        *self == PIN13R::NOTLATCHED
    }
    #[doc = "Checks if the value of the field is `LATCHED`"]
    #[inline]
    pub fn is_latched(&self) -> bool {
        *self == PIN13R::LATCHED
    }
}
#[doc = "Possible values of the field `PIN14`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PIN14R {
    #[doc = "Criteria has not been met"]
    NOTLATCHED,
    #[doc = "Criteria has been met"]
    LATCHED,
}
impl PIN14R {
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
            PIN14R::NOTLATCHED => false,
            PIN14R::LATCHED => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> PIN14R {
        match value {
            false => PIN14R::NOTLATCHED,
            true => PIN14R::LATCHED,
        }
    }
    #[doc = "Checks if the value of the field is `NOTLATCHED`"]
    #[inline]
    pub fn is_not_latched(&self) -> bool {
        *self == PIN14R::NOTLATCHED
    }
    #[doc = "Checks if the value of the field is `LATCHED`"]
    #[inline]
    pub fn is_latched(&self) -> bool {
        *self == PIN14R::LATCHED
    }
}
#[doc = "Possible values of the field `PIN15`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PIN15R {
    #[doc = "Criteria has not been met"]
    NOTLATCHED,
    #[doc = "Criteria has been met"]
    LATCHED,
}
impl PIN15R {
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
            PIN15R::NOTLATCHED => false,
            PIN15R::LATCHED => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> PIN15R {
        match value {
            false => PIN15R::NOTLATCHED,
            true => PIN15R::LATCHED,
        }
    }
    #[doc = "Checks if the value of the field is `NOTLATCHED`"]
    #[inline]
    pub fn is_not_latched(&self) -> bool {
        *self == PIN15R::NOTLATCHED
    }
    #[doc = "Checks if the value of the field is `LATCHED`"]
    #[inline]
    pub fn is_latched(&self) -> bool {
        *self == PIN15R::LATCHED
    }
}
#[doc = "Possible values of the field `PIN16`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PIN16R {
    #[doc = "Criteria has not been met"]
    NOTLATCHED,
    #[doc = "Criteria has been met"]
    LATCHED,
}
impl PIN16R {
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
            PIN16R::NOTLATCHED => false,
            PIN16R::LATCHED => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> PIN16R {
        match value {
            false => PIN16R::NOTLATCHED,
            true => PIN16R::LATCHED,
        }
    }
    #[doc = "Checks if the value of the field is `NOTLATCHED`"]
    #[inline]
    pub fn is_not_latched(&self) -> bool {
        *self == PIN16R::NOTLATCHED
    }
    #[doc = "Checks if the value of the field is `LATCHED`"]
    #[inline]
    pub fn is_latched(&self) -> bool {
        *self == PIN16R::LATCHED
    }
}
#[doc = "Possible values of the field `PIN17`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PIN17R {
    #[doc = "Criteria has not been met"]
    NOTLATCHED,
    #[doc = "Criteria has been met"]
    LATCHED,
}
impl PIN17R {
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
            PIN17R::NOTLATCHED => false,
            PIN17R::LATCHED => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> PIN17R {
        match value {
            false => PIN17R::NOTLATCHED,
            true => PIN17R::LATCHED,
        }
    }
    #[doc = "Checks if the value of the field is `NOTLATCHED`"]
    #[inline]
    pub fn is_not_latched(&self) -> bool {
        *self == PIN17R::NOTLATCHED
    }
    #[doc = "Checks if the value of the field is `LATCHED`"]
    #[inline]
    pub fn is_latched(&self) -> bool {
        *self == PIN17R::LATCHED
    }
}
#[doc = "Possible values of the field `PIN18`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PIN18R {
    #[doc = "Criteria has not been met"]
    NOTLATCHED,
    #[doc = "Criteria has been met"]
    LATCHED,
}
impl PIN18R {
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
            PIN18R::NOTLATCHED => false,
            PIN18R::LATCHED => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> PIN18R {
        match value {
            false => PIN18R::NOTLATCHED,
            true => PIN18R::LATCHED,
        }
    }
    #[doc = "Checks if the value of the field is `NOTLATCHED`"]
    #[inline]
    pub fn is_not_latched(&self) -> bool {
        *self == PIN18R::NOTLATCHED
    }
    #[doc = "Checks if the value of the field is `LATCHED`"]
    #[inline]
    pub fn is_latched(&self) -> bool {
        *self == PIN18R::LATCHED
    }
}
#[doc = "Possible values of the field `PIN19`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PIN19R {
    #[doc = "Criteria has not been met"]
    NOTLATCHED,
    #[doc = "Criteria has been met"]
    LATCHED,
}
impl PIN19R {
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
            PIN19R::NOTLATCHED => false,
            PIN19R::LATCHED => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> PIN19R {
        match value {
            false => PIN19R::NOTLATCHED,
            true => PIN19R::LATCHED,
        }
    }
    #[doc = "Checks if the value of the field is `NOTLATCHED`"]
    #[inline]
    pub fn is_not_latched(&self) -> bool {
        *self == PIN19R::NOTLATCHED
    }
    #[doc = "Checks if the value of the field is `LATCHED`"]
    #[inline]
    pub fn is_latched(&self) -> bool {
        *self == PIN19R::LATCHED
    }
}
#[doc = "Possible values of the field `PIN20`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PIN20R {
    #[doc = "Criteria has not been met"]
    NOTLATCHED,
    #[doc = "Criteria has been met"]
    LATCHED,
}
impl PIN20R {
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
            PIN20R::NOTLATCHED => false,
            PIN20R::LATCHED => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> PIN20R {
        match value {
            false => PIN20R::NOTLATCHED,
            true => PIN20R::LATCHED,
        }
    }
    #[doc = "Checks if the value of the field is `NOTLATCHED`"]
    #[inline]
    pub fn is_not_latched(&self) -> bool {
        *self == PIN20R::NOTLATCHED
    }
    #[doc = "Checks if the value of the field is `LATCHED`"]
    #[inline]
    pub fn is_latched(&self) -> bool {
        *self == PIN20R::LATCHED
    }
}
#[doc = "Possible values of the field `PIN21`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PIN21R {
    #[doc = "Criteria has not been met"]
    NOTLATCHED,
    #[doc = "Criteria has been met"]
    LATCHED,
}
impl PIN21R {
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
            PIN21R::NOTLATCHED => false,
            PIN21R::LATCHED => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> PIN21R {
        match value {
            false => PIN21R::NOTLATCHED,
            true => PIN21R::LATCHED,
        }
    }
    #[doc = "Checks if the value of the field is `NOTLATCHED`"]
    #[inline]
    pub fn is_not_latched(&self) -> bool {
        *self == PIN21R::NOTLATCHED
    }
    #[doc = "Checks if the value of the field is `LATCHED`"]
    #[inline]
    pub fn is_latched(&self) -> bool {
        *self == PIN21R::LATCHED
    }
}
#[doc = "Possible values of the field `PIN22`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PIN22R {
    #[doc = "Criteria has not been met"]
    NOTLATCHED,
    #[doc = "Criteria has been met"]
    LATCHED,
}
impl PIN22R {
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
            PIN22R::NOTLATCHED => false,
            PIN22R::LATCHED => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> PIN22R {
        match value {
            false => PIN22R::NOTLATCHED,
            true => PIN22R::LATCHED,
        }
    }
    #[doc = "Checks if the value of the field is `NOTLATCHED`"]
    #[inline]
    pub fn is_not_latched(&self) -> bool {
        *self == PIN22R::NOTLATCHED
    }
    #[doc = "Checks if the value of the field is `LATCHED`"]
    #[inline]
    pub fn is_latched(&self) -> bool {
        *self == PIN22R::LATCHED
    }
}
#[doc = "Possible values of the field `PIN23`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PIN23R {
    #[doc = "Criteria has not been met"]
    NOTLATCHED,
    #[doc = "Criteria has been met"]
    LATCHED,
}
impl PIN23R {
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
            PIN23R::NOTLATCHED => false,
            PIN23R::LATCHED => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> PIN23R {
        match value {
            false => PIN23R::NOTLATCHED,
            true => PIN23R::LATCHED,
        }
    }
    #[doc = "Checks if the value of the field is `NOTLATCHED`"]
    #[inline]
    pub fn is_not_latched(&self) -> bool {
        *self == PIN23R::NOTLATCHED
    }
    #[doc = "Checks if the value of the field is `LATCHED`"]
    #[inline]
    pub fn is_latched(&self) -> bool {
        *self == PIN23R::LATCHED
    }
}
#[doc = "Possible values of the field `PIN24`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PIN24R {
    #[doc = "Criteria has not been met"]
    NOTLATCHED,
    #[doc = "Criteria has been met"]
    LATCHED,
}
impl PIN24R {
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
            PIN24R::NOTLATCHED => false,
            PIN24R::LATCHED => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> PIN24R {
        match value {
            false => PIN24R::NOTLATCHED,
            true => PIN24R::LATCHED,
        }
    }
    #[doc = "Checks if the value of the field is `NOTLATCHED`"]
    #[inline]
    pub fn is_not_latched(&self) -> bool {
        *self == PIN24R::NOTLATCHED
    }
    #[doc = "Checks if the value of the field is `LATCHED`"]
    #[inline]
    pub fn is_latched(&self) -> bool {
        *self == PIN24R::LATCHED
    }
}
#[doc = "Possible values of the field `PIN25`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PIN25R {
    #[doc = "Criteria has not been met"]
    NOTLATCHED,
    #[doc = "Criteria has been met"]
    LATCHED,
}
impl PIN25R {
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
            PIN25R::NOTLATCHED => false,
            PIN25R::LATCHED => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> PIN25R {
        match value {
            false => PIN25R::NOTLATCHED,
            true => PIN25R::LATCHED,
        }
    }
    #[doc = "Checks if the value of the field is `NOTLATCHED`"]
    #[inline]
    pub fn is_not_latched(&self) -> bool {
        *self == PIN25R::NOTLATCHED
    }
    #[doc = "Checks if the value of the field is `LATCHED`"]
    #[inline]
    pub fn is_latched(&self) -> bool {
        *self == PIN25R::LATCHED
    }
}
#[doc = "Possible values of the field `PIN26`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PIN26R {
    #[doc = "Criteria has not been met"]
    NOTLATCHED,
    #[doc = "Criteria has been met"]
    LATCHED,
}
impl PIN26R {
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
            PIN26R::NOTLATCHED => false,
            PIN26R::LATCHED => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> PIN26R {
        match value {
            false => PIN26R::NOTLATCHED,
            true => PIN26R::LATCHED,
        }
    }
    #[doc = "Checks if the value of the field is `NOTLATCHED`"]
    #[inline]
    pub fn is_not_latched(&self) -> bool {
        *self == PIN26R::NOTLATCHED
    }
    #[doc = "Checks if the value of the field is `LATCHED`"]
    #[inline]
    pub fn is_latched(&self) -> bool {
        *self == PIN26R::LATCHED
    }
}
#[doc = "Possible values of the field `PIN27`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PIN27R {
    #[doc = "Criteria has not been met"]
    NOTLATCHED,
    #[doc = "Criteria has been met"]
    LATCHED,
}
impl PIN27R {
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
            PIN27R::NOTLATCHED => false,
            PIN27R::LATCHED => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> PIN27R {
        match value {
            false => PIN27R::NOTLATCHED,
            true => PIN27R::LATCHED,
        }
    }
    #[doc = "Checks if the value of the field is `NOTLATCHED`"]
    #[inline]
    pub fn is_not_latched(&self) -> bool {
        *self == PIN27R::NOTLATCHED
    }
    #[doc = "Checks if the value of the field is `LATCHED`"]
    #[inline]
    pub fn is_latched(&self) -> bool {
        *self == PIN27R::LATCHED
    }
}
#[doc = "Possible values of the field `PIN28`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PIN28R {
    #[doc = "Criteria has not been met"]
    NOTLATCHED,
    #[doc = "Criteria has been met"]
    LATCHED,
}
impl PIN28R {
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
            PIN28R::NOTLATCHED => false,
            PIN28R::LATCHED => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> PIN28R {
        match value {
            false => PIN28R::NOTLATCHED,
            true => PIN28R::LATCHED,
        }
    }
    #[doc = "Checks if the value of the field is `NOTLATCHED`"]
    #[inline]
    pub fn is_not_latched(&self) -> bool {
        *self == PIN28R::NOTLATCHED
    }
    #[doc = "Checks if the value of the field is `LATCHED`"]
    #[inline]
    pub fn is_latched(&self) -> bool {
        *self == PIN28R::LATCHED
    }
}
#[doc = "Possible values of the field `PIN29`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PIN29R {
    #[doc = "Criteria has not been met"]
    NOTLATCHED,
    #[doc = "Criteria has been met"]
    LATCHED,
}
impl PIN29R {
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
            PIN29R::NOTLATCHED => false,
            PIN29R::LATCHED => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> PIN29R {
        match value {
            false => PIN29R::NOTLATCHED,
            true => PIN29R::LATCHED,
        }
    }
    #[doc = "Checks if the value of the field is `NOTLATCHED`"]
    #[inline]
    pub fn is_not_latched(&self) -> bool {
        *self == PIN29R::NOTLATCHED
    }
    #[doc = "Checks if the value of the field is `LATCHED`"]
    #[inline]
    pub fn is_latched(&self) -> bool {
        *self == PIN29R::LATCHED
    }
}
#[doc = "Possible values of the field `PIN30`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PIN30R {
    #[doc = "Criteria has not been met"]
    NOTLATCHED,
    #[doc = "Criteria has been met"]
    LATCHED,
}
impl PIN30R {
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
            PIN30R::NOTLATCHED => false,
            PIN30R::LATCHED => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> PIN30R {
        match value {
            false => PIN30R::NOTLATCHED,
            true => PIN30R::LATCHED,
        }
    }
    #[doc = "Checks if the value of the field is `NOTLATCHED`"]
    #[inline]
    pub fn is_not_latched(&self) -> bool {
        *self == PIN30R::NOTLATCHED
    }
    #[doc = "Checks if the value of the field is `LATCHED`"]
    #[inline]
    pub fn is_latched(&self) -> bool {
        *self == PIN30R::LATCHED
    }
}
#[doc = "Possible values of the field `PIN31`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PIN31R {
    #[doc = "Criteria has not been met"]
    NOTLATCHED,
    #[doc = "Criteria has been met"]
    LATCHED,
}
impl PIN31R {
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
            PIN31R::NOTLATCHED => false,
            PIN31R::LATCHED => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> PIN31R {
        match value {
            false => PIN31R::NOTLATCHED,
            true => PIN31R::LATCHED,
        }
    }
    #[doc = "Checks if the value of the field is `NOTLATCHED`"]
    #[inline]
    pub fn is_not_latched(&self) -> bool {
        *self == PIN31R::NOTLATCHED
    }
    #[doc = "Checks if the value of the field is `LATCHED`"]
    #[inline]
    pub fn is_latched(&self) -> bool {
        *self == PIN31R::LATCHED
    }
}
#[doc = "Values that can be written to the field `PIN0`"]
pub enum PIN0W {
    #[doc = "Criteria has not been met"]
    NOTLATCHED,
    #[doc = "Criteria has been met"]
    LATCHED,
}
impl PIN0W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            PIN0W::NOTLATCHED => false,
            PIN0W::LATCHED => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _PIN0W<'a> {
    w: &'a mut W,
}
impl<'a> _PIN0W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: PIN0W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Criteria has not been met"]
    #[inline]
    pub fn not_latched(self) -> &'a mut W {
        self.variant(PIN0W::NOTLATCHED)
    }
    #[doc = "Criteria has been met"]
    #[inline]
    pub fn latched(self) -> &'a mut W {
        self.variant(PIN0W::LATCHED)
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
#[doc = "Values that can be written to the field `PIN1`"]
pub enum PIN1W {
    #[doc = "Criteria has not been met"]
    NOTLATCHED,
    #[doc = "Criteria has been met"]
    LATCHED,
}
impl PIN1W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            PIN1W::NOTLATCHED => false,
            PIN1W::LATCHED => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _PIN1W<'a> {
    w: &'a mut W,
}
impl<'a> _PIN1W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: PIN1W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Criteria has not been met"]
    #[inline]
    pub fn not_latched(self) -> &'a mut W {
        self.variant(PIN1W::NOTLATCHED)
    }
    #[doc = "Criteria has been met"]
    #[inline]
    pub fn latched(self) -> &'a mut W {
        self.variant(PIN1W::LATCHED)
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
#[doc = "Values that can be written to the field `PIN2`"]
pub enum PIN2W {
    #[doc = "Criteria has not been met"]
    NOTLATCHED,
    #[doc = "Criteria has been met"]
    LATCHED,
}
impl PIN2W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            PIN2W::NOTLATCHED => false,
            PIN2W::LATCHED => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _PIN2W<'a> {
    w: &'a mut W,
}
impl<'a> _PIN2W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: PIN2W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Criteria has not been met"]
    #[inline]
    pub fn not_latched(self) -> &'a mut W {
        self.variant(PIN2W::NOTLATCHED)
    }
    #[doc = "Criteria has been met"]
    #[inline]
    pub fn latched(self) -> &'a mut W {
        self.variant(PIN2W::LATCHED)
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
#[doc = "Values that can be written to the field `PIN3`"]
pub enum PIN3W {
    #[doc = "Criteria has not been met"]
    NOTLATCHED,
    #[doc = "Criteria has been met"]
    LATCHED,
}
impl PIN3W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            PIN3W::NOTLATCHED => false,
            PIN3W::LATCHED => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _PIN3W<'a> {
    w: &'a mut W,
}
impl<'a> _PIN3W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: PIN3W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Criteria has not been met"]
    #[inline]
    pub fn not_latched(self) -> &'a mut W {
        self.variant(PIN3W::NOTLATCHED)
    }
    #[doc = "Criteria has been met"]
    #[inline]
    pub fn latched(self) -> &'a mut W {
        self.variant(PIN3W::LATCHED)
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
#[doc = "Values that can be written to the field `PIN4`"]
pub enum PIN4W {
    #[doc = "Criteria has not been met"]
    NOTLATCHED,
    #[doc = "Criteria has been met"]
    LATCHED,
}
impl PIN4W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            PIN4W::NOTLATCHED => false,
            PIN4W::LATCHED => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _PIN4W<'a> {
    w: &'a mut W,
}
impl<'a> _PIN4W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: PIN4W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Criteria has not been met"]
    #[inline]
    pub fn not_latched(self) -> &'a mut W {
        self.variant(PIN4W::NOTLATCHED)
    }
    #[doc = "Criteria has been met"]
    #[inline]
    pub fn latched(self) -> &'a mut W {
        self.variant(PIN4W::LATCHED)
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
        const OFFSET: u8 = 4;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `PIN5`"]
pub enum PIN5W {
    #[doc = "Criteria has not been met"]
    NOTLATCHED,
    #[doc = "Criteria has been met"]
    LATCHED,
}
impl PIN5W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            PIN5W::NOTLATCHED => false,
            PIN5W::LATCHED => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _PIN5W<'a> {
    w: &'a mut W,
}
impl<'a> _PIN5W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: PIN5W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Criteria has not been met"]
    #[inline]
    pub fn not_latched(self) -> &'a mut W {
        self.variant(PIN5W::NOTLATCHED)
    }
    #[doc = "Criteria has been met"]
    #[inline]
    pub fn latched(self) -> &'a mut W {
        self.variant(PIN5W::LATCHED)
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
        const OFFSET: u8 = 5;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `PIN6`"]
pub enum PIN6W {
    #[doc = "Criteria has not been met"]
    NOTLATCHED,
    #[doc = "Criteria has been met"]
    LATCHED,
}
impl PIN6W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            PIN6W::NOTLATCHED => false,
            PIN6W::LATCHED => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _PIN6W<'a> {
    w: &'a mut W,
}
impl<'a> _PIN6W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: PIN6W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Criteria has not been met"]
    #[inline]
    pub fn not_latched(self) -> &'a mut W {
        self.variant(PIN6W::NOTLATCHED)
    }
    #[doc = "Criteria has been met"]
    #[inline]
    pub fn latched(self) -> &'a mut W {
        self.variant(PIN6W::LATCHED)
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
#[doc = "Values that can be written to the field `PIN7`"]
pub enum PIN7W {
    #[doc = "Criteria has not been met"]
    NOTLATCHED,
    #[doc = "Criteria has been met"]
    LATCHED,
}
impl PIN7W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            PIN7W::NOTLATCHED => false,
            PIN7W::LATCHED => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _PIN7W<'a> {
    w: &'a mut W,
}
impl<'a> _PIN7W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: PIN7W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Criteria has not been met"]
    #[inline]
    pub fn not_latched(self) -> &'a mut W {
        self.variant(PIN7W::NOTLATCHED)
    }
    #[doc = "Criteria has been met"]
    #[inline]
    pub fn latched(self) -> &'a mut W {
        self.variant(PIN7W::LATCHED)
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
#[doc = "Values that can be written to the field `PIN8`"]
pub enum PIN8W {
    #[doc = "Criteria has not been met"]
    NOTLATCHED,
    #[doc = "Criteria has been met"]
    LATCHED,
}
impl PIN8W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            PIN8W::NOTLATCHED => false,
            PIN8W::LATCHED => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _PIN8W<'a> {
    w: &'a mut W,
}
impl<'a> _PIN8W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: PIN8W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Criteria has not been met"]
    #[inline]
    pub fn not_latched(self) -> &'a mut W {
        self.variant(PIN8W::NOTLATCHED)
    }
    #[doc = "Criteria has been met"]
    #[inline]
    pub fn latched(self) -> &'a mut W {
        self.variant(PIN8W::LATCHED)
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
        const OFFSET: u8 = 8;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `PIN9`"]
pub enum PIN9W {
    #[doc = "Criteria has not been met"]
    NOTLATCHED,
    #[doc = "Criteria has been met"]
    LATCHED,
}
impl PIN9W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            PIN9W::NOTLATCHED => false,
            PIN9W::LATCHED => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _PIN9W<'a> {
    w: &'a mut W,
}
impl<'a> _PIN9W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: PIN9W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Criteria has not been met"]
    #[inline]
    pub fn not_latched(self) -> &'a mut W {
        self.variant(PIN9W::NOTLATCHED)
    }
    #[doc = "Criteria has been met"]
    #[inline]
    pub fn latched(self) -> &'a mut W {
        self.variant(PIN9W::LATCHED)
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
        const OFFSET: u8 = 9;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `PIN10`"]
pub enum PIN10W {
    #[doc = "Criteria has not been met"]
    NOTLATCHED,
    #[doc = "Criteria has been met"]
    LATCHED,
}
impl PIN10W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            PIN10W::NOTLATCHED => false,
            PIN10W::LATCHED => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _PIN10W<'a> {
    w: &'a mut W,
}
impl<'a> _PIN10W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: PIN10W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Criteria has not been met"]
    #[inline]
    pub fn not_latched(self) -> &'a mut W {
        self.variant(PIN10W::NOTLATCHED)
    }
    #[doc = "Criteria has been met"]
    #[inline]
    pub fn latched(self) -> &'a mut W {
        self.variant(PIN10W::LATCHED)
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
        const OFFSET: u8 = 10;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `PIN11`"]
pub enum PIN11W {
    #[doc = "Criteria has not been met"]
    NOTLATCHED,
    #[doc = "Criteria has been met"]
    LATCHED,
}
impl PIN11W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            PIN11W::NOTLATCHED => false,
            PIN11W::LATCHED => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _PIN11W<'a> {
    w: &'a mut W,
}
impl<'a> _PIN11W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: PIN11W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Criteria has not been met"]
    #[inline]
    pub fn not_latched(self) -> &'a mut W {
        self.variant(PIN11W::NOTLATCHED)
    }
    #[doc = "Criteria has been met"]
    #[inline]
    pub fn latched(self) -> &'a mut W {
        self.variant(PIN11W::LATCHED)
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
        const OFFSET: u8 = 11;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `PIN12`"]
pub enum PIN12W {
    #[doc = "Criteria has not been met"]
    NOTLATCHED,
    #[doc = "Criteria has been met"]
    LATCHED,
}
impl PIN12W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            PIN12W::NOTLATCHED => false,
            PIN12W::LATCHED => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _PIN12W<'a> {
    w: &'a mut W,
}
impl<'a> _PIN12W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: PIN12W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Criteria has not been met"]
    #[inline]
    pub fn not_latched(self) -> &'a mut W {
        self.variant(PIN12W::NOTLATCHED)
    }
    #[doc = "Criteria has been met"]
    #[inline]
    pub fn latched(self) -> &'a mut W {
        self.variant(PIN12W::LATCHED)
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
#[doc = "Values that can be written to the field `PIN13`"]
pub enum PIN13W {
    #[doc = "Criteria has not been met"]
    NOTLATCHED,
    #[doc = "Criteria has been met"]
    LATCHED,
}
impl PIN13W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            PIN13W::NOTLATCHED => false,
            PIN13W::LATCHED => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _PIN13W<'a> {
    w: &'a mut W,
}
impl<'a> _PIN13W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: PIN13W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Criteria has not been met"]
    #[inline]
    pub fn not_latched(self) -> &'a mut W {
        self.variant(PIN13W::NOTLATCHED)
    }
    #[doc = "Criteria has been met"]
    #[inline]
    pub fn latched(self) -> &'a mut W {
        self.variant(PIN13W::LATCHED)
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
        const OFFSET: u8 = 13;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `PIN14`"]
pub enum PIN14W {
    #[doc = "Criteria has not been met"]
    NOTLATCHED,
    #[doc = "Criteria has been met"]
    LATCHED,
}
impl PIN14W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            PIN14W::NOTLATCHED => false,
            PIN14W::LATCHED => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _PIN14W<'a> {
    w: &'a mut W,
}
impl<'a> _PIN14W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: PIN14W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Criteria has not been met"]
    #[inline]
    pub fn not_latched(self) -> &'a mut W {
        self.variant(PIN14W::NOTLATCHED)
    }
    #[doc = "Criteria has been met"]
    #[inline]
    pub fn latched(self) -> &'a mut W {
        self.variant(PIN14W::LATCHED)
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
        const OFFSET: u8 = 14;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `PIN15`"]
pub enum PIN15W {
    #[doc = "Criteria has not been met"]
    NOTLATCHED,
    #[doc = "Criteria has been met"]
    LATCHED,
}
impl PIN15W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            PIN15W::NOTLATCHED => false,
            PIN15W::LATCHED => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _PIN15W<'a> {
    w: &'a mut W,
}
impl<'a> _PIN15W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: PIN15W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Criteria has not been met"]
    #[inline]
    pub fn not_latched(self) -> &'a mut W {
        self.variant(PIN15W::NOTLATCHED)
    }
    #[doc = "Criteria has been met"]
    #[inline]
    pub fn latched(self) -> &'a mut W {
        self.variant(PIN15W::LATCHED)
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
        const OFFSET: u8 = 15;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `PIN16`"]
pub enum PIN16W {
    #[doc = "Criteria has not been met"]
    NOTLATCHED,
    #[doc = "Criteria has been met"]
    LATCHED,
}
impl PIN16W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            PIN16W::NOTLATCHED => false,
            PIN16W::LATCHED => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _PIN16W<'a> {
    w: &'a mut W,
}
impl<'a> _PIN16W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: PIN16W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Criteria has not been met"]
    #[inline]
    pub fn not_latched(self) -> &'a mut W {
        self.variant(PIN16W::NOTLATCHED)
    }
    #[doc = "Criteria has been met"]
    #[inline]
    pub fn latched(self) -> &'a mut W {
        self.variant(PIN16W::LATCHED)
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
#[doc = "Values that can be written to the field `PIN17`"]
pub enum PIN17W {
    #[doc = "Criteria has not been met"]
    NOTLATCHED,
    #[doc = "Criteria has been met"]
    LATCHED,
}
impl PIN17W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            PIN17W::NOTLATCHED => false,
            PIN17W::LATCHED => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _PIN17W<'a> {
    w: &'a mut W,
}
impl<'a> _PIN17W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: PIN17W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Criteria has not been met"]
    #[inline]
    pub fn not_latched(self) -> &'a mut W {
        self.variant(PIN17W::NOTLATCHED)
    }
    #[doc = "Criteria has been met"]
    #[inline]
    pub fn latched(self) -> &'a mut W {
        self.variant(PIN17W::LATCHED)
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
#[doc = "Values that can be written to the field `PIN18`"]
pub enum PIN18W {
    #[doc = "Criteria has not been met"]
    NOTLATCHED,
    #[doc = "Criteria has been met"]
    LATCHED,
}
impl PIN18W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            PIN18W::NOTLATCHED => false,
            PIN18W::LATCHED => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _PIN18W<'a> {
    w: &'a mut W,
}
impl<'a> _PIN18W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: PIN18W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Criteria has not been met"]
    #[inline]
    pub fn not_latched(self) -> &'a mut W {
        self.variant(PIN18W::NOTLATCHED)
    }
    #[doc = "Criteria has been met"]
    #[inline]
    pub fn latched(self) -> &'a mut W {
        self.variant(PIN18W::LATCHED)
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
        const OFFSET: u8 = 18;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `PIN19`"]
pub enum PIN19W {
    #[doc = "Criteria has not been met"]
    NOTLATCHED,
    #[doc = "Criteria has been met"]
    LATCHED,
}
impl PIN19W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            PIN19W::NOTLATCHED => false,
            PIN19W::LATCHED => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _PIN19W<'a> {
    w: &'a mut W,
}
impl<'a> _PIN19W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: PIN19W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Criteria has not been met"]
    #[inline]
    pub fn not_latched(self) -> &'a mut W {
        self.variant(PIN19W::NOTLATCHED)
    }
    #[doc = "Criteria has been met"]
    #[inline]
    pub fn latched(self) -> &'a mut W {
        self.variant(PIN19W::LATCHED)
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
        const OFFSET: u8 = 19;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `PIN20`"]
pub enum PIN20W {
    #[doc = "Criteria has not been met"]
    NOTLATCHED,
    #[doc = "Criteria has been met"]
    LATCHED,
}
impl PIN20W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            PIN20W::NOTLATCHED => false,
            PIN20W::LATCHED => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _PIN20W<'a> {
    w: &'a mut W,
}
impl<'a> _PIN20W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: PIN20W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Criteria has not been met"]
    #[inline]
    pub fn not_latched(self) -> &'a mut W {
        self.variant(PIN20W::NOTLATCHED)
    }
    #[doc = "Criteria has been met"]
    #[inline]
    pub fn latched(self) -> &'a mut W {
        self.variant(PIN20W::LATCHED)
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
        const OFFSET: u8 = 20;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `PIN21`"]
pub enum PIN21W {
    #[doc = "Criteria has not been met"]
    NOTLATCHED,
    #[doc = "Criteria has been met"]
    LATCHED,
}
impl PIN21W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            PIN21W::NOTLATCHED => false,
            PIN21W::LATCHED => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _PIN21W<'a> {
    w: &'a mut W,
}
impl<'a> _PIN21W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: PIN21W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Criteria has not been met"]
    #[inline]
    pub fn not_latched(self) -> &'a mut W {
        self.variant(PIN21W::NOTLATCHED)
    }
    #[doc = "Criteria has been met"]
    #[inline]
    pub fn latched(self) -> &'a mut W {
        self.variant(PIN21W::LATCHED)
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
        const OFFSET: u8 = 21;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `PIN22`"]
pub enum PIN22W {
    #[doc = "Criteria has not been met"]
    NOTLATCHED,
    #[doc = "Criteria has been met"]
    LATCHED,
}
impl PIN22W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            PIN22W::NOTLATCHED => false,
            PIN22W::LATCHED => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _PIN22W<'a> {
    w: &'a mut W,
}
impl<'a> _PIN22W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: PIN22W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Criteria has not been met"]
    #[inline]
    pub fn not_latched(self) -> &'a mut W {
        self.variant(PIN22W::NOTLATCHED)
    }
    #[doc = "Criteria has been met"]
    #[inline]
    pub fn latched(self) -> &'a mut W {
        self.variant(PIN22W::LATCHED)
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
        const OFFSET: u8 = 22;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `PIN23`"]
pub enum PIN23W {
    #[doc = "Criteria has not been met"]
    NOTLATCHED,
    #[doc = "Criteria has been met"]
    LATCHED,
}
impl PIN23W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            PIN23W::NOTLATCHED => false,
            PIN23W::LATCHED => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _PIN23W<'a> {
    w: &'a mut W,
}
impl<'a> _PIN23W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: PIN23W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Criteria has not been met"]
    #[inline]
    pub fn not_latched(self) -> &'a mut W {
        self.variant(PIN23W::NOTLATCHED)
    }
    #[doc = "Criteria has been met"]
    #[inline]
    pub fn latched(self) -> &'a mut W {
        self.variant(PIN23W::LATCHED)
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
        const OFFSET: u8 = 23;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `PIN24`"]
pub enum PIN24W {
    #[doc = "Criteria has not been met"]
    NOTLATCHED,
    #[doc = "Criteria has been met"]
    LATCHED,
}
impl PIN24W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            PIN24W::NOTLATCHED => false,
            PIN24W::LATCHED => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _PIN24W<'a> {
    w: &'a mut W,
}
impl<'a> _PIN24W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: PIN24W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Criteria has not been met"]
    #[inline]
    pub fn not_latched(self) -> &'a mut W {
        self.variant(PIN24W::NOTLATCHED)
    }
    #[doc = "Criteria has been met"]
    #[inline]
    pub fn latched(self) -> &'a mut W {
        self.variant(PIN24W::LATCHED)
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
#[doc = "Values that can be written to the field `PIN25`"]
pub enum PIN25W {
    #[doc = "Criteria has not been met"]
    NOTLATCHED,
    #[doc = "Criteria has been met"]
    LATCHED,
}
impl PIN25W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            PIN25W::NOTLATCHED => false,
            PIN25W::LATCHED => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _PIN25W<'a> {
    w: &'a mut W,
}
impl<'a> _PIN25W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: PIN25W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Criteria has not been met"]
    #[inline]
    pub fn not_latched(self) -> &'a mut W {
        self.variant(PIN25W::NOTLATCHED)
    }
    #[doc = "Criteria has been met"]
    #[inline]
    pub fn latched(self) -> &'a mut W {
        self.variant(PIN25W::LATCHED)
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
#[doc = "Values that can be written to the field `PIN26`"]
pub enum PIN26W {
    #[doc = "Criteria has not been met"]
    NOTLATCHED,
    #[doc = "Criteria has been met"]
    LATCHED,
}
impl PIN26W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            PIN26W::NOTLATCHED => false,
            PIN26W::LATCHED => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _PIN26W<'a> {
    w: &'a mut W,
}
impl<'a> _PIN26W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: PIN26W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Criteria has not been met"]
    #[inline]
    pub fn not_latched(self) -> &'a mut W {
        self.variant(PIN26W::NOTLATCHED)
    }
    #[doc = "Criteria has been met"]
    #[inline]
    pub fn latched(self) -> &'a mut W {
        self.variant(PIN26W::LATCHED)
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
#[doc = "Values that can be written to the field `PIN27`"]
pub enum PIN27W {
    #[doc = "Criteria has not been met"]
    NOTLATCHED,
    #[doc = "Criteria has been met"]
    LATCHED,
}
impl PIN27W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            PIN27W::NOTLATCHED => false,
            PIN27W::LATCHED => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _PIN27W<'a> {
    w: &'a mut W,
}
impl<'a> _PIN27W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: PIN27W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Criteria has not been met"]
    #[inline]
    pub fn not_latched(self) -> &'a mut W {
        self.variant(PIN27W::NOTLATCHED)
    }
    #[doc = "Criteria has been met"]
    #[inline]
    pub fn latched(self) -> &'a mut W {
        self.variant(PIN27W::LATCHED)
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
#[doc = "Values that can be written to the field `PIN28`"]
pub enum PIN28W {
    #[doc = "Criteria has not been met"]
    NOTLATCHED,
    #[doc = "Criteria has been met"]
    LATCHED,
}
impl PIN28W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            PIN28W::NOTLATCHED => false,
            PIN28W::LATCHED => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _PIN28W<'a> {
    w: &'a mut W,
}
impl<'a> _PIN28W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: PIN28W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Criteria has not been met"]
    #[inline]
    pub fn not_latched(self) -> &'a mut W {
        self.variant(PIN28W::NOTLATCHED)
    }
    #[doc = "Criteria has been met"]
    #[inline]
    pub fn latched(self) -> &'a mut W {
        self.variant(PIN28W::LATCHED)
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
        const OFFSET: u8 = 28;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `PIN29`"]
pub enum PIN29W {
    #[doc = "Criteria has not been met"]
    NOTLATCHED,
    #[doc = "Criteria has been met"]
    LATCHED,
}
impl PIN29W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            PIN29W::NOTLATCHED => false,
            PIN29W::LATCHED => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _PIN29W<'a> {
    w: &'a mut W,
}
impl<'a> _PIN29W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: PIN29W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Criteria has not been met"]
    #[inline]
    pub fn not_latched(self) -> &'a mut W {
        self.variant(PIN29W::NOTLATCHED)
    }
    #[doc = "Criteria has been met"]
    #[inline]
    pub fn latched(self) -> &'a mut W {
        self.variant(PIN29W::LATCHED)
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
        const OFFSET: u8 = 29;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `PIN30`"]
pub enum PIN30W {
    #[doc = "Criteria has not been met"]
    NOTLATCHED,
    #[doc = "Criteria has been met"]
    LATCHED,
}
impl PIN30W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            PIN30W::NOTLATCHED => false,
            PIN30W::LATCHED => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _PIN30W<'a> {
    w: &'a mut W,
}
impl<'a> _PIN30W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: PIN30W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Criteria has not been met"]
    #[inline]
    pub fn not_latched(self) -> &'a mut W {
        self.variant(PIN30W::NOTLATCHED)
    }
    #[doc = "Criteria has been met"]
    #[inline]
    pub fn latched(self) -> &'a mut W {
        self.variant(PIN30W::LATCHED)
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
        const OFFSET: u8 = 30;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `PIN31`"]
pub enum PIN31W {
    #[doc = "Criteria has not been met"]
    NOTLATCHED,
    #[doc = "Criteria has been met"]
    LATCHED,
}
impl PIN31W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            PIN31W::NOTLATCHED => false,
            PIN31W::LATCHED => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _PIN31W<'a> {
    w: &'a mut W,
}
impl<'a> _PIN31W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: PIN31W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Criteria has not been met"]
    #[inline]
    pub fn not_latched(self) -> &'a mut W {
        self.variant(PIN31W::NOTLATCHED)
    }
    #[doc = "Criteria has been met"]
    #[inline]
    pub fn latched(self) -> &'a mut W {
        self.variant(PIN31W::LATCHED)
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
        const OFFSET: u8 = 31;
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
    #[doc = "Bit 0 - Status on whether PIN0 has met criteria set in PIN_CNF0.SENSE register. Write '1' to clear."]
    #[inline]
    pub fn pin0(&self) -> PIN0R {
        PIN0R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 1 - Status on whether PIN1 has met criteria set in PIN_CNF1.SENSE register. Write '1' to clear."]
    #[inline]
    pub fn pin1(&self) -> PIN1R {
        PIN1R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 1;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 2 - Status on whether PIN2 has met criteria set in PIN_CNF2.SENSE register. Write '1' to clear."]
    #[inline]
    pub fn pin2(&self) -> PIN2R {
        PIN2R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 2;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 3 - Status on whether PIN3 has met criteria set in PIN_CNF3.SENSE register. Write '1' to clear."]
    #[inline]
    pub fn pin3(&self) -> PIN3R {
        PIN3R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 3;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 4 - Status on whether PIN4 has met criteria set in PIN_CNF4.SENSE register. Write '1' to clear."]
    #[inline]
    pub fn pin4(&self) -> PIN4R {
        PIN4R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 4;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 5 - Status on whether PIN5 has met criteria set in PIN_CNF5.SENSE register. Write '1' to clear."]
    #[inline]
    pub fn pin5(&self) -> PIN5R {
        PIN5R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 5;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 6 - Status on whether PIN6 has met criteria set in PIN_CNF6.SENSE register. Write '1' to clear."]
    #[inline]
    pub fn pin6(&self) -> PIN6R {
        PIN6R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 6;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 7 - Status on whether PIN7 has met criteria set in PIN_CNF7.SENSE register. Write '1' to clear."]
    #[inline]
    pub fn pin7(&self) -> PIN7R {
        PIN7R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 7;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 8 - Status on whether PIN8 has met criteria set in PIN_CNF8.SENSE register. Write '1' to clear."]
    #[inline]
    pub fn pin8(&self) -> PIN8R {
        PIN8R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 8;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 9 - Status on whether PIN9 has met criteria set in PIN_CNF9.SENSE register. Write '1' to clear."]
    #[inline]
    pub fn pin9(&self) -> PIN9R {
        PIN9R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 9;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 10 - Status on whether PIN10 has met criteria set in PIN_CNF10.SENSE register. Write '1' to clear."]
    #[inline]
    pub fn pin10(&self) -> PIN10R {
        PIN10R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 10;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 11 - Status on whether PIN11 has met criteria set in PIN_CNF11.SENSE register. Write '1' to clear."]
    #[inline]
    pub fn pin11(&self) -> PIN11R {
        PIN11R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 11;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 12 - Status on whether PIN12 has met criteria set in PIN_CNF12.SENSE register. Write '1' to clear."]
    #[inline]
    pub fn pin12(&self) -> PIN12R {
        PIN12R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 12;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 13 - Status on whether PIN13 has met criteria set in PIN_CNF13.SENSE register. Write '1' to clear."]
    #[inline]
    pub fn pin13(&self) -> PIN13R {
        PIN13R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 13;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 14 - Status on whether PIN14 has met criteria set in PIN_CNF14.SENSE register. Write '1' to clear."]
    #[inline]
    pub fn pin14(&self) -> PIN14R {
        PIN14R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 14;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 15 - Status on whether PIN15 has met criteria set in PIN_CNF15.SENSE register. Write '1' to clear."]
    #[inline]
    pub fn pin15(&self) -> PIN15R {
        PIN15R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 15;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 16 - Status on whether PIN16 has met criteria set in PIN_CNF16.SENSE register. Write '1' to clear."]
    #[inline]
    pub fn pin16(&self) -> PIN16R {
        PIN16R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 16;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 17 - Status on whether PIN17 has met criteria set in PIN_CNF17.SENSE register. Write '1' to clear."]
    #[inline]
    pub fn pin17(&self) -> PIN17R {
        PIN17R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 17;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 18 - Status on whether PIN18 has met criteria set in PIN_CNF18.SENSE register. Write '1' to clear."]
    #[inline]
    pub fn pin18(&self) -> PIN18R {
        PIN18R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 18;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 19 - Status on whether PIN19 has met criteria set in PIN_CNF19.SENSE register. Write '1' to clear."]
    #[inline]
    pub fn pin19(&self) -> PIN19R {
        PIN19R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 19;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 20 - Status on whether PIN20 has met criteria set in PIN_CNF20.SENSE register. Write '1' to clear."]
    #[inline]
    pub fn pin20(&self) -> PIN20R {
        PIN20R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 20;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 21 - Status on whether PIN21 has met criteria set in PIN_CNF21.SENSE register. Write '1' to clear."]
    #[inline]
    pub fn pin21(&self) -> PIN21R {
        PIN21R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 21;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 22 - Status on whether PIN22 has met criteria set in PIN_CNF22.SENSE register. Write '1' to clear."]
    #[inline]
    pub fn pin22(&self) -> PIN22R {
        PIN22R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 22;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 23 - Status on whether PIN23 has met criteria set in PIN_CNF23.SENSE register. Write '1' to clear."]
    #[inline]
    pub fn pin23(&self) -> PIN23R {
        PIN23R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 23;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 24 - Status on whether PIN24 has met criteria set in PIN_CNF24.SENSE register. Write '1' to clear."]
    #[inline]
    pub fn pin24(&self) -> PIN24R {
        PIN24R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 24;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 25 - Status on whether PIN25 has met criteria set in PIN_CNF25.SENSE register. Write '1' to clear."]
    #[inline]
    pub fn pin25(&self) -> PIN25R {
        PIN25R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 25;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 26 - Status on whether PIN26 has met criteria set in PIN_CNF26.SENSE register. Write '1' to clear."]
    #[inline]
    pub fn pin26(&self) -> PIN26R {
        PIN26R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 26;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 27 - Status on whether PIN27 has met criteria set in PIN_CNF27.SENSE register. Write '1' to clear."]
    #[inline]
    pub fn pin27(&self) -> PIN27R {
        PIN27R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 27;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 28 - Status on whether PIN28 has met criteria set in PIN_CNF28.SENSE register. Write '1' to clear."]
    #[inline]
    pub fn pin28(&self) -> PIN28R {
        PIN28R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 28;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 29 - Status on whether PIN29 has met criteria set in PIN_CNF29.SENSE register. Write '1' to clear."]
    #[inline]
    pub fn pin29(&self) -> PIN29R {
        PIN29R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 29;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 30 - Status on whether PIN30 has met criteria set in PIN_CNF30.SENSE register. Write '1' to clear."]
    #[inline]
    pub fn pin30(&self) -> PIN30R {
        PIN30R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 30;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 31 - Status on whether PIN31 has met criteria set in PIN_CNF31.SENSE register. Write '1' to clear."]
    #[inline]
    pub fn pin31(&self) -> PIN31R {
        PIN31R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 31;
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
    #[doc = "Bit 0 - Status on whether PIN0 has met criteria set in PIN_CNF0.SENSE register. Write '1' to clear."]
    #[inline]
    pub fn pin0(&mut self) -> _PIN0W {
        _PIN0W { w: self }
    }
    #[doc = "Bit 1 - Status on whether PIN1 has met criteria set in PIN_CNF1.SENSE register. Write '1' to clear."]
    #[inline]
    pub fn pin1(&mut self) -> _PIN1W {
        _PIN1W { w: self }
    }
    #[doc = "Bit 2 - Status on whether PIN2 has met criteria set in PIN_CNF2.SENSE register. Write '1' to clear."]
    #[inline]
    pub fn pin2(&mut self) -> _PIN2W {
        _PIN2W { w: self }
    }
    #[doc = "Bit 3 - Status on whether PIN3 has met criteria set in PIN_CNF3.SENSE register. Write '1' to clear."]
    #[inline]
    pub fn pin3(&mut self) -> _PIN3W {
        _PIN3W { w: self }
    }
    #[doc = "Bit 4 - Status on whether PIN4 has met criteria set in PIN_CNF4.SENSE register. Write '1' to clear."]
    #[inline]
    pub fn pin4(&mut self) -> _PIN4W {
        _PIN4W { w: self }
    }
    #[doc = "Bit 5 - Status on whether PIN5 has met criteria set in PIN_CNF5.SENSE register. Write '1' to clear."]
    #[inline]
    pub fn pin5(&mut self) -> _PIN5W {
        _PIN5W { w: self }
    }
    #[doc = "Bit 6 - Status on whether PIN6 has met criteria set in PIN_CNF6.SENSE register. Write '1' to clear."]
    #[inline]
    pub fn pin6(&mut self) -> _PIN6W {
        _PIN6W { w: self }
    }
    #[doc = "Bit 7 - Status on whether PIN7 has met criteria set in PIN_CNF7.SENSE register. Write '1' to clear."]
    #[inline]
    pub fn pin7(&mut self) -> _PIN7W {
        _PIN7W { w: self }
    }
    #[doc = "Bit 8 - Status on whether PIN8 has met criteria set in PIN_CNF8.SENSE register. Write '1' to clear."]
    #[inline]
    pub fn pin8(&mut self) -> _PIN8W {
        _PIN8W { w: self }
    }
    #[doc = "Bit 9 - Status on whether PIN9 has met criteria set in PIN_CNF9.SENSE register. Write '1' to clear."]
    #[inline]
    pub fn pin9(&mut self) -> _PIN9W {
        _PIN9W { w: self }
    }
    #[doc = "Bit 10 - Status on whether PIN10 has met criteria set in PIN_CNF10.SENSE register. Write '1' to clear."]
    #[inline]
    pub fn pin10(&mut self) -> _PIN10W {
        _PIN10W { w: self }
    }
    #[doc = "Bit 11 - Status on whether PIN11 has met criteria set in PIN_CNF11.SENSE register. Write '1' to clear."]
    #[inline]
    pub fn pin11(&mut self) -> _PIN11W {
        _PIN11W { w: self }
    }
    #[doc = "Bit 12 - Status on whether PIN12 has met criteria set in PIN_CNF12.SENSE register. Write '1' to clear."]
    #[inline]
    pub fn pin12(&mut self) -> _PIN12W {
        _PIN12W { w: self }
    }
    #[doc = "Bit 13 - Status on whether PIN13 has met criteria set in PIN_CNF13.SENSE register. Write '1' to clear."]
    #[inline]
    pub fn pin13(&mut self) -> _PIN13W {
        _PIN13W { w: self }
    }
    #[doc = "Bit 14 - Status on whether PIN14 has met criteria set in PIN_CNF14.SENSE register. Write '1' to clear."]
    #[inline]
    pub fn pin14(&mut self) -> _PIN14W {
        _PIN14W { w: self }
    }
    #[doc = "Bit 15 - Status on whether PIN15 has met criteria set in PIN_CNF15.SENSE register. Write '1' to clear."]
    #[inline]
    pub fn pin15(&mut self) -> _PIN15W {
        _PIN15W { w: self }
    }
    #[doc = "Bit 16 - Status on whether PIN16 has met criteria set in PIN_CNF16.SENSE register. Write '1' to clear."]
    #[inline]
    pub fn pin16(&mut self) -> _PIN16W {
        _PIN16W { w: self }
    }
    #[doc = "Bit 17 - Status on whether PIN17 has met criteria set in PIN_CNF17.SENSE register. Write '1' to clear."]
    #[inline]
    pub fn pin17(&mut self) -> _PIN17W {
        _PIN17W { w: self }
    }
    #[doc = "Bit 18 - Status on whether PIN18 has met criteria set in PIN_CNF18.SENSE register. Write '1' to clear."]
    #[inline]
    pub fn pin18(&mut self) -> _PIN18W {
        _PIN18W { w: self }
    }
    #[doc = "Bit 19 - Status on whether PIN19 has met criteria set in PIN_CNF19.SENSE register. Write '1' to clear."]
    #[inline]
    pub fn pin19(&mut self) -> _PIN19W {
        _PIN19W { w: self }
    }
    #[doc = "Bit 20 - Status on whether PIN20 has met criteria set in PIN_CNF20.SENSE register. Write '1' to clear."]
    #[inline]
    pub fn pin20(&mut self) -> _PIN20W {
        _PIN20W { w: self }
    }
    #[doc = "Bit 21 - Status on whether PIN21 has met criteria set in PIN_CNF21.SENSE register. Write '1' to clear."]
    #[inline]
    pub fn pin21(&mut self) -> _PIN21W {
        _PIN21W { w: self }
    }
    #[doc = "Bit 22 - Status on whether PIN22 has met criteria set in PIN_CNF22.SENSE register. Write '1' to clear."]
    #[inline]
    pub fn pin22(&mut self) -> _PIN22W {
        _PIN22W { w: self }
    }
    #[doc = "Bit 23 - Status on whether PIN23 has met criteria set in PIN_CNF23.SENSE register. Write '1' to clear."]
    #[inline]
    pub fn pin23(&mut self) -> _PIN23W {
        _PIN23W { w: self }
    }
    #[doc = "Bit 24 - Status on whether PIN24 has met criteria set in PIN_CNF24.SENSE register. Write '1' to clear."]
    #[inline]
    pub fn pin24(&mut self) -> _PIN24W {
        _PIN24W { w: self }
    }
    #[doc = "Bit 25 - Status on whether PIN25 has met criteria set in PIN_CNF25.SENSE register. Write '1' to clear."]
    #[inline]
    pub fn pin25(&mut self) -> _PIN25W {
        _PIN25W { w: self }
    }
    #[doc = "Bit 26 - Status on whether PIN26 has met criteria set in PIN_CNF26.SENSE register. Write '1' to clear."]
    #[inline]
    pub fn pin26(&mut self) -> _PIN26W {
        _PIN26W { w: self }
    }
    #[doc = "Bit 27 - Status on whether PIN27 has met criteria set in PIN_CNF27.SENSE register. Write '1' to clear."]
    #[inline]
    pub fn pin27(&mut self) -> _PIN27W {
        _PIN27W { w: self }
    }
    #[doc = "Bit 28 - Status on whether PIN28 has met criteria set in PIN_CNF28.SENSE register. Write '1' to clear."]
    #[inline]
    pub fn pin28(&mut self) -> _PIN28W {
        _PIN28W { w: self }
    }
    #[doc = "Bit 29 - Status on whether PIN29 has met criteria set in PIN_CNF29.SENSE register. Write '1' to clear."]
    #[inline]
    pub fn pin29(&mut self) -> _PIN29W {
        _PIN29W { w: self }
    }
    #[doc = "Bit 30 - Status on whether PIN30 has met criteria set in PIN_CNF30.SENSE register. Write '1' to clear."]
    #[inline]
    pub fn pin30(&mut self) -> _PIN30W {
        _PIN30W { w: self }
    }
    #[doc = "Bit 31 - Status on whether PIN31 has met criteria set in PIN_CNF31.SENSE register. Write '1' to clear."]
    #[inline]
    pub fn pin31(&mut self) -> _PIN31W {
        _PIN31W { w: self }
    }
}
