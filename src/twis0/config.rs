#[doc = "Reader of register CONFIG"]
pub type R = crate::R<u32, super::CONFIG>;
#[doc = "Writer for register CONFIG"]
pub type W = crate::W<u32, super::CONFIG>;
#[doc = "Register CONFIG `reset()`'s with value 0x01"]
impl crate::ResetValue for super::CONFIG {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0x01
    }
}
#[doc = "Enable or disable address matching on ADDRESS\\[0\\]\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ADDRESS0_A {
    #[doc = "0: Disabled"]
    DISABLED = 0,
    #[doc = "1: Enabled"]
    ENABLED = 1,
}
impl From<ADDRESS0_A> for bool {
    #[inline(always)]
    fn from(variant: ADDRESS0_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `ADDRESS0`"]
pub type ADDRESS0_R = crate::R<bool, ADDRESS0_A>;
impl ADDRESS0_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ADDRESS0_A {
        match self.bits {
            false => ADDRESS0_A::DISABLED,
            true => ADDRESS0_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == ADDRESS0_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == ADDRESS0_A::ENABLED
    }
}
#[doc = "Write proxy for field `ADDRESS0`"]
pub struct ADDRESS0_W<'a> {
    w: &'a mut W,
}
impl<'a> ADDRESS0_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: ADDRESS0_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(ADDRESS0_A::DISABLED)
    }
    #[doc = "Enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(ADDRESS0_A::ENABLED)
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
#[doc = "Enable or disable address matching on ADDRESS\\[1\\]\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ADDRESS1_A {
    #[doc = "0: Disabled"]
    DISABLED = 0,
    #[doc = "1: Enabled"]
    ENABLED = 1,
}
impl From<ADDRESS1_A> for bool {
    #[inline(always)]
    fn from(variant: ADDRESS1_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `ADDRESS1`"]
pub type ADDRESS1_R = crate::R<bool, ADDRESS1_A>;
impl ADDRESS1_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ADDRESS1_A {
        match self.bits {
            false => ADDRESS1_A::DISABLED,
            true => ADDRESS1_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == ADDRESS1_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == ADDRESS1_A::ENABLED
    }
}
#[doc = "Write proxy for field `ADDRESS1`"]
pub struct ADDRESS1_W<'a> {
    w: &'a mut W,
}
impl<'a> ADDRESS1_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: ADDRESS1_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(ADDRESS1_A::DISABLED)
    }
    #[doc = "Enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(ADDRESS1_A::ENABLED)
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
impl R {
    #[doc = "Bit 0 - Enable or disable address matching on ADDRESS\\[0\\]"]
    #[inline(always)]
    pub fn address0(&self) -> ADDRESS0_R {
        ADDRESS0_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - Enable or disable address matching on ADDRESS\\[1\\]"]
    #[inline(always)]
    pub fn address1(&self) -> ADDRESS1_R {
        ADDRESS1_R::new(((self.bits >> 1) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Enable or disable address matching on ADDRESS\\[0\\]"]
    #[inline(always)]
    pub fn address0(&mut self) -> ADDRESS0_W {
        ADDRESS0_W { w: self }
    }
    #[doc = "Bit 1 - Enable or disable address matching on ADDRESS\\[1\\]"]
    #[inline(always)]
    pub fn address1(&mut self) -> ADDRESS1_W {
        ADDRESS1_W { w: self }
    }
}
