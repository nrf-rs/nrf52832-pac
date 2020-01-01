#[doc = "Reader of register RAMON"]
pub type R = crate::R<u32, super::RAMON>;
#[doc = "Writer for register RAMON"]
pub type W = crate::W<u32, super::RAMON>;
#[doc = "Register RAMON `reset()`'s with value 0x03"]
impl crate::ResetValue for super::RAMON {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0x03
    }
}
#[doc = "Keep RAM block 0 on or off in system ON Mode\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ONRAM0_A {
    #[doc = "0: Off"]
    RAM0OFF = 0,
    #[doc = "1: On"]
    RAM0ON = 1,
}
impl From<ONRAM0_A> for bool {
    #[inline(always)]
    fn from(variant: ONRAM0_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `ONRAM0`"]
pub type ONRAM0_R = crate::R<bool, ONRAM0_A>;
impl ONRAM0_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ONRAM0_A {
        match self.bits {
            false => ONRAM0_A::RAM0OFF,
            true => ONRAM0_A::RAM0ON,
        }
    }
    #[doc = "Checks if the value of the field is `RAM0OFF`"]
    #[inline(always)]
    pub fn is_ram0off(&self) -> bool {
        *self == ONRAM0_A::RAM0OFF
    }
    #[doc = "Checks if the value of the field is `RAM0ON`"]
    #[inline(always)]
    pub fn is_ram0on(&self) -> bool {
        *self == ONRAM0_A::RAM0ON
    }
}
#[doc = "Write proxy for field `ONRAM0`"]
pub struct ONRAM0_W<'a> {
    w: &'a mut W,
}
impl<'a> ONRAM0_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: ONRAM0_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Off"]
    #[inline(always)]
    pub fn ram0off(self) -> &'a mut W {
        self.variant(ONRAM0_A::RAM0OFF)
    }
    #[doc = "On"]
    #[inline(always)]
    pub fn ram0on(self) -> &'a mut W {
        self.variant(ONRAM0_A::RAM0ON)
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
#[doc = "Keep RAM block 1 on or off in system ON Mode\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ONRAM1_A {
    #[doc = "0: Off"]
    RAM1OFF = 0,
    #[doc = "1: On"]
    RAM1ON = 1,
}
impl From<ONRAM1_A> for bool {
    #[inline(always)]
    fn from(variant: ONRAM1_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `ONRAM1`"]
pub type ONRAM1_R = crate::R<bool, ONRAM1_A>;
impl ONRAM1_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ONRAM1_A {
        match self.bits {
            false => ONRAM1_A::RAM1OFF,
            true => ONRAM1_A::RAM1ON,
        }
    }
    #[doc = "Checks if the value of the field is `RAM1OFF`"]
    #[inline(always)]
    pub fn is_ram1off(&self) -> bool {
        *self == ONRAM1_A::RAM1OFF
    }
    #[doc = "Checks if the value of the field is `RAM1ON`"]
    #[inline(always)]
    pub fn is_ram1on(&self) -> bool {
        *self == ONRAM1_A::RAM1ON
    }
}
#[doc = "Write proxy for field `ONRAM1`"]
pub struct ONRAM1_W<'a> {
    w: &'a mut W,
}
impl<'a> ONRAM1_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: ONRAM1_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Off"]
    #[inline(always)]
    pub fn ram1off(self) -> &'a mut W {
        self.variant(ONRAM1_A::RAM1OFF)
    }
    #[doc = "On"]
    #[inline(always)]
    pub fn ram1on(self) -> &'a mut W {
        self.variant(ONRAM1_A::RAM1ON)
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
#[doc = "Keep retention on RAM block 0 when RAM block is switched off\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum OFFRAM0_A {
    #[doc = "0: Off"]
    RAM0OFF = 0,
    #[doc = "1: On"]
    RAM0ON = 1,
}
impl From<OFFRAM0_A> for bool {
    #[inline(always)]
    fn from(variant: OFFRAM0_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `OFFRAM0`"]
pub type OFFRAM0_R = crate::R<bool, OFFRAM0_A>;
impl OFFRAM0_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> OFFRAM0_A {
        match self.bits {
            false => OFFRAM0_A::RAM0OFF,
            true => OFFRAM0_A::RAM0ON,
        }
    }
    #[doc = "Checks if the value of the field is `RAM0OFF`"]
    #[inline(always)]
    pub fn is_ram0off(&self) -> bool {
        *self == OFFRAM0_A::RAM0OFF
    }
    #[doc = "Checks if the value of the field is `RAM0ON`"]
    #[inline(always)]
    pub fn is_ram0on(&self) -> bool {
        *self == OFFRAM0_A::RAM0ON
    }
}
#[doc = "Write proxy for field `OFFRAM0`"]
pub struct OFFRAM0_W<'a> {
    w: &'a mut W,
}
impl<'a> OFFRAM0_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: OFFRAM0_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Off"]
    #[inline(always)]
    pub fn ram0off(self) -> &'a mut W {
        self.variant(OFFRAM0_A::RAM0OFF)
    }
    #[doc = "On"]
    #[inline(always)]
    pub fn ram0on(self) -> &'a mut W {
        self.variant(OFFRAM0_A::RAM0ON)
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
#[doc = "Keep retention on RAM block 1 when RAM block is switched off\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum OFFRAM1_A {
    #[doc = "0: Off"]
    RAM1OFF = 0,
    #[doc = "1: On"]
    RAM1ON = 1,
}
impl From<OFFRAM1_A> for bool {
    #[inline(always)]
    fn from(variant: OFFRAM1_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `OFFRAM1`"]
pub type OFFRAM1_R = crate::R<bool, OFFRAM1_A>;
impl OFFRAM1_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> OFFRAM1_A {
        match self.bits {
            false => OFFRAM1_A::RAM1OFF,
            true => OFFRAM1_A::RAM1ON,
        }
    }
    #[doc = "Checks if the value of the field is `RAM1OFF`"]
    #[inline(always)]
    pub fn is_ram1off(&self) -> bool {
        *self == OFFRAM1_A::RAM1OFF
    }
    #[doc = "Checks if the value of the field is `RAM1ON`"]
    #[inline(always)]
    pub fn is_ram1on(&self) -> bool {
        *self == OFFRAM1_A::RAM1ON
    }
}
#[doc = "Write proxy for field `OFFRAM1`"]
pub struct OFFRAM1_W<'a> {
    w: &'a mut W,
}
impl<'a> OFFRAM1_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: OFFRAM1_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Off"]
    #[inline(always)]
    pub fn ram1off(self) -> &'a mut W {
        self.variant(OFFRAM1_A::RAM1OFF)
    }
    #[doc = "On"]
    #[inline(always)]
    pub fn ram1on(self) -> &'a mut W {
        self.variant(OFFRAM1_A::RAM1ON)
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
impl R {
    #[doc = "Bit 0 - Keep RAM block 0 on or off in system ON Mode"]
    #[inline(always)]
    pub fn onram0(&self) -> ONRAM0_R {
        ONRAM0_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - Keep RAM block 1 on or off in system ON Mode"]
    #[inline(always)]
    pub fn onram1(&self) -> ONRAM1_R {
        ONRAM1_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 16 - Keep retention on RAM block 0 when RAM block is switched off"]
    #[inline(always)]
    pub fn offram0(&self) -> OFFRAM0_R {
        OFFRAM0_R::new(((self.bits >> 16) & 0x01) != 0)
    }
    #[doc = "Bit 17 - Keep retention on RAM block 1 when RAM block is switched off"]
    #[inline(always)]
    pub fn offram1(&self) -> OFFRAM1_R {
        OFFRAM1_R::new(((self.bits >> 17) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Keep RAM block 0 on or off in system ON Mode"]
    #[inline(always)]
    pub fn onram0(&mut self) -> ONRAM0_W {
        ONRAM0_W { w: self }
    }
    #[doc = "Bit 1 - Keep RAM block 1 on or off in system ON Mode"]
    #[inline(always)]
    pub fn onram1(&mut self) -> ONRAM1_W {
        ONRAM1_W { w: self }
    }
    #[doc = "Bit 16 - Keep retention on RAM block 0 when RAM block is switched off"]
    #[inline(always)]
    pub fn offram0(&mut self) -> OFFRAM0_W {
        OFFRAM0_W { w: self }
    }
    #[doc = "Bit 17 - Keep retention on RAM block 1 when RAM block is switched off"]
    #[inline(always)]
    pub fn offram1(&mut self) -> OFFRAM1_W {
        OFFRAM1_W { w: self }
    }
}
