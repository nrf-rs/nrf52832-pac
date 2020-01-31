#[doc = "Reader of register SHORTS"]
pub type R = crate::R<u32, super::SHORTS>;
#[doc = "Writer for register SHORTS"]
pub type W = crate::W<u32, super::SHORTS>;
#[doc = "Register SHORTS `reset()`'s with value 0"]
impl crate::ResetValue for super::SHORTS {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Shortcut between READY event and START task\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum READY_START_A {
    #[doc = "0: Disable shortcut"]
    DISABLED = 0,
    #[doc = "1: Enable shortcut"]
    ENABLED = 1,
}
impl From<READY_START_A> for bool {
    #[inline(always)]
    fn from(variant: READY_START_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `READY_START`"]
pub type READY_START_R = crate::R<bool, READY_START_A>;
impl READY_START_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> READY_START_A {
        match self.bits {
            false => READY_START_A::DISABLED,
            true => READY_START_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == READY_START_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == READY_START_A::ENABLED
    }
}
#[doc = "Write proxy for field `READY_START`"]
pub struct READY_START_W<'a> {
    w: &'a mut W,
}
impl<'a> READY_START_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: READY_START_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Disable shortcut"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(READY_START_A::DISABLED)
    }
    #[doc = "Enable shortcut"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(READY_START_A::ENABLED)
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
#[doc = "Shortcut between END event and DISABLE task\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum END_DISABLE_A {
    #[doc = "0: Disable shortcut"]
    DISABLED = 0,
    #[doc = "1: Enable shortcut"]
    ENABLED = 1,
}
impl From<END_DISABLE_A> for bool {
    #[inline(always)]
    fn from(variant: END_DISABLE_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `END_DISABLE`"]
pub type END_DISABLE_R = crate::R<bool, END_DISABLE_A>;
impl END_DISABLE_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> END_DISABLE_A {
        match self.bits {
            false => END_DISABLE_A::DISABLED,
            true => END_DISABLE_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == END_DISABLE_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == END_DISABLE_A::ENABLED
    }
}
#[doc = "Write proxy for field `END_DISABLE`"]
pub struct END_DISABLE_W<'a> {
    w: &'a mut W,
}
impl<'a> END_DISABLE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: END_DISABLE_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Disable shortcut"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(END_DISABLE_A::DISABLED)
    }
    #[doc = "Enable shortcut"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(END_DISABLE_A::ENABLED)
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
#[doc = "Shortcut between DISABLED event and TXEN task\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DISABLED_TXEN_A {
    #[doc = "0: Disable shortcut"]
    DISABLED = 0,
    #[doc = "1: Enable shortcut"]
    ENABLED = 1,
}
impl From<DISABLED_TXEN_A> for bool {
    #[inline(always)]
    fn from(variant: DISABLED_TXEN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `DISABLED_TXEN`"]
pub type DISABLED_TXEN_R = crate::R<bool, DISABLED_TXEN_A>;
impl DISABLED_TXEN_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DISABLED_TXEN_A {
        match self.bits {
            false => DISABLED_TXEN_A::DISABLED,
            true => DISABLED_TXEN_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == DISABLED_TXEN_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == DISABLED_TXEN_A::ENABLED
    }
}
#[doc = "Write proxy for field `DISABLED_TXEN`"]
pub struct DISABLED_TXEN_W<'a> {
    w: &'a mut W,
}
impl<'a> DISABLED_TXEN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: DISABLED_TXEN_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Disable shortcut"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(DISABLED_TXEN_A::DISABLED)
    }
    #[doc = "Enable shortcut"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(DISABLED_TXEN_A::ENABLED)
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
#[doc = "Shortcut between DISABLED event and RXEN task\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DISABLED_RXEN_A {
    #[doc = "0: Disable shortcut"]
    DISABLED = 0,
    #[doc = "1: Enable shortcut"]
    ENABLED = 1,
}
impl From<DISABLED_RXEN_A> for bool {
    #[inline(always)]
    fn from(variant: DISABLED_RXEN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `DISABLED_RXEN`"]
pub type DISABLED_RXEN_R = crate::R<bool, DISABLED_RXEN_A>;
impl DISABLED_RXEN_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DISABLED_RXEN_A {
        match self.bits {
            false => DISABLED_RXEN_A::DISABLED,
            true => DISABLED_RXEN_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == DISABLED_RXEN_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == DISABLED_RXEN_A::ENABLED
    }
}
#[doc = "Write proxy for field `DISABLED_RXEN`"]
pub struct DISABLED_RXEN_W<'a> {
    w: &'a mut W,
}
impl<'a> DISABLED_RXEN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: DISABLED_RXEN_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Disable shortcut"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(DISABLED_RXEN_A::DISABLED)
    }
    #[doc = "Enable shortcut"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(DISABLED_RXEN_A::ENABLED)
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
#[doc = "Shortcut between ADDRESS event and RSSISTART task\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ADDRESS_RSSISTART_A {
    #[doc = "0: Disable shortcut"]
    DISABLED = 0,
    #[doc = "1: Enable shortcut"]
    ENABLED = 1,
}
impl From<ADDRESS_RSSISTART_A> for bool {
    #[inline(always)]
    fn from(variant: ADDRESS_RSSISTART_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `ADDRESS_RSSISTART`"]
pub type ADDRESS_RSSISTART_R = crate::R<bool, ADDRESS_RSSISTART_A>;
impl ADDRESS_RSSISTART_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ADDRESS_RSSISTART_A {
        match self.bits {
            false => ADDRESS_RSSISTART_A::DISABLED,
            true => ADDRESS_RSSISTART_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == ADDRESS_RSSISTART_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == ADDRESS_RSSISTART_A::ENABLED
    }
}
#[doc = "Write proxy for field `ADDRESS_RSSISTART`"]
pub struct ADDRESS_RSSISTART_W<'a> {
    w: &'a mut W,
}
impl<'a> ADDRESS_RSSISTART_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: ADDRESS_RSSISTART_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Disable shortcut"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(ADDRESS_RSSISTART_A::DISABLED)
    }
    #[doc = "Enable shortcut"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(ADDRESS_RSSISTART_A::ENABLED)
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
#[doc = "Shortcut between END event and START task\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum END_START_A {
    #[doc = "0: Disable shortcut"]
    DISABLED = 0,
    #[doc = "1: Enable shortcut"]
    ENABLED = 1,
}
impl From<END_START_A> for bool {
    #[inline(always)]
    fn from(variant: END_START_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `END_START`"]
pub type END_START_R = crate::R<bool, END_START_A>;
impl END_START_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> END_START_A {
        match self.bits {
            false => END_START_A::DISABLED,
            true => END_START_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == END_START_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == END_START_A::ENABLED
    }
}
#[doc = "Write proxy for field `END_START`"]
pub struct END_START_W<'a> {
    w: &'a mut W,
}
impl<'a> END_START_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: END_START_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Disable shortcut"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(END_START_A::DISABLED)
    }
    #[doc = "Enable shortcut"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(END_START_A::ENABLED)
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
#[doc = "Shortcut between ADDRESS event and BCSTART task\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ADDRESS_BCSTART_A {
    #[doc = "0: Disable shortcut"]
    DISABLED = 0,
    #[doc = "1: Enable shortcut"]
    ENABLED = 1,
}
impl From<ADDRESS_BCSTART_A> for bool {
    #[inline(always)]
    fn from(variant: ADDRESS_BCSTART_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `ADDRESS_BCSTART`"]
pub type ADDRESS_BCSTART_R = crate::R<bool, ADDRESS_BCSTART_A>;
impl ADDRESS_BCSTART_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ADDRESS_BCSTART_A {
        match self.bits {
            false => ADDRESS_BCSTART_A::DISABLED,
            true => ADDRESS_BCSTART_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == ADDRESS_BCSTART_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == ADDRESS_BCSTART_A::ENABLED
    }
}
#[doc = "Write proxy for field `ADDRESS_BCSTART`"]
pub struct ADDRESS_BCSTART_W<'a> {
    w: &'a mut W,
}
impl<'a> ADDRESS_BCSTART_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: ADDRESS_BCSTART_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Disable shortcut"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(ADDRESS_BCSTART_A::DISABLED)
    }
    #[doc = "Enable shortcut"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(ADDRESS_BCSTART_A::ENABLED)
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
#[doc = "Shortcut between DISABLED event and RSSISTOP task\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DISABLED_RSSISTOP_A {
    #[doc = "0: Disable shortcut"]
    DISABLED = 0,
    #[doc = "1: Enable shortcut"]
    ENABLED = 1,
}
impl From<DISABLED_RSSISTOP_A> for bool {
    #[inline(always)]
    fn from(variant: DISABLED_RSSISTOP_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `DISABLED_RSSISTOP`"]
pub type DISABLED_RSSISTOP_R = crate::R<bool, DISABLED_RSSISTOP_A>;
impl DISABLED_RSSISTOP_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DISABLED_RSSISTOP_A {
        match self.bits {
            false => DISABLED_RSSISTOP_A::DISABLED,
            true => DISABLED_RSSISTOP_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == DISABLED_RSSISTOP_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == DISABLED_RSSISTOP_A::ENABLED
    }
}
#[doc = "Write proxy for field `DISABLED_RSSISTOP`"]
pub struct DISABLED_RSSISTOP_W<'a> {
    w: &'a mut W,
}
impl<'a> DISABLED_RSSISTOP_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: DISABLED_RSSISTOP_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Disable shortcut"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(DISABLED_RSSISTOP_A::DISABLED)
    }
    #[doc = "Enable shortcut"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(DISABLED_RSSISTOP_A::ENABLED)
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
impl R {
    #[doc = "Bit 0 - Shortcut between READY event and START task"]
    #[inline(always)]
    pub fn ready_start(&self) -> READY_START_R {
        READY_START_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - Shortcut between END event and DISABLE task"]
    #[inline(always)]
    pub fn end_disable(&self) -> END_DISABLE_R {
        END_DISABLE_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - Shortcut between DISABLED event and TXEN task"]
    #[inline(always)]
    pub fn disabled_txen(&self) -> DISABLED_TXEN_R {
        DISABLED_TXEN_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - Shortcut between DISABLED event and RXEN task"]
    #[inline(always)]
    pub fn disabled_rxen(&self) -> DISABLED_RXEN_R {
        DISABLED_RXEN_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - Shortcut between ADDRESS event and RSSISTART task"]
    #[inline(always)]
    pub fn address_rssistart(&self) -> ADDRESS_RSSISTART_R {
        ADDRESS_RSSISTART_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - Shortcut between END event and START task"]
    #[inline(always)]
    pub fn end_start(&self) -> END_START_R {
        END_START_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - Shortcut between ADDRESS event and BCSTART task"]
    #[inline(always)]
    pub fn address_bcstart(&self) -> ADDRESS_BCSTART_R {
        ADDRESS_BCSTART_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 8 - Shortcut between DISABLED event and RSSISTOP task"]
    #[inline(always)]
    pub fn disabled_rssistop(&self) -> DISABLED_RSSISTOP_R {
        DISABLED_RSSISTOP_R::new(((self.bits >> 8) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Shortcut between READY event and START task"]
    #[inline(always)]
    pub fn ready_start(&mut self) -> READY_START_W {
        READY_START_W { w: self }
    }
    #[doc = "Bit 1 - Shortcut between END event and DISABLE task"]
    #[inline(always)]
    pub fn end_disable(&mut self) -> END_DISABLE_W {
        END_DISABLE_W { w: self }
    }
    #[doc = "Bit 2 - Shortcut between DISABLED event and TXEN task"]
    #[inline(always)]
    pub fn disabled_txen(&mut self) -> DISABLED_TXEN_W {
        DISABLED_TXEN_W { w: self }
    }
    #[doc = "Bit 3 - Shortcut between DISABLED event and RXEN task"]
    #[inline(always)]
    pub fn disabled_rxen(&mut self) -> DISABLED_RXEN_W {
        DISABLED_RXEN_W { w: self }
    }
    #[doc = "Bit 4 - Shortcut between ADDRESS event and RSSISTART task"]
    #[inline(always)]
    pub fn address_rssistart(&mut self) -> ADDRESS_RSSISTART_W {
        ADDRESS_RSSISTART_W { w: self }
    }
    #[doc = "Bit 5 - Shortcut between END event and START task"]
    #[inline(always)]
    pub fn end_start(&mut self) -> END_START_W {
        END_START_W { w: self }
    }
    #[doc = "Bit 6 - Shortcut between ADDRESS event and BCSTART task"]
    #[inline(always)]
    pub fn address_bcstart(&mut self) -> ADDRESS_BCSTART_W {
        ADDRESS_BCSTART_W { w: self }
    }
    #[doc = "Bit 8 - Shortcut between DISABLED event and RSSISTOP task"]
    #[inline(always)]
    pub fn disabled_rssistop(&mut self) -> DISABLED_RSSISTOP_W {
        DISABLED_RSSISTOP_W { w: self }
    }
}
