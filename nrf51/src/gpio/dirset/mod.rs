#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::DIRSET {
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
    #[doc = "Pin set as input."]
    INPUT,
    #[doc = "Pin set as output."]
    OUTPUT,
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
            PIN0R::INPUT => false,
            PIN0R::OUTPUT => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> PIN0R {
        match value {
            false => PIN0R::INPUT,
            true => PIN0R::OUTPUT,
        }
    }
    #[doc = "Checks if the value of the field is `INPUT`"]
    #[inline]
    pub fn is_input(&self) -> bool {
        *self == PIN0R::INPUT
    }
    #[doc = "Checks if the value of the field is `OUTPUT`"]
    #[inline]
    pub fn is_output(&self) -> bool {
        *self == PIN0R::OUTPUT
    }
}
#[doc = "Possible values of the field `PIN1`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PIN1R {
    #[doc = "Pin set as input."]
    INPUT,
    #[doc = "Pin set as output."]
    OUTPUT,
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
            PIN1R::INPUT => false,
            PIN1R::OUTPUT => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> PIN1R {
        match value {
            false => PIN1R::INPUT,
            true => PIN1R::OUTPUT,
        }
    }
    #[doc = "Checks if the value of the field is `INPUT`"]
    #[inline]
    pub fn is_input(&self) -> bool {
        *self == PIN1R::INPUT
    }
    #[doc = "Checks if the value of the field is `OUTPUT`"]
    #[inline]
    pub fn is_output(&self) -> bool {
        *self == PIN1R::OUTPUT
    }
}
#[doc = "Possible values of the field `PIN2`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PIN2R {
    #[doc = "Pin set as input."]
    INPUT,
    #[doc = "Pin set as output."]
    OUTPUT,
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
            PIN2R::INPUT => false,
            PIN2R::OUTPUT => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> PIN2R {
        match value {
            false => PIN2R::INPUT,
            true => PIN2R::OUTPUT,
        }
    }
    #[doc = "Checks if the value of the field is `INPUT`"]
    #[inline]
    pub fn is_input(&self) -> bool {
        *self == PIN2R::INPUT
    }
    #[doc = "Checks if the value of the field is `OUTPUT`"]
    #[inline]
    pub fn is_output(&self) -> bool {
        *self == PIN2R::OUTPUT
    }
}
#[doc = "Possible values of the field `PIN3`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PIN3R {
    #[doc = "Pin set as input."]
    INPUT,
    #[doc = "Pin set as output."]
    OUTPUT,
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
            PIN3R::INPUT => false,
            PIN3R::OUTPUT => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> PIN3R {
        match value {
            false => PIN3R::INPUT,
            true => PIN3R::OUTPUT,
        }
    }
    #[doc = "Checks if the value of the field is `INPUT`"]
    #[inline]
    pub fn is_input(&self) -> bool {
        *self == PIN3R::INPUT
    }
    #[doc = "Checks if the value of the field is `OUTPUT`"]
    #[inline]
    pub fn is_output(&self) -> bool {
        *self == PIN3R::OUTPUT
    }
}
#[doc = "Possible values of the field `PIN4`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PIN4R {
    #[doc = "Pin set as input."]
    INPUT,
    #[doc = "Pin set as output."]
    OUTPUT,
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
            PIN4R::INPUT => false,
            PIN4R::OUTPUT => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> PIN4R {
        match value {
            false => PIN4R::INPUT,
            true => PIN4R::OUTPUT,
        }
    }
    #[doc = "Checks if the value of the field is `INPUT`"]
    #[inline]
    pub fn is_input(&self) -> bool {
        *self == PIN4R::INPUT
    }
    #[doc = "Checks if the value of the field is `OUTPUT`"]
    #[inline]
    pub fn is_output(&self) -> bool {
        *self == PIN4R::OUTPUT
    }
}
#[doc = "Possible values of the field `PIN5`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PIN5R {
    #[doc = "Pin set as input."]
    INPUT,
    #[doc = "Pin set as output."]
    OUTPUT,
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
            PIN5R::INPUT => false,
            PIN5R::OUTPUT => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> PIN5R {
        match value {
            false => PIN5R::INPUT,
            true => PIN5R::OUTPUT,
        }
    }
    #[doc = "Checks if the value of the field is `INPUT`"]
    #[inline]
    pub fn is_input(&self) -> bool {
        *self == PIN5R::INPUT
    }
    #[doc = "Checks if the value of the field is `OUTPUT`"]
    #[inline]
    pub fn is_output(&self) -> bool {
        *self == PIN5R::OUTPUT
    }
}
#[doc = "Possible values of the field `PIN6`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PIN6R {
    #[doc = "Pin set as input."]
    INPUT,
    #[doc = "Pin set as output."]
    OUTPUT,
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
            PIN6R::INPUT => false,
            PIN6R::OUTPUT => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> PIN6R {
        match value {
            false => PIN6R::INPUT,
            true => PIN6R::OUTPUT,
        }
    }
    #[doc = "Checks if the value of the field is `INPUT`"]
    #[inline]
    pub fn is_input(&self) -> bool {
        *self == PIN6R::INPUT
    }
    #[doc = "Checks if the value of the field is `OUTPUT`"]
    #[inline]
    pub fn is_output(&self) -> bool {
        *self == PIN6R::OUTPUT
    }
}
#[doc = "Possible values of the field `PIN7`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PIN7R {
    #[doc = "Pin set as input."]
    INPUT,
    #[doc = "Pin set as output."]
    OUTPUT,
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
            PIN7R::INPUT => false,
            PIN7R::OUTPUT => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> PIN7R {
        match value {
            false => PIN7R::INPUT,
            true => PIN7R::OUTPUT,
        }
    }
    #[doc = "Checks if the value of the field is `INPUT`"]
    #[inline]
    pub fn is_input(&self) -> bool {
        *self == PIN7R::INPUT
    }
    #[doc = "Checks if the value of the field is `OUTPUT`"]
    #[inline]
    pub fn is_output(&self) -> bool {
        *self == PIN7R::OUTPUT
    }
}
#[doc = "Possible values of the field `PIN8`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PIN8R {
    #[doc = "Pin set as input."]
    INPUT,
    #[doc = "Pin set as output."]
    OUTPUT,
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
            PIN8R::INPUT => false,
            PIN8R::OUTPUT => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> PIN8R {
        match value {
            false => PIN8R::INPUT,
            true => PIN8R::OUTPUT,
        }
    }
    #[doc = "Checks if the value of the field is `INPUT`"]
    #[inline]
    pub fn is_input(&self) -> bool {
        *self == PIN8R::INPUT
    }
    #[doc = "Checks if the value of the field is `OUTPUT`"]
    #[inline]
    pub fn is_output(&self) -> bool {
        *self == PIN8R::OUTPUT
    }
}
#[doc = "Possible values of the field `PIN9`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PIN9R {
    #[doc = "Pin set as input."]
    INPUT,
    #[doc = "Pin set as output."]
    OUTPUT,
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
            PIN9R::INPUT => false,
            PIN9R::OUTPUT => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> PIN9R {
        match value {
            false => PIN9R::INPUT,
            true => PIN9R::OUTPUT,
        }
    }
    #[doc = "Checks if the value of the field is `INPUT`"]
    #[inline]
    pub fn is_input(&self) -> bool {
        *self == PIN9R::INPUT
    }
    #[doc = "Checks if the value of the field is `OUTPUT`"]
    #[inline]
    pub fn is_output(&self) -> bool {
        *self == PIN9R::OUTPUT
    }
}
#[doc = "Possible values of the field `PIN10`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PIN10R {
    #[doc = "Pin set as input."]
    INPUT,
    #[doc = "Pin set as output."]
    OUTPUT,
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
            PIN10R::INPUT => false,
            PIN10R::OUTPUT => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> PIN10R {
        match value {
            false => PIN10R::INPUT,
            true => PIN10R::OUTPUT,
        }
    }
    #[doc = "Checks if the value of the field is `INPUT`"]
    #[inline]
    pub fn is_input(&self) -> bool {
        *self == PIN10R::INPUT
    }
    #[doc = "Checks if the value of the field is `OUTPUT`"]
    #[inline]
    pub fn is_output(&self) -> bool {
        *self == PIN10R::OUTPUT
    }
}
#[doc = "Possible values of the field `PIN11`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PIN11R {
    #[doc = "Pin set as input."]
    INPUT,
    #[doc = "Pin set as output."]
    OUTPUT,
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
            PIN11R::INPUT => false,
            PIN11R::OUTPUT => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> PIN11R {
        match value {
            false => PIN11R::INPUT,
            true => PIN11R::OUTPUT,
        }
    }
    #[doc = "Checks if the value of the field is `INPUT`"]
    #[inline]
    pub fn is_input(&self) -> bool {
        *self == PIN11R::INPUT
    }
    #[doc = "Checks if the value of the field is `OUTPUT`"]
    #[inline]
    pub fn is_output(&self) -> bool {
        *self == PIN11R::OUTPUT
    }
}
#[doc = "Possible values of the field `PIN12`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PIN12R {
    #[doc = "Pin set as input."]
    INPUT,
    #[doc = "Pin set as output."]
    OUTPUT,
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
            PIN12R::INPUT => false,
            PIN12R::OUTPUT => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> PIN12R {
        match value {
            false => PIN12R::INPUT,
            true => PIN12R::OUTPUT,
        }
    }
    #[doc = "Checks if the value of the field is `INPUT`"]
    #[inline]
    pub fn is_input(&self) -> bool {
        *self == PIN12R::INPUT
    }
    #[doc = "Checks if the value of the field is `OUTPUT`"]
    #[inline]
    pub fn is_output(&self) -> bool {
        *self == PIN12R::OUTPUT
    }
}
#[doc = "Possible values of the field `PIN13`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PIN13R {
    #[doc = "Pin set as input."]
    INPUT,
    #[doc = "Pin set as output."]
    OUTPUT,
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
            PIN13R::INPUT => false,
            PIN13R::OUTPUT => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> PIN13R {
        match value {
            false => PIN13R::INPUT,
            true => PIN13R::OUTPUT,
        }
    }
    #[doc = "Checks if the value of the field is `INPUT`"]
    #[inline]
    pub fn is_input(&self) -> bool {
        *self == PIN13R::INPUT
    }
    #[doc = "Checks if the value of the field is `OUTPUT`"]
    #[inline]
    pub fn is_output(&self) -> bool {
        *self == PIN13R::OUTPUT
    }
}
#[doc = "Possible values of the field `PIN14`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PIN14R {
    #[doc = "Pin set as input."]
    INPUT,
    #[doc = "Pin set as output."]
    OUTPUT,
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
            PIN14R::INPUT => false,
            PIN14R::OUTPUT => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> PIN14R {
        match value {
            false => PIN14R::INPUT,
            true => PIN14R::OUTPUT,
        }
    }
    #[doc = "Checks if the value of the field is `INPUT`"]
    #[inline]
    pub fn is_input(&self) -> bool {
        *self == PIN14R::INPUT
    }
    #[doc = "Checks if the value of the field is `OUTPUT`"]
    #[inline]
    pub fn is_output(&self) -> bool {
        *self == PIN14R::OUTPUT
    }
}
#[doc = "Possible values of the field `PIN15`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PIN15R {
    #[doc = "Pin set as input."]
    INPUT,
    #[doc = "Pin set as output."]
    OUTPUT,
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
            PIN15R::INPUT => false,
            PIN15R::OUTPUT => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> PIN15R {
        match value {
            false => PIN15R::INPUT,
            true => PIN15R::OUTPUT,
        }
    }
    #[doc = "Checks if the value of the field is `INPUT`"]
    #[inline]
    pub fn is_input(&self) -> bool {
        *self == PIN15R::INPUT
    }
    #[doc = "Checks if the value of the field is `OUTPUT`"]
    #[inline]
    pub fn is_output(&self) -> bool {
        *self == PIN15R::OUTPUT
    }
}
#[doc = "Possible values of the field `PIN16`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PIN16R {
    #[doc = "Pin set as input."]
    INPUT,
    #[doc = "Pin set as output."]
    OUTPUT,
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
            PIN16R::INPUT => false,
            PIN16R::OUTPUT => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> PIN16R {
        match value {
            false => PIN16R::INPUT,
            true => PIN16R::OUTPUT,
        }
    }
    #[doc = "Checks if the value of the field is `INPUT`"]
    #[inline]
    pub fn is_input(&self) -> bool {
        *self == PIN16R::INPUT
    }
    #[doc = "Checks if the value of the field is `OUTPUT`"]
    #[inline]
    pub fn is_output(&self) -> bool {
        *self == PIN16R::OUTPUT
    }
}
#[doc = "Possible values of the field `PIN17`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PIN17R {
    #[doc = "Pin set as input."]
    INPUT,
    #[doc = "Pin set as output."]
    OUTPUT,
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
            PIN17R::INPUT => false,
            PIN17R::OUTPUT => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> PIN17R {
        match value {
            false => PIN17R::INPUT,
            true => PIN17R::OUTPUT,
        }
    }
    #[doc = "Checks if the value of the field is `INPUT`"]
    #[inline]
    pub fn is_input(&self) -> bool {
        *self == PIN17R::INPUT
    }
    #[doc = "Checks if the value of the field is `OUTPUT`"]
    #[inline]
    pub fn is_output(&self) -> bool {
        *self == PIN17R::OUTPUT
    }
}
#[doc = "Possible values of the field `PIN18`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PIN18R {
    #[doc = "Pin set as input."]
    INPUT,
    #[doc = "Pin set as output."]
    OUTPUT,
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
            PIN18R::INPUT => false,
            PIN18R::OUTPUT => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> PIN18R {
        match value {
            false => PIN18R::INPUT,
            true => PIN18R::OUTPUT,
        }
    }
    #[doc = "Checks if the value of the field is `INPUT`"]
    #[inline]
    pub fn is_input(&self) -> bool {
        *self == PIN18R::INPUT
    }
    #[doc = "Checks if the value of the field is `OUTPUT`"]
    #[inline]
    pub fn is_output(&self) -> bool {
        *self == PIN18R::OUTPUT
    }
}
#[doc = "Possible values of the field `PIN19`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PIN19R {
    #[doc = "Pin set as input."]
    INPUT,
    #[doc = "Pin set as output."]
    OUTPUT,
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
            PIN19R::INPUT => false,
            PIN19R::OUTPUT => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> PIN19R {
        match value {
            false => PIN19R::INPUT,
            true => PIN19R::OUTPUT,
        }
    }
    #[doc = "Checks if the value of the field is `INPUT`"]
    #[inline]
    pub fn is_input(&self) -> bool {
        *self == PIN19R::INPUT
    }
    #[doc = "Checks if the value of the field is `OUTPUT`"]
    #[inline]
    pub fn is_output(&self) -> bool {
        *self == PIN19R::OUTPUT
    }
}
#[doc = "Possible values of the field `PIN20`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PIN20R {
    #[doc = "Pin set as input."]
    INPUT,
    #[doc = "Pin set as output."]
    OUTPUT,
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
            PIN20R::INPUT => false,
            PIN20R::OUTPUT => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> PIN20R {
        match value {
            false => PIN20R::INPUT,
            true => PIN20R::OUTPUT,
        }
    }
    #[doc = "Checks if the value of the field is `INPUT`"]
    #[inline]
    pub fn is_input(&self) -> bool {
        *self == PIN20R::INPUT
    }
    #[doc = "Checks if the value of the field is `OUTPUT`"]
    #[inline]
    pub fn is_output(&self) -> bool {
        *self == PIN20R::OUTPUT
    }
}
#[doc = "Possible values of the field `PIN21`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PIN21R {
    #[doc = "Pin set as input."]
    INPUT,
    #[doc = "Pin set as output."]
    OUTPUT,
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
            PIN21R::INPUT => false,
            PIN21R::OUTPUT => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> PIN21R {
        match value {
            false => PIN21R::INPUT,
            true => PIN21R::OUTPUT,
        }
    }
    #[doc = "Checks if the value of the field is `INPUT`"]
    #[inline]
    pub fn is_input(&self) -> bool {
        *self == PIN21R::INPUT
    }
    #[doc = "Checks if the value of the field is `OUTPUT`"]
    #[inline]
    pub fn is_output(&self) -> bool {
        *self == PIN21R::OUTPUT
    }
}
#[doc = "Possible values of the field `PIN22`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PIN22R {
    #[doc = "Pin set as input."]
    INPUT,
    #[doc = "Pin set as output."]
    OUTPUT,
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
            PIN22R::INPUT => false,
            PIN22R::OUTPUT => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> PIN22R {
        match value {
            false => PIN22R::INPUT,
            true => PIN22R::OUTPUT,
        }
    }
    #[doc = "Checks if the value of the field is `INPUT`"]
    #[inline]
    pub fn is_input(&self) -> bool {
        *self == PIN22R::INPUT
    }
    #[doc = "Checks if the value of the field is `OUTPUT`"]
    #[inline]
    pub fn is_output(&self) -> bool {
        *self == PIN22R::OUTPUT
    }
}
#[doc = "Possible values of the field `PIN23`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PIN23R {
    #[doc = "Pin set as input."]
    INPUT,
    #[doc = "Pin set as output."]
    OUTPUT,
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
            PIN23R::INPUT => false,
            PIN23R::OUTPUT => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> PIN23R {
        match value {
            false => PIN23R::INPUT,
            true => PIN23R::OUTPUT,
        }
    }
    #[doc = "Checks if the value of the field is `INPUT`"]
    #[inline]
    pub fn is_input(&self) -> bool {
        *self == PIN23R::INPUT
    }
    #[doc = "Checks if the value of the field is `OUTPUT`"]
    #[inline]
    pub fn is_output(&self) -> bool {
        *self == PIN23R::OUTPUT
    }
}
#[doc = "Possible values of the field `PIN24`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PIN24R {
    #[doc = "Pin set as input."]
    INPUT,
    #[doc = "Pin set as output."]
    OUTPUT,
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
            PIN24R::INPUT => false,
            PIN24R::OUTPUT => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> PIN24R {
        match value {
            false => PIN24R::INPUT,
            true => PIN24R::OUTPUT,
        }
    }
    #[doc = "Checks if the value of the field is `INPUT`"]
    #[inline]
    pub fn is_input(&self) -> bool {
        *self == PIN24R::INPUT
    }
    #[doc = "Checks if the value of the field is `OUTPUT`"]
    #[inline]
    pub fn is_output(&self) -> bool {
        *self == PIN24R::OUTPUT
    }
}
#[doc = "Possible values of the field `PIN25`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PIN25R {
    #[doc = "Pin set as input."]
    INPUT,
    #[doc = "Pin set as output."]
    OUTPUT,
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
            PIN25R::INPUT => false,
            PIN25R::OUTPUT => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> PIN25R {
        match value {
            false => PIN25R::INPUT,
            true => PIN25R::OUTPUT,
        }
    }
    #[doc = "Checks if the value of the field is `INPUT`"]
    #[inline]
    pub fn is_input(&self) -> bool {
        *self == PIN25R::INPUT
    }
    #[doc = "Checks if the value of the field is `OUTPUT`"]
    #[inline]
    pub fn is_output(&self) -> bool {
        *self == PIN25R::OUTPUT
    }
}
#[doc = "Possible values of the field `PIN26`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PIN26R {
    #[doc = "Pin set as input."]
    INPUT,
    #[doc = "Pin set as output."]
    OUTPUT,
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
            PIN26R::INPUT => false,
            PIN26R::OUTPUT => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> PIN26R {
        match value {
            false => PIN26R::INPUT,
            true => PIN26R::OUTPUT,
        }
    }
    #[doc = "Checks if the value of the field is `INPUT`"]
    #[inline]
    pub fn is_input(&self) -> bool {
        *self == PIN26R::INPUT
    }
    #[doc = "Checks if the value of the field is `OUTPUT`"]
    #[inline]
    pub fn is_output(&self) -> bool {
        *self == PIN26R::OUTPUT
    }
}
#[doc = "Possible values of the field `PIN27`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PIN27R {
    #[doc = "Pin set as input."]
    INPUT,
    #[doc = "Pin set as output."]
    OUTPUT,
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
            PIN27R::INPUT => false,
            PIN27R::OUTPUT => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> PIN27R {
        match value {
            false => PIN27R::INPUT,
            true => PIN27R::OUTPUT,
        }
    }
    #[doc = "Checks if the value of the field is `INPUT`"]
    #[inline]
    pub fn is_input(&self) -> bool {
        *self == PIN27R::INPUT
    }
    #[doc = "Checks if the value of the field is `OUTPUT`"]
    #[inline]
    pub fn is_output(&self) -> bool {
        *self == PIN27R::OUTPUT
    }
}
#[doc = "Possible values of the field `PIN28`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PIN28R {
    #[doc = "Pin set as input."]
    INPUT,
    #[doc = "Pin set as output."]
    OUTPUT,
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
            PIN28R::INPUT => false,
            PIN28R::OUTPUT => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> PIN28R {
        match value {
            false => PIN28R::INPUT,
            true => PIN28R::OUTPUT,
        }
    }
    #[doc = "Checks if the value of the field is `INPUT`"]
    #[inline]
    pub fn is_input(&self) -> bool {
        *self == PIN28R::INPUT
    }
    #[doc = "Checks if the value of the field is `OUTPUT`"]
    #[inline]
    pub fn is_output(&self) -> bool {
        *self == PIN28R::OUTPUT
    }
}
#[doc = "Possible values of the field `PIN29`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PIN29R {
    #[doc = "Pin set as input."]
    INPUT,
    #[doc = "Pin set as output."]
    OUTPUT,
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
            PIN29R::INPUT => false,
            PIN29R::OUTPUT => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> PIN29R {
        match value {
            false => PIN29R::INPUT,
            true => PIN29R::OUTPUT,
        }
    }
    #[doc = "Checks if the value of the field is `INPUT`"]
    #[inline]
    pub fn is_input(&self) -> bool {
        *self == PIN29R::INPUT
    }
    #[doc = "Checks if the value of the field is `OUTPUT`"]
    #[inline]
    pub fn is_output(&self) -> bool {
        *self == PIN29R::OUTPUT
    }
}
#[doc = "Possible values of the field `PIN30`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PIN30R {
    #[doc = "Pin set as input."]
    INPUT,
    #[doc = "Pin set as output."]
    OUTPUT,
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
            PIN30R::INPUT => false,
            PIN30R::OUTPUT => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> PIN30R {
        match value {
            false => PIN30R::INPUT,
            true => PIN30R::OUTPUT,
        }
    }
    #[doc = "Checks if the value of the field is `INPUT`"]
    #[inline]
    pub fn is_input(&self) -> bool {
        *self == PIN30R::INPUT
    }
    #[doc = "Checks if the value of the field is `OUTPUT`"]
    #[inline]
    pub fn is_output(&self) -> bool {
        *self == PIN30R::OUTPUT
    }
}
#[doc = "Possible values of the field `PIN31`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PIN31R {
    #[doc = "Pin set as input."]
    INPUT,
    #[doc = "Pin set as output."]
    OUTPUT,
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
            PIN31R::INPUT => false,
            PIN31R::OUTPUT => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> PIN31R {
        match value {
            false => PIN31R::INPUT,
            true => PIN31R::OUTPUT,
        }
    }
    #[doc = "Checks if the value of the field is `INPUT`"]
    #[inline]
    pub fn is_input(&self) -> bool {
        *self == PIN31R::INPUT
    }
    #[doc = "Checks if the value of the field is `OUTPUT`"]
    #[inline]
    pub fn is_output(&self) -> bool {
        *self == PIN31R::OUTPUT
    }
}
#[doc = "Values that can be written to the field `PIN0`"]
pub enum PIN0W {
    #[doc = "Set pin as output."]
    SET,
}
impl PIN0W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            PIN0W::SET => true,
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
    #[doc = "Set pin as output."]
    #[inline]
    pub fn set(self) -> &'a mut W {
        self.variant(PIN0W::SET)
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
    #[doc = "Set pin as output."]
    SET,
}
impl PIN1W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            PIN1W::SET => true,
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
    #[doc = "Set pin as output."]
    #[inline]
    pub fn set(self) -> &'a mut W {
        self.variant(PIN1W::SET)
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
    #[doc = "Set pin as output."]
    SET,
}
impl PIN2W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            PIN2W::SET => true,
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
    #[doc = "Set pin as output."]
    #[inline]
    pub fn set(self) -> &'a mut W {
        self.variant(PIN2W::SET)
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
    #[doc = "Set pin as output."]
    SET,
}
impl PIN3W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            PIN3W::SET => true,
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
    #[doc = "Set pin as output."]
    #[inline]
    pub fn set(self) -> &'a mut W {
        self.variant(PIN3W::SET)
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
    #[doc = "Set pin as output."]
    SET,
}
impl PIN4W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            PIN4W::SET => true,
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
    #[doc = "Set pin as output."]
    #[inline]
    pub fn set(self) -> &'a mut W {
        self.variant(PIN4W::SET)
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
    #[doc = "Set pin as output."]
    SET,
}
impl PIN5W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            PIN5W::SET => true,
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
    #[doc = "Set pin as output."]
    #[inline]
    pub fn set(self) -> &'a mut W {
        self.variant(PIN5W::SET)
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
    #[doc = "Set pin as output."]
    SET,
}
impl PIN6W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            PIN6W::SET => true,
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
    #[doc = "Set pin as output."]
    #[inline]
    pub fn set(self) -> &'a mut W {
        self.variant(PIN6W::SET)
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
    #[doc = "Set pin as output."]
    SET,
}
impl PIN7W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            PIN7W::SET => true,
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
    #[doc = "Set pin as output."]
    #[inline]
    pub fn set(self) -> &'a mut W {
        self.variant(PIN7W::SET)
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
    #[doc = "Set pin as output."]
    SET,
}
impl PIN8W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            PIN8W::SET => true,
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
    #[doc = "Set pin as output."]
    #[inline]
    pub fn set(self) -> &'a mut W {
        self.variant(PIN8W::SET)
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
    #[doc = "Set pin as output."]
    SET,
}
impl PIN9W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            PIN9W::SET => true,
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
    #[doc = "Set pin as output."]
    #[inline]
    pub fn set(self) -> &'a mut W {
        self.variant(PIN9W::SET)
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
    #[doc = "Set pin as output."]
    SET,
}
impl PIN10W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            PIN10W::SET => true,
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
    #[doc = "Set pin as output."]
    #[inline]
    pub fn set(self) -> &'a mut W {
        self.variant(PIN10W::SET)
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
    #[doc = "Set pin as output."]
    SET,
}
impl PIN11W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            PIN11W::SET => true,
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
    #[doc = "Set pin as output."]
    #[inline]
    pub fn set(self) -> &'a mut W {
        self.variant(PIN11W::SET)
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
    #[doc = "Set pin as output."]
    SET,
}
impl PIN12W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            PIN12W::SET => true,
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
    #[doc = "Set pin as output."]
    #[inline]
    pub fn set(self) -> &'a mut W {
        self.variant(PIN12W::SET)
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
    #[doc = "Set pin as output."]
    SET,
}
impl PIN13W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            PIN13W::SET => true,
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
    #[doc = "Set pin as output."]
    #[inline]
    pub fn set(self) -> &'a mut W {
        self.variant(PIN13W::SET)
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
    #[doc = "Set pin as output."]
    SET,
}
impl PIN14W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            PIN14W::SET => true,
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
    #[doc = "Set pin as output."]
    #[inline]
    pub fn set(self) -> &'a mut W {
        self.variant(PIN14W::SET)
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
    #[doc = "Set pin as output."]
    SET,
}
impl PIN15W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            PIN15W::SET => true,
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
    #[doc = "Set pin as output."]
    #[inline]
    pub fn set(self) -> &'a mut W {
        self.variant(PIN15W::SET)
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
    #[doc = "Set pin as output."]
    SET,
}
impl PIN16W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            PIN16W::SET => true,
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
    #[doc = "Set pin as output."]
    #[inline]
    pub fn set(self) -> &'a mut W {
        self.variant(PIN16W::SET)
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
    #[doc = "Set pin as output."]
    SET,
}
impl PIN17W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            PIN17W::SET => true,
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
    #[doc = "Set pin as output."]
    #[inline]
    pub fn set(self) -> &'a mut W {
        self.variant(PIN17W::SET)
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
    #[doc = "Set pin as output."]
    SET,
}
impl PIN18W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            PIN18W::SET => true,
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
    #[doc = "Set pin as output."]
    #[inline]
    pub fn set(self) -> &'a mut W {
        self.variant(PIN18W::SET)
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
    #[doc = "Set pin as output."]
    SET,
}
impl PIN19W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            PIN19W::SET => true,
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
    #[doc = "Set pin as output."]
    #[inline]
    pub fn set(self) -> &'a mut W {
        self.variant(PIN19W::SET)
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
    #[doc = "Set pin as output."]
    SET,
}
impl PIN20W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            PIN20W::SET => true,
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
    #[doc = "Set pin as output."]
    #[inline]
    pub fn set(self) -> &'a mut W {
        self.variant(PIN20W::SET)
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
    #[doc = "Set pin as output."]
    SET,
}
impl PIN21W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            PIN21W::SET => true,
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
    #[doc = "Set pin as output."]
    #[inline]
    pub fn set(self) -> &'a mut W {
        self.variant(PIN21W::SET)
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
    #[doc = "Set pin as output."]
    SET,
}
impl PIN22W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            PIN22W::SET => true,
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
    #[doc = "Set pin as output."]
    #[inline]
    pub fn set(self) -> &'a mut W {
        self.variant(PIN22W::SET)
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
    #[doc = "Set pin as output."]
    SET,
}
impl PIN23W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            PIN23W::SET => true,
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
    #[doc = "Set pin as output."]
    #[inline]
    pub fn set(self) -> &'a mut W {
        self.variant(PIN23W::SET)
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
    #[doc = "Set pin as output."]
    SET,
}
impl PIN24W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            PIN24W::SET => true,
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
    #[doc = "Set pin as output."]
    #[inline]
    pub fn set(self) -> &'a mut W {
        self.variant(PIN24W::SET)
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
    #[doc = "Set pin as output."]
    SET,
}
impl PIN25W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            PIN25W::SET => true,
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
    #[doc = "Set pin as output."]
    #[inline]
    pub fn set(self) -> &'a mut W {
        self.variant(PIN25W::SET)
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
    #[doc = "Set pin as output."]
    SET,
}
impl PIN26W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            PIN26W::SET => true,
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
    #[doc = "Set pin as output."]
    #[inline]
    pub fn set(self) -> &'a mut W {
        self.variant(PIN26W::SET)
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
    #[doc = "Set pin as output."]
    SET,
}
impl PIN27W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            PIN27W::SET => true,
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
    #[doc = "Set pin as output."]
    #[inline]
    pub fn set(self) -> &'a mut W {
        self.variant(PIN27W::SET)
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
    #[doc = "Set pin as output."]
    SET,
}
impl PIN28W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            PIN28W::SET => true,
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
    #[doc = "Set pin as output."]
    #[inline]
    pub fn set(self) -> &'a mut W {
        self.variant(PIN28W::SET)
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
    #[doc = "Set pin as output."]
    SET,
}
impl PIN29W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            PIN29W::SET => true,
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
    #[doc = "Set pin as output."]
    #[inline]
    pub fn set(self) -> &'a mut W {
        self.variant(PIN29W::SET)
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
    #[doc = "Set pin as output."]
    SET,
}
impl PIN30W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            PIN30W::SET => true,
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
    #[doc = "Set pin as output."]
    #[inline]
    pub fn set(self) -> &'a mut W {
        self.variant(PIN30W::SET)
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
    #[doc = "Set pin as output."]
    SET,
}
impl PIN31W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            PIN31W::SET => true,
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
    #[doc = "Set pin as output."]
    #[inline]
    pub fn set(self) -> &'a mut W {
        self.variant(PIN31W::SET)
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
    #[doc = "Bit 0 - Set as output pin 0."]
    #[inline]
    pub fn pin0(&self) -> PIN0R {
        PIN0R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 1 - Set as output pin 1."]
    #[inline]
    pub fn pin1(&self) -> PIN1R {
        PIN1R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 1;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 2 - Set as output pin 2."]
    #[inline]
    pub fn pin2(&self) -> PIN2R {
        PIN2R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 2;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 3 - Set as output pin 3."]
    #[inline]
    pub fn pin3(&self) -> PIN3R {
        PIN3R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 3;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 4 - Set as output pin 4."]
    #[inline]
    pub fn pin4(&self) -> PIN4R {
        PIN4R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 4;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 5 - Set as output pin 5."]
    #[inline]
    pub fn pin5(&self) -> PIN5R {
        PIN5R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 5;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 6 - Set as output pin 6."]
    #[inline]
    pub fn pin6(&self) -> PIN6R {
        PIN6R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 6;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 7 - Set as output pin 7."]
    #[inline]
    pub fn pin7(&self) -> PIN7R {
        PIN7R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 7;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 8 - Set as output pin 8."]
    #[inline]
    pub fn pin8(&self) -> PIN8R {
        PIN8R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 8;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 9 - Set as output pin 9."]
    #[inline]
    pub fn pin9(&self) -> PIN9R {
        PIN9R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 9;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 10 - Set as output pin 10."]
    #[inline]
    pub fn pin10(&self) -> PIN10R {
        PIN10R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 10;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 11 - Set as output pin 11."]
    #[inline]
    pub fn pin11(&self) -> PIN11R {
        PIN11R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 11;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 12 - Set as output pin 12."]
    #[inline]
    pub fn pin12(&self) -> PIN12R {
        PIN12R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 12;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 13 - Set as output pin 13."]
    #[inline]
    pub fn pin13(&self) -> PIN13R {
        PIN13R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 13;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 14 - Set as output pin 14."]
    #[inline]
    pub fn pin14(&self) -> PIN14R {
        PIN14R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 14;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 15 - Set as output pin 15."]
    #[inline]
    pub fn pin15(&self) -> PIN15R {
        PIN15R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 15;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 16 - Set as output pin 16."]
    #[inline]
    pub fn pin16(&self) -> PIN16R {
        PIN16R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 16;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 17 - Set as output pin 17."]
    #[inline]
    pub fn pin17(&self) -> PIN17R {
        PIN17R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 17;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 18 - Set as output pin 18."]
    #[inline]
    pub fn pin18(&self) -> PIN18R {
        PIN18R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 18;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 19 - Set as output pin 19."]
    #[inline]
    pub fn pin19(&self) -> PIN19R {
        PIN19R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 19;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 20 - Set as output pin 20."]
    #[inline]
    pub fn pin20(&self) -> PIN20R {
        PIN20R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 20;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 21 - Set as output pin 21."]
    #[inline]
    pub fn pin21(&self) -> PIN21R {
        PIN21R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 21;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 22 - Set as output pin 22."]
    #[inline]
    pub fn pin22(&self) -> PIN22R {
        PIN22R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 22;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 23 - Set as output pin 23."]
    #[inline]
    pub fn pin23(&self) -> PIN23R {
        PIN23R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 23;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 24 - Set as output pin 24."]
    #[inline]
    pub fn pin24(&self) -> PIN24R {
        PIN24R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 24;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 25 - Set as output pin 25."]
    #[inline]
    pub fn pin25(&self) -> PIN25R {
        PIN25R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 25;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 26 - Set as output pin 26."]
    #[inline]
    pub fn pin26(&self) -> PIN26R {
        PIN26R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 26;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 27 - Set as output pin 27."]
    #[inline]
    pub fn pin27(&self) -> PIN27R {
        PIN27R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 27;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 28 - Set as output pin 28."]
    #[inline]
    pub fn pin28(&self) -> PIN28R {
        PIN28R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 28;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 29 - Set as output pin 29."]
    #[inline]
    pub fn pin29(&self) -> PIN29R {
        PIN29R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 29;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 30 - Set as output pin 30."]
    #[inline]
    pub fn pin30(&self) -> PIN30R {
        PIN30R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 30;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 31 - Set as output pin 31."]
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
    #[doc = "Bit 0 - Set as output pin 0."]
    #[inline]
    pub fn pin0(&mut self) -> _PIN0W {
        _PIN0W { w: self }
    }
    #[doc = "Bit 1 - Set as output pin 1."]
    #[inline]
    pub fn pin1(&mut self) -> _PIN1W {
        _PIN1W { w: self }
    }
    #[doc = "Bit 2 - Set as output pin 2."]
    #[inline]
    pub fn pin2(&mut self) -> _PIN2W {
        _PIN2W { w: self }
    }
    #[doc = "Bit 3 - Set as output pin 3."]
    #[inline]
    pub fn pin3(&mut self) -> _PIN3W {
        _PIN3W { w: self }
    }
    #[doc = "Bit 4 - Set as output pin 4."]
    #[inline]
    pub fn pin4(&mut self) -> _PIN4W {
        _PIN4W { w: self }
    }
    #[doc = "Bit 5 - Set as output pin 5."]
    #[inline]
    pub fn pin5(&mut self) -> _PIN5W {
        _PIN5W { w: self }
    }
    #[doc = "Bit 6 - Set as output pin 6."]
    #[inline]
    pub fn pin6(&mut self) -> _PIN6W {
        _PIN6W { w: self }
    }
    #[doc = "Bit 7 - Set as output pin 7."]
    #[inline]
    pub fn pin7(&mut self) -> _PIN7W {
        _PIN7W { w: self }
    }
    #[doc = "Bit 8 - Set as output pin 8."]
    #[inline]
    pub fn pin8(&mut self) -> _PIN8W {
        _PIN8W { w: self }
    }
    #[doc = "Bit 9 - Set as output pin 9."]
    #[inline]
    pub fn pin9(&mut self) -> _PIN9W {
        _PIN9W { w: self }
    }
    #[doc = "Bit 10 - Set as output pin 10."]
    #[inline]
    pub fn pin10(&mut self) -> _PIN10W {
        _PIN10W { w: self }
    }
    #[doc = "Bit 11 - Set as output pin 11."]
    #[inline]
    pub fn pin11(&mut self) -> _PIN11W {
        _PIN11W { w: self }
    }
    #[doc = "Bit 12 - Set as output pin 12."]
    #[inline]
    pub fn pin12(&mut self) -> _PIN12W {
        _PIN12W { w: self }
    }
    #[doc = "Bit 13 - Set as output pin 13."]
    #[inline]
    pub fn pin13(&mut self) -> _PIN13W {
        _PIN13W { w: self }
    }
    #[doc = "Bit 14 - Set as output pin 14."]
    #[inline]
    pub fn pin14(&mut self) -> _PIN14W {
        _PIN14W { w: self }
    }
    #[doc = "Bit 15 - Set as output pin 15."]
    #[inline]
    pub fn pin15(&mut self) -> _PIN15W {
        _PIN15W { w: self }
    }
    #[doc = "Bit 16 - Set as output pin 16."]
    #[inline]
    pub fn pin16(&mut self) -> _PIN16W {
        _PIN16W { w: self }
    }
    #[doc = "Bit 17 - Set as output pin 17."]
    #[inline]
    pub fn pin17(&mut self) -> _PIN17W {
        _PIN17W { w: self }
    }
    #[doc = "Bit 18 - Set as output pin 18."]
    #[inline]
    pub fn pin18(&mut self) -> _PIN18W {
        _PIN18W { w: self }
    }
    #[doc = "Bit 19 - Set as output pin 19."]
    #[inline]
    pub fn pin19(&mut self) -> _PIN19W {
        _PIN19W { w: self }
    }
    #[doc = "Bit 20 - Set as output pin 20."]
    #[inline]
    pub fn pin20(&mut self) -> _PIN20W {
        _PIN20W { w: self }
    }
    #[doc = "Bit 21 - Set as output pin 21."]
    #[inline]
    pub fn pin21(&mut self) -> _PIN21W {
        _PIN21W { w: self }
    }
    #[doc = "Bit 22 - Set as output pin 22."]
    #[inline]
    pub fn pin22(&mut self) -> _PIN22W {
        _PIN22W { w: self }
    }
    #[doc = "Bit 23 - Set as output pin 23."]
    #[inline]
    pub fn pin23(&mut self) -> _PIN23W {
        _PIN23W { w: self }
    }
    #[doc = "Bit 24 - Set as output pin 24."]
    #[inline]
    pub fn pin24(&mut self) -> _PIN24W {
        _PIN24W { w: self }
    }
    #[doc = "Bit 25 - Set as output pin 25."]
    #[inline]
    pub fn pin25(&mut self) -> _PIN25W {
        _PIN25W { w: self }
    }
    #[doc = "Bit 26 - Set as output pin 26."]
    #[inline]
    pub fn pin26(&mut self) -> _PIN26W {
        _PIN26W { w: self }
    }
    #[doc = "Bit 27 - Set as output pin 27."]
    #[inline]
    pub fn pin27(&mut self) -> _PIN27W {
        _PIN27W { w: self }
    }
    #[doc = "Bit 28 - Set as output pin 28."]
    #[inline]
    pub fn pin28(&mut self) -> _PIN28W {
        _PIN28W { w: self }
    }
    #[doc = "Bit 29 - Set as output pin 29."]
    #[inline]
    pub fn pin29(&mut self) -> _PIN29W {
        _PIN29W { w: self }
    }
    #[doc = "Bit 30 - Set as output pin 30."]
    #[inline]
    pub fn pin30(&mut self) -> _PIN30W {
        _PIN30W { w: self }
    }
    #[doc = "Bit 31 - Set as output pin 31."]
    #[inline]
    pub fn pin31(&mut self) -> _PIN31W {
        _PIN31W { w: self }
    }
}
