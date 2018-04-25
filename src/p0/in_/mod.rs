#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
impl super::IN {
    #[doc = r" Reads the contents of the register"]
    #[inline]
    pub fn read(&self) -> R {
        R {
            bits: self.register.get(),
        }
    }
}
#[doc = "Possible values of the field `PIN0`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PIN0R {
    #[doc = "Pin input is low"]
    LOW,
    #[doc = "Pin input is high"]
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
    #[doc = "Pin input is low"]
    LOW,
    #[doc = "Pin input is high"]
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
    #[doc = "Pin input is low"]
    LOW,
    #[doc = "Pin input is high"]
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
    #[doc = "Pin input is low"]
    LOW,
    #[doc = "Pin input is high"]
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
    #[doc = "Pin input is low"]
    LOW,
    #[doc = "Pin input is high"]
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
    #[doc = "Pin input is low"]
    LOW,
    #[doc = "Pin input is high"]
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
    #[doc = "Pin input is low"]
    LOW,
    #[doc = "Pin input is high"]
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
    #[doc = "Pin input is low"]
    LOW,
    #[doc = "Pin input is high"]
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
    #[doc = "Pin input is low"]
    LOW,
    #[doc = "Pin input is high"]
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
    #[doc = "Pin input is low"]
    LOW,
    #[doc = "Pin input is high"]
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
    #[doc = "Pin input is low"]
    LOW,
    #[doc = "Pin input is high"]
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
    #[doc = "Pin input is low"]
    LOW,
    #[doc = "Pin input is high"]
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
    #[doc = "Pin input is low"]
    LOW,
    #[doc = "Pin input is high"]
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
    #[doc = "Pin input is low"]
    LOW,
    #[doc = "Pin input is high"]
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
    #[doc = "Pin input is low"]
    LOW,
    #[doc = "Pin input is high"]
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
    #[doc = "Pin input is low"]
    LOW,
    #[doc = "Pin input is high"]
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
    #[doc = "Pin input is low"]
    LOW,
    #[doc = "Pin input is high"]
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
    #[doc = "Pin input is low"]
    LOW,
    #[doc = "Pin input is high"]
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
    #[doc = "Pin input is low"]
    LOW,
    #[doc = "Pin input is high"]
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
    #[doc = "Pin input is low"]
    LOW,
    #[doc = "Pin input is high"]
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
    #[doc = "Pin input is low"]
    LOW,
    #[doc = "Pin input is high"]
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
    #[doc = "Pin input is low"]
    LOW,
    #[doc = "Pin input is high"]
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
    #[doc = "Pin input is low"]
    LOW,
    #[doc = "Pin input is high"]
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
    #[doc = "Pin input is low"]
    LOW,
    #[doc = "Pin input is high"]
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
    #[doc = "Pin input is low"]
    LOW,
    #[doc = "Pin input is high"]
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
    #[doc = "Pin input is low"]
    LOW,
    #[doc = "Pin input is high"]
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
    #[doc = "Pin input is low"]
    LOW,
    #[doc = "Pin input is high"]
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
    #[doc = "Pin input is low"]
    LOW,
    #[doc = "Pin input is high"]
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
    #[doc = "Pin input is low"]
    LOW,
    #[doc = "Pin input is high"]
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
    #[doc = "Pin input is low"]
    LOW,
    #[doc = "Pin input is high"]
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
    #[doc = "Pin input is low"]
    LOW,
    #[doc = "Pin input is high"]
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
    #[doc = "Pin input is low"]
    LOW,
    #[doc = "Pin input is high"]
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
