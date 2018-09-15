#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
impl super::REQSTATUS {
    #[doc = r" Reads the contents of the register"]
    #[inline]
    pub fn read(&self) -> R {
        R {
            bits: self.register.get(),
        }
    }
}
#[doc = "Possible values of the field `RR0`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RR0R {
    #[doc = "RR[0] register is not enabled or has already requested reload."]
    DISABLEDORREQUESTED,
    #[doc = "RR[0] register is enabled and has not jet requested."]
    ENABLEDANDUNREQUESTED,
}
impl RR0R {
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
            RR0R::DISABLEDORREQUESTED => false,
            RR0R::ENABLEDANDUNREQUESTED => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> RR0R {
        match value {
            false => RR0R::DISABLEDORREQUESTED,
            true => RR0R::ENABLEDANDUNREQUESTED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLEDORREQUESTED`"]
    #[inline]
    pub fn is_disabled_or_requested(&self) -> bool {
        *self == RR0R::DISABLEDORREQUESTED
    }
    #[doc = "Checks if the value of the field is `ENABLEDANDUNREQUESTED`"]
    #[inline]
    pub fn is_enabled_and_unrequested(&self) -> bool {
        *self == RR0R::ENABLEDANDUNREQUESTED
    }
}
#[doc = "Possible values of the field `RR1`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RR1R {
    #[doc = "RR[1] register is not enabled or has already requested reload."]
    DISABLEDORREQUESTED,
    #[doc = "RR[1] register is enabled and has not jet requested."]
    ENABLEDANDUNREQUESTED,
}
impl RR1R {
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
            RR1R::DISABLEDORREQUESTED => false,
            RR1R::ENABLEDANDUNREQUESTED => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> RR1R {
        match value {
            false => RR1R::DISABLEDORREQUESTED,
            true => RR1R::ENABLEDANDUNREQUESTED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLEDORREQUESTED`"]
    #[inline]
    pub fn is_disabled_or_requested(&self) -> bool {
        *self == RR1R::DISABLEDORREQUESTED
    }
    #[doc = "Checks if the value of the field is `ENABLEDANDUNREQUESTED`"]
    #[inline]
    pub fn is_enabled_and_unrequested(&self) -> bool {
        *self == RR1R::ENABLEDANDUNREQUESTED
    }
}
#[doc = "Possible values of the field `RR2`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RR2R {
    #[doc = "RR[2] register is not enabled or has already requested reload."]
    DISABLEDORREQUESTED,
    #[doc = "RR[2] register is enabled and has not jet requested."]
    ENABLEDANDUNREQUESTED,
}
impl RR2R {
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
            RR2R::DISABLEDORREQUESTED => false,
            RR2R::ENABLEDANDUNREQUESTED => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> RR2R {
        match value {
            false => RR2R::DISABLEDORREQUESTED,
            true => RR2R::ENABLEDANDUNREQUESTED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLEDORREQUESTED`"]
    #[inline]
    pub fn is_disabled_or_requested(&self) -> bool {
        *self == RR2R::DISABLEDORREQUESTED
    }
    #[doc = "Checks if the value of the field is `ENABLEDANDUNREQUESTED`"]
    #[inline]
    pub fn is_enabled_and_unrequested(&self) -> bool {
        *self == RR2R::ENABLEDANDUNREQUESTED
    }
}
#[doc = "Possible values of the field `RR3`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RR3R {
    #[doc = "RR[3] register is not enabled or has already requested reload."]
    DISABLEDORREQUESTED,
    #[doc = "RR[3] register is enabled and has not jet requested."]
    ENABLEDANDUNREQUESTED,
}
impl RR3R {
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
            RR3R::DISABLEDORREQUESTED => false,
            RR3R::ENABLEDANDUNREQUESTED => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> RR3R {
        match value {
            false => RR3R::DISABLEDORREQUESTED,
            true => RR3R::ENABLEDANDUNREQUESTED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLEDORREQUESTED`"]
    #[inline]
    pub fn is_disabled_or_requested(&self) -> bool {
        *self == RR3R::DISABLEDORREQUESTED
    }
    #[doc = "Checks if the value of the field is `ENABLEDANDUNREQUESTED`"]
    #[inline]
    pub fn is_enabled_and_unrequested(&self) -> bool {
        *self == RR3R::ENABLEDANDUNREQUESTED
    }
}
#[doc = "Possible values of the field `RR4`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RR4R {
    #[doc = "RR[4] register is not enabled or has already requested reload."]
    DISABLEDORREQUESTED,
    #[doc = "RR[4] register is enabled and has not jet requested."]
    ENABLEDANDUNREQUESTED,
}
impl RR4R {
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
            RR4R::DISABLEDORREQUESTED => false,
            RR4R::ENABLEDANDUNREQUESTED => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> RR4R {
        match value {
            false => RR4R::DISABLEDORREQUESTED,
            true => RR4R::ENABLEDANDUNREQUESTED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLEDORREQUESTED`"]
    #[inline]
    pub fn is_disabled_or_requested(&self) -> bool {
        *self == RR4R::DISABLEDORREQUESTED
    }
    #[doc = "Checks if the value of the field is `ENABLEDANDUNREQUESTED`"]
    #[inline]
    pub fn is_enabled_and_unrequested(&self) -> bool {
        *self == RR4R::ENABLEDANDUNREQUESTED
    }
}
#[doc = "Possible values of the field `RR5`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RR5R {
    #[doc = "RR[5] register is not enabled or has already requested reload."]
    DISABLEDORREQUESTED,
    #[doc = "RR[5] register is enabled and has not jet requested."]
    ENABLEDANDUNREQUESTED,
}
impl RR5R {
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
            RR5R::DISABLEDORREQUESTED => false,
            RR5R::ENABLEDANDUNREQUESTED => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> RR5R {
        match value {
            false => RR5R::DISABLEDORREQUESTED,
            true => RR5R::ENABLEDANDUNREQUESTED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLEDORREQUESTED`"]
    #[inline]
    pub fn is_disabled_or_requested(&self) -> bool {
        *self == RR5R::DISABLEDORREQUESTED
    }
    #[doc = "Checks if the value of the field is `ENABLEDANDUNREQUESTED`"]
    #[inline]
    pub fn is_enabled_and_unrequested(&self) -> bool {
        *self == RR5R::ENABLEDANDUNREQUESTED
    }
}
#[doc = "Possible values of the field `RR6`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RR6R {
    #[doc = "RR[6] register is not enabled or has already requested reload."]
    DISABLEDORREQUESTED,
    #[doc = "RR[6] register is enabled and has not jet requested."]
    ENABLEDANDUNREQUESTED,
}
impl RR6R {
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
            RR6R::DISABLEDORREQUESTED => false,
            RR6R::ENABLEDANDUNREQUESTED => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> RR6R {
        match value {
            false => RR6R::DISABLEDORREQUESTED,
            true => RR6R::ENABLEDANDUNREQUESTED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLEDORREQUESTED`"]
    #[inline]
    pub fn is_disabled_or_requested(&self) -> bool {
        *self == RR6R::DISABLEDORREQUESTED
    }
    #[doc = "Checks if the value of the field is `ENABLEDANDUNREQUESTED`"]
    #[inline]
    pub fn is_enabled_and_unrequested(&self) -> bool {
        *self == RR6R::ENABLEDANDUNREQUESTED
    }
}
#[doc = "Possible values of the field `RR7`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RR7R {
    #[doc = "RR[7] register is not enabled or has already requested reload."]
    DISABLEDORREQUESTED,
    #[doc = "RR[7] register is enabled and has not jet requested."]
    ENABLEDANDUNREQUESTED,
}
impl RR7R {
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
            RR7R::DISABLEDORREQUESTED => false,
            RR7R::ENABLEDANDUNREQUESTED => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> RR7R {
        match value {
            false => RR7R::DISABLEDORREQUESTED,
            true => RR7R::ENABLEDANDUNREQUESTED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLEDORREQUESTED`"]
    #[inline]
    pub fn is_disabled_or_requested(&self) -> bool {
        *self == RR7R::DISABLEDORREQUESTED
    }
    #[doc = "Checks if the value of the field is `ENABLEDANDUNREQUESTED`"]
    #[inline]
    pub fn is_enabled_and_unrequested(&self) -> bool {
        *self == RR7R::ENABLEDANDUNREQUESTED
    }
}
impl R {
    #[doc = r" Value of the register as raw bits"]
    #[inline]
    pub fn bits(&self) -> u32 {
        self.bits
    }
    #[doc = "Bit 0 - Request status for RR[0]."]
    #[inline]
    pub fn rr0(&self) -> RR0R {
        RR0R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 1 - Request status for RR[1]."]
    #[inline]
    pub fn rr1(&self) -> RR1R {
        RR1R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 1;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 2 - Request status for RR[2]."]
    #[inline]
    pub fn rr2(&self) -> RR2R {
        RR2R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 2;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 3 - Request status for RR[3]."]
    #[inline]
    pub fn rr3(&self) -> RR3R {
        RR3R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 3;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 4 - Request status for RR[4]."]
    #[inline]
    pub fn rr4(&self) -> RR4R {
        RR4R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 4;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 5 - Request status for RR[5]."]
    #[inline]
    pub fn rr5(&self) -> RR5R {
        RR5R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 5;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 6 - Request status for RR[6]."]
    #[inline]
    pub fn rr6(&self) -> RR6R {
        RR6R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 6;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 7 - Request status for RR[7]."]
    #[inline]
    pub fn rr7(&self) -> RR7R {
        RR7R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 7;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
}
