#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
impl super::RAMSTATUS {
    #[doc = r" Reads the contents of the register"]
    #[inline]
    pub fn read(&self) -> R {
        R {
            bits: self.register.get(),
        }
    }
}
#[doc = "Possible values of the field `RAMBLOCK0`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RAMBLOCK0R {
    #[doc = "Off"]
    OFF,
    #[doc = "On"]
    ON,
}
impl RAMBLOCK0R {
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
            RAMBLOCK0R::OFF => false,
            RAMBLOCK0R::ON => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> RAMBLOCK0R {
        match value {
            false => RAMBLOCK0R::OFF,
            true => RAMBLOCK0R::ON,
        }
    }
    #[doc = "Checks if the value of the field is `OFF`"]
    #[inline]
    pub fn is_off(&self) -> bool {
        *self == RAMBLOCK0R::OFF
    }
    #[doc = "Checks if the value of the field is `ON`"]
    #[inline]
    pub fn is_on(&self) -> bool {
        *self == RAMBLOCK0R::ON
    }
}
#[doc = "Possible values of the field `RAMBLOCK1`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RAMBLOCK1R {
    #[doc = "Off"]
    OFF,
    #[doc = "On"]
    ON,
}
impl RAMBLOCK1R {
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
            RAMBLOCK1R::OFF => false,
            RAMBLOCK1R::ON => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> RAMBLOCK1R {
        match value {
            false => RAMBLOCK1R::OFF,
            true => RAMBLOCK1R::ON,
        }
    }
    #[doc = "Checks if the value of the field is `OFF`"]
    #[inline]
    pub fn is_off(&self) -> bool {
        *self == RAMBLOCK1R::OFF
    }
    #[doc = "Checks if the value of the field is `ON`"]
    #[inline]
    pub fn is_on(&self) -> bool {
        *self == RAMBLOCK1R::ON
    }
}
#[doc = "Possible values of the field `RAMBLOCK2`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RAMBLOCK2R {
    #[doc = "Off"]
    OFF,
    #[doc = "On"]
    ON,
}
impl RAMBLOCK2R {
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
            RAMBLOCK2R::OFF => false,
            RAMBLOCK2R::ON => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> RAMBLOCK2R {
        match value {
            false => RAMBLOCK2R::OFF,
            true => RAMBLOCK2R::ON,
        }
    }
    #[doc = "Checks if the value of the field is `OFF`"]
    #[inline]
    pub fn is_off(&self) -> bool {
        *self == RAMBLOCK2R::OFF
    }
    #[doc = "Checks if the value of the field is `ON`"]
    #[inline]
    pub fn is_on(&self) -> bool {
        *self == RAMBLOCK2R::ON
    }
}
#[doc = "Possible values of the field `RAMBLOCK3`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RAMBLOCK3R {
    #[doc = "Off"]
    OFF,
    #[doc = "On"]
    ON,
}
impl RAMBLOCK3R {
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
            RAMBLOCK3R::OFF => false,
            RAMBLOCK3R::ON => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> RAMBLOCK3R {
        match value {
            false => RAMBLOCK3R::OFF,
            true => RAMBLOCK3R::ON,
        }
    }
    #[doc = "Checks if the value of the field is `OFF`"]
    #[inline]
    pub fn is_off(&self) -> bool {
        *self == RAMBLOCK3R::OFF
    }
    #[doc = "Checks if the value of the field is `ON`"]
    #[inline]
    pub fn is_on(&self) -> bool {
        *self == RAMBLOCK3R::ON
    }
}
impl R {
    #[doc = r" Value of the register as raw bits"]
    #[inline]
    pub fn bits(&self) -> u32 {
        self.bits
    }
    #[doc = "Bit 0 - RAM block 0 is on or off/powering up"]
    #[inline]
    pub fn ramblock0(&self) -> RAMBLOCK0R {
        RAMBLOCK0R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 1 - RAM block 1 is on or off/powering up"]
    #[inline]
    pub fn ramblock1(&self) -> RAMBLOCK1R {
        RAMBLOCK1R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 1;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 2 - RAM block 2 is on or off/powering up"]
    #[inline]
    pub fn ramblock2(&self) -> RAMBLOCK2R {
        RAMBLOCK2R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 2;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 3 - RAM block 3 is on or off/powering up"]
    #[inline]
    pub fn ramblock3(&self) -> RAMBLOCK3R {
        RAMBLOCK3R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 3;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
}
