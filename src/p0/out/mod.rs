#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::OUT {
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
    #[doc = "Pin driver is low"]
    LOW,
    #[doc = "Pin driver is high"]
    HIGH,
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
            PIN0R::LOW => false,
            PIN0R::HIGH => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> PIN0R {
        match value {
            false => PIN0R::LOW,
            true => PIN0R::HIGH,
        }
    }
    #[doc = "Checks if the value of the field is `LOW`"]
    #[inline]
    pub fn is_low(&self) -> bool {
        *self == PIN0R::LOW
    }
    #[doc = "Checks if the value of the field is `HIGH`"]
    #[inline]
    pub fn is_high(&self) -> bool {
        *self == PIN0R::HIGH
    }
}
#[doc = "Possible values of the field `PIN1`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PIN1R {
    #[doc = "Pin driver is low"]
    LOW,
    #[doc = "Pin driver is high"]
    HIGH,
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
            PIN1R::LOW => false,
            PIN1R::HIGH => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> PIN1R {
        match value {
            false => PIN1R::LOW,
            true => PIN1R::HIGH,
        }
    }
    #[doc = "Checks if the value of the field is `LOW`"]
    #[inline]
    pub fn is_low(&self) -> bool {
        *self == PIN1R::LOW
    }
    #[doc = "Checks if the value of the field is `HIGH`"]
    #[inline]
    pub fn is_high(&self) -> bool {
        *self == PIN1R::HIGH
    }
}
#[doc = "Possible values of the field `PIN2`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PIN2R {
    #[doc = "Pin driver is low"]
    LOW,
    #[doc = "Pin driver is high"]
    HIGH,
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
            PIN2R::LOW => false,
            PIN2R::HIGH => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> PIN2R {
        match value {
            false => PIN2R::LOW,
            true => PIN2R::HIGH,
        }
    }
    #[doc = "Checks if the value of the field is `LOW`"]
    #[inline]
    pub fn is_low(&self) -> bool {
        *self == PIN2R::LOW
    }
    #[doc = "Checks if the value of the field is `HIGH`"]
    #[inline]
    pub fn is_high(&self) -> bool {
        *self == PIN2R::HIGH
    }
}
#[doc = "Possible values of the field `PIN3`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PIN3R {
    #[doc = "Pin driver is low"]
    LOW,
    #[doc = "Pin driver is high"]
    HIGH,
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
            PIN3R::LOW => false,
            PIN3R::HIGH => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> PIN3R {
        match value {
            false => PIN3R::LOW,
            true => PIN3R::HIGH,
        }
    }
    #[doc = "Checks if the value of the field is `LOW`"]
    #[inline]
    pub fn is_low(&self) -> bool {
        *self == PIN3R::LOW
    }
    #[doc = "Checks if the value of the field is `HIGH`"]
    #[inline]
    pub fn is_high(&self) -> bool {
        *self == PIN3R::HIGH
    }
}
#[doc = "Possible values of the field `PIN4`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PIN4R {
    #[doc = "Pin driver is low"]
    LOW,
    #[doc = "Pin driver is high"]
    HIGH,
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
            PIN4R::LOW => false,
            PIN4R::HIGH => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> PIN4R {
        match value {
            false => PIN4R::LOW,
            true => PIN4R::HIGH,
        }
    }
    #[doc = "Checks if the value of the field is `LOW`"]
    #[inline]
    pub fn is_low(&self) -> bool {
        *self == PIN4R::LOW
    }
    #[doc = "Checks if the value of the field is `HIGH`"]
    #[inline]
    pub fn is_high(&self) -> bool {
        *self == PIN4R::HIGH
    }
}
#[doc = "Possible values of the field `PIN5`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PIN5R {
    #[doc = "Pin driver is low"]
    LOW,
    #[doc = "Pin driver is high"]
    HIGH,
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
            PIN5R::LOW => false,
            PIN5R::HIGH => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> PIN5R {
        match value {
            false => PIN5R::LOW,
            true => PIN5R::HIGH,
        }
    }
    #[doc = "Checks if the value of the field is `LOW`"]
    #[inline]
    pub fn is_low(&self) -> bool {
        *self == PIN5R::LOW
    }
    #[doc = "Checks if the value of the field is `HIGH`"]
    #[inline]
    pub fn is_high(&self) -> bool {
        *self == PIN5R::HIGH
    }
}
#[doc = "Possible values of the field `PIN6`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PIN6R {
    #[doc = "Pin driver is low"]
    LOW,
    #[doc = "Pin driver is high"]
    HIGH,
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
            PIN6R::LOW => false,
            PIN6R::HIGH => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> PIN6R {
        match value {
            false => PIN6R::LOW,
            true => PIN6R::HIGH,
        }
    }
    #[doc = "Checks if the value of the field is `LOW`"]
    #[inline]
    pub fn is_low(&self) -> bool {
        *self == PIN6R::LOW
    }
    #[doc = "Checks if the value of the field is `HIGH`"]
    #[inline]
    pub fn is_high(&self) -> bool {
        *self == PIN6R::HIGH
    }
}
#[doc = "Possible values of the field `PIN7`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PIN7R {
    #[doc = "Pin driver is low"]
    LOW,
    #[doc = "Pin driver is high"]
    HIGH,
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
            PIN7R::LOW => false,
            PIN7R::HIGH => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> PIN7R {
        match value {
            false => PIN7R::LOW,
            true => PIN7R::HIGH,
        }
    }
    #[doc = "Checks if the value of the field is `LOW`"]
    #[inline]
    pub fn is_low(&self) -> bool {
        *self == PIN7R::LOW
    }
    #[doc = "Checks if the value of the field is `HIGH`"]
    #[inline]
    pub fn is_high(&self) -> bool {
        *self == PIN7R::HIGH
    }
}
#[doc = "Possible values of the field `PIN8`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PIN8R {
    #[doc = "Pin driver is low"]
    LOW,
    #[doc = "Pin driver is high"]
    HIGH,
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
            PIN8R::LOW => false,
            PIN8R::HIGH => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> PIN8R {
        match value {
            false => PIN8R::LOW,
            true => PIN8R::HIGH,
        }
    }
    #[doc = "Checks if the value of the field is `LOW`"]
    #[inline]
    pub fn is_low(&self) -> bool {
        *self == PIN8R::LOW
    }
    #[doc = "Checks if the value of the field is `HIGH`"]
    #[inline]
    pub fn is_high(&self) -> bool {
        *self == PIN8R::HIGH
    }
}
#[doc = "Possible values of the field `PIN9`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PIN9R {
    #[doc = "Pin driver is low"]
    LOW,
    #[doc = "Pin driver is high"]
    HIGH,
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
            PIN9R::LOW => false,
            PIN9R::HIGH => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> PIN9R {
        match value {
            false => PIN9R::LOW,
            true => PIN9R::HIGH,
        }
    }
    #[doc = "Checks if the value of the field is `LOW`"]
    #[inline]
    pub fn is_low(&self) -> bool {
        *self == PIN9R::LOW
    }
    #[doc = "Checks if the value of the field is `HIGH`"]
    #[inline]
    pub fn is_high(&self) -> bool {
        *self == PIN9R::HIGH
    }
}
#[doc = "Possible values of the field `PIN10`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PIN10R {
    #[doc = "Pin driver is low"]
    LOW,
    #[doc = "Pin driver is high"]
    HIGH,
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
            PIN10R::LOW => false,
            PIN10R::HIGH => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> PIN10R {
        match value {
            false => PIN10R::LOW,
            true => PIN10R::HIGH,
        }
    }
    #[doc = "Checks if the value of the field is `LOW`"]
    #[inline]
    pub fn is_low(&self) -> bool {
        *self == PIN10R::LOW
    }
    #[doc = "Checks if the value of the field is `HIGH`"]
    #[inline]
    pub fn is_high(&self) -> bool {
        *self == PIN10R::HIGH
    }
}
#[doc = "Possible values of the field `PIN11`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PIN11R {
    #[doc = "Pin driver is low"]
    LOW,
    #[doc = "Pin driver is high"]
    HIGH,
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
            PIN11R::LOW => false,
            PIN11R::HIGH => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> PIN11R {
        match value {
            false => PIN11R::LOW,
            true => PIN11R::HIGH,
        }
    }
    #[doc = "Checks if the value of the field is `LOW`"]
    #[inline]
    pub fn is_low(&self) -> bool {
        *self == PIN11R::LOW
    }
    #[doc = "Checks if the value of the field is `HIGH`"]
    #[inline]
    pub fn is_high(&self) -> bool {
        *self == PIN11R::HIGH
    }
}
#[doc = "Possible values of the field `PIN12`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PIN12R {
    #[doc = "Pin driver is low"]
    LOW,
    #[doc = "Pin driver is high"]
    HIGH,
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
            PIN12R::LOW => false,
            PIN12R::HIGH => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> PIN12R {
        match value {
            false => PIN12R::LOW,
            true => PIN12R::HIGH,
        }
    }
    #[doc = "Checks if the value of the field is `LOW`"]
    #[inline]
    pub fn is_low(&self) -> bool {
        *self == PIN12R::LOW
    }
    #[doc = "Checks if the value of the field is `HIGH`"]
    #[inline]
    pub fn is_high(&self) -> bool {
        *self == PIN12R::HIGH
    }
}
#[doc = "Possible values of the field `PIN13`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PIN13R {
    #[doc = "Pin driver is low"]
    LOW,
    #[doc = "Pin driver is high"]
    HIGH,
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
            PIN13R::LOW => false,
            PIN13R::HIGH => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> PIN13R {
        match value {
            false => PIN13R::LOW,
            true => PIN13R::HIGH,
        }
    }
    #[doc = "Checks if the value of the field is `LOW`"]
    #[inline]
    pub fn is_low(&self) -> bool {
        *self == PIN13R::LOW
    }
    #[doc = "Checks if the value of the field is `HIGH`"]
    #[inline]
    pub fn is_high(&self) -> bool {
        *self == PIN13R::HIGH
    }
}
#[doc = "Possible values of the field `PIN14`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PIN14R {
    #[doc = "Pin driver is low"]
    LOW,
    #[doc = "Pin driver is high"]
    HIGH,
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
            PIN14R::LOW => false,
            PIN14R::HIGH => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> PIN14R {
        match value {
            false => PIN14R::LOW,
            true => PIN14R::HIGH,
        }
    }
    #[doc = "Checks if the value of the field is `LOW`"]
    #[inline]
    pub fn is_low(&self) -> bool {
        *self == PIN14R::LOW
    }
    #[doc = "Checks if the value of the field is `HIGH`"]
    #[inline]
    pub fn is_high(&self) -> bool {
        *self == PIN14R::HIGH
    }
}
#[doc = "Possible values of the field `PIN15`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PIN15R {
    #[doc = "Pin driver is low"]
    LOW,
    #[doc = "Pin driver is high"]
    HIGH,
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
            PIN15R::LOW => false,
            PIN15R::HIGH => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> PIN15R {
        match value {
            false => PIN15R::LOW,
            true => PIN15R::HIGH,
        }
    }
    #[doc = "Checks if the value of the field is `LOW`"]
    #[inline]
    pub fn is_low(&self) -> bool {
        *self == PIN15R::LOW
    }
    #[doc = "Checks if the value of the field is `HIGH`"]
    #[inline]
    pub fn is_high(&self) -> bool {
        *self == PIN15R::HIGH
    }
}
#[doc = "Possible values of the field `PIN16`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PIN16R {
    #[doc = "Pin driver is low"]
    LOW,
    #[doc = "Pin driver is high"]
    HIGH,
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
            PIN16R::LOW => false,
            PIN16R::HIGH => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> PIN16R {
        match value {
            false => PIN16R::LOW,
            true => PIN16R::HIGH,
        }
    }
    #[doc = "Checks if the value of the field is `LOW`"]
    #[inline]
    pub fn is_low(&self) -> bool {
        *self == PIN16R::LOW
    }
    #[doc = "Checks if the value of the field is `HIGH`"]
    #[inline]
    pub fn is_high(&self) -> bool {
        *self == PIN16R::HIGH
    }
}
#[doc = "Possible values of the field `PIN17`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PIN17R {
    #[doc = "Pin driver is low"]
    LOW,
    #[doc = "Pin driver is high"]
    HIGH,
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
            PIN17R::LOW => false,
            PIN17R::HIGH => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> PIN17R {
        match value {
            false => PIN17R::LOW,
            true => PIN17R::HIGH,
        }
    }
    #[doc = "Checks if the value of the field is `LOW`"]
    #[inline]
    pub fn is_low(&self) -> bool {
        *self == PIN17R::LOW
    }
    #[doc = "Checks if the value of the field is `HIGH`"]
    #[inline]
    pub fn is_high(&self) -> bool {
        *self == PIN17R::HIGH
    }
}
#[doc = "Possible values of the field `PIN18`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PIN18R {
    #[doc = "Pin driver is low"]
    LOW,
    #[doc = "Pin driver is high"]
    HIGH,
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
            PIN18R::LOW => false,
            PIN18R::HIGH => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> PIN18R {
        match value {
            false => PIN18R::LOW,
            true => PIN18R::HIGH,
        }
    }
    #[doc = "Checks if the value of the field is `LOW`"]
    #[inline]
    pub fn is_low(&self) -> bool {
        *self == PIN18R::LOW
    }
    #[doc = "Checks if the value of the field is `HIGH`"]
    #[inline]
    pub fn is_high(&self) -> bool {
        *self == PIN18R::HIGH
    }
}
#[doc = "Possible values of the field `PIN19`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PIN19R {
    #[doc = "Pin driver is low"]
    LOW,
    #[doc = "Pin driver is high"]
    HIGH,
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
            PIN19R::LOW => false,
            PIN19R::HIGH => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> PIN19R {
        match value {
            false => PIN19R::LOW,
            true => PIN19R::HIGH,
        }
    }
    #[doc = "Checks if the value of the field is `LOW`"]
    #[inline]
    pub fn is_low(&self) -> bool {
        *self == PIN19R::LOW
    }
    #[doc = "Checks if the value of the field is `HIGH`"]
    #[inline]
    pub fn is_high(&self) -> bool {
        *self == PIN19R::HIGH
    }
}
#[doc = "Possible values of the field `PIN20`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PIN20R {
    #[doc = "Pin driver is low"]
    LOW,
    #[doc = "Pin driver is high"]
    HIGH,
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
            PIN20R::LOW => false,
            PIN20R::HIGH => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> PIN20R {
        match value {
            false => PIN20R::LOW,
            true => PIN20R::HIGH,
        }
    }
    #[doc = "Checks if the value of the field is `LOW`"]
    #[inline]
    pub fn is_low(&self) -> bool {
        *self == PIN20R::LOW
    }
    #[doc = "Checks if the value of the field is `HIGH`"]
    #[inline]
    pub fn is_high(&self) -> bool {
        *self == PIN20R::HIGH
    }
}
#[doc = "Possible values of the field `PIN21`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PIN21R {
    #[doc = "Pin driver is low"]
    LOW,
    #[doc = "Pin driver is high"]
    HIGH,
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
            PIN21R::LOW => false,
            PIN21R::HIGH => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> PIN21R {
        match value {
            false => PIN21R::LOW,
            true => PIN21R::HIGH,
        }
    }
    #[doc = "Checks if the value of the field is `LOW`"]
    #[inline]
    pub fn is_low(&self) -> bool {
        *self == PIN21R::LOW
    }
    #[doc = "Checks if the value of the field is `HIGH`"]
    #[inline]
    pub fn is_high(&self) -> bool {
        *self == PIN21R::HIGH
    }
}
#[doc = "Possible values of the field `PIN22`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PIN22R {
    #[doc = "Pin driver is low"]
    LOW,
    #[doc = "Pin driver is high"]
    HIGH,
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
            PIN22R::LOW => false,
            PIN22R::HIGH => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> PIN22R {
        match value {
            false => PIN22R::LOW,
            true => PIN22R::HIGH,
        }
    }
    #[doc = "Checks if the value of the field is `LOW`"]
    #[inline]
    pub fn is_low(&self) -> bool {
        *self == PIN22R::LOW
    }
    #[doc = "Checks if the value of the field is `HIGH`"]
    #[inline]
    pub fn is_high(&self) -> bool {
        *self == PIN22R::HIGH
    }
}
#[doc = "Possible values of the field `PIN23`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PIN23R {
    #[doc = "Pin driver is low"]
    LOW,
    #[doc = "Pin driver is high"]
    HIGH,
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
            PIN23R::LOW => false,
            PIN23R::HIGH => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> PIN23R {
        match value {
            false => PIN23R::LOW,
            true => PIN23R::HIGH,
        }
    }
    #[doc = "Checks if the value of the field is `LOW`"]
    #[inline]
    pub fn is_low(&self) -> bool {
        *self == PIN23R::LOW
    }
    #[doc = "Checks if the value of the field is `HIGH`"]
    #[inline]
    pub fn is_high(&self) -> bool {
        *self == PIN23R::HIGH
    }
}
#[doc = "Possible values of the field `PIN24`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PIN24R {
    #[doc = "Pin driver is low"]
    LOW,
    #[doc = "Pin driver is high"]
    HIGH,
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
            PIN24R::LOW => false,
            PIN24R::HIGH => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> PIN24R {
        match value {
            false => PIN24R::LOW,
            true => PIN24R::HIGH,
        }
    }
    #[doc = "Checks if the value of the field is `LOW`"]
    #[inline]
    pub fn is_low(&self) -> bool {
        *self == PIN24R::LOW
    }
    #[doc = "Checks if the value of the field is `HIGH`"]
    #[inline]
    pub fn is_high(&self) -> bool {
        *self == PIN24R::HIGH
    }
}
#[doc = "Possible values of the field `PIN25`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PIN25R {
    #[doc = "Pin driver is low"]
    LOW,
    #[doc = "Pin driver is high"]
    HIGH,
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
            PIN25R::LOW => false,
            PIN25R::HIGH => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> PIN25R {
        match value {
            false => PIN25R::LOW,
            true => PIN25R::HIGH,
        }
    }
    #[doc = "Checks if the value of the field is `LOW`"]
    #[inline]
    pub fn is_low(&self) -> bool {
        *self == PIN25R::LOW
    }
    #[doc = "Checks if the value of the field is `HIGH`"]
    #[inline]
    pub fn is_high(&self) -> bool {
        *self == PIN25R::HIGH
    }
}
#[doc = "Possible values of the field `PIN26`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PIN26R {
    #[doc = "Pin driver is low"]
    LOW,
    #[doc = "Pin driver is high"]
    HIGH,
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
            PIN26R::LOW => false,
            PIN26R::HIGH => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> PIN26R {
        match value {
            false => PIN26R::LOW,
            true => PIN26R::HIGH,
        }
    }
    #[doc = "Checks if the value of the field is `LOW`"]
    #[inline]
    pub fn is_low(&self) -> bool {
        *self == PIN26R::LOW
    }
    #[doc = "Checks if the value of the field is `HIGH`"]
    #[inline]
    pub fn is_high(&self) -> bool {
        *self == PIN26R::HIGH
    }
}
#[doc = "Possible values of the field `PIN27`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PIN27R {
    #[doc = "Pin driver is low"]
    LOW,
    #[doc = "Pin driver is high"]
    HIGH,
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
            PIN27R::LOW => false,
            PIN27R::HIGH => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> PIN27R {
        match value {
            false => PIN27R::LOW,
            true => PIN27R::HIGH,
        }
    }
    #[doc = "Checks if the value of the field is `LOW`"]
    #[inline]
    pub fn is_low(&self) -> bool {
        *self == PIN27R::LOW
    }
    #[doc = "Checks if the value of the field is `HIGH`"]
    #[inline]
    pub fn is_high(&self) -> bool {
        *self == PIN27R::HIGH
    }
}
#[doc = "Possible values of the field `PIN28`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PIN28R {
    #[doc = "Pin driver is low"]
    LOW,
    #[doc = "Pin driver is high"]
    HIGH,
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
            PIN28R::LOW => false,
            PIN28R::HIGH => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> PIN28R {
        match value {
            false => PIN28R::LOW,
            true => PIN28R::HIGH,
        }
    }
    #[doc = "Checks if the value of the field is `LOW`"]
    #[inline]
    pub fn is_low(&self) -> bool {
        *self == PIN28R::LOW
    }
    #[doc = "Checks if the value of the field is `HIGH`"]
    #[inline]
    pub fn is_high(&self) -> bool {
        *self == PIN28R::HIGH
    }
}
#[doc = "Possible values of the field `PIN29`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PIN29R {
    #[doc = "Pin driver is low"]
    LOW,
    #[doc = "Pin driver is high"]
    HIGH,
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
            PIN29R::LOW => false,
            PIN29R::HIGH => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> PIN29R {
        match value {
            false => PIN29R::LOW,
            true => PIN29R::HIGH,
        }
    }
    #[doc = "Checks if the value of the field is `LOW`"]
    #[inline]
    pub fn is_low(&self) -> bool {
        *self == PIN29R::LOW
    }
    #[doc = "Checks if the value of the field is `HIGH`"]
    #[inline]
    pub fn is_high(&self) -> bool {
        *self == PIN29R::HIGH
    }
}
#[doc = "Possible values of the field `PIN30`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PIN30R {
    #[doc = "Pin driver is low"]
    LOW,
    #[doc = "Pin driver is high"]
    HIGH,
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
            PIN30R::LOW => false,
            PIN30R::HIGH => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> PIN30R {
        match value {
            false => PIN30R::LOW,
            true => PIN30R::HIGH,
        }
    }
    #[doc = "Checks if the value of the field is `LOW`"]
    #[inline]
    pub fn is_low(&self) -> bool {
        *self == PIN30R::LOW
    }
    #[doc = "Checks if the value of the field is `HIGH`"]
    #[inline]
    pub fn is_high(&self) -> bool {
        *self == PIN30R::HIGH
    }
}
#[doc = "Possible values of the field `PIN31`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PIN31R {
    #[doc = "Pin driver is low"]
    LOW,
    #[doc = "Pin driver is high"]
    HIGH,
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
            PIN31R::LOW => false,
            PIN31R::HIGH => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> PIN31R {
        match value {
            false => PIN31R::LOW,
            true => PIN31R::HIGH,
        }
    }
    #[doc = "Checks if the value of the field is `LOW`"]
    #[inline]
    pub fn is_low(&self) -> bool {
        *self == PIN31R::LOW
    }
    #[doc = "Checks if the value of the field is `HIGH`"]
    #[inline]
    pub fn is_high(&self) -> bool {
        *self == PIN31R::HIGH
    }
}
#[doc = "Values that can be written to the field `PIN0`"]
pub enum PIN0W {
    #[doc = "Pin driver is low"]
    LOW,
    #[doc = "Pin driver is high"]
    HIGH,
}
impl PIN0W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            PIN0W::LOW => false,
            PIN0W::HIGH => true,
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
    #[doc = "Pin driver is low"]
    #[inline]
    pub fn low(self) -> &'a mut W {
        self.variant(PIN0W::LOW)
    }
    #[doc = "Pin driver is high"]
    #[inline]
    pub fn high(self) -> &'a mut W {
        self.variant(PIN0W::HIGH)
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
    #[doc = "Pin driver is low"]
    LOW,
    #[doc = "Pin driver is high"]
    HIGH,
}
impl PIN1W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            PIN1W::LOW => false,
            PIN1W::HIGH => true,
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
    #[doc = "Pin driver is low"]
    #[inline]
    pub fn low(self) -> &'a mut W {
        self.variant(PIN1W::LOW)
    }
    #[doc = "Pin driver is high"]
    #[inline]
    pub fn high(self) -> &'a mut W {
        self.variant(PIN1W::HIGH)
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
    #[doc = "Pin driver is low"]
    LOW,
    #[doc = "Pin driver is high"]
    HIGH,
}
impl PIN2W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            PIN2W::LOW => false,
            PIN2W::HIGH => true,
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
    #[doc = "Pin driver is low"]
    #[inline]
    pub fn low(self) -> &'a mut W {
        self.variant(PIN2W::LOW)
    }
    #[doc = "Pin driver is high"]
    #[inline]
    pub fn high(self) -> &'a mut W {
        self.variant(PIN2W::HIGH)
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
    #[doc = "Pin driver is low"]
    LOW,
    #[doc = "Pin driver is high"]
    HIGH,
}
impl PIN3W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            PIN3W::LOW => false,
            PIN3W::HIGH => true,
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
    #[doc = "Pin driver is low"]
    #[inline]
    pub fn low(self) -> &'a mut W {
        self.variant(PIN3W::LOW)
    }
    #[doc = "Pin driver is high"]
    #[inline]
    pub fn high(self) -> &'a mut W {
        self.variant(PIN3W::HIGH)
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
    #[doc = "Pin driver is low"]
    LOW,
    #[doc = "Pin driver is high"]
    HIGH,
}
impl PIN4W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            PIN4W::LOW => false,
            PIN4W::HIGH => true,
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
    #[doc = "Pin driver is low"]
    #[inline]
    pub fn low(self) -> &'a mut W {
        self.variant(PIN4W::LOW)
    }
    #[doc = "Pin driver is high"]
    #[inline]
    pub fn high(self) -> &'a mut W {
        self.variant(PIN4W::HIGH)
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
    #[doc = "Pin driver is low"]
    LOW,
    #[doc = "Pin driver is high"]
    HIGH,
}
impl PIN5W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            PIN5W::LOW => false,
            PIN5W::HIGH => true,
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
    #[doc = "Pin driver is low"]
    #[inline]
    pub fn low(self) -> &'a mut W {
        self.variant(PIN5W::LOW)
    }
    #[doc = "Pin driver is high"]
    #[inline]
    pub fn high(self) -> &'a mut W {
        self.variant(PIN5W::HIGH)
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
    #[doc = "Pin driver is low"]
    LOW,
    #[doc = "Pin driver is high"]
    HIGH,
}
impl PIN6W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            PIN6W::LOW => false,
            PIN6W::HIGH => true,
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
    #[doc = "Pin driver is low"]
    #[inline]
    pub fn low(self) -> &'a mut W {
        self.variant(PIN6W::LOW)
    }
    #[doc = "Pin driver is high"]
    #[inline]
    pub fn high(self) -> &'a mut W {
        self.variant(PIN6W::HIGH)
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
    #[doc = "Pin driver is low"]
    LOW,
    #[doc = "Pin driver is high"]
    HIGH,
}
impl PIN7W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            PIN7W::LOW => false,
            PIN7W::HIGH => true,
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
    #[doc = "Pin driver is low"]
    #[inline]
    pub fn low(self) -> &'a mut W {
        self.variant(PIN7W::LOW)
    }
    #[doc = "Pin driver is high"]
    #[inline]
    pub fn high(self) -> &'a mut W {
        self.variant(PIN7W::HIGH)
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
    #[doc = "Pin driver is low"]
    LOW,
    #[doc = "Pin driver is high"]
    HIGH,
}
impl PIN8W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            PIN8W::LOW => false,
            PIN8W::HIGH => true,
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
    #[doc = "Pin driver is low"]
    #[inline]
    pub fn low(self) -> &'a mut W {
        self.variant(PIN8W::LOW)
    }
    #[doc = "Pin driver is high"]
    #[inline]
    pub fn high(self) -> &'a mut W {
        self.variant(PIN8W::HIGH)
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
    #[doc = "Pin driver is low"]
    LOW,
    #[doc = "Pin driver is high"]
    HIGH,
}
impl PIN9W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            PIN9W::LOW => false,
            PIN9W::HIGH => true,
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
    #[doc = "Pin driver is low"]
    #[inline]
    pub fn low(self) -> &'a mut W {
        self.variant(PIN9W::LOW)
    }
    #[doc = "Pin driver is high"]
    #[inline]
    pub fn high(self) -> &'a mut W {
        self.variant(PIN9W::HIGH)
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
    #[doc = "Pin driver is low"]
    LOW,
    #[doc = "Pin driver is high"]
    HIGH,
}
impl PIN10W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            PIN10W::LOW => false,
            PIN10W::HIGH => true,
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
    #[doc = "Pin driver is low"]
    #[inline]
    pub fn low(self) -> &'a mut W {
        self.variant(PIN10W::LOW)
    }
    #[doc = "Pin driver is high"]
    #[inline]
    pub fn high(self) -> &'a mut W {
        self.variant(PIN10W::HIGH)
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
    #[doc = "Pin driver is low"]
    LOW,
    #[doc = "Pin driver is high"]
    HIGH,
}
impl PIN11W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            PIN11W::LOW => false,
            PIN11W::HIGH => true,
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
    #[doc = "Pin driver is low"]
    #[inline]
    pub fn low(self) -> &'a mut W {
        self.variant(PIN11W::LOW)
    }
    #[doc = "Pin driver is high"]
    #[inline]
    pub fn high(self) -> &'a mut W {
        self.variant(PIN11W::HIGH)
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
    #[doc = "Pin driver is low"]
    LOW,
    #[doc = "Pin driver is high"]
    HIGH,
}
impl PIN12W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            PIN12W::LOW => false,
            PIN12W::HIGH => true,
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
    #[doc = "Pin driver is low"]
    #[inline]
    pub fn low(self) -> &'a mut W {
        self.variant(PIN12W::LOW)
    }
    #[doc = "Pin driver is high"]
    #[inline]
    pub fn high(self) -> &'a mut W {
        self.variant(PIN12W::HIGH)
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
    #[doc = "Pin driver is low"]
    LOW,
    #[doc = "Pin driver is high"]
    HIGH,
}
impl PIN13W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            PIN13W::LOW => false,
            PIN13W::HIGH => true,
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
    #[doc = "Pin driver is low"]
    #[inline]
    pub fn low(self) -> &'a mut W {
        self.variant(PIN13W::LOW)
    }
    #[doc = "Pin driver is high"]
    #[inline]
    pub fn high(self) -> &'a mut W {
        self.variant(PIN13W::HIGH)
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
    #[doc = "Pin driver is low"]
    LOW,
    #[doc = "Pin driver is high"]
    HIGH,
}
impl PIN14W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            PIN14W::LOW => false,
            PIN14W::HIGH => true,
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
    #[doc = "Pin driver is low"]
    #[inline]
    pub fn low(self) -> &'a mut W {
        self.variant(PIN14W::LOW)
    }
    #[doc = "Pin driver is high"]
    #[inline]
    pub fn high(self) -> &'a mut W {
        self.variant(PIN14W::HIGH)
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
    #[doc = "Pin driver is low"]
    LOW,
    #[doc = "Pin driver is high"]
    HIGH,
}
impl PIN15W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            PIN15W::LOW => false,
            PIN15W::HIGH => true,
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
    #[doc = "Pin driver is low"]
    #[inline]
    pub fn low(self) -> &'a mut W {
        self.variant(PIN15W::LOW)
    }
    #[doc = "Pin driver is high"]
    #[inline]
    pub fn high(self) -> &'a mut W {
        self.variant(PIN15W::HIGH)
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
    #[doc = "Pin driver is low"]
    LOW,
    #[doc = "Pin driver is high"]
    HIGH,
}
impl PIN16W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            PIN16W::LOW => false,
            PIN16W::HIGH => true,
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
    #[doc = "Pin driver is low"]
    #[inline]
    pub fn low(self) -> &'a mut W {
        self.variant(PIN16W::LOW)
    }
    #[doc = "Pin driver is high"]
    #[inline]
    pub fn high(self) -> &'a mut W {
        self.variant(PIN16W::HIGH)
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
    #[doc = "Pin driver is low"]
    LOW,
    #[doc = "Pin driver is high"]
    HIGH,
}
impl PIN17W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            PIN17W::LOW => false,
            PIN17W::HIGH => true,
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
    #[doc = "Pin driver is low"]
    #[inline]
    pub fn low(self) -> &'a mut W {
        self.variant(PIN17W::LOW)
    }
    #[doc = "Pin driver is high"]
    #[inline]
    pub fn high(self) -> &'a mut W {
        self.variant(PIN17W::HIGH)
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
    #[doc = "Pin driver is low"]
    LOW,
    #[doc = "Pin driver is high"]
    HIGH,
}
impl PIN18W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            PIN18W::LOW => false,
            PIN18W::HIGH => true,
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
    #[doc = "Pin driver is low"]
    #[inline]
    pub fn low(self) -> &'a mut W {
        self.variant(PIN18W::LOW)
    }
    #[doc = "Pin driver is high"]
    #[inline]
    pub fn high(self) -> &'a mut W {
        self.variant(PIN18W::HIGH)
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
    #[doc = "Pin driver is low"]
    LOW,
    #[doc = "Pin driver is high"]
    HIGH,
}
impl PIN19W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            PIN19W::LOW => false,
            PIN19W::HIGH => true,
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
    #[doc = "Pin driver is low"]
    #[inline]
    pub fn low(self) -> &'a mut W {
        self.variant(PIN19W::LOW)
    }
    #[doc = "Pin driver is high"]
    #[inline]
    pub fn high(self) -> &'a mut W {
        self.variant(PIN19W::HIGH)
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
    #[doc = "Pin driver is low"]
    LOW,
    #[doc = "Pin driver is high"]
    HIGH,
}
impl PIN20W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            PIN20W::LOW => false,
            PIN20W::HIGH => true,
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
    #[doc = "Pin driver is low"]
    #[inline]
    pub fn low(self) -> &'a mut W {
        self.variant(PIN20W::LOW)
    }
    #[doc = "Pin driver is high"]
    #[inline]
    pub fn high(self) -> &'a mut W {
        self.variant(PIN20W::HIGH)
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
    #[doc = "Pin driver is low"]
    LOW,
    #[doc = "Pin driver is high"]
    HIGH,
}
impl PIN21W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            PIN21W::LOW => false,
            PIN21W::HIGH => true,
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
    #[doc = "Pin driver is low"]
    #[inline]
    pub fn low(self) -> &'a mut W {
        self.variant(PIN21W::LOW)
    }
    #[doc = "Pin driver is high"]
    #[inline]
    pub fn high(self) -> &'a mut W {
        self.variant(PIN21W::HIGH)
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
    #[doc = "Pin driver is low"]
    LOW,
    #[doc = "Pin driver is high"]
    HIGH,
}
impl PIN22W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            PIN22W::LOW => false,
            PIN22W::HIGH => true,
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
    #[doc = "Pin driver is low"]
    #[inline]
    pub fn low(self) -> &'a mut W {
        self.variant(PIN22W::LOW)
    }
    #[doc = "Pin driver is high"]
    #[inline]
    pub fn high(self) -> &'a mut W {
        self.variant(PIN22W::HIGH)
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
    #[doc = "Pin driver is low"]
    LOW,
    #[doc = "Pin driver is high"]
    HIGH,
}
impl PIN23W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            PIN23W::LOW => false,
            PIN23W::HIGH => true,
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
    #[doc = "Pin driver is low"]
    #[inline]
    pub fn low(self) -> &'a mut W {
        self.variant(PIN23W::LOW)
    }
    #[doc = "Pin driver is high"]
    #[inline]
    pub fn high(self) -> &'a mut W {
        self.variant(PIN23W::HIGH)
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
    #[doc = "Pin driver is low"]
    LOW,
    #[doc = "Pin driver is high"]
    HIGH,
}
impl PIN24W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            PIN24W::LOW => false,
            PIN24W::HIGH => true,
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
    #[doc = "Pin driver is low"]
    #[inline]
    pub fn low(self) -> &'a mut W {
        self.variant(PIN24W::LOW)
    }
    #[doc = "Pin driver is high"]
    #[inline]
    pub fn high(self) -> &'a mut W {
        self.variant(PIN24W::HIGH)
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
    #[doc = "Pin driver is low"]
    LOW,
    #[doc = "Pin driver is high"]
    HIGH,
}
impl PIN25W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            PIN25W::LOW => false,
            PIN25W::HIGH => true,
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
    #[doc = "Pin driver is low"]
    #[inline]
    pub fn low(self) -> &'a mut W {
        self.variant(PIN25W::LOW)
    }
    #[doc = "Pin driver is high"]
    #[inline]
    pub fn high(self) -> &'a mut W {
        self.variant(PIN25W::HIGH)
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
    #[doc = "Pin driver is low"]
    LOW,
    #[doc = "Pin driver is high"]
    HIGH,
}
impl PIN26W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            PIN26W::LOW => false,
            PIN26W::HIGH => true,
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
    #[doc = "Pin driver is low"]
    #[inline]
    pub fn low(self) -> &'a mut W {
        self.variant(PIN26W::LOW)
    }
    #[doc = "Pin driver is high"]
    #[inline]
    pub fn high(self) -> &'a mut W {
        self.variant(PIN26W::HIGH)
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
    #[doc = "Pin driver is low"]
    LOW,
    #[doc = "Pin driver is high"]
    HIGH,
}
impl PIN27W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            PIN27W::LOW => false,
            PIN27W::HIGH => true,
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
    #[doc = "Pin driver is low"]
    #[inline]
    pub fn low(self) -> &'a mut W {
        self.variant(PIN27W::LOW)
    }
    #[doc = "Pin driver is high"]
    #[inline]
    pub fn high(self) -> &'a mut W {
        self.variant(PIN27W::HIGH)
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
    #[doc = "Pin driver is low"]
    LOW,
    #[doc = "Pin driver is high"]
    HIGH,
}
impl PIN28W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            PIN28W::LOW => false,
            PIN28W::HIGH => true,
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
    #[doc = "Pin driver is low"]
    #[inline]
    pub fn low(self) -> &'a mut W {
        self.variant(PIN28W::LOW)
    }
    #[doc = "Pin driver is high"]
    #[inline]
    pub fn high(self) -> &'a mut W {
        self.variant(PIN28W::HIGH)
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
    #[doc = "Pin driver is low"]
    LOW,
    #[doc = "Pin driver is high"]
    HIGH,
}
impl PIN29W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            PIN29W::LOW => false,
            PIN29W::HIGH => true,
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
    #[doc = "Pin driver is low"]
    #[inline]
    pub fn low(self) -> &'a mut W {
        self.variant(PIN29W::LOW)
    }
    #[doc = "Pin driver is high"]
    #[inline]
    pub fn high(self) -> &'a mut W {
        self.variant(PIN29W::HIGH)
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
    #[doc = "Pin driver is low"]
    LOW,
    #[doc = "Pin driver is high"]
    HIGH,
}
impl PIN30W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            PIN30W::LOW => false,
            PIN30W::HIGH => true,
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
    #[doc = "Pin driver is low"]
    #[inline]
    pub fn low(self) -> &'a mut W {
        self.variant(PIN30W::LOW)
    }
    #[doc = "Pin driver is high"]
    #[inline]
    pub fn high(self) -> &'a mut W {
        self.variant(PIN30W::HIGH)
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
    #[doc = "Pin driver is low"]
    LOW,
    #[doc = "Pin driver is high"]
    HIGH,
}
impl PIN31W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            PIN31W::LOW => false,
            PIN31W::HIGH => true,
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
    #[doc = "Pin driver is low"]
    #[inline]
    pub fn low(self) -> &'a mut W {
        self.variant(PIN31W::LOW)
    }
    #[doc = "Pin driver is high"]
    #[inline]
    pub fn high(self) -> &'a mut W {
        self.variant(PIN31W::HIGH)
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
    #[doc = "Bit 0 - Pin 0"]
    #[inline]
    pub fn pin0(&self) -> PIN0R {
        PIN0R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 1 - Pin 1"]
    #[inline]
    pub fn pin1(&self) -> PIN1R {
        PIN1R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 1;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 2 - Pin 2"]
    #[inline]
    pub fn pin2(&self) -> PIN2R {
        PIN2R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 2;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 3 - Pin 3"]
    #[inline]
    pub fn pin3(&self) -> PIN3R {
        PIN3R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 3;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 4 - Pin 4"]
    #[inline]
    pub fn pin4(&self) -> PIN4R {
        PIN4R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 4;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 5 - Pin 5"]
    #[inline]
    pub fn pin5(&self) -> PIN5R {
        PIN5R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 5;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 6 - Pin 6"]
    #[inline]
    pub fn pin6(&self) -> PIN6R {
        PIN6R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 6;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 7 - Pin 7"]
    #[inline]
    pub fn pin7(&self) -> PIN7R {
        PIN7R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 7;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 8 - Pin 8"]
    #[inline]
    pub fn pin8(&self) -> PIN8R {
        PIN8R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 8;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 9 - Pin 9"]
    #[inline]
    pub fn pin9(&self) -> PIN9R {
        PIN9R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 9;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 10 - Pin 10"]
    #[inline]
    pub fn pin10(&self) -> PIN10R {
        PIN10R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 10;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 11 - Pin 11"]
    #[inline]
    pub fn pin11(&self) -> PIN11R {
        PIN11R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 11;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 12 - Pin 12"]
    #[inline]
    pub fn pin12(&self) -> PIN12R {
        PIN12R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 12;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 13 - Pin 13"]
    #[inline]
    pub fn pin13(&self) -> PIN13R {
        PIN13R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 13;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 14 - Pin 14"]
    #[inline]
    pub fn pin14(&self) -> PIN14R {
        PIN14R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 14;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 15 - Pin 15"]
    #[inline]
    pub fn pin15(&self) -> PIN15R {
        PIN15R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 15;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 16 - Pin 16"]
    #[inline]
    pub fn pin16(&self) -> PIN16R {
        PIN16R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 16;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 17 - Pin 17"]
    #[inline]
    pub fn pin17(&self) -> PIN17R {
        PIN17R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 17;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 18 - Pin 18"]
    #[inline]
    pub fn pin18(&self) -> PIN18R {
        PIN18R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 18;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 19 - Pin 19"]
    #[inline]
    pub fn pin19(&self) -> PIN19R {
        PIN19R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 19;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 20 - Pin 20"]
    #[inline]
    pub fn pin20(&self) -> PIN20R {
        PIN20R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 20;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 21 - Pin 21"]
    #[inline]
    pub fn pin21(&self) -> PIN21R {
        PIN21R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 21;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 22 - Pin 22"]
    #[inline]
    pub fn pin22(&self) -> PIN22R {
        PIN22R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 22;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 23 - Pin 23"]
    #[inline]
    pub fn pin23(&self) -> PIN23R {
        PIN23R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 23;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 24 - Pin 24"]
    #[inline]
    pub fn pin24(&self) -> PIN24R {
        PIN24R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 24;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 25 - Pin 25"]
    #[inline]
    pub fn pin25(&self) -> PIN25R {
        PIN25R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 25;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 26 - Pin 26"]
    #[inline]
    pub fn pin26(&self) -> PIN26R {
        PIN26R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 26;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 27 - Pin 27"]
    #[inline]
    pub fn pin27(&self) -> PIN27R {
        PIN27R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 27;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 28 - Pin 28"]
    #[inline]
    pub fn pin28(&self) -> PIN28R {
        PIN28R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 28;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 29 - Pin 29"]
    #[inline]
    pub fn pin29(&self) -> PIN29R {
        PIN29R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 29;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 30 - Pin 30"]
    #[inline]
    pub fn pin30(&self) -> PIN30R {
        PIN30R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 30;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 31 - Pin 31"]
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
    #[doc = "Bit 0 - Pin 0"]
    #[inline]
    pub fn pin0(&mut self) -> _PIN0W {
        _PIN0W { w: self }
    }
    #[doc = "Bit 1 - Pin 1"]
    #[inline]
    pub fn pin1(&mut self) -> _PIN1W {
        _PIN1W { w: self }
    }
    #[doc = "Bit 2 - Pin 2"]
    #[inline]
    pub fn pin2(&mut self) -> _PIN2W {
        _PIN2W { w: self }
    }
    #[doc = "Bit 3 - Pin 3"]
    #[inline]
    pub fn pin3(&mut self) -> _PIN3W {
        _PIN3W { w: self }
    }
    #[doc = "Bit 4 - Pin 4"]
    #[inline]
    pub fn pin4(&mut self) -> _PIN4W {
        _PIN4W { w: self }
    }
    #[doc = "Bit 5 - Pin 5"]
    #[inline]
    pub fn pin5(&mut self) -> _PIN5W {
        _PIN5W { w: self }
    }
    #[doc = "Bit 6 - Pin 6"]
    #[inline]
    pub fn pin6(&mut self) -> _PIN6W {
        _PIN6W { w: self }
    }
    #[doc = "Bit 7 - Pin 7"]
    #[inline]
    pub fn pin7(&mut self) -> _PIN7W {
        _PIN7W { w: self }
    }
    #[doc = "Bit 8 - Pin 8"]
    #[inline]
    pub fn pin8(&mut self) -> _PIN8W {
        _PIN8W { w: self }
    }
    #[doc = "Bit 9 - Pin 9"]
    #[inline]
    pub fn pin9(&mut self) -> _PIN9W {
        _PIN9W { w: self }
    }
    #[doc = "Bit 10 - Pin 10"]
    #[inline]
    pub fn pin10(&mut self) -> _PIN10W {
        _PIN10W { w: self }
    }
    #[doc = "Bit 11 - Pin 11"]
    #[inline]
    pub fn pin11(&mut self) -> _PIN11W {
        _PIN11W { w: self }
    }
    #[doc = "Bit 12 - Pin 12"]
    #[inline]
    pub fn pin12(&mut self) -> _PIN12W {
        _PIN12W { w: self }
    }
    #[doc = "Bit 13 - Pin 13"]
    #[inline]
    pub fn pin13(&mut self) -> _PIN13W {
        _PIN13W { w: self }
    }
    #[doc = "Bit 14 - Pin 14"]
    #[inline]
    pub fn pin14(&mut self) -> _PIN14W {
        _PIN14W { w: self }
    }
    #[doc = "Bit 15 - Pin 15"]
    #[inline]
    pub fn pin15(&mut self) -> _PIN15W {
        _PIN15W { w: self }
    }
    #[doc = "Bit 16 - Pin 16"]
    #[inline]
    pub fn pin16(&mut self) -> _PIN16W {
        _PIN16W { w: self }
    }
    #[doc = "Bit 17 - Pin 17"]
    #[inline]
    pub fn pin17(&mut self) -> _PIN17W {
        _PIN17W { w: self }
    }
    #[doc = "Bit 18 - Pin 18"]
    #[inline]
    pub fn pin18(&mut self) -> _PIN18W {
        _PIN18W { w: self }
    }
    #[doc = "Bit 19 - Pin 19"]
    #[inline]
    pub fn pin19(&mut self) -> _PIN19W {
        _PIN19W { w: self }
    }
    #[doc = "Bit 20 - Pin 20"]
    #[inline]
    pub fn pin20(&mut self) -> _PIN20W {
        _PIN20W { w: self }
    }
    #[doc = "Bit 21 - Pin 21"]
    #[inline]
    pub fn pin21(&mut self) -> _PIN21W {
        _PIN21W { w: self }
    }
    #[doc = "Bit 22 - Pin 22"]
    #[inline]
    pub fn pin22(&mut self) -> _PIN22W {
        _PIN22W { w: self }
    }
    #[doc = "Bit 23 - Pin 23"]
    #[inline]
    pub fn pin23(&mut self) -> _PIN23W {
        _PIN23W { w: self }
    }
    #[doc = "Bit 24 - Pin 24"]
    #[inline]
    pub fn pin24(&mut self) -> _PIN24W {
        _PIN24W { w: self }
    }
    #[doc = "Bit 25 - Pin 25"]
    #[inline]
    pub fn pin25(&mut self) -> _PIN25W {
        _PIN25W { w: self }
    }
    #[doc = "Bit 26 - Pin 26"]
    #[inline]
    pub fn pin26(&mut self) -> _PIN26W {
        _PIN26W { w: self }
    }
    #[doc = "Bit 27 - Pin 27"]
    #[inline]
    pub fn pin27(&mut self) -> _PIN27W {
        _PIN27W { w: self }
    }
    #[doc = "Bit 28 - Pin 28"]
    #[inline]
    pub fn pin28(&mut self) -> _PIN28W {
        _PIN28W { w: self }
    }
    #[doc = "Bit 29 - Pin 29"]
    #[inline]
    pub fn pin29(&mut self) -> _PIN29W {
        _PIN29W { w: self }
    }
    #[doc = "Bit 30 - Pin 30"]
    #[inline]
    pub fn pin30(&mut self) -> _PIN30W {
        _PIN30W { w: self }
    }
    #[doc = "Bit 31 - Pin 31"]
    #[inline]
    pub fn pin31(&mut self) -> _PIN31W {
        _PIN31W { w: self }
    }
}
