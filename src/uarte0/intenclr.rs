#[doc = "Reader of register INTENCLR"]
pub type R = crate::R<u32, super::INTENCLR>;
#[doc = "Writer for register INTENCLR"]
pub type W = crate::W<u32, super::INTENCLR>;
#[doc = "Register INTENCLR `reset()`'s with value 0"]
impl crate::ResetValue for super::INTENCLR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Write '1' to Disable interrupt for CTS event\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CTS_A {
    #[doc = "0: Read: Disabled"]
    DISABLED = 0,
    #[doc = "1: Read: Enabled"]
    ENABLED = 1,
}
impl From<CTS_A> for bool {
    #[inline(always)]
    fn from(variant: CTS_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `CTS`"]
pub type CTS_R = crate::R<bool, CTS_A>;
impl CTS_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CTS_A {
        match self.bits {
            false => CTS_A::DISABLED,
            true => CTS_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == CTS_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == CTS_A::ENABLED
    }
}
#[doc = "Write '1' to Disable interrupt for CTS event\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CTS_AW {
    #[doc = "1: Disable"]
    CLEAR = 1,
}
impl From<CTS_AW> for bool {
    #[inline(always)]
    fn from(variant: CTS_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Write proxy for field `CTS`"]
pub struct CTS_W<'a> {
    w: &'a mut W,
}
impl<'a> CTS_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CTS_AW) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Disable"]
    #[inline(always)]
    pub fn clear(self) -> &'a mut W {
        self.variant(CTS_AW::CLEAR)
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
#[doc = "Write '1' to Disable interrupt for NCTS event\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum NCTS_A {
    #[doc = "0: Read: Disabled"]
    DISABLED = 0,
    #[doc = "1: Read: Enabled"]
    ENABLED = 1,
}
impl From<NCTS_A> for bool {
    #[inline(always)]
    fn from(variant: NCTS_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `NCTS`"]
pub type NCTS_R = crate::R<bool, NCTS_A>;
impl NCTS_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> NCTS_A {
        match self.bits {
            false => NCTS_A::DISABLED,
            true => NCTS_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == NCTS_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == NCTS_A::ENABLED
    }
}
#[doc = "Write '1' to Disable interrupt for NCTS event\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum NCTS_AW {
    #[doc = "1: Disable"]
    CLEAR = 1,
}
impl From<NCTS_AW> for bool {
    #[inline(always)]
    fn from(variant: NCTS_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Write proxy for field `NCTS`"]
pub struct NCTS_W<'a> {
    w: &'a mut W,
}
impl<'a> NCTS_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: NCTS_AW) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Disable"]
    #[inline(always)]
    pub fn clear(self) -> &'a mut W {
        self.variant(NCTS_AW::CLEAR)
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
#[doc = "Write '1' to Disable interrupt for RXDRDY event\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RXDRDY_A {
    #[doc = "0: Read: Disabled"]
    DISABLED = 0,
    #[doc = "1: Read: Enabled"]
    ENABLED = 1,
}
impl From<RXDRDY_A> for bool {
    #[inline(always)]
    fn from(variant: RXDRDY_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `RXDRDY`"]
pub type RXDRDY_R = crate::R<bool, RXDRDY_A>;
impl RXDRDY_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RXDRDY_A {
        match self.bits {
            false => RXDRDY_A::DISABLED,
            true => RXDRDY_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == RXDRDY_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == RXDRDY_A::ENABLED
    }
}
#[doc = "Write '1' to Disable interrupt for RXDRDY event\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RXDRDY_AW {
    #[doc = "1: Disable"]
    CLEAR = 1,
}
impl From<RXDRDY_AW> for bool {
    #[inline(always)]
    fn from(variant: RXDRDY_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Write proxy for field `RXDRDY`"]
pub struct RXDRDY_W<'a> {
    w: &'a mut W,
}
impl<'a> RXDRDY_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: RXDRDY_AW) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Disable"]
    #[inline(always)]
    pub fn clear(self) -> &'a mut W {
        self.variant(RXDRDY_AW::CLEAR)
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
#[doc = "Write '1' to Disable interrupt for ENDRX event\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ENDRX_A {
    #[doc = "0: Read: Disabled"]
    DISABLED = 0,
    #[doc = "1: Read: Enabled"]
    ENABLED = 1,
}
impl From<ENDRX_A> for bool {
    #[inline(always)]
    fn from(variant: ENDRX_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `ENDRX`"]
pub type ENDRX_R = crate::R<bool, ENDRX_A>;
impl ENDRX_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ENDRX_A {
        match self.bits {
            false => ENDRX_A::DISABLED,
            true => ENDRX_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == ENDRX_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == ENDRX_A::ENABLED
    }
}
#[doc = "Write '1' to Disable interrupt for ENDRX event\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ENDRX_AW {
    #[doc = "1: Disable"]
    CLEAR = 1,
}
impl From<ENDRX_AW> for bool {
    #[inline(always)]
    fn from(variant: ENDRX_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Write proxy for field `ENDRX`"]
pub struct ENDRX_W<'a> {
    w: &'a mut W,
}
impl<'a> ENDRX_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: ENDRX_AW) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Disable"]
    #[inline(always)]
    pub fn clear(self) -> &'a mut W {
        self.variant(ENDRX_AW::CLEAR)
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
#[doc = "Write '1' to Disable interrupt for TXDRDY event\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TXDRDY_A {
    #[doc = "0: Read: Disabled"]
    DISABLED = 0,
    #[doc = "1: Read: Enabled"]
    ENABLED = 1,
}
impl From<TXDRDY_A> for bool {
    #[inline(always)]
    fn from(variant: TXDRDY_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `TXDRDY`"]
pub type TXDRDY_R = crate::R<bool, TXDRDY_A>;
impl TXDRDY_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TXDRDY_A {
        match self.bits {
            false => TXDRDY_A::DISABLED,
            true => TXDRDY_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == TXDRDY_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == TXDRDY_A::ENABLED
    }
}
#[doc = "Write '1' to Disable interrupt for TXDRDY event\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TXDRDY_AW {
    #[doc = "1: Disable"]
    CLEAR = 1,
}
impl From<TXDRDY_AW> for bool {
    #[inline(always)]
    fn from(variant: TXDRDY_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Write proxy for field `TXDRDY`"]
pub struct TXDRDY_W<'a> {
    w: &'a mut W,
}
impl<'a> TXDRDY_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: TXDRDY_AW) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Disable"]
    #[inline(always)]
    pub fn clear(self) -> &'a mut W {
        self.variant(TXDRDY_AW::CLEAR)
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
#[doc = "Write '1' to Disable interrupt for ENDTX event\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ENDTX_A {
    #[doc = "0: Read: Disabled"]
    DISABLED = 0,
    #[doc = "1: Read: Enabled"]
    ENABLED = 1,
}
impl From<ENDTX_A> for bool {
    #[inline(always)]
    fn from(variant: ENDTX_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `ENDTX`"]
pub type ENDTX_R = crate::R<bool, ENDTX_A>;
impl ENDTX_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ENDTX_A {
        match self.bits {
            false => ENDTX_A::DISABLED,
            true => ENDTX_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == ENDTX_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == ENDTX_A::ENABLED
    }
}
#[doc = "Write '1' to Disable interrupt for ENDTX event\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ENDTX_AW {
    #[doc = "1: Disable"]
    CLEAR = 1,
}
impl From<ENDTX_AW> for bool {
    #[inline(always)]
    fn from(variant: ENDTX_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Write proxy for field `ENDTX`"]
pub struct ENDTX_W<'a> {
    w: &'a mut W,
}
impl<'a> ENDTX_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: ENDTX_AW) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Disable"]
    #[inline(always)]
    pub fn clear(self) -> &'a mut W {
        self.variant(ENDTX_AW::CLEAR)
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
#[doc = "Write '1' to Disable interrupt for ERROR event\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ERROR_A {
    #[doc = "0: Read: Disabled"]
    DISABLED = 0,
    #[doc = "1: Read: Enabled"]
    ENABLED = 1,
}
impl From<ERROR_A> for bool {
    #[inline(always)]
    fn from(variant: ERROR_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `ERROR`"]
pub type ERROR_R = crate::R<bool, ERROR_A>;
impl ERROR_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ERROR_A {
        match self.bits {
            false => ERROR_A::DISABLED,
            true => ERROR_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == ERROR_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == ERROR_A::ENABLED
    }
}
#[doc = "Write '1' to Disable interrupt for ERROR event\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ERROR_AW {
    #[doc = "1: Disable"]
    CLEAR = 1,
}
impl From<ERROR_AW> for bool {
    #[inline(always)]
    fn from(variant: ERROR_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Write proxy for field `ERROR`"]
pub struct ERROR_W<'a> {
    w: &'a mut W,
}
impl<'a> ERROR_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: ERROR_AW) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Disable"]
    #[inline(always)]
    pub fn clear(self) -> &'a mut W {
        self.variant(ERROR_AW::CLEAR)
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
#[doc = "Write '1' to Disable interrupt for RXTO event\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RXTO_A {
    #[doc = "0: Read: Disabled"]
    DISABLED = 0,
    #[doc = "1: Read: Enabled"]
    ENABLED = 1,
}
impl From<RXTO_A> for bool {
    #[inline(always)]
    fn from(variant: RXTO_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `RXTO`"]
pub type RXTO_R = crate::R<bool, RXTO_A>;
impl RXTO_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RXTO_A {
        match self.bits {
            false => RXTO_A::DISABLED,
            true => RXTO_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == RXTO_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == RXTO_A::ENABLED
    }
}
#[doc = "Write '1' to Disable interrupt for RXTO event\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RXTO_AW {
    #[doc = "1: Disable"]
    CLEAR = 1,
}
impl From<RXTO_AW> for bool {
    #[inline(always)]
    fn from(variant: RXTO_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Write proxy for field `RXTO`"]
pub struct RXTO_W<'a> {
    w: &'a mut W,
}
impl<'a> RXTO_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: RXTO_AW) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Disable"]
    #[inline(always)]
    pub fn clear(self) -> &'a mut W {
        self.variant(RXTO_AW::CLEAR)
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
#[doc = "Write '1' to Disable interrupt for RXSTARTED event\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RXSTARTED_A {
    #[doc = "0: Read: Disabled"]
    DISABLED = 0,
    #[doc = "1: Read: Enabled"]
    ENABLED = 1,
}
impl From<RXSTARTED_A> for bool {
    #[inline(always)]
    fn from(variant: RXSTARTED_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `RXSTARTED`"]
pub type RXSTARTED_R = crate::R<bool, RXSTARTED_A>;
impl RXSTARTED_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RXSTARTED_A {
        match self.bits {
            false => RXSTARTED_A::DISABLED,
            true => RXSTARTED_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == RXSTARTED_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == RXSTARTED_A::ENABLED
    }
}
#[doc = "Write '1' to Disable interrupt for RXSTARTED event\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RXSTARTED_AW {
    #[doc = "1: Disable"]
    CLEAR = 1,
}
impl From<RXSTARTED_AW> for bool {
    #[inline(always)]
    fn from(variant: RXSTARTED_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Write proxy for field `RXSTARTED`"]
pub struct RXSTARTED_W<'a> {
    w: &'a mut W,
}
impl<'a> RXSTARTED_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: RXSTARTED_AW) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Disable"]
    #[inline(always)]
    pub fn clear(self) -> &'a mut W {
        self.variant(RXSTARTED_AW::CLEAR)
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
#[doc = "Write '1' to Disable interrupt for TXSTARTED event\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TXSTARTED_A {
    #[doc = "0: Read: Disabled"]
    DISABLED = 0,
    #[doc = "1: Read: Enabled"]
    ENABLED = 1,
}
impl From<TXSTARTED_A> for bool {
    #[inline(always)]
    fn from(variant: TXSTARTED_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `TXSTARTED`"]
pub type TXSTARTED_R = crate::R<bool, TXSTARTED_A>;
impl TXSTARTED_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TXSTARTED_A {
        match self.bits {
            false => TXSTARTED_A::DISABLED,
            true => TXSTARTED_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == TXSTARTED_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == TXSTARTED_A::ENABLED
    }
}
#[doc = "Write '1' to Disable interrupt for TXSTARTED event\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TXSTARTED_AW {
    #[doc = "1: Disable"]
    CLEAR = 1,
}
impl From<TXSTARTED_AW> for bool {
    #[inline(always)]
    fn from(variant: TXSTARTED_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Write proxy for field `TXSTARTED`"]
pub struct TXSTARTED_W<'a> {
    w: &'a mut W,
}
impl<'a> TXSTARTED_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: TXSTARTED_AW) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Disable"]
    #[inline(always)]
    pub fn clear(self) -> &'a mut W {
        self.variant(TXSTARTED_AW::CLEAR)
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
#[doc = "Write '1' to Disable interrupt for TXSTOPPED event\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TXSTOPPED_A {
    #[doc = "0: Read: Disabled"]
    DISABLED = 0,
    #[doc = "1: Read: Enabled"]
    ENABLED = 1,
}
impl From<TXSTOPPED_A> for bool {
    #[inline(always)]
    fn from(variant: TXSTOPPED_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `TXSTOPPED`"]
pub type TXSTOPPED_R = crate::R<bool, TXSTOPPED_A>;
impl TXSTOPPED_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TXSTOPPED_A {
        match self.bits {
            false => TXSTOPPED_A::DISABLED,
            true => TXSTOPPED_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == TXSTOPPED_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == TXSTOPPED_A::ENABLED
    }
}
#[doc = "Write '1' to Disable interrupt for TXSTOPPED event\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TXSTOPPED_AW {
    #[doc = "1: Disable"]
    CLEAR = 1,
}
impl From<TXSTOPPED_AW> for bool {
    #[inline(always)]
    fn from(variant: TXSTOPPED_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Write proxy for field `TXSTOPPED`"]
pub struct TXSTOPPED_W<'a> {
    w: &'a mut W,
}
impl<'a> TXSTOPPED_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: TXSTOPPED_AW) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Disable"]
    #[inline(always)]
    pub fn clear(self) -> &'a mut W {
        self.variant(TXSTOPPED_AW::CLEAR)
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
impl R {
    #[doc = "Bit 0 - Write '1' to Disable interrupt for CTS event"]
    #[inline(always)]
    pub fn cts(&self) -> CTS_R {
        CTS_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - Write '1' to Disable interrupt for NCTS event"]
    #[inline(always)]
    pub fn ncts(&self) -> NCTS_R {
        NCTS_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - Write '1' to Disable interrupt for RXDRDY event"]
    #[inline(always)]
    pub fn rxdrdy(&self) -> RXDRDY_R {
        RXDRDY_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 4 - Write '1' to Disable interrupt for ENDRX event"]
    #[inline(always)]
    pub fn endrx(&self) -> ENDRX_R {
        ENDRX_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 7 - Write '1' to Disable interrupt for TXDRDY event"]
    #[inline(always)]
    pub fn txdrdy(&self) -> TXDRDY_R {
        TXDRDY_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 8 - Write '1' to Disable interrupt for ENDTX event"]
    #[inline(always)]
    pub fn endtx(&self) -> ENDTX_R {
        ENDTX_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 9 - Write '1' to Disable interrupt for ERROR event"]
    #[inline(always)]
    pub fn error(&self) -> ERROR_R {
        ERROR_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 17 - Write '1' to Disable interrupt for RXTO event"]
    #[inline(always)]
    pub fn rxto(&self) -> RXTO_R {
        RXTO_R::new(((self.bits >> 17) & 0x01) != 0)
    }
    #[doc = "Bit 19 - Write '1' to Disable interrupt for RXSTARTED event"]
    #[inline(always)]
    pub fn rxstarted(&self) -> RXSTARTED_R {
        RXSTARTED_R::new(((self.bits >> 19) & 0x01) != 0)
    }
    #[doc = "Bit 20 - Write '1' to Disable interrupt for TXSTARTED event"]
    #[inline(always)]
    pub fn txstarted(&self) -> TXSTARTED_R {
        TXSTARTED_R::new(((self.bits >> 20) & 0x01) != 0)
    }
    #[doc = "Bit 22 - Write '1' to Disable interrupt for TXSTOPPED event"]
    #[inline(always)]
    pub fn txstopped(&self) -> TXSTOPPED_R {
        TXSTOPPED_R::new(((self.bits >> 22) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Write '1' to Disable interrupt for CTS event"]
    #[inline(always)]
    pub fn cts(&mut self) -> CTS_W {
        CTS_W { w: self }
    }
    #[doc = "Bit 1 - Write '1' to Disable interrupt for NCTS event"]
    #[inline(always)]
    pub fn ncts(&mut self) -> NCTS_W {
        NCTS_W { w: self }
    }
    #[doc = "Bit 2 - Write '1' to Disable interrupt for RXDRDY event"]
    #[inline(always)]
    pub fn rxdrdy(&mut self) -> RXDRDY_W {
        RXDRDY_W { w: self }
    }
    #[doc = "Bit 4 - Write '1' to Disable interrupt for ENDRX event"]
    #[inline(always)]
    pub fn endrx(&mut self) -> ENDRX_W {
        ENDRX_W { w: self }
    }
    #[doc = "Bit 7 - Write '1' to Disable interrupt for TXDRDY event"]
    #[inline(always)]
    pub fn txdrdy(&mut self) -> TXDRDY_W {
        TXDRDY_W { w: self }
    }
    #[doc = "Bit 8 - Write '1' to Disable interrupt for ENDTX event"]
    #[inline(always)]
    pub fn endtx(&mut self) -> ENDTX_W {
        ENDTX_W { w: self }
    }
    #[doc = "Bit 9 - Write '1' to Disable interrupt for ERROR event"]
    #[inline(always)]
    pub fn error(&mut self) -> ERROR_W {
        ERROR_W { w: self }
    }
    #[doc = "Bit 17 - Write '1' to Disable interrupt for RXTO event"]
    #[inline(always)]
    pub fn rxto(&mut self) -> RXTO_W {
        RXTO_W { w: self }
    }
    #[doc = "Bit 19 - Write '1' to Disable interrupt for RXSTARTED event"]
    #[inline(always)]
    pub fn rxstarted(&mut self) -> RXSTARTED_W {
        RXSTARTED_W { w: self }
    }
    #[doc = "Bit 20 - Write '1' to Disable interrupt for TXSTARTED event"]
    #[inline(always)]
    pub fn txstarted(&mut self) -> TXSTARTED_W {
        TXSTARTED_W { w: self }
    }
    #[doc = "Bit 22 - Write '1' to Disable interrupt for TXSTOPPED event"]
    #[inline(always)]
    pub fn txstopped(&mut self) -> TXSTOPPED_W {
        TXSTOPPED_W { w: self }
    }
}
