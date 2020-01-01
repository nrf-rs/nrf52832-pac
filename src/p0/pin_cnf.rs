#[doc = "Reader of register PIN_CNF[%s]"]
pub type R = crate::R<u32, super::PIN_CNF>;
#[doc = "Writer for register PIN_CNF[%s]"]
pub type W = crate::W<u32, super::PIN_CNF>;
#[doc = "Register PIN_CNF[%s]
`reset()`'s with value 0x02"]
impl crate::ResetValue for super::PIN_CNF {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0x02
    }
}
#[doc = "Pin direction. Same physical register as DIR register\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DIR_A {
    #[doc = "0: Configure pin as an input pin"]
    INPUT = 0,
    #[doc = "1: Configure pin as an output pin"]
    OUTPUT = 1,
}
impl From<DIR_A> for bool {
    #[inline(always)]
    fn from(variant: DIR_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `DIR`"]
pub type DIR_R = crate::R<bool, DIR_A>;
impl DIR_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DIR_A {
        match self.bits {
            false => DIR_A::INPUT,
            true => DIR_A::OUTPUT,
        }
    }
    #[doc = "Checks if the value of the field is `INPUT`"]
    #[inline(always)]
    pub fn is_input(&self) -> bool {
        *self == DIR_A::INPUT
    }
    #[doc = "Checks if the value of the field is `OUTPUT`"]
    #[inline(always)]
    pub fn is_output(&self) -> bool {
        *self == DIR_A::OUTPUT
    }
}
#[doc = "Write proxy for field `DIR`"]
pub struct DIR_W<'a> {
    w: &'a mut W,
}
impl<'a> DIR_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: DIR_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Configure pin as an input pin"]
    #[inline(always)]
    pub fn input(self) -> &'a mut W {
        self.variant(DIR_A::INPUT)
    }
    #[doc = "Configure pin as an output pin"]
    #[inline(always)]
    pub fn output(self) -> &'a mut W {
        self.variant(DIR_A::OUTPUT)
    }
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x01) | ((value as u32) & 0x01);
        self.w
    }
}
#[doc = "Connect or disconnect input buffer\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum INPUT_A {
    #[doc = "0: Connect input buffer"]
    CONNECT = 0,
    #[doc = "1: Disconnect input buffer"]
    DISCONNECT = 1,
}
impl From<INPUT_A> for bool {
    #[inline(always)]
    fn from(variant: INPUT_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `INPUT`"]
pub type INPUT_R = crate::R<bool, INPUT_A>;
impl INPUT_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> INPUT_A {
        match self.bits {
            false => INPUT_A::CONNECT,
            true => INPUT_A::DISCONNECT,
        }
    }
    #[doc = "Checks if the value of the field is `CONNECT`"]
    #[inline(always)]
    pub fn is_connect(&self) -> bool {
        *self == INPUT_A::CONNECT
    }
    #[doc = "Checks if the value of the field is `DISCONNECT`"]
    #[inline(always)]
    pub fn is_disconnect(&self) -> bool {
        *self == INPUT_A::DISCONNECT
    }
}
#[doc = "Write proxy for field `INPUT`"]
pub struct INPUT_W<'a> {
    w: &'a mut W,
}
impl<'a> INPUT_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: INPUT_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Connect input buffer"]
    #[inline(always)]
    pub fn connect(self) -> &'a mut W {
        self.variant(INPUT_A::CONNECT)
    }
    #[doc = "Disconnect input buffer"]
    #[inline(always)]
    pub fn disconnect(self) -> &'a mut W {
        self.variant(INPUT_A::DISCONNECT)
    }
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 1)) | (((value as u32) & 0x01) << 1);
        self.w
    }
}
#[doc = "Pull configuration\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum PULL_A {
    #[doc = "0: No pull"]
    DISABLED = 0,
    #[doc = "1: Pull down on pin"]
    PULLDOWN = 1,
    #[doc = "3: Pull up on pin"]
    PULLUP = 3,
}
impl From<PULL_A> for u8 {
    #[inline(always)]
    fn from(variant: PULL_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `PULL`"]
pub type PULL_R = crate::R<u8, PULL_A>;
impl PULL_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, PULL_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(PULL_A::DISABLED),
            1 => Val(PULL_A::PULLDOWN),
            3 => Val(PULL_A::PULLUP),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == PULL_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `PULLDOWN`"]
    #[inline(always)]
    pub fn is_pulldown(&self) -> bool {
        *self == PULL_A::PULLDOWN
    }
    #[doc = "Checks if the value of the field is `PULLUP`"]
    #[inline(always)]
    pub fn is_pullup(&self) -> bool {
        *self == PULL_A::PULLUP
    }
}
#[doc = "Write proxy for field `PULL`"]
pub struct PULL_W<'a> {
    w: &'a mut W,
}
impl<'a> PULL_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PULL_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "No pull"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(PULL_A::DISABLED)
    }
    #[doc = "Pull down on pin"]
    #[inline(always)]
    pub fn pulldown(self) -> &'a mut W {
        self.variant(PULL_A::PULLDOWN)
    }
    #[doc = "Pull up on pin"]
    #[inline(always)]
    pub fn pullup(self) -> &'a mut W {
        self.variant(PULL_A::PULLUP)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 2)) | (((value as u32) & 0x03) << 2);
        self.w
    }
}
#[doc = "Drive configuration\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum DRIVE_A {
    #[doc = "0: Standard '0', standard '1'"]
    S0S1 = 0,
    #[doc = "1: High drive '0', standard '1'"]
    H0S1 = 1,
    #[doc = "2: Standard '0', high drive '1'"]
    S0H1 = 2,
    #[doc = "3: High drive '0', high 'drive '1''"]
    H0H1 = 3,
    #[doc = "4: Disconnect '0' standard '1' (normally used for wired-or connections)"]
    D0S1 = 4,
    #[doc = "5: Disconnect '0', high drive '1' (normally used for wired-or connections)"]
    D0H1 = 5,
    #[doc = "6: Standard '0'. disconnect '1' (normally used for wired-and connections)"]
    S0D1 = 6,
    #[doc = "7: High drive '0', disconnect '1' (normally used for wired-and connections)"]
    H0D1 = 7,
}
impl From<DRIVE_A> for u8 {
    #[inline(always)]
    fn from(variant: DRIVE_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `DRIVE`"]
pub type DRIVE_R = crate::R<u8, DRIVE_A>;
impl DRIVE_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DRIVE_A {
        match self.bits {
            0 => DRIVE_A::S0S1,
            1 => DRIVE_A::H0S1,
            2 => DRIVE_A::S0H1,
            3 => DRIVE_A::H0H1,
            4 => DRIVE_A::D0S1,
            5 => DRIVE_A::D0H1,
            6 => DRIVE_A::S0D1,
            7 => DRIVE_A::H0D1,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `S0S1`"]
    #[inline(always)]
    pub fn is_s0s1(&self) -> bool {
        *self == DRIVE_A::S0S1
    }
    #[doc = "Checks if the value of the field is `H0S1`"]
    #[inline(always)]
    pub fn is_h0s1(&self) -> bool {
        *self == DRIVE_A::H0S1
    }
    #[doc = "Checks if the value of the field is `S0H1`"]
    #[inline(always)]
    pub fn is_s0h1(&self) -> bool {
        *self == DRIVE_A::S0H1
    }
    #[doc = "Checks if the value of the field is `H0H1`"]
    #[inline(always)]
    pub fn is_h0h1(&self) -> bool {
        *self == DRIVE_A::H0H1
    }
    #[doc = "Checks if the value of the field is `D0S1`"]
    #[inline(always)]
    pub fn is_d0s1(&self) -> bool {
        *self == DRIVE_A::D0S1
    }
    #[doc = "Checks if the value of the field is `D0H1`"]
    #[inline(always)]
    pub fn is_d0h1(&self) -> bool {
        *self == DRIVE_A::D0H1
    }
    #[doc = "Checks if the value of the field is `S0D1`"]
    #[inline(always)]
    pub fn is_s0d1(&self) -> bool {
        *self == DRIVE_A::S0D1
    }
    #[doc = "Checks if the value of the field is `H0D1`"]
    #[inline(always)]
    pub fn is_h0d1(&self) -> bool {
        *self == DRIVE_A::H0D1
    }
}
#[doc = "Write proxy for field `DRIVE`"]
pub struct DRIVE_W<'a> {
    w: &'a mut W,
}
impl<'a> DRIVE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: DRIVE_A) -> &'a mut W {
        {
            self.bits(variant.into())
        }
    }
    #[doc = "Standard '0', standard '1'"]
    #[inline(always)]
    pub fn s0s1(self) -> &'a mut W {
        self.variant(DRIVE_A::S0S1)
    }
    #[doc = "High drive '0', standard '1'"]
    #[inline(always)]
    pub fn h0s1(self) -> &'a mut W {
        self.variant(DRIVE_A::H0S1)
    }
    #[doc = "Standard '0', high drive '1'"]
    #[inline(always)]
    pub fn s0h1(self) -> &'a mut W {
        self.variant(DRIVE_A::S0H1)
    }
    #[doc = "High drive '0', high 'drive '1''"]
    #[inline(always)]
    pub fn h0h1(self) -> &'a mut W {
        self.variant(DRIVE_A::H0H1)
    }
    #[doc = "Disconnect '0' standard '1' (normally used for wired-or connections)"]
    #[inline(always)]
    pub fn d0s1(self) -> &'a mut W {
        self.variant(DRIVE_A::D0S1)
    }
    #[doc = "Disconnect '0', high drive '1' (normally used for wired-or connections)"]
    #[inline(always)]
    pub fn d0h1(self) -> &'a mut W {
        self.variant(DRIVE_A::D0H1)
    }
    #[doc = "Standard '0'. disconnect '1' (normally used for wired-and connections)"]
    #[inline(always)]
    pub fn s0d1(self) -> &'a mut W {
        self.variant(DRIVE_A::S0D1)
    }
    #[doc = "High drive '0', disconnect '1' (normally used for wired-and connections)"]
    #[inline(always)]
    pub fn h0d1(self) -> &'a mut W {
        self.variant(DRIVE_A::H0D1)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 8)) | (((value as u32) & 0x07) << 8);
        self.w
    }
}
#[doc = "Pin sensing mechanism\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum SENSE_A {
    #[doc = "0: Disabled"]
    DISABLED = 0,
    #[doc = "2: Sense for high level"]
    HIGH = 2,
    #[doc = "3: Sense for low level"]
    LOW = 3,
}
impl From<SENSE_A> for u8 {
    #[inline(always)]
    fn from(variant: SENSE_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `SENSE`"]
pub type SENSE_R = crate::R<u8, SENSE_A>;
impl SENSE_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, SENSE_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(SENSE_A::DISABLED),
            2 => Val(SENSE_A::HIGH),
            3 => Val(SENSE_A::LOW),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == SENSE_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `HIGH`"]
    #[inline(always)]
    pub fn is_high(&self) -> bool {
        *self == SENSE_A::HIGH
    }
    #[doc = "Checks if the value of the field is `LOW`"]
    #[inline(always)]
    pub fn is_low(&self) -> bool {
        *self == SENSE_A::LOW
    }
}
#[doc = "Write proxy for field `SENSE`"]
pub struct SENSE_W<'a> {
    w: &'a mut W,
}
impl<'a> SENSE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SENSE_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "Disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(SENSE_A::DISABLED)
    }
    #[doc = "Sense for high level"]
    #[inline(always)]
    pub fn high(self) -> &'a mut W {
        self.variant(SENSE_A::HIGH)
    }
    #[doc = "Sense for low level"]
    #[inline(always)]
    pub fn low(self) -> &'a mut W {
        self.variant(SENSE_A::LOW)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 16)) | (((value as u32) & 0x03) << 16);
        self.w
    }
}
impl R {
    #[doc = "Bit 0 - Pin direction. Same physical register as DIR register"]
    #[inline(always)]
    pub fn dir(&self) -> DIR_R {
        DIR_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - Connect or disconnect input buffer"]
    #[inline(always)]
    pub fn input(&self) -> INPUT_R {
        INPUT_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bits 2:3 - Pull configuration"]
    #[inline(always)]
    pub fn pull(&self) -> PULL_R {
        PULL_R::new(((self.bits >> 2) & 0x03) as u8)
    }
    #[doc = "Bits 8:10 - Drive configuration"]
    #[inline(always)]
    pub fn drive(&self) -> DRIVE_R {
        DRIVE_R::new(((self.bits >> 8) & 0x07) as u8)
    }
    #[doc = "Bits 16:17 - Pin sensing mechanism"]
    #[inline(always)]
    pub fn sense(&self) -> SENSE_R {
        SENSE_R::new(((self.bits >> 16) & 0x03) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - Pin direction. Same physical register as DIR register"]
    #[inline(always)]
    pub fn dir(&mut self) -> DIR_W {
        DIR_W { w: self }
    }
    #[doc = "Bit 1 - Connect or disconnect input buffer"]
    #[inline(always)]
    pub fn input(&mut self) -> INPUT_W {
        INPUT_W { w: self }
    }
    #[doc = "Bits 2:3 - Pull configuration"]
    #[inline(always)]
    pub fn pull(&mut self) -> PULL_W {
        PULL_W { w: self }
    }
    #[doc = "Bits 8:10 - Drive configuration"]
    #[inline(always)]
    pub fn drive(&mut self) -> DRIVE_W {
        DRIVE_W { w: self }
    }
    #[doc = "Bits 16:17 - Pin sensing mechanism"]
    #[inline(always)]
    pub fn sense(&mut self) -> SENSE_W {
        SENSE_W { w: self }
    }
}
