#[doc = "Reader of register LATCH"]
pub type R = crate::R<u32, super::LATCH>;
#[doc = "Writer for register LATCH"]
pub type W = crate::W<u32, super::LATCH>;
#[doc = "Register LATCH `reset()`'s with value 0"]
impl crate::ResetValue for super::LATCH {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Status on whether PIN0 has met criteria set in PIN_CNF0.SENSE register. Write '1' to clear.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PIN0_A {
    #[doc = "0: Criteria has not been met"]
    NOTLATCHED = 0,
    #[doc = "1: Criteria has been met"]
    LATCHED = 1,
}
impl From<PIN0_A> for bool {
    #[inline(always)]
    fn from(variant: PIN0_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `PIN0`"]
pub type PIN0_R = crate::R<bool, PIN0_A>;
impl PIN0_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PIN0_A {
        match self.bits {
            false => PIN0_A::NOTLATCHED,
            true => PIN0_A::LATCHED,
        }
    }
    #[doc = "Checks if the value of the field is `NOTLATCHED`"]
    #[inline(always)]
    pub fn is_not_latched(&self) -> bool {
        *self == PIN0_A::NOTLATCHED
    }
    #[doc = "Checks if the value of the field is `LATCHED`"]
    #[inline(always)]
    pub fn is_latched(&self) -> bool {
        *self == PIN0_A::LATCHED
    }
}
#[doc = "Write proxy for field `PIN0`"]
pub struct PIN0_W<'a> {
    w: &'a mut W,
}
impl<'a> PIN0_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PIN0_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Criteria has not been met"]
    #[inline(always)]
    pub fn not_latched(self) -> &'a mut W {
        self.variant(PIN0_A::NOTLATCHED)
    }
    #[doc = "Criteria has been met"]
    #[inline(always)]
    pub fn latched(self) -> &'a mut W {
        self.variant(PIN0_A::LATCHED)
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
#[doc = "Status on whether PIN1 has met criteria set in PIN_CNF1.SENSE register. Write '1' to clear.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PIN1_A {
    #[doc = "0: Criteria has not been met"]
    NOTLATCHED = 0,
    #[doc = "1: Criteria has been met"]
    LATCHED = 1,
}
impl From<PIN1_A> for bool {
    #[inline(always)]
    fn from(variant: PIN1_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `PIN1`"]
pub type PIN1_R = crate::R<bool, PIN1_A>;
impl PIN1_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PIN1_A {
        match self.bits {
            false => PIN1_A::NOTLATCHED,
            true => PIN1_A::LATCHED,
        }
    }
    #[doc = "Checks if the value of the field is `NOTLATCHED`"]
    #[inline(always)]
    pub fn is_not_latched(&self) -> bool {
        *self == PIN1_A::NOTLATCHED
    }
    #[doc = "Checks if the value of the field is `LATCHED`"]
    #[inline(always)]
    pub fn is_latched(&self) -> bool {
        *self == PIN1_A::LATCHED
    }
}
#[doc = "Write proxy for field `PIN1`"]
pub struct PIN1_W<'a> {
    w: &'a mut W,
}
impl<'a> PIN1_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PIN1_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Criteria has not been met"]
    #[inline(always)]
    pub fn not_latched(self) -> &'a mut W {
        self.variant(PIN1_A::NOTLATCHED)
    }
    #[doc = "Criteria has been met"]
    #[inline(always)]
    pub fn latched(self) -> &'a mut W {
        self.variant(PIN1_A::LATCHED)
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
#[doc = "Status on whether PIN2 has met criteria set in PIN_CNF2.SENSE register. Write '1' to clear.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PIN2_A {
    #[doc = "0: Criteria has not been met"]
    NOTLATCHED = 0,
    #[doc = "1: Criteria has been met"]
    LATCHED = 1,
}
impl From<PIN2_A> for bool {
    #[inline(always)]
    fn from(variant: PIN2_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `PIN2`"]
pub type PIN2_R = crate::R<bool, PIN2_A>;
impl PIN2_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PIN2_A {
        match self.bits {
            false => PIN2_A::NOTLATCHED,
            true => PIN2_A::LATCHED,
        }
    }
    #[doc = "Checks if the value of the field is `NOTLATCHED`"]
    #[inline(always)]
    pub fn is_not_latched(&self) -> bool {
        *self == PIN2_A::NOTLATCHED
    }
    #[doc = "Checks if the value of the field is `LATCHED`"]
    #[inline(always)]
    pub fn is_latched(&self) -> bool {
        *self == PIN2_A::LATCHED
    }
}
#[doc = "Write proxy for field `PIN2`"]
pub struct PIN2_W<'a> {
    w: &'a mut W,
}
impl<'a> PIN2_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PIN2_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Criteria has not been met"]
    #[inline(always)]
    pub fn not_latched(self) -> &'a mut W {
        self.variant(PIN2_A::NOTLATCHED)
    }
    #[doc = "Criteria has been met"]
    #[inline(always)]
    pub fn latched(self) -> &'a mut W {
        self.variant(PIN2_A::LATCHED)
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
        self.w.bits = (self.w.bits & !(0x01 << 2)) | (((value as u32) & 0x01) << 2);
        self.w
    }
}
#[doc = "Status on whether PIN3 has met criteria set in PIN_CNF3.SENSE register. Write '1' to clear.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PIN3_A {
    #[doc = "0: Criteria has not been met"]
    NOTLATCHED = 0,
    #[doc = "1: Criteria has been met"]
    LATCHED = 1,
}
impl From<PIN3_A> for bool {
    #[inline(always)]
    fn from(variant: PIN3_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `PIN3`"]
pub type PIN3_R = crate::R<bool, PIN3_A>;
impl PIN3_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PIN3_A {
        match self.bits {
            false => PIN3_A::NOTLATCHED,
            true => PIN3_A::LATCHED,
        }
    }
    #[doc = "Checks if the value of the field is `NOTLATCHED`"]
    #[inline(always)]
    pub fn is_not_latched(&self) -> bool {
        *self == PIN3_A::NOTLATCHED
    }
    #[doc = "Checks if the value of the field is `LATCHED`"]
    #[inline(always)]
    pub fn is_latched(&self) -> bool {
        *self == PIN3_A::LATCHED
    }
}
#[doc = "Write proxy for field `PIN3`"]
pub struct PIN3_W<'a> {
    w: &'a mut W,
}
impl<'a> PIN3_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PIN3_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Criteria has not been met"]
    #[inline(always)]
    pub fn not_latched(self) -> &'a mut W {
        self.variant(PIN3_A::NOTLATCHED)
    }
    #[doc = "Criteria has been met"]
    #[inline(always)]
    pub fn latched(self) -> &'a mut W {
        self.variant(PIN3_A::LATCHED)
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
        self.w.bits = (self.w.bits & !(0x01 << 3)) | (((value as u32) & 0x01) << 3);
        self.w
    }
}
#[doc = "Status on whether PIN4 has met criteria set in PIN_CNF4.SENSE register. Write '1' to clear.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PIN4_A {
    #[doc = "0: Criteria has not been met"]
    NOTLATCHED = 0,
    #[doc = "1: Criteria has been met"]
    LATCHED = 1,
}
impl From<PIN4_A> for bool {
    #[inline(always)]
    fn from(variant: PIN4_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `PIN4`"]
pub type PIN4_R = crate::R<bool, PIN4_A>;
impl PIN4_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PIN4_A {
        match self.bits {
            false => PIN4_A::NOTLATCHED,
            true => PIN4_A::LATCHED,
        }
    }
    #[doc = "Checks if the value of the field is `NOTLATCHED`"]
    #[inline(always)]
    pub fn is_not_latched(&self) -> bool {
        *self == PIN4_A::NOTLATCHED
    }
    #[doc = "Checks if the value of the field is `LATCHED`"]
    #[inline(always)]
    pub fn is_latched(&self) -> bool {
        *self == PIN4_A::LATCHED
    }
}
#[doc = "Write proxy for field `PIN4`"]
pub struct PIN4_W<'a> {
    w: &'a mut W,
}
impl<'a> PIN4_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PIN4_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Criteria has not been met"]
    #[inline(always)]
    pub fn not_latched(self) -> &'a mut W {
        self.variant(PIN4_A::NOTLATCHED)
    }
    #[doc = "Criteria has been met"]
    #[inline(always)]
    pub fn latched(self) -> &'a mut W {
        self.variant(PIN4_A::LATCHED)
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
        self.w.bits = (self.w.bits & !(0x01 << 4)) | (((value as u32) & 0x01) << 4);
        self.w
    }
}
#[doc = "Status on whether PIN5 has met criteria set in PIN_CNF5.SENSE register. Write '1' to clear.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PIN5_A {
    #[doc = "0: Criteria has not been met"]
    NOTLATCHED = 0,
    #[doc = "1: Criteria has been met"]
    LATCHED = 1,
}
impl From<PIN5_A> for bool {
    #[inline(always)]
    fn from(variant: PIN5_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `PIN5`"]
pub type PIN5_R = crate::R<bool, PIN5_A>;
impl PIN5_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PIN5_A {
        match self.bits {
            false => PIN5_A::NOTLATCHED,
            true => PIN5_A::LATCHED,
        }
    }
    #[doc = "Checks if the value of the field is `NOTLATCHED`"]
    #[inline(always)]
    pub fn is_not_latched(&self) -> bool {
        *self == PIN5_A::NOTLATCHED
    }
    #[doc = "Checks if the value of the field is `LATCHED`"]
    #[inline(always)]
    pub fn is_latched(&self) -> bool {
        *self == PIN5_A::LATCHED
    }
}
#[doc = "Write proxy for field `PIN5`"]
pub struct PIN5_W<'a> {
    w: &'a mut W,
}
impl<'a> PIN5_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PIN5_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Criteria has not been met"]
    #[inline(always)]
    pub fn not_latched(self) -> &'a mut W {
        self.variant(PIN5_A::NOTLATCHED)
    }
    #[doc = "Criteria has been met"]
    #[inline(always)]
    pub fn latched(self) -> &'a mut W {
        self.variant(PIN5_A::LATCHED)
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
        self.w.bits = (self.w.bits & !(0x01 << 5)) | (((value as u32) & 0x01) << 5);
        self.w
    }
}
#[doc = "Status on whether PIN6 has met criteria set in PIN_CNF6.SENSE register. Write '1' to clear.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PIN6_A {
    #[doc = "0: Criteria has not been met"]
    NOTLATCHED = 0,
    #[doc = "1: Criteria has been met"]
    LATCHED = 1,
}
impl From<PIN6_A> for bool {
    #[inline(always)]
    fn from(variant: PIN6_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `PIN6`"]
pub type PIN6_R = crate::R<bool, PIN6_A>;
impl PIN6_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PIN6_A {
        match self.bits {
            false => PIN6_A::NOTLATCHED,
            true => PIN6_A::LATCHED,
        }
    }
    #[doc = "Checks if the value of the field is `NOTLATCHED`"]
    #[inline(always)]
    pub fn is_not_latched(&self) -> bool {
        *self == PIN6_A::NOTLATCHED
    }
    #[doc = "Checks if the value of the field is `LATCHED`"]
    #[inline(always)]
    pub fn is_latched(&self) -> bool {
        *self == PIN6_A::LATCHED
    }
}
#[doc = "Write proxy for field `PIN6`"]
pub struct PIN6_W<'a> {
    w: &'a mut W,
}
impl<'a> PIN6_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PIN6_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Criteria has not been met"]
    #[inline(always)]
    pub fn not_latched(self) -> &'a mut W {
        self.variant(PIN6_A::NOTLATCHED)
    }
    #[doc = "Criteria has been met"]
    #[inline(always)]
    pub fn latched(self) -> &'a mut W {
        self.variant(PIN6_A::LATCHED)
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
        self.w.bits = (self.w.bits & !(0x01 << 6)) | (((value as u32) & 0x01) << 6);
        self.w
    }
}
#[doc = "Status on whether PIN7 has met criteria set in PIN_CNF7.SENSE register. Write '1' to clear.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PIN7_A {
    #[doc = "0: Criteria has not been met"]
    NOTLATCHED = 0,
    #[doc = "1: Criteria has been met"]
    LATCHED = 1,
}
impl From<PIN7_A> for bool {
    #[inline(always)]
    fn from(variant: PIN7_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `PIN7`"]
pub type PIN7_R = crate::R<bool, PIN7_A>;
impl PIN7_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PIN7_A {
        match self.bits {
            false => PIN7_A::NOTLATCHED,
            true => PIN7_A::LATCHED,
        }
    }
    #[doc = "Checks if the value of the field is `NOTLATCHED`"]
    #[inline(always)]
    pub fn is_not_latched(&self) -> bool {
        *self == PIN7_A::NOTLATCHED
    }
    #[doc = "Checks if the value of the field is `LATCHED`"]
    #[inline(always)]
    pub fn is_latched(&self) -> bool {
        *self == PIN7_A::LATCHED
    }
}
#[doc = "Write proxy for field `PIN7`"]
pub struct PIN7_W<'a> {
    w: &'a mut W,
}
impl<'a> PIN7_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PIN7_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Criteria has not been met"]
    #[inline(always)]
    pub fn not_latched(self) -> &'a mut W {
        self.variant(PIN7_A::NOTLATCHED)
    }
    #[doc = "Criteria has been met"]
    #[inline(always)]
    pub fn latched(self) -> &'a mut W {
        self.variant(PIN7_A::LATCHED)
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
        self.w.bits = (self.w.bits & !(0x01 << 7)) | (((value as u32) & 0x01) << 7);
        self.w
    }
}
#[doc = "Status on whether PIN8 has met criteria set in PIN_CNF8.SENSE register. Write '1' to clear.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PIN8_A {
    #[doc = "0: Criteria has not been met"]
    NOTLATCHED = 0,
    #[doc = "1: Criteria has been met"]
    LATCHED = 1,
}
impl From<PIN8_A> for bool {
    #[inline(always)]
    fn from(variant: PIN8_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `PIN8`"]
pub type PIN8_R = crate::R<bool, PIN8_A>;
impl PIN8_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PIN8_A {
        match self.bits {
            false => PIN8_A::NOTLATCHED,
            true => PIN8_A::LATCHED,
        }
    }
    #[doc = "Checks if the value of the field is `NOTLATCHED`"]
    #[inline(always)]
    pub fn is_not_latched(&self) -> bool {
        *self == PIN8_A::NOTLATCHED
    }
    #[doc = "Checks if the value of the field is `LATCHED`"]
    #[inline(always)]
    pub fn is_latched(&self) -> bool {
        *self == PIN8_A::LATCHED
    }
}
#[doc = "Write proxy for field `PIN8`"]
pub struct PIN8_W<'a> {
    w: &'a mut W,
}
impl<'a> PIN8_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PIN8_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Criteria has not been met"]
    #[inline(always)]
    pub fn not_latched(self) -> &'a mut W {
        self.variant(PIN8_A::NOTLATCHED)
    }
    #[doc = "Criteria has been met"]
    #[inline(always)]
    pub fn latched(self) -> &'a mut W {
        self.variant(PIN8_A::LATCHED)
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
        self.w.bits = (self.w.bits & !(0x01 << 8)) | (((value as u32) & 0x01) << 8);
        self.w
    }
}
#[doc = "Status on whether PIN9 has met criteria set in PIN_CNF9.SENSE register. Write '1' to clear.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PIN9_A {
    #[doc = "0: Criteria has not been met"]
    NOTLATCHED = 0,
    #[doc = "1: Criteria has been met"]
    LATCHED = 1,
}
impl From<PIN9_A> for bool {
    #[inline(always)]
    fn from(variant: PIN9_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `PIN9`"]
pub type PIN9_R = crate::R<bool, PIN9_A>;
impl PIN9_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PIN9_A {
        match self.bits {
            false => PIN9_A::NOTLATCHED,
            true => PIN9_A::LATCHED,
        }
    }
    #[doc = "Checks if the value of the field is `NOTLATCHED`"]
    #[inline(always)]
    pub fn is_not_latched(&self) -> bool {
        *self == PIN9_A::NOTLATCHED
    }
    #[doc = "Checks if the value of the field is `LATCHED`"]
    #[inline(always)]
    pub fn is_latched(&self) -> bool {
        *self == PIN9_A::LATCHED
    }
}
#[doc = "Write proxy for field `PIN9`"]
pub struct PIN9_W<'a> {
    w: &'a mut W,
}
impl<'a> PIN9_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PIN9_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Criteria has not been met"]
    #[inline(always)]
    pub fn not_latched(self) -> &'a mut W {
        self.variant(PIN9_A::NOTLATCHED)
    }
    #[doc = "Criteria has been met"]
    #[inline(always)]
    pub fn latched(self) -> &'a mut W {
        self.variant(PIN9_A::LATCHED)
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
        self.w.bits = (self.w.bits & !(0x01 << 9)) | (((value as u32) & 0x01) << 9);
        self.w
    }
}
#[doc = "Status on whether PIN10 has met criteria set in PIN_CNF10.SENSE register. Write '1' to clear.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PIN10_A {
    #[doc = "0: Criteria has not been met"]
    NOTLATCHED = 0,
    #[doc = "1: Criteria has been met"]
    LATCHED = 1,
}
impl From<PIN10_A> for bool {
    #[inline(always)]
    fn from(variant: PIN10_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `PIN10`"]
pub type PIN10_R = crate::R<bool, PIN10_A>;
impl PIN10_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PIN10_A {
        match self.bits {
            false => PIN10_A::NOTLATCHED,
            true => PIN10_A::LATCHED,
        }
    }
    #[doc = "Checks if the value of the field is `NOTLATCHED`"]
    #[inline(always)]
    pub fn is_not_latched(&self) -> bool {
        *self == PIN10_A::NOTLATCHED
    }
    #[doc = "Checks if the value of the field is `LATCHED`"]
    #[inline(always)]
    pub fn is_latched(&self) -> bool {
        *self == PIN10_A::LATCHED
    }
}
#[doc = "Write proxy for field `PIN10`"]
pub struct PIN10_W<'a> {
    w: &'a mut W,
}
impl<'a> PIN10_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PIN10_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Criteria has not been met"]
    #[inline(always)]
    pub fn not_latched(self) -> &'a mut W {
        self.variant(PIN10_A::NOTLATCHED)
    }
    #[doc = "Criteria has been met"]
    #[inline(always)]
    pub fn latched(self) -> &'a mut W {
        self.variant(PIN10_A::LATCHED)
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
        self.w.bits = (self.w.bits & !(0x01 << 10)) | (((value as u32) & 0x01) << 10);
        self.w
    }
}
#[doc = "Status on whether PIN11 has met criteria set in PIN_CNF11.SENSE register. Write '1' to clear.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PIN11_A {
    #[doc = "0: Criteria has not been met"]
    NOTLATCHED = 0,
    #[doc = "1: Criteria has been met"]
    LATCHED = 1,
}
impl From<PIN11_A> for bool {
    #[inline(always)]
    fn from(variant: PIN11_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `PIN11`"]
pub type PIN11_R = crate::R<bool, PIN11_A>;
impl PIN11_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PIN11_A {
        match self.bits {
            false => PIN11_A::NOTLATCHED,
            true => PIN11_A::LATCHED,
        }
    }
    #[doc = "Checks if the value of the field is `NOTLATCHED`"]
    #[inline(always)]
    pub fn is_not_latched(&self) -> bool {
        *self == PIN11_A::NOTLATCHED
    }
    #[doc = "Checks if the value of the field is `LATCHED`"]
    #[inline(always)]
    pub fn is_latched(&self) -> bool {
        *self == PIN11_A::LATCHED
    }
}
#[doc = "Write proxy for field `PIN11`"]
pub struct PIN11_W<'a> {
    w: &'a mut W,
}
impl<'a> PIN11_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PIN11_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Criteria has not been met"]
    #[inline(always)]
    pub fn not_latched(self) -> &'a mut W {
        self.variant(PIN11_A::NOTLATCHED)
    }
    #[doc = "Criteria has been met"]
    #[inline(always)]
    pub fn latched(self) -> &'a mut W {
        self.variant(PIN11_A::LATCHED)
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
        self.w.bits = (self.w.bits & !(0x01 << 11)) | (((value as u32) & 0x01) << 11);
        self.w
    }
}
#[doc = "Status on whether PIN12 has met criteria set in PIN_CNF12.SENSE register. Write '1' to clear.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PIN12_A {
    #[doc = "0: Criteria has not been met"]
    NOTLATCHED = 0,
    #[doc = "1: Criteria has been met"]
    LATCHED = 1,
}
impl From<PIN12_A> for bool {
    #[inline(always)]
    fn from(variant: PIN12_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `PIN12`"]
pub type PIN12_R = crate::R<bool, PIN12_A>;
impl PIN12_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PIN12_A {
        match self.bits {
            false => PIN12_A::NOTLATCHED,
            true => PIN12_A::LATCHED,
        }
    }
    #[doc = "Checks if the value of the field is `NOTLATCHED`"]
    #[inline(always)]
    pub fn is_not_latched(&self) -> bool {
        *self == PIN12_A::NOTLATCHED
    }
    #[doc = "Checks if the value of the field is `LATCHED`"]
    #[inline(always)]
    pub fn is_latched(&self) -> bool {
        *self == PIN12_A::LATCHED
    }
}
#[doc = "Write proxy for field `PIN12`"]
pub struct PIN12_W<'a> {
    w: &'a mut W,
}
impl<'a> PIN12_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PIN12_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Criteria has not been met"]
    #[inline(always)]
    pub fn not_latched(self) -> &'a mut W {
        self.variant(PIN12_A::NOTLATCHED)
    }
    #[doc = "Criteria has been met"]
    #[inline(always)]
    pub fn latched(self) -> &'a mut W {
        self.variant(PIN12_A::LATCHED)
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
        self.w.bits = (self.w.bits & !(0x01 << 12)) | (((value as u32) & 0x01) << 12);
        self.w
    }
}
#[doc = "Status on whether PIN13 has met criteria set in PIN_CNF13.SENSE register. Write '1' to clear.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PIN13_A {
    #[doc = "0: Criteria has not been met"]
    NOTLATCHED = 0,
    #[doc = "1: Criteria has been met"]
    LATCHED = 1,
}
impl From<PIN13_A> for bool {
    #[inline(always)]
    fn from(variant: PIN13_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `PIN13`"]
pub type PIN13_R = crate::R<bool, PIN13_A>;
impl PIN13_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PIN13_A {
        match self.bits {
            false => PIN13_A::NOTLATCHED,
            true => PIN13_A::LATCHED,
        }
    }
    #[doc = "Checks if the value of the field is `NOTLATCHED`"]
    #[inline(always)]
    pub fn is_not_latched(&self) -> bool {
        *self == PIN13_A::NOTLATCHED
    }
    #[doc = "Checks if the value of the field is `LATCHED`"]
    #[inline(always)]
    pub fn is_latched(&self) -> bool {
        *self == PIN13_A::LATCHED
    }
}
#[doc = "Write proxy for field `PIN13`"]
pub struct PIN13_W<'a> {
    w: &'a mut W,
}
impl<'a> PIN13_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PIN13_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Criteria has not been met"]
    #[inline(always)]
    pub fn not_latched(self) -> &'a mut W {
        self.variant(PIN13_A::NOTLATCHED)
    }
    #[doc = "Criteria has been met"]
    #[inline(always)]
    pub fn latched(self) -> &'a mut W {
        self.variant(PIN13_A::LATCHED)
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
        self.w.bits = (self.w.bits & !(0x01 << 13)) | (((value as u32) & 0x01) << 13);
        self.w
    }
}
#[doc = "Status on whether PIN14 has met criteria set in PIN_CNF14.SENSE register. Write '1' to clear.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PIN14_A {
    #[doc = "0: Criteria has not been met"]
    NOTLATCHED = 0,
    #[doc = "1: Criteria has been met"]
    LATCHED = 1,
}
impl From<PIN14_A> for bool {
    #[inline(always)]
    fn from(variant: PIN14_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `PIN14`"]
pub type PIN14_R = crate::R<bool, PIN14_A>;
impl PIN14_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PIN14_A {
        match self.bits {
            false => PIN14_A::NOTLATCHED,
            true => PIN14_A::LATCHED,
        }
    }
    #[doc = "Checks if the value of the field is `NOTLATCHED`"]
    #[inline(always)]
    pub fn is_not_latched(&self) -> bool {
        *self == PIN14_A::NOTLATCHED
    }
    #[doc = "Checks if the value of the field is `LATCHED`"]
    #[inline(always)]
    pub fn is_latched(&self) -> bool {
        *self == PIN14_A::LATCHED
    }
}
#[doc = "Write proxy for field `PIN14`"]
pub struct PIN14_W<'a> {
    w: &'a mut W,
}
impl<'a> PIN14_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PIN14_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Criteria has not been met"]
    #[inline(always)]
    pub fn not_latched(self) -> &'a mut W {
        self.variant(PIN14_A::NOTLATCHED)
    }
    #[doc = "Criteria has been met"]
    #[inline(always)]
    pub fn latched(self) -> &'a mut W {
        self.variant(PIN14_A::LATCHED)
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
        self.w.bits = (self.w.bits & !(0x01 << 14)) | (((value as u32) & 0x01) << 14);
        self.w
    }
}
#[doc = "Status on whether PIN15 has met criteria set in PIN_CNF15.SENSE register. Write '1' to clear.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PIN15_A {
    #[doc = "0: Criteria has not been met"]
    NOTLATCHED = 0,
    #[doc = "1: Criteria has been met"]
    LATCHED = 1,
}
impl From<PIN15_A> for bool {
    #[inline(always)]
    fn from(variant: PIN15_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `PIN15`"]
pub type PIN15_R = crate::R<bool, PIN15_A>;
impl PIN15_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PIN15_A {
        match self.bits {
            false => PIN15_A::NOTLATCHED,
            true => PIN15_A::LATCHED,
        }
    }
    #[doc = "Checks if the value of the field is `NOTLATCHED`"]
    #[inline(always)]
    pub fn is_not_latched(&self) -> bool {
        *self == PIN15_A::NOTLATCHED
    }
    #[doc = "Checks if the value of the field is `LATCHED`"]
    #[inline(always)]
    pub fn is_latched(&self) -> bool {
        *self == PIN15_A::LATCHED
    }
}
#[doc = "Write proxy for field `PIN15`"]
pub struct PIN15_W<'a> {
    w: &'a mut W,
}
impl<'a> PIN15_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PIN15_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Criteria has not been met"]
    #[inline(always)]
    pub fn not_latched(self) -> &'a mut W {
        self.variant(PIN15_A::NOTLATCHED)
    }
    #[doc = "Criteria has been met"]
    #[inline(always)]
    pub fn latched(self) -> &'a mut W {
        self.variant(PIN15_A::LATCHED)
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
        self.w.bits = (self.w.bits & !(0x01 << 15)) | (((value as u32) & 0x01) << 15);
        self.w
    }
}
#[doc = "Status on whether PIN16 has met criteria set in PIN_CNF16.SENSE register. Write '1' to clear.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PIN16_A {
    #[doc = "0: Criteria has not been met"]
    NOTLATCHED = 0,
    #[doc = "1: Criteria has been met"]
    LATCHED = 1,
}
impl From<PIN16_A> for bool {
    #[inline(always)]
    fn from(variant: PIN16_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `PIN16`"]
pub type PIN16_R = crate::R<bool, PIN16_A>;
impl PIN16_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PIN16_A {
        match self.bits {
            false => PIN16_A::NOTLATCHED,
            true => PIN16_A::LATCHED,
        }
    }
    #[doc = "Checks if the value of the field is `NOTLATCHED`"]
    #[inline(always)]
    pub fn is_not_latched(&self) -> bool {
        *self == PIN16_A::NOTLATCHED
    }
    #[doc = "Checks if the value of the field is `LATCHED`"]
    #[inline(always)]
    pub fn is_latched(&self) -> bool {
        *self == PIN16_A::LATCHED
    }
}
#[doc = "Write proxy for field `PIN16`"]
pub struct PIN16_W<'a> {
    w: &'a mut W,
}
impl<'a> PIN16_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PIN16_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Criteria has not been met"]
    #[inline(always)]
    pub fn not_latched(self) -> &'a mut W {
        self.variant(PIN16_A::NOTLATCHED)
    }
    #[doc = "Criteria has been met"]
    #[inline(always)]
    pub fn latched(self) -> &'a mut W {
        self.variant(PIN16_A::LATCHED)
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
        self.w.bits = (self.w.bits & !(0x01 << 16)) | (((value as u32) & 0x01) << 16);
        self.w
    }
}
#[doc = "Status on whether PIN17 has met criteria set in PIN_CNF17.SENSE register. Write '1' to clear.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PIN17_A {
    #[doc = "0: Criteria has not been met"]
    NOTLATCHED = 0,
    #[doc = "1: Criteria has been met"]
    LATCHED = 1,
}
impl From<PIN17_A> for bool {
    #[inline(always)]
    fn from(variant: PIN17_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `PIN17`"]
pub type PIN17_R = crate::R<bool, PIN17_A>;
impl PIN17_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PIN17_A {
        match self.bits {
            false => PIN17_A::NOTLATCHED,
            true => PIN17_A::LATCHED,
        }
    }
    #[doc = "Checks if the value of the field is `NOTLATCHED`"]
    #[inline(always)]
    pub fn is_not_latched(&self) -> bool {
        *self == PIN17_A::NOTLATCHED
    }
    #[doc = "Checks if the value of the field is `LATCHED`"]
    #[inline(always)]
    pub fn is_latched(&self) -> bool {
        *self == PIN17_A::LATCHED
    }
}
#[doc = "Write proxy for field `PIN17`"]
pub struct PIN17_W<'a> {
    w: &'a mut W,
}
impl<'a> PIN17_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PIN17_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Criteria has not been met"]
    #[inline(always)]
    pub fn not_latched(self) -> &'a mut W {
        self.variant(PIN17_A::NOTLATCHED)
    }
    #[doc = "Criteria has been met"]
    #[inline(always)]
    pub fn latched(self) -> &'a mut W {
        self.variant(PIN17_A::LATCHED)
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
        self.w.bits = (self.w.bits & !(0x01 << 17)) | (((value as u32) & 0x01) << 17);
        self.w
    }
}
#[doc = "Status on whether PIN18 has met criteria set in PIN_CNF18.SENSE register. Write '1' to clear.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PIN18_A {
    #[doc = "0: Criteria has not been met"]
    NOTLATCHED = 0,
    #[doc = "1: Criteria has been met"]
    LATCHED = 1,
}
impl From<PIN18_A> for bool {
    #[inline(always)]
    fn from(variant: PIN18_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `PIN18`"]
pub type PIN18_R = crate::R<bool, PIN18_A>;
impl PIN18_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PIN18_A {
        match self.bits {
            false => PIN18_A::NOTLATCHED,
            true => PIN18_A::LATCHED,
        }
    }
    #[doc = "Checks if the value of the field is `NOTLATCHED`"]
    #[inline(always)]
    pub fn is_not_latched(&self) -> bool {
        *self == PIN18_A::NOTLATCHED
    }
    #[doc = "Checks if the value of the field is `LATCHED`"]
    #[inline(always)]
    pub fn is_latched(&self) -> bool {
        *self == PIN18_A::LATCHED
    }
}
#[doc = "Write proxy for field `PIN18`"]
pub struct PIN18_W<'a> {
    w: &'a mut W,
}
impl<'a> PIN18_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PIN18_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Criteria has not been met"]
    #[inline(always)]
    pub fn not_latched(self) -> &'a mut W {
        self.variant(PIN18_A::NOTLATCHED)
    }
    #[doc = "Criteria has been met"]
    #[inline(always)]
    pub fn latched(self) -> &'a mut W {
        self.variant(PIN18_A::LATCHED)
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
        self.w.bits = (self.w.bits & !(0x01 << 18)) | (((value as u32) & 0x01) << 18);
        self.w
    }
}
#[doc = "Status on whether PIN19 has met criteria set in PIN_CNF19.SENSE register. Write '1' to clear.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PIN19_A {
    #[doc = "0: Criteria has not been met"]
    NOTLATCHED = 0,
    #[doc = "1: Criteria has been met"]
    LATCHED = 1,
}
impl From<PIN19_A> for bool {
    #[inline(always)]
    fn from(variant: PIN19_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `PIN19`"]
pub type PIN19_R = crate::R<bool, PIN19_A>;
impl PIN19_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PIN19_A {
        match self.bits {
            false => PIN19_A::NOTLATCHED,
            true => PIN19_A::LATCHED,
        }
    }
    #[doc = "Checks if the value of the field is `NOTLATCHED`"]
    #[inline(always)]
    pub fn is_not_latched(&self) -> bool {
        *self == PIN19_A::NOTLATCHED
    }
    #[doc = "Checks if the value of the field is `LATCHED`"]
    #[inline(always)]
    pub fn is_latched(&self) -> bool {
        *self == PIN19_A::LATCHED
    }
}
#[doc = "Write proxy for field `PIN19`"]
pub struct PIN19_W<'a> {
    w: &'a mut W,
}
impl<'a> PIN19_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PIN19_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Criteria has not been met"]
    #[inline(always)]
    pub fn not_latched(self) -> &'a mut W {
        self.variant(PIN19_A::NOTLATCHED)
    }
    #[doc = "Criteria has been met"]
    #[inline(always)]
    pub fn latched(self) -> &'a mut W {
        self.variant(PIN19_A::LATCHED)
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
        self.w.bits = (self.w.bits & !(0x01 << 19)) | (((value as u32) & 0x01) << 19);
        self.w
    }
}
#[doc = "Status on whether PIN20 has met criteria set in PIN_CNF20.SENSE register. Write '1' to clear.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PIN20_A {
    #[doc = "0: Criteria has not been met"]
    NOTLATCHED = 0,
    #[doc = "1: Criteria has been met"]
    LATCHED = 1,
}
impl From<PIN20_A> for bool {
    #[inline(always)]
    fn from(variant: PIN20_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `PIN20`"]
pub type PIN20_R = crate::R<bool, PIN20_A>;
impl PIN20_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PIN20_A {
        match self.bits {
            false => PIN20_A::NOTLATCHED,
            true => PIN20_A::LATCHED,
        }
    }
    #[doc = "Checks if the value of the field is `NOTLATCHED`"]
    #[inline(always)]
    pub fn is_not_latched(&self) -> bool {
        *self == PIN20_A::NOTLATCHED
    }
    #[doc = "Checks if the value of the field is `LATCHED`"]
    #[inline(always)]
    pub fn is_latched(&self) -> bool {
        *self == PIN20_A::LATCHED
    }
}
#[doc = "Write proxy for field `PIN20`"]
pub struct PIN20_W<'a> {
    w: &'a mut W,
}
impl<'a> PIN20_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PIN20_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Criteria has not been met"]
    #[inline(always)]
    pub fn not_latched(self) -> &'a mut W {
        self.variant(PIN20_A::NOTLATCHED)
    }
    #[doc = "Criteria has been met"]
    #[inline(always)]
    pub fn latched(self) -> &'a mut W {
        self.variant(PIN20_A::LATCHED)
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
        self.w.bits = (self.w.bits & !(0x01 << 20)) | (((value as u32) & 0x01) << 20);
        self.w
    }
}
#[doc = "Status on whether PIN21 has met criteria set in PIN_CNF21.SENSE register. Write '1' to clear.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PIN21_A {
    #[doc = "0: Criteria has not been met"]
    NOTLATCHED = 0,
    #[doc = "1: Criteria has been met"]
    LATCHED = 1,
}
impl From<PIN21_A> for bool {
    #[inline(always)]
    fn from(variant: PIN21_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `PIN21`"]
pub type PIN21_R = crate::R<bool, PIN21_A>;
impl PIN21_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PIN21_A {
        match self.bits {
            false => PIN21_A::NOTLATCHED,
            true => PIN21_A::LATCHED,
        }
    }
    #[doc = "Checks if the value of the field is `NOTLATCHED`"]
    #[inline(always)]
    pub fn is_not_latched(&self) -> bool {
        *self == PIN21_A::NOTLATCHED
    }
    #[doc = "Checks if the value of the field is `LATCHED`"]
    #[inline(always)]
    pub fn is_latched(&self) -> bool {
        *self == PIN21_A::LATCHED
    }
}
#[doc = "Write proxy for field `PIN21`"]
pub struct PIN21_W<'a> {
    w: &'a mut W,
}
impl<'a> PIN21_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PIN21_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Criteria has not been met"]
    #[inline(always)]
    pub fn not_latched(self) -> &'a mut W {
        self.variant(PIN21_A::NOTLATCHED)
    }
    #[doc = "Criteria has been met"]
    #[inline(always)]
    pub fn latched(self) -> &'a mut W {
        self.variant(PIN21_A::LATCHED)
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
        self.w.bits = (self.w.bits & !(0x01 << 21)) | (((value as u32) & 0x01) << 21);
        self.w
    }
}
#[doc = "Status on whether PIN22 has met criteria set in PIN_CNF22.SENSE register. Write '1' to clear.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PIN22_A {
    #[doc = "0: Criteria has not been met"]
    NOTLATCHED = 0,
    #[doc = "1: Criteria has been met"]
    LATCHED = 1,
}
impl From<PIN22_A> for bool {
    #[inline(always)]
    fn from(variant: PIN22_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `PIN22`"]
pub type PIN22_R = crate::R<bool, PIN22_A>;
impl PIN22_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PIN22_A {
        match self.bits {
            false => PIN22_A::NOTLATCHED,
            true => PIN22_A::LATCHED,
        }
    }
    #[doc = "Checks if the value of the field is `NOTLATCHED`"]
    #[inline(always)]
    pub fn is_not_latched(&self) -> bool {
        *self == PIN22_A::NOTLATCHED
    }
    #[doc = "Checks if the value of the field is `LATCHED`"]
    #[inline(always)]
    pub fn is_latched(&self) -> bool {
        *self == PIN22_A::LATCHED
    }
}
#[doc = "Write proxy for field `PIN22`"]
pub struct PIN22_W<'a> {
    w: &'a mut W,
}
impl<'a> PIN22_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PIN22_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Criteria has not been met"]
    #[inline(always)]
    pub fn not_latched(self) -> &'a mut W {
        self.variant(PIN22_A::NOTLATCHED)
    }
    #[doc = "Criteria has been met"]
    #[inline(always)]
    pub fn latched(self) -> &'a mut W {
        self.variant(PIN22_A::LATCHED)
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
        self.w.bits = (self.w.bits & !(0x01 << 22)) | (((value as u32) & 0x01) << 22);
        self.w
    }
}
#[doc = "Status on whether PIN23 has met criteria set in PIN_CNF23.SENSE register. Write '1' to clear.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PIN23_A {
    #[doc = "0: Criteria has not been met"]
    NOTLATCHED = 0,
    #[doc = "1: Criteria has been met"]
    LATCHED = 1,
}
impl From<PIN23_A> for bool {
    #[inline(always)]
    fn from(variant: PIN23_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `PIN23`"]
pub type PIN23_R = crate::R<bool, PIN23_A>;
impl PIN23_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PIN23_A {
        match self.bits {
            false => PIN23_A::NOTLATCHED,
            true => PIN23_A::LATCHED,
        }
    }
    #[doc = "Checks if the value of the field is `NOTLATCHED`"]
    #[inline(always)]
    pub fn is_not_latched(&self) -> bool {
        *self == PIN23_A::NOTLATCHED
    }
    #[doc = "Checks if the value of the field is `LATCHED`"]
    #[inline(always)]
    pub fn is_latched(&self) -> bool {
        *self == PIN23_A::LATCHED
    }
}
#[doc = "Write proxy for field `PIN23`"]
pub struct PIN23_W<'a> {
    w: &'a mut W,
}
impl<'a> PIN23_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PIN23_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Criteria has not been met"]
    #[inline(always)]
    pub fn not_latched(self) -> &'a mut W {
        self.variant(PIN23_A::NOTLATCHED)
    }
    #[doc = "Criteria has been met"]
    #[inline(always)]
    pub fn latched(self) -> &'a mut W {
        self.variant(PIN23_A::LATCHED)
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
        self.w.bits = (self.w.bits & !(0x01 << 23)) | (((value as u32) & 0x01) << 23);
        self.w
    }
}
#[doc = "Status on whether PIN24 has met criteria set in PIN_CNF24.SENSE register. Write '1' to clear.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PIN24_A {
    #[doc = "0: Criteria has not been met"]
    NOTLATCHED = 0,
    #[doc = "1: Criteria has been met"]
    LATCHED = 1,
}
impl From<PIN24_A> for bool {
    #[inline(always)]
    fn from(variant: PIN24_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `PIN24`"]
pub type PIN24_R = crate::R<bool, PIN24_A>;
impl PIN24_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PIN24_A {
        match self.bits {
            false => PIN24_A::NOTLATCHED,
            true => PIN24_A::LATCHED,
        }
    }
    #[doc = "Checks if the value of the field is `NOTLATCHED`"]
    #[inline(always)]
    pub fn is_not_latched(&self) -> bool {
        *self == PIN24_A::NOTLATCHED
    }
    #[doc = "Checks if the value of the field is `LATCHED`"]
    #[inline(always)]
    pub fn is_latched(&self) -> bool {
        *self == PIN24_A::LATCHED
    }
}
#[doc = "Write proxy for field `PIN24`"]
pub struct PIN24_W<'a> {
    w: &'a mut W,
}
impl<'a> PIN24_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PIN24_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Criteria has not been met"]
    #[inline(always)]
    pub fn not_latched(self) -> &'a mut W {
        self.variant(PIN24_A::NOTLATCHED)
    }
    #[doc = "Criteria has been met"]
    #[inline(always)]
    pub fn latched(self) -> &'a mut W {
        self.variant(PIN24_A::LATCHED)
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
        self.w.bits = (self.w.bits & !(0x01 << 24)) | (((value as u32) & 0x01) << 24);
        self.w
    }
}
#[doc = "Status on whether PIN25 has met criteria set in PIN_CNF25.SENSE register. Write '1' to clear.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PIN25_A {
    #[doc = "0: Criteria has not been met"]
    NOTLATCHED = 0,
    #[doc = "1: Criteria has been met"]
    LATCHED = 1,
}
impl From<PIN25_A> for bool {
    #[inline(always)]
    fn from(variant: PIN25_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `PIN25`"]
pub type PIN25_R = crate::R<bool, PIN25_A>;
impl PIN25_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PIN25_A {
        match self.bits {
            false => PIN25_A::NOTLATCHED,
            true => PIN25_A::LATCHED,
        }
    }
    #[doc = "Checks if the value of the field is `NOTLATCHED`"]
    #[inline(always)]
    pub fn is_not_latched(&self) -> bool {
        *self == PIN25_A::NOTLATCHED
    }
    #[doc = "Checks if the value of the field is `LATCHED`"]
    #[inline(always)]
    pub fn is_latched(&self) -> bool {
        *self == PIN25_A::LATCHED
    }
}
#[doc = "Write proxy for field `PIN25`"]
pub struct PIN25_W<'a> {
    w: &'a mut W,
}
impl<'a> PIN25_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PIN25_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Criteria has not been met"]
    #[inline(always)]
    pub fn not_latched(self) -> &'a mut W {
        self.variant(PIN25_A::NOTLATCHED)
    }
    #[doc = "Criteria has been met"]
    #[inline(always)]
    pub fn latched(self) -> &'a mut W {
        self.variant(PIN25_A::LATCHED)
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
        self.w.bits = (self.w.bits & !(0x01 << 25)) | (((value as u32) & 0x01) << 25);
        self.w
    }
}
#[doc = "Status on whether PIN26 has met criteria set in PIN_CNF26.SENSE register. Write '1' to clear.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PIN26_A {
    #[doc = "0: Criteria has not been met"]
    NOTLATCHED = 0,
    #[doc = "1: Criteria has been met"]
    LATCHED = 1,
}
impl From<PIN26_A> for bool {
    #[inline(always)]
    fn from(variant: PIN26_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `PIN26`"]
pub type PIN26_R = crate::R<bool, PIN26_A>;
impl PIN26_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PIN26_A {
        match self.bits {
            false => PIN26_A::NOTLATCHED,
            true => PIN26_A::LATCHED,
        }
    }
    #[doc = "Checks if the value of the field is `NOTLATCHED`"]
    #[inline(always)]
    pub fn is_not_latched(&self) -> bool {
        *self == PIN26_A::NOTLATCHED
    }
    #[doc = "Checks if the value of the field is `LATCHED`"]
    #[inline(always)]
    pub fn is_latched(&self) -> bool {
        *self == PIN26_A::LATCHED
    }
}
#[doc = "Write proxy for field `PIN26`"]
pub struct PIN26_W<'a> {
    w: &'a mut W,
}
impl<'a> PIN26_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PIN26_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Criteria has not been met"]
    #[inline(always)]
    pub fn not_latched(self) -> &'a mut W {
        self.variant(PIN26_A::NOTLATCHED)
    }
    #[doc = "Criteria has been met"]
    #[inline(always)]
    pub fn latched(self) -> &'a mut W {
        self.variant(PIN26_A::LATCHED)
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
        self.w.bits = (self.w.bits & !(0x01 << 26)) | (((value as u32) & 0x01) << 26);
        self.w
    }
}
#[doc = "Status on whether PIN27 has met criteria set in PIN_CNF27.SENSE register. Write '1' to clear.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PIN27_A {
    #[doc = "0: Criteria has not been met"]
    NOTLATCHED = 0,
    #[doc = "1: Criteria has been met"]
    LATCHED = 1,
}
impl From<PIN27_A> for bool {
    #[inline(always)]
    fn from(variant: PIN27_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `PIN27`"]
pub type PIN27_R = crate::R<bool, PIN27_A>;
impl PIN27_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PIN27_A {
        match self.bits {
            false => PIN27_A::NOTLATCHED,
            true => PIN27_A::LATCHED,
        }
    }
    #[doc = "Checks if the value of the field is `NOTLATCHED`"]
    #[inline(always)]
    pub fn is_not_latched(&self) -> bool {
        *self == PIN27_A::NOTLATCHED
    }
    #[doc = "Checks if the value of the field is `LATCHED`"]
    #[inline(always)]
    pub fn is_latched(&self) -> bool {
        *self == PIN27_A::LATCHED
    }
}
#[doc = "Write proxy for field `PIN27`"]
pub struct PIN27_W<'a> {
    w: &'a mut W,
}
impl<'a> PIN27_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PIN27_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Criteria has not been met"]
    #[inline(always)]
    pub fn not_latched(self) -> &'a mut W {
        self.variant(PIN27_A::NOTLATCHED)
    }
    #[doc = "Criteria has been met"]
    #[inline(always)]
    pub fn latched(self) -> &'a mut W {
        self.variant(PIN27_A::LATCHED)
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
        self.w.bits = (self.w.bits & !(0x01 << 27)) | (((value as u32) & 0x01) << 27);
        self.w
    }
}
#[doc = "Status on whether PIN28 has met criteria set in PIN_CNF28.SENSE register. Write '1' to clear.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PIN28_A {
    #[doc = "0: Criteria has not been met"]
    NOTLATCHED = 0,
    #[doc = "1: Criteria has been met"]
    LATCHED = 1,
}
impl From<PIN28_A> for bool {
    #[inline(always)]
    fn from(variant: PIN28_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `PIN28`"]
pub type PIN28_R = crate::R<bool, PIN28_A>;
impl PIN28_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PIN28_A {
        match self.bits {
            false => PIN28_A::NOTLATCHED,
            true => PIN28_A::LATCHED,
        }
    }
    #[doc = "Checks if the value of the field is `NOTLATCHED`"]
    #[inline(always)]
    pub fn is_not_latched(&self) -> bool {
        *self == PIN28_A::NOTLATCHED
    }
    #[doc = "Checks if the value of the field is `LATCHED`"]
    #[inline(always)]
    pub fn is_latched(&self) -> bool {
        *self == PIN28_A::LATCHED
    }
}
#[doc = "Write proxy for field `PIN28`"]
pub struct PIN28_W<'a> {
    w: &'a mut W,
}
impl<'a> PIN28_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PIN28_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Criteria has not been met"]
    #[inline(always)]
    pub fn not_latched(self) -> &'a mut W {
        self.variant(PIN28_A::NOTLATCHED)
    }
    #[doc = "Criteria has been met"]
    #[inline(always)]
    pub fn latched(self) -> &'a mut W {
        self.variant(PIN28_A::LATCHED)
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
        self.w.bits = (self.w.bits & !(0x01 << 28)) | (((value as u32) & 0x01) << 28);
        self.w
    }
}
#[doc = "Status on whether PIN29 has met criteria set in PIN_CNF29.SENSE register. Write '1' to clear.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PIN29_A {
    #[doc = "0: Criteria has not been met"]
    NOTLATCHED = 0,
    #[doc = "1: Criteria has been met"]
    LATCHED = 1,
}
impl From<PIN29_A> for bool {
    #[inline(always)]
    fn from(variant: PIN29_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `PIN29`"]
pub type PIN29_R = crate::R<bool, PIN29_A>;
impl PIN29_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PIN29_A {
        match self.bits {
            false => PIN29_A::NOTLATCHED,
            true => PIN29_A::LATCHED,
        }
    }
    #[doc = "Checks if the value of the field is `NOTLATCHED`"]
    #[inline(always)]
    pub fn is_not_latched(&self) -> bool {
        *self == PIN29_A::NOTLATCHED
    }
    #[doc = "Checks if the value of the field is `LATCHED`"]
    #[inline(always)]
    pub fn is_latched(&self) -> bool {
        *self == PIN29_A::LATCHED
    }
}
#[doc = "Write proxy for field `PIN29`"]
pub struct PIN29_W<'a> {
    w: &'a mut W,
}
impl<'a> PIN29_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PIN29_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Criteria has not been met"]
    #[inline(always)]
    pub fn not_latched(self) -> &'a mut W {
        self.variant(PIN29_A::NOTLATCHED)
    }
    #[doc = "Criteria has been met"]
    #[inline(always)]
    pub fn latched(self) -> &'a mut W {
        self.variant(PIN29_A::LATCHED)
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
        self.w.bits = (self.w.bits & !(0x01 << 29)) | (((value as u32) & 0x01) << 29);
        self.w
    }
}
#[doc = "Status on whether PIN30 has met criteria set in PIN_CNF30.SENSE register. Write '1' to clear.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PIN30_A {
    #[doc = "0: Criteria has not been met"]
    NOTLATCHED = 0,
    #[doc = "1: Criteria has been met"]
    LATCHED = 1,
}
impl From<PIN30_A> for bool {
    #[inline(always)]
    fn from(variant: PIN30_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `PIN30`"]
pub type PIN30_R = crate::R<bool, PIN30_A>;
impl PIN30_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PIN30_A {
        match self.bits {
            false => PIN30_A::NOTLATCHED,
            true => PIN30_A::LATCHED,
        }
    }
    #[doc = "Checks if the value of the field is `NOTLATCHED`"]
    #[inline(always)]
    pub fn is_not_latched(&self) -> bool {
        *self == PIN30_A::NOTLATCHED
    }
    #[doc = "Checks if the value of the field is `LATCHED`"]
    #[inline(always)]
    pub fn is_latched(&self) -> bool {
        *self == PIN30_A::LATCHED
    }
}
#[doc = "Write proxy for field `PIN30`"]
pub struct PIN30_W<'a> {
    w: &'a mut W,
}
impl<'a> PIN30_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PIN30_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Criteria has not been met"]
    #[inline(always)]
    pub fn not_latched(self) -> &'a mut W {
        self.variant(PIN30_A::NOTLATCHED)
    }
    #[doc = "Criteria has been met"]
    #[inline(always)]
    pub fn latched(self) -> &'a mut W {
        self.variant(PIN30_A::LATCHED)
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
        self.w.bits = (self.w.bits & !(0x01 << 30)) | (((value as u32) & 0x01) << 30);
        self.w
    }
}
#[doc = "Status on whether PIN31 has met criteria set in PIN_CNF31.SENSE register. Write '1' to clear.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PIN31_A {
    #[doc = "0: Criteria has not been met"]
    NOTLATCHED = 0,
    #[doc = "1: Criteria has been met"]
    LATCHED = 1,
}
impl From<PIN31_A> for bool {
    #[inline(always)]
    fn from(variant: PIN31_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `PIN31`"]
pub type PIN31_R = crate::R<bool, PIN31_A>;
impl PIN31_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PIN31_A {
        match self.bits {
            false => PIN31_A::NOTLATCHED,
            true => PIN31_A::LATCHED,
        }
    }
    #[doc = "Checks if the value of the field is `NOTLATCHED`"]
    #[inline(always)]
    pub fn is_not_latched(&self) -> bool {
        *self == PIN31_A::NOTLATCHED
    }
    #[doc = "Checks if the value of the field is `LATCHED`"]
    #[inline(always)]
    pub fn is_latched(&self) -> bool {
        *self == PIN31_A::LATCHED
    }
}
#[doc = "Write proxy for field `PIN31`"]
pub struct PIN31_W<'a> {
    w: &'a mut W,
}
impl<'a> PIN31_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PIN31_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Criteria has not been met"]
    #[inline(always)]
    pub fn not_latched(self) -> &'a mut W {
        self.variant(PIN31_A::NOTLATCHED)
    }
    #[doc = "Criteria has been met"]
    #[inline(always)]
    pub fn latched(self) -> &'a mut W {
        self.variant(PIN31_A::LATCHED)
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
        self.w.bits = (self.w.bits & !(0x01 << 31)) | (((value as u32) & 0x01) << 31);
        self.w
    }
}
impl R {
    #[doc = "Bit 0 - Status on whether PIN0 has met criteria set in PIN_CNF0.SENSE register. Write '1' to clear."]
    #[inline(always)]
    pub fn pin0(&self) -> PIN0_R {
        PIN0_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - Status on whether PIN1 has met criteria set in PIN_CNF1.SENSE register. Write '1' to clear."]
    #[inline(always)]
    pub fn pin1(&self) -> PIN1_R {
        PIN1_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - Status on whether PIN2 has met criteria set in PIN_CNF2.SENSE register. Write '1' to clear."]
    #[inline(always)]
    pub fn pin2(&self) -> PIN2_R {
        PIN2_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - Status on whether PIN3 has met criteria set in PIN_CNF3.SENSE register. Write '1' to clear."]
    #[inline(always)]
    pub fn pin3(&self) -> PIN3_R {
        PIN3_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - Status on whether PIN4 has met criteria set in PIN_CNF4.SENSE register. Write '1' to clear."]
    #[inline(always)]
    pub fn pin4(&self) -> PIN4_R {
        PIN4_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - Status on whether PIN5 has met criteria set in PIN_CNF5.SENSE register. Write '1' to clear."]
    #[inline(always)]
    pub fn pin5(&self) -> PIN5_R {
        PIN5_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - Status on whether PIN6 has met criteria set in PIN_CNF6.SENSE register. Write '1' to clear."]
    #[inline(always)]
    pub fn pin6(&self) -> PIN6_R {
        PIN6_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7 - Status on whether PIN7 has met criteria set in PIN_CNF7.SENSE register. Write '1' to clear."]
    #[inline(always)]
    pub fn pin7(&self) -> PIN7_R {
        PIN7_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 8 - Status on whether PIN8 has met criteria set in PIN_CNF8.SENSE register. Write '1' to clear."]
    #[inline(always)]
    pub fn pin8(&self) -> PIN8_R {
        PIN8_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 9 - Status on whether PIN9 has met criteria set in PIN_CNF9.SENSE register. Write '1' to clear."]
    #[inline(always)]
    pub fn pin9(&self) -> PIN9_R {
        PIN9_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 10 - Status on whether PIN10 has met criteria set in PIN_CNF10.SENSE register. Write '1' to clear."]
    #[inline(always)]
    pub fn pin10(&self) -> PIN10_R {
        PIN10_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    #[doc = "Bit 11 - Status on whether PIN11 has met criteria set in PIN_CNF11.SENSE register. Write '1' to clear."]
    #[inline(always)]
    pub fn pin11(&self) -> PIN11_R {
        PIN11_R::new(((self.bits >> 11) & 0x01) != 0)
    }
    #[doc = "Bit 12 - Status on whether PIN12 has met criteria set in PIN_CNF12.SENSE register. Write '1' to clear."]
    #[inline(always)]
    pub fn pin12(&self) -> PIN12_R {
        PIN12_R::new(((self.bits >> 12) & 0x01) != 0)
    }
    #[doc = "Bit 13 - Status on whether PIN13 has met criteria set in PIN_CNF13.SENSE register. Write '1' to clear."]
    #[inline(always)]
    pub fn pin13(&self) -> PIN13_R {
        PIN13_R::new(((self.bits >> 13) & 0x01) != 0)
    }
    #[doc = "Bit 14 - Status on whether PIN14 has met criteria set in PIN_CNF14.SENSE register. Write '1' to clear."]
    #[inline(always)]
    pub fn pin14(&self) -> PIN14_R {
        PIN14_R::new(((self.bits >> 14) & 0x01) != 0)
    }
    #[doc = "Bit 15 - Status on whether PIN15 has met criteria set in PIN_CNF15.SENSE register. Write '1' to clear."]
    #[inline(always)]
    pub fn pin15(&self) -> PIN15_R {
        PIN15_R::new(((self.bits >> 15) & 0x01) != 0)
    }
    #[doc = "Bit 16 - Status on whether PIN16 has met criteria set in PIN_CNF16.SENSE register. Write '1' to clear."]
    #[inline(always)]
    pub fn pin16(&self) -> PIN16_R {
        PIN16_R::new(((self.bits >> 16) & 0x01) != 0)
    }
    #[doc = "Bit 17 - Status on whether PIN17 has met criteria set in PIN_CNF17.SENSE register. Write '1' to clear."]
    #[inline(always)]
    pub fn pin17(&self) -> PIN17_R {
        PIN17_R::new(((self.bits >> 17) & 0x01) != 0)
    }
    #[doc = "Bit 18 - Status on whether PIN18 has met criteria set in PIN_CNF18.SENSE register. Write '1' to clear."]
    #[inline(always)]
    pub fn pin18(&self) -> PIN18_R {
        PIN18_R::new(((self.bits >> 18) & 0x01) != 0)
    }
    #[doc = "Bit 19 - Status on whether PIN19 has met criteria set in PIN_CNF19.SENSE register. Write '1' to clear."]
    #[inline(always)]
    pub fn pin19(&self) -> PIN19_R {
        PIN19_R::new(((self.bits >> 19) & 0x01) != 0)
    }
    #[doc = "Bit 20 - Status on whether PIN20 has met criteria set in PIN_CNF20.SENSE register. Write '1' to clear."]
    #[inline(always)]
    pub fn pin20(&self) -> PIN20_R {
        PIN20_R::new(((self.bits >> 20) & 0x01) != 0)
    }
    #[doc = "Bit 21 - Status on whether PIN21 has met criteria set in PIN_CNF21.SENSE register. Write '1' to clear."]
    #[inline(always)]
    pub fn pin21(&self) -> PIN21_R {
        PIN21_R::new(((self.bits >> 21) & 0x01) != 0)
    }
    #[doc = "Bit 22 - Status on whether PIN22 has met criteria set in PIN_CNF22.SENSE register. Write '1' to clear."]
    #[inline(always)]
    pub fn pin22(&self) -> PIN22_R {
        PIN22_R::new(((self.bits >> 22) & 0x01) != 0)
    }
    #[doc = "Bit 23 - Status on whether PIN23 has met criteria set in PIN_CNF23.SENSE register. Write '1' to clear."]
    #[inline(always)]
    pub fn pin23(&self) -> PIN23_R {
        PIN23_R::new(((self.bits >> 23) & 0x01) != 0)
    }
    #[doc = "Bit 24 - Status on whether PIN24 has met criteria set in PIN_CNF24.SENSE register. Write '1' to clear."]
    #[inline(always)]
    pub fn pin24(&self) -> PIN24_R {
        PIN24_R::new(((self.bits >> 24) & 0x01) != 0)
    }
    #[doc = "Bit 25 - Status on whether PIN25 has met criteria set in PIN_CNF25.SENSE register. Write '1' to clear."]
    #[inline(always)]
    pub fn pin25(&self) -> PIN25_R {
        PIN25_R::new(((self.bits >> 25) & 0x01) != 0)
    }
    #[doc = "Bit 26 - Status on whether PIN26 has met criteria set in PIN_CNF26.SENSE register. Write '1' to clear."]
    #[inline(always)]
    pub fn pin26(&self) -> PIN26_R {
        PIN26_R::new(((self.bits >> 26) & 0x01) != 0)
    }
    #[doc = "Bit 27 - Status on whether PIN27 has met criteria set in PIN_CNF27.SENSE register. Write '1' to clear."]
    #[inline(always)]
    pub fn pin27(&self) -> PIN27_R {
        PIN27_R::new(((self.bits >> 27) & 0x01) != 0)
    }
    #[doc = "Bit 28 - Status on whether PIN28 has met criteria set in PIN_CNF28.SENSE register. Write '1' to clear."]
    #[inline(always)]
    pub fn pin28(&self) -> PIN28_R {
        PIN28_R::new(((self.bits >> 28) & 0x01) != 0)
    }
    #[doc = "Bit 29 - Status on whether PIN29 has met criteria set in PIN_CNF29.SENSE register. Write '1' to clear."]
    #[inline(always)]
    pub fn pin29(&self) -> PIN29_R {
        PIN29_R::new(((self.bits >> 29) & 0x01) != 0)
    }
    #[doc = "Bit 30 - Status on whether PIN30 has met criteria set in PIN_CNF30.SENSE register. Write '1' to clear."]
    #[inline(always)]
    pub fn pin30(&self) -> PIN30_R {
        PIN30_R::new(((self.bits >> 30) & 0x01) != 0)
    }
    #[doc = "Bit 31 - Status on whether PIN31 has met criteria set in PIN_CNF31.SENSE register. Write '1' to clear."]
    #[inline(always)]
    pub fn pin31(&self) -> PIN31_R {
        PIN31_R::new(((self.bits >> 31) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Status on whether PIN0 has met criteria set in PIN_CNF0.SENSE register. Write '1' to clear."]
    #[inline(always)]
    pub fn pin0(&mut self) -> PIN0_W {
        PIN0_W { w: self }
    }
    #[doc = "Bit 1 - Status on whether PIN1 has met criteria set in PIN_CNF1.SENSE register. Write '1' to clear."]
    #[inline(always)]
    pub fn pin1(&mut self) -> PIN1_W {
        PIN1_W { w: self }
    }
    #[doc = "Bit 2 - Status on whether PIN2 has met criteria set in PIN_CNF2.SENSE register. Write '1' to clear."]
    #[inline(always)]
    pub fn pin2(&mut self) -> PIN2_W {
        PIN2_W { w: self }
    }
    #[doc = "Bit 3 - Status on whether PIN3 has met criteria set in PIN_CNF3.SENSE register. Write '1' to clear."]
    #[inline(always)]
    pub fn pin3(&mut self) -> PIN3_W {
        PIN3_W { w: self }
    }
    #[doc = "Bit 4 - Status on whether PIN4 has met criteria set in PIN_CNF4.SENSE register. Write '1' to clear."]
    #[inline(always)]
    pub fn pin4(&mut self) -> PIN4_W {
        PIN4_W { w: self }
    }
    #[doc = "Bit 5 - Status on whether PIN5 has met criteria set in PIN_CNF5.SENSE register. Write '1' to clear."]
    #[inline(always)]
    pub fn pin5(&mut self) -> PIN5_W {
        PIN5_W { w: self }
    }
    #[doc = "Bit 6 - Status on whether PIN6 has met criteria set in PIN_CNF6.SENSE register. Write '1' to clear."]
    #[inline(always)]
    pub fn pin6(&mut self) -> PIN6_W {
        PIN6_W { w: self }
    }
    #[doc = "Bit 7 - Status on whether PIN7 has met criteria set in PIN_CNF7.SENSE register. Write '1' to clear."]
    #[inline(always)]
    pub fn pin7(&mut self) -> PIN7_W {
        PIN7_W { w: self }
    }
    #[doc = "Bit 8 - Status on whether PIN8 has met criteria set in PIN_CNF8.SENSE register. Write '1' to clear."]
    #[inline(always)]
    pub fn pin8(&mut self) -> PIN8_W {
        PIN8_W { w: self }
    }
    #[doc = "Bit 9 - Status on whether PIN9 has met criteria set in PIN_CNF9.SENSE register. Write '1' to clear."]
    #[inline(always)]
    pub fn pin9(&mut self) -> PIN9_W {
        PIN9_W { w: self }
    }
    #[doc = "Bit 10 - Status on whether PIN10 has met criteria set in PIN_CNF10.SENSE register. Write '1' to clear."]
    #[inline(always)]
    pub fn pin10(&mut self) -> PIN10_W {
        PIN10_W { w: self }
    }
    #[doc = "Bit 11 - Status on whether PIN11 has met criteria set in PIN_CNF11.SENSE register. Write '1' to clear."]
    #[inline(always)]
    pub fn pin11(&mut self) -> PIN11_W {
        PIN11_W { w: self }
    }
    #[doc = "Bit 12 - Status on whether PIN12 has met criteria set in PIN_CNF12.SENSE register. Write '1' to clear."]
    #[inline(always)]
    pub fn pin12(&mut self) -> PIN12_W {
        PIN12_W { w: self }
    }
    #[doc = "Bit 13 - Status on whether PIN13 has met criteria set in PIN_CNF13.SENSE register. Write '1' to clear."]
    #[inline(always)]
    pub fn pin13(&mut self) -> PIN13_W {
        PIN13_W { w: self }
    }
    #[doc = "Bit 14 - Status on whether PIN14 has met criteria set in PIN_CNF14.SENSE register. Write '1' to clear."]
    #[inline(always)]
    pub fn pin14(&mut self) -> PIN14_W {
        PIN14_W { w: self }
    }
    #[doc = "Bit 15 - Status on whether PIN15 has met criteria set in PIN_CNF15.SENSE register. Write '1' to clear."]
    #[inline(always)]
    pub fn pin15(&mut self) -> PIN15_W {
        PIN15_W { w: self }
    }
    #[doc = "Bit 16 - Status on whether PIN16 has met criteria set in PIN_CNF16.SENSE register. Write '1' to clear."]
    #[inline(always)]
    pub fn pin16(&mut self) -> PIN16_W {
        PIN16_W { w: self }
    }
    #[doc = "Bit 17 - Status on whether PIN17 has met criteria set in PIN_CNF17.SENSE register. Write '1' to clear."]
    #[inline(always)]
    pub fn pin17(&mut self) -> PIN17_W {
        PIN17_W { w: self }
    }
    #[doc = "Bit 18 - Status on whether PIN18 has met criteria set in PIN_CNF18.SENSE register. Write '1' to clear."]
    #[inline(always)]
    pub fn pin18(&mut self) -> PIN18_W {
        PIN18_W { w: self }
    }
    #[doc = "Bit 19 - Status on whether PIN19 has met criteria set in PIN_CNF19.SENSE register. Write '1' to clear."]
    #[inline(always)]
    pub fn pin19(&mut self) -> PIN19_W {
        PIN19_W { w: self }
    }
    #[doc = "Bit 20 - Status on whether PIN20 has met criteria set in PIN_CNF20.SENSE register. Write '1' to clear."]
    #[inline(always)]
    pub fn pin20(&mut self) -> PIN20_W {
        PIN20_W { w: self }
    }
    #[doc = "Bit 21 - Status on whether PIN21 has met criteria set in PIN_CNF21.SENSE register. Write '1' to clear."]
    #[inline(always)]
    pub fn pin21(&mut self) -> PIN21_W {
        PIN21_W { w: self }
    }
    #[doc = "Bit 22 - Status on whether PIN22 has met criteria set in PIN_CNF22.SENSE register. Write '1' to clear."]
    #[inline(always)]
    pub fn pin22(&mut self) -> PIN22_W {
        PIN22_W { w: self }
    }
    #[doc = "Bit 23 - Status on whether PIN23 has met criteria set in PIN_CNF23.SENSE register. Write '1' to clear."]
    #[inline(always)]
    pub fn pin23(&mut self) -> PIN23_W {
        PIN23_W { w: self }
    }
    #[doc = "Bit 24 - Status on whether PIN24 has met criteria set in PIN_CNF24.SENSE register. Write '1' to clear."]
    #[inline(always)]
    pub fn pin24(&mut self) -> PIN24_W {
        PIN24_W { w: self }
    }
    #[doc = "Bit 25 - Status on whether PIN25 has met criteria set in PIN_CNF25.SENSE register. Write '1' to clear."]
    #[inline(always)]
    pub fn pin25(&mut self) -> PIN25_W {
        PIN25_W { w: self }
    }
    #[doc = "Bit 26 - Status on whether PIN26 has met criteria set in PIN_CNF26.SENSE register. Write '1' to clear."]
    #[inline(always)]
    pub fn pin26(&mut self) -> PIN26_W {
        PIN26_W { w: self }
    }
    #[doc = "Bit 27 - Status on whether PIN27 has met criteria set in PIN_CNF27.SENSE register. Write '1' to clear."]
    #[inline(always)]
    pub fn pin27(&mut self) -> PIN27_W {
        PIN27_W { w: self }
    }
    #[doc = "Bit 28 - Status on whether PIN28 has met criteria set in PIN_CNF28.SENSE register. Write '1' to clear."]
    #[inline(always)]
    pub fn pin28(&mut self) -> PIN28_W {
        PIN28_W { w: self }
    }
    #[doc = "Bit 29 - Status on whether PIN29 has met criteria set in PIN_CNF29.SENSE register. Write '1' to clear."]
    #[inline(always)]
    pub fn pin29(&mut self) -> PIN29_W {
        PIN29_W { w: self }
    }
    #[doc = "Bit 30 - Status on whether PIN30 has met criteria set in PIN_CNF30.SENSE register. Write '1' to clear."]
    #[inline(always)]
    pub fn pin30(&mut self) -> PIN30_W {
        PIN30_W { w: self }
    }
    #[doc = "Bit 31 - Status on whether PIN31 has met criteria set in PIN_CNF31.SENSE register. Write '1' to clear."]
    #[inline(always)]
    pub fn pin31(&mut self) -> PIN31_W {
        PIN31_W { w: self }
    }
}
