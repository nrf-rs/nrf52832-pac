#[doc = "Reader of register REQSTATUS"]
pub type R = crate::R<u32, super::REQSTATUS>;
#[doc = "Request status for RR\\[0\\]
register\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RR0_A {
    #[doc = "0: RR\\[0\\]
register is not enabled, or are already requesting reload"]
    DISABLEDORREQUESTED = 0,
    #[doc = "1: RR\\[0\\]
register is enabled, and are not yet requesting reload"]
    ENABLEDANDUNREQUESTED = 1,
}
impl From<RR0_A> for bool {
    #[inline(always)]
    fn from(variant: RR0_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `RR0`"]
pub type RR0_R = crate::R<bool, RR0_A>;
impl RR0_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RR0_A {
        match self.bits {
            false => RR0_A::DISABLEDORREQUESTED,
            true => RR0_A::ENABLEDANDUNREQUESTED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLEDORREQUESTED`"]
    #[inline(always)]
    pub fn is_disabled_or_requested(&self) -> bool {
        *self == RR0_A::DISABLEDORREQUESTED
    }
    #[doc = "Checks if the value of the field is `ENABLEDANDUNREQUESTED`"]
    #[inline(always)]
    pub fn is_enabled_and_unrequested(&self) -> bool {
        *self == RR0_A::ENABLEDANDUNREQUESTED
    }
}
#[doc = "Request status for RR\\[1\\]
register\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RR1_A {
    #[doc = "0: RR\\[1\\]
register is not enabled, or are already requesting reload"]
    DISABLEDORREQUESTED = 0,
    #[doc = "1: RR\\[1\\]
register is enabled, and are not yet requesting reload"]
    ENABLEDANDUNREQUESTED = 1,
}
impl From<RR1_A> for bool {
    #[inline(always)]
    fn from(variant: RR1_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `RR1`"]
pub type RR1_R = crate::R<bool, RR1_A>;
impl RR1_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RR1_A {
        match self.bits {
            false => RR1_A::DISABLEDORREQUESTED,
            true => RR1_A::ENABLEDANDUNREQUESTED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLEDORREQUESTED`"]
    #[inline(always)]
    pub fn is_disabled_or_requested(&self) -> bool {
        *self == RR1_A::DISABLEDORREQUESTED
    }
    #[doc = "Checks if the value of the field is `ENABLEDANDUNREQUESTED`"]
    #[inline(always)]
    pub fn is_enabled_and_unrequested(&self) -> bool {
        *self == RR1_A::ENABLEDANDUNREQUESTED
    }
}
#[doc = "Request status for RR\\[2\\]
register\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RR2_A {
    #[doc = "0: RR\\[2\\]
register is not enabled, or are already requesting reload"]
    DISABLEDORREQUESTED = 0,
    #[doc = "1: RR\\[2\\]
register is enabled, and are not yet requesting reload"]
    ENABLEDANDUNREQUESTED = 1,
}
impl From<RR2_A> for bool {
    #[inline(always)]
    fn from(variant: RR2_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `RR2`"]
pub type RR2_R = crate::R<bool, RR2_A>;
impl RR2_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RR2_A {
        match self.bits {
            false => RR2_A::DISABLEDORREQUESTED,
            true => RR2_A::ENABLEDANDUNREQUESTED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLEDORREQUESTED`"]
    #[inline(always)]
    pub fn is_disabled_or_requested(&self) -> bool {
        *self == RR2_A::DISABLEDORREQUESTED
    }
    #[doc = "Checks if the value of the field is `ENABLEDANDUNREQUESTED`"]
    #[inline(always)]
    pub fn is_enabled_and_unrequested(&self) -> bool {
        *self == RR2_A::ENABLEDANDUNREQUESTED
    }
}
#[doc = "Request status for RR\\[3\\]
register\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RR3_A {
    #[doc = "0: RR\\[3\\]
register is not enabled, or are already requesting reload"]
    DISABLEDORREQUESTED = 0,
    #[doc = "1: RR\\[3\\]
register is enabled, and are not yet requesting reload"]
    ENABLEDANDUNREQUESTED = 1,
}
impl From<RR3_A> for bool {
    #[inline(always)]
    fn from(variant: RR3_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `RR3`"]
pub type RR3_R = crate::R<bool, RR3_A>;
impl RR3_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RR3_A {
        match self.bits {
            false => RR3_A::DISABLEDORREQUESTED,
            true => RR3_A::ENABLEDANDUNREQUESTED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLEDORREQUESTED`"]
    #[inline(always)]
    pub fn is_disabled_or_requested(&self) -> bool {
        *self == RR3_A::DISABLEDORREQUESTED
    }
    #[doc = "Checks if the value of the field is `ENABLEDANDUNREQUESTED`"]
    #[inline(always)]
    pub fn is_enabled_and_unrequested(&self) -> bool {
        *self == RR3_A::ENABLEDANDUNREQUESTED
    }
}
#[doc = "Request status for RR\\[4\\]
register\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RR4_A {
    #[doc = "0: RR\\[4\\]
register is not enabled, or are already requesting reload"]
    DISABLEDORREQUESTED = 0,
    #[doc = "1: RR\\[4\\]
register is enabled, and are not yet requesting reload"]
    ENABLEDANDUNREQUESTED = 1,
}
impl From<RR4_A> for bool {
    #[inline(always)]
    fn from(variant: RR4_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `RR4`"]
pub type RR4_R = crate::R<bool, RR4_A>;
impl RR4_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RR4_A {
        match self.bits {
            false => RR4_A::DISABLEDORREQUESTED,
            true => RR4_A::ENABLEDANDUNREQUESTED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLEDORREQUESTED`"]
    #[inline(always)]
    pub fn is_disabled_or_requested(&self) -> bool {
        *self == RR4_A::DISABLEDORREQUESTED
    }
    #[doc = "Checks if the value of the field is `ENABLEDANDUNREQUESTED`"]
    #[inline(always)]
    pub fn is_enabled_and_unrequested(&self) -> bool {
        *self == RR4_A::ENABLEDANDUNREQUESTED
    }
}
#[doc = "Request status for RR\\[5\\]
register\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RR5_A {
    #[doc = "0: RR\\[5\\]
register is not enabled, or are already requesting reload"]
    DISABLEDORREQUESTED = 0,
    #[doc = "1: RR\\[5\\]
register is enabled, and are not yet requesting reload"]
    ENABLEDANDUNREQUESTED = 1,
}
impl From<RR5_A> for bool {
    #[inline(always)]
    fn from(variant: RR5_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `RR5`"]
pub type RR5_R = crate::R<bool, RR5_A>;
impl RR5_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RR5_A {
        match self.bits {
            false => RR5_A::DISABLEDORREQUESTED,
            true => RR5_A::ENABLEDANDUNREQUESTED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLEDORREQUESTED`"]
    #[inline(always)]
    pub fn is_disabled_or_requested(&self) -> bool {
        *self == RR5_A::DISABLEDORREQUESTED
    }
    #[doc = "Checks if the value of the field is `ENABLEDANDUNREQUESTED`"]
    #[inline(always)]
    pub fn is_enabled_and_unrequested(&self) -> bool {
        *self == RR5_A::ENABLEDANDUNREQUESTED
    }
}
#[doc = "Request status for RR\\[6\\]
register\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RR6_A {
    #[doc = "0: RR\\[6\\]
register is not enabled, or are already requesting reload"]
    DISABLEDORREQUESTED = 0,
    #[doc = "1: RR\\[6\\]
register is enabled, and are not yet requesting reload"]
    ENABLEDANDUNREQUESTED = 1,
}
impl From<RR6_A> for bool {
    #[inline(always)]
    fn from(variant: RR6_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `RR6`"]
pub type RR6_R = crate::R<bool, RR6_A>;
impl RR6_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RR6_A {
        match self.bits {
            false => RR6_A::DISABLEDORREQUESTED,
            true => RR6_A::ENABLEDANDUNREQUESTED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLEDORREQUESTED`"]
    #[inline(always)]
    pub fn is_disabled_or_requested(&self) -> bool {
        *self == RR6_A::DISABLEDORREQUESTED
    }
    #[doc = "Checks if the value of the field is `ENABLEDANDUNREQUESTED`"]
    #[inline(always)]
    pub fn is_enabled_and_unrequested(&self) -> bool {
        *self == RR6_A::ENABLEDANDUNREQUESTED
    }
}
#[doc = "Request status for RR\\[7\\]
register\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RR7_A {
    #[doc = "0: RR\\[7\\]
register is not enabled, or are already requesting reload"]
    DISABLEDORREQUESTED = 0,
    #[doc = "1: RR\\[7\\]
register is enabled, and are not yet requesting reload"]
    ENABLEDANDUNREQUESTED = 1,
}
impl From<RR7_A> for bool {
    #[inline(always)]
    fn from(variant: RR7_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `RR7`"]
pub type RR7_R = crate::R<bool, RR7_A>;
impl RR7_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RR7_A {
        match self.bits {
            false => RR7_A::DISABLEDORREQUESTED,
            true => RR7_A::ENABLEDANDUNREQUESTED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLEDORREQUESTED`"]
    #[inline(always)]
    pub fn is_disabled_or_requested(&self) -> bool {
        *self == RR7_A::DISABLEDORREQUESTED
    }
    #[doc = "Checks if the value of the field is `ENABLEDANDUNREQUESTED`"]
    #[inline(always)]
    pub fn is_enabled_and_unrequested(&self) -> bool {
        *self == RR7_A::ENABLEDANDUNREQUESTED
    }
}
impl R {
    #[doc = "Bit 0 - Request status for RR\\[0\\]
register"]
    #[inline(always)]
    pub fn rr0(&self) -> RR0_R {
        RR0_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - Request status for RR\\[1\\]
register"]
    #[inline(always)]
    pub fn rr1(&self) -> RR1_R {
        RR1_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - Request status for RR\\[2\\]
register"]
    #[inline(always)]
    pub fn rr2(&self) -> RR2_R {
        RR2_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - Request status for RR\\[3\\]
register"]
    #[inline(always)]
    pub fn rr3(&self) -> RR3_R {
        RR3_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - Request status for RR\\[4\\]
register"]
    #[inline(always)]
    pub fn rr4(&self) -> RR4_R {
        RR4_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - Request status for RR\\[5\\]
register"]
    #[inline(always)]
    pub fn rr5(&self) -> RR5_R {
        RR5_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - Request status for RR\\[6\\]
register"]
    #[inline(always)]
    pub fn rr6(&self) -> RR6_R {
        RR6_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7 - Request status for RR\\[7\\]
register"]
    #[inline(always)]
    pub fn rr7(&self) -> RR7_R {
        RR7_R::new(((self.bits >> 7) & 0x01) != 0)
    }
}
