#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::POWERCLR {
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
}
#[doc = "Values that can be written to the field `S0POWER`"]
pub enum S0POWERW {
    #[doc = "Off"]
    OFF,
}
impl S0POWERW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            S0POWERW::OFF => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _S0POWERW<'a> {
    w: &'a mut W,
}
impl<'a> _S0POWERW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: S0POWERW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Off"]
    #[inline]
    pub fn off(self) -> &'a mut W {
        self.variant(S0POWERW::OFF)
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
#[doc = "Values that can be written to the field `S1POWER`"]
pub enum S1POWERW {
    #[doc = "Off"]
    OFF,
}
impl S1POWERW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            S1POWERW::OFF => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _S1POWERW<'a> {
    w: &'a mut W,
}
impl<'a> _S1POWERW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: S1POWERW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Off"]
    #[inline]
    pub fn off(self) -> &'a mut W {
        self.variant(S1POWERW::OFF)
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
#[doc = "Values that can be written to the field `S2POWER`"]
pub enum S2POWERW {
    #[doc = "Off"]
    OFF,
}
impl S2POWERW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            S2POWERW::OFF => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _S2POWERW<'a> {
    w: &'a mut W,
}
impl<'a> _S2POWERW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: S2POWERW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Off"]
    #[inline]
    pub fn off(self) -> &'a mut W {
        self.variant(S2POWERW::OFF)
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
#[doc = "Values that can be written to the field `S3POWER`"]
pub enum S3POWERW {
    #[doc = "Off"]
    OFF,
}
impl S3POWERW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            S3POWERW::OFF => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _S3POWERW<'a> {
    w: &'a mut W,
}
impl<'a> _S3POWERW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: S3POWERW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Off"]
    #[inline]
    pub fn off(self) -> &'a mut W {
        self.variant(S3POWERW::OFF)
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
#[doc = "Values that can be written to the field `S4POWER`"]
pub enum S4POWERW {
    #[doc = "Off"]
    OFF,
}
impl S4POWERW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            S4POWERW::OFF => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _S4POWERW<'a> {
    w: &'a mut W,
}
impl<'a> _S4POWERW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: S4POWERW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Off"]
    #[inline]
    pub fn off(self) -> &'a mut W {
        self.variant(S4POWERW::OFF)
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
#[doc = "Values that can be written to the field `S5POWER`"]
pub enum S5POWERW {
    #[doc = "Off"]
    OFF,
}
impl S5POWERW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            S5POWERW::OFF => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _S5POWERW<'a> {
    w: &'a mut W,
}
impl<'a> _S5POWERW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: S5POWERW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Off"]
    #[inline]
    pub fn off(self) -> &'a mut W {
        self.variant(S5POWERW::OFF)
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
#[doc = "Values that can be written to the field `S6POWER`"]
pub enum S6POWERW {
    #[doc = "Off"]
    OFF,
}
impl S6POWERW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            S6POWERW::OFF => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _S6POWERW<'a> {
    w: &'a mut W,
}
impl<'a> _S6POWERW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: S6POWERW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Off"]
    #[inline]
    pub fn off(self) -> &'a mut W {
        self.variant(S6POWERW::OFF)
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
#[doc = "Values that can be written to the field `S7POWER`"]
pub enum S7POWERW {
    #[doc = "Off"]
    OFF,
}
impl S7POWERW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            S7POWERW::OFF => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _S7POWERW<'a> {
    w: &'a mut W,
}
impl<'a> _S7POWERW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: S7POWERW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Off"]
    #[inline]
    pub fn off(self) -> &'a mut W {
        self.variant(S7POWERW::OFF)
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
#[doc = "Values that can be written to the field `S8POWER`"]
pub enum S8POWERW {
    #[doc = "Off"]
    OFF,
}
impl S8POWERW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            S8POWERW::OFF => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _S8POWERW<'a> {
    w: &'a mut W,
}
impl<'a> _S8POWERW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: S8POWERW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Off"]
    #[inline]
    pub fn off(self) -> &'a mut W {
        self.variant(S8POWERW::OFF)
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
#[doc = "Values that can be written to the field `S9POWER`"]
pub enum S9POWERW {
    #[doc = "Off"]
    OFF,
}
impl S9POWERW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            S9POWERW::OFF => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _S9POWERW<'a> {
    w: &'a mut W,
}
impl<'a> _S9POWERW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: S9POWERW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Off"]
    #[inline]
    pub fn off(self) -> &'a mut W {
        self.variant(S9POWERW::OFF)
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
#[doc = "Values that can be written to the field `S10POWER`"]
pub enum S10POWERW {
    #[doc = "Off"]
    OFF,
}
impl S10POWERW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            S10POWERW::OFF => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _S10POWERW<'a> {
    w: &'a mut W,
}
impl<'a> _S10POWERW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: S10POWERW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Off"]
    #[inline]
    pub fn off(self) -> &'a mut W {
        self.variant(S10POWERW::OFF)
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
#[doc = "Values that can be written to the field `S11POWER`"]
pub enum S11POWERW {
    #[doc = "Off"]
    OFF,
}
impl S11POWERW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            S11POWERW::OFF => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _S11POWERW<'a> {
    w: &'a mut W,
}
impl<'a> _S11POWERW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: S11POWERW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Off"]
    #[inline]
    pub fn off(self) -> &'a mut W {
        self.variant(S11POWERW::OFF)
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
#[doc = "Values that can be written to the field `S12POWER`"]
pub enum S12POWERW {
    #[doc = "Off"]
    OFF,
}
impl S12POWERW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            S12POWERW::OFF => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _S12POWERW<'a> {
    w: &'a mut W,
}
impl<'a> _S12POWERW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: S12POWERW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Off"]
    #[inline]
    pub fn off(self) -> &'a mut W {
        self.variant(S12POWERW::OFF)
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
#[doc = "Values that can be written to the field `S13POWER`"]
pub enum S13POWERW {
    #[doc = "Off"]
    OFF,
}
impl S13POWERW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            S13POWERW::OFF => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _S13POWERW<'a> {
    w: &'a mut W,
}
impl<'a> _S13POWERW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: S13POWERW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Off"]
    #[inline]
    pub fn off(self) -> &'a mut W {
        self.variant(S13POWERW::OFF)
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
#[doc = "Values that can be written to the field `S14POWER`"]
pub enum S14POWERW {
    #[doc = "Off"]
    OFF,
}
impl S14POWERW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            S14POWERW::OFF => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _S14POWERW<'a> {
    w: &'a mut W,
}
impl<'a> _S14POWERW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: S14POWERW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Off"]
    #[inline]
    pub fn off(self) -> &'a mut W {
        self.variant(S14POWERW::OFF)
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
#[doc = "Values that can be written to the field `S15POWER`"]
pub enum S15POWERW {
    #[doc = "Off"]
    OFF,
}
impl S15POWERW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            S15POWERW::OFF => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _S15POWERW<'a> {
    w: &'a mut W,
}
impl<'a> _S15POWERW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: S15POWERW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Off"]
    #[inline]
    pub fn off(self) -> &'a mut W {
        self.variant(S15POWERW::OFF)
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
#[doc = "Values that can be written to the field `S0RETENTION`"]
pub enum S0RETENTIONW {
    #[doc = "Off"]
    OFF,
}
impl S0RETENTIONW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            S0RETENTIONW::OFF => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _S0RETENTIONW<'a> {
    w: &'a mut W,
}
impl<'a> _S0RETENTIONW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: S0RETENTIONW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Off"]
    #[inline]
    pub fn off(self) -> &'a mut W {
        self.variant(S0RETENTIONW::OFF)
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
#[doc = "Values that can be written to the field `S1RETENTION`"]
pub enum S1RETENTIONW {
    #[doc = "Off"]
    OFF,
}
impl S1RETENTIONW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            S1RETENTIONW::OFF => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _S1RETENTIONW<'a> {
    w: &'a mut W,
}
impl<'a> _S1RETENTIONW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: S1RETENTIONW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Off"]
    #[inline]
    pub fn off(self) -> &'a mut W {
        self.variant(S1RETENTIONW::OFF)
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
#[doc = "Values that can be written to the field `S2RETENTION`"]
pub enum S2RETENTIONW {
    #[doc = "Off"]
    OFF,
}
impl S2RETENTIONW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            S2RETENTIONW::OFF => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _S2RETENTIONW<'a> {
    w: &'a mut W,
}
impl<'a> _S2RETENTIONW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: S2RETENTIONW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Off"]
    #[inline]
    pub fn off(self) -> &'a mut W {
        self.variant(S2RETENTIONW::OFF)
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
#[doc = "Values that can be written to the field `S3RETENTION`"]
pub enum S3RETENTIONW {
    #[doc = "Off"]
    OFF,
}
impl S3RETENTIONW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            S3RETENTIONW::OFF => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _S3RETENTIONW<'a> {
    w: &'a mut W,
}
impl<'a> _S3RETENTIONW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: S3RETENTIONW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Off"]
    #[inline]
    pub fn off(self) -> &'a mut W {
        self.variant(S3RETENTIONW::OFF)
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
#[doc = "Values that can be written to the field `S4RETENTION`"]
pub enum S4RETENTIONW {
    #[doc = "Off"]
    OFF,
}
impl S4RETENTIONW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            S4RETENTIONW::OFF => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _S4RETENTIONW<'a> {
    w: &'a mut W,
}
impl<'a> _S4RETENTIONW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: S4RETENTIONW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Off"]
    #[inline]
    pub fn off(self) -> &'a mut W {
        self.variant(S4RETENTIONW::OFF)
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
#[doc = "Values that can be written to the field `S5RETENTION`"]
pub enum S5RETENTIONW {
    #[doc = "Off"]
    OFF,
}
impl S5RETENTIONW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            S5RETENTIONW::OFF => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _S5RETENTIONW<'a> {
    w: &'a mut W,
}
impl<'a> _S5RETENTIONW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: S5RETENTIONW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Off"]
    #[inline]
    pub fn off(self) -> &'a mut W {
        self.variant(S5RETENTIONW::OFF)
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
#[doc = "Values that can be written to the field `S6RETENTION`"]
pub enum S6RETENTIONW {
    #[doc = "Off"]
    OFF,
}
impl S6RETENTIONW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            S6RETENTIONW::OFF => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _S6RETENTIONW<'a> {
    w: &'a mut W,
}
impl<'a> _S6RETENTIONW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: S6RETENTIONW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Off"]
    #[inline]
    pub fn off(self) -> &'a mut W {
        self.variant(S6RETENTIONW::OFF)
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
#[doc = "Values that can be written to the field `S7RETENTION`"]
pub enum S7RETENTIONW {
    #[doc = "Off"]
    OFF,
}
impl S7RETENTIONW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            S7RETENTIONW::OFF => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _S7RETENTIONW<'a> {
    w: &'a mut W,
}
impl<'a> _S7RETENTIONW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: S7RETENTIONW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Off"]
    #[inline]
    pub fn off(self) -> &'a mut W {
        self.variant(S7RETENTIONW::OFF)
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
#[doc = "Values that can be written to the field `S8RETENTION`"]
pub enum S8RETENTIONW {
    #[doc = "Off"]
    OFF,
}
impl S8RETENTIONW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            S8RETENTIONW::OFF => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _S8RETENTIONW<'a> {
    w: &'a mut W,
}
impl<'a> _S8RETENTIONW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: S8RETENTIONW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Off"]
    #[inline]
    pub fn off(self) -> &'a mut W {
        self.variant(S8RETENTIONW::OFF)
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
#[doc = "Values that can be written to the field `S9RETENTION`"]
pub enum S9RETENTIONW {
    #[doc = "Off"]
    OFF,
}
impl S9RETENTIONW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            S9RETENTIONW::OFF => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _S9RETENTIONW<'a> {
    w: &'a mut W,
}
impl<'a> _S9RETENTIONW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: S9RETENTIONW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Off"]
    #[inline]
    pub fn off(self) -> &'a mut W {
        self.variant(S9RETENTIONW::OFF)
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
#[doc = "Values that can be written to the field `S10RETENTION`"]
pub enum S10RETENTIONW {
    #[doc = "Off"]
    OFF,
}
impl S10RETENTIONW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            S10RETENTIONW::OFF => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _S10RETENTIONW<'a> {
    w: &'a mut W,
}
impl<'a> _S10RETENTIONW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: S10RETENTIONW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Off"]
    #[inline]
    pub fn off(self) -> &'a mut W {
        self.variant(S10RETENTIONW::OFF)
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
#[doc = "Values that can be written to the field `S11RETENTION`"]
pub enum S11RETENTIONW {
    #[doc = "Off"]
    OFF,
}
impl S11RETENTIONW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            S11RETENTIONW::OFF => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _S11RETENTIONW<'a> {
    w: &'a mut W,
}
impl<'a> _S11RETENTIONW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: S11RETENTIONW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Off"]
    #[inline]
    pub fn off(self) -> &'a mut W {
        self.variant(S11RETENTIONW::OFF)
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
#[doc = "Values that can be written to the field `S12RETENTION`"]
pub enum S12RETENTIONW {
    #[doc = "Off"]
    OFF,
}
impl S12RETENTIONW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            S12RETENTIONW::OFF => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _S12RETENTIONW<'a> {
    w: &'a mut W,
}
impl<'a> _S12RETENTIONW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: S12RETENTIONW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Off"]
    #[inline]
    pub fn off(self) -> &'a mut W {
        self.variant(S12RETENTIONW::OFF)
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
#[doc = "Values that can be written to the field `S13RETENTION`"]
pub enum S13RETENTIONW {
    #[doc = "Off"]
    OFF,
}
impl S13RETENTIONW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            S13RETENTIONW::OFF => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _S13RETENTIONW<'a> {
    w: &'a mut W,
}
impl<'a> _S13RETENTIONW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: S13RETENTIONW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Off"]
    #[inline]
    pub fn off(self) -> &'a mut W {
        self.variant(S13RETENTIONW::OFF)
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
#[doc = "Values that can be written to the field `S14RETENTION`"]
pub enum S14RETENTIONW {
    #[doc = "Off"]
    OFF,
}
impl S14RETENTIONW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            S14RETENTIONW::OFF => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _S14RETENTIONW<'a> {
    w: &'a mut W,
}
impl<'a> _S14RETENTIONW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: S14RETENTIONW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Off"]
    #[inline]
    pub fn off(self) -> &'a mut W {
        self.variant(S14RETENTIONW::OFF)
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
#[doc = "Values that can be written to the field `S15RETENTION`"]
pub enum S15RETENTIONW {
    #[doc = "Off"]
    OFF,
}
impl S15RETENTIONW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            S15RETENTIONW::OFF => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _S15RETENTIONW<'a> {
    w: &'a mut W,
}
impl<'a> _S15RETENTIONW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: S15RETENTIONW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Off"]
    #[inline]
    pub fn off(self) -> &'a mut W {
        self.variant(S15RETENTIONW::OFF)
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
impl W {
    #[doc = r" Reset value of the register"]
    #[inline]
    pub fn reset_value() -> W {
        W { bits: 65535 }
    }
    #[doc = r" Writes raw bits to the register"]
    #[inline]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
    #[doc = "Bit 0 - Keep RAM section S0 of RAMn on or off in System ON mode"]
    #[inline]
    pub fn s0power(&mut self) -> _S0POWERW {
        _S0POWERW { w: self }
    }
    #[doc = "Bit 1 - Keep RAM section S1 of RAMn on or off in System ON mode"]
    #[inline]
    pub fn s1power(&mut self) -> _S1POWERW {
        _S1POWERW { w: self }
    }
    #[doc = "Bit 2 - Keep RAM section S2 of RAMn on or off in System ON mode"]
    #[inline]
    pub fn s2power(&mut self) -> _S2POWERW {
        _S2POWERW { w: self }
    }
    #[doc = "Bit 3 - Keep RAM section S3 of RAMn on or off in System ON mode"]
    #[inline]
    pub fn s3power(&mut self) -> _S3POWERW {
        _S3POWERW { w: self }
    }
    #[doc = "Bit 4 - Keep RAM section S4 of RAMn on or off in System ON mode"]
    #[inline]
    pub fn s4power(&mut self) -> _S4POWERW {
        _S4POWERW { w: self }
    }
    #[doc = "Bit 5 - Keep RAM section S5 of RAMn on or off in System ON mode"]
    #[inline]
    pub fn s5power(&mut self) -> _S5POWERW {
        _S5POWERW { w: self }
    }
    #[doc = "Bit 6 - Keep RAM section S6 of RAMn on or off in System ON mode"]
    #[inline]
    pub fn s6power(&mut self) -> _S6POWERW {
        _S6POWERW { w: self }
    }
    #[doc = "Bit 7 - Keep RAM section S7 of RAMn on or off in System ON mode"]
    #[inline]
    pub fn s7power(&mut self) -> _S7POWERW {
        _S7POWERW { w: self }
    }
    #[doc = "Bit 8 - Keep RAM section S8 of RAMn on or off in System ON mode"]
    #[inline]
    pub fn s8power(&mut self) -> _S8POWERW {
        _S8POWERW { w: self }
    }
    #[doc = "Bit 9 - Keep RAM section S9 of RAMn on or off in System ON mode"]
    #[inline]
    pub fn s9power(&mut self) -> _S9POWERW {
        _S9POWERW { w: self }
    }
    #[doc = "Bit 10 - Keep RAM section S10 of RAMn on or off in System ON mode"]
    #[inline]
    pub fn s10power(&mut self) -> _S10POWERW {
        _S10POWERW { w: self }
    }
    #[doc = "Bit 11 - Keep RAM section S11 of RAMn on or off in System ON mode"]
    #[inline]
    pub fn s11power(&mut self) -> _S11POWERW {
        _S11POWERW { w: self }
    }
    #[doc = "Bit 12 - Keep RAM section S12 of RAMn on or off in System ON mode"]
    #[inline]
    pub fn s12power(&mut self) -> _S12POWERW {
        _S12POWERW { w: self }
    }
    #[doc = "Bit 13 - Keep RAM section S13 of RAMn on or off in System ON mode"]
    #[inline]
    pub fn s13power(&mut self) -> _S13POWERW {
        _S13POWERW { w: self }
    }
    #[doc = "Bit 14 - Keep RAM section S14 of RAMn on or off in System ON mode"]
    #[inline]
    pub fn s14power(&mut self) -> _S14POWERW {
        _S14POWERW { w: self }
    }
    #[doc = "Bit 15 - Keep RAM section S15 of RAMn on or off in System ON mode"]
    #[inline]
    pub fn s15power(&mut self) -> _S15POWERW {
        _S15POWERW { w: self }
    }
    #[doc = "Bit 16 - Keep retention on RAM section S0 when RAM section is switched off"]
    #[inline]
    pub fn s0retention(&mut self) -> _S0RETENTIONW {
        _S0RETENTIONW { w: self }
    }
    #[doc = "Bit 17 - Keep retention on RAM section S1 when RAM section is switched off"]
    #[inline]
    pub fn s1retention(&mut self) -> _S1RETENTIONW {
        _S1RETENTIONW { w: self }
    }
    #[doc = "Bit 18 - Keep retention on RAM section S2 when RAM section is switched off"]
    #[inline]
    pub fn s2retention(&mut self) -> _S2RETENTIONW {
        _S2RETENTIONW { w: self }
    }
    #[doc = "Bit 19 - Keep retention on RAM section S3 when RAM section is switched off"]
    #[inline]
    pub fn s3retention(&mut self) -> _S3RETENTIONW {
        _S3RETENTIONW { w: self }
    }
    #[doc = "Bit 20 - Keep retention on RAM section S4 when RAM section is switched off"]
    #[inline]
    pub fn s4retention(&mut self) -> _S4RETENTIONW {
        _S4RETENTIONW { w: self }
    }
    #[doc = "Bit 21 - Keep retention on RAM section S5 when RAM section is switched off"]
    #[inline]
    pub fn s5retention(&mut self) -> _S5RETENTIONW {
        _S5RETENTIONW { w: self }
    }
    #[doc = "Bit 22 - Keep retention on RAM section S6 when RAM section is switched off"]
    #[inline]
    pub fn s6retention(&mut self) -> _S6RETENTIONW {
        _S6RETENTIONW { w: self }
    }
    #[doc = "Bit 23 - Keep retention on RAM section S7 when RAM section is switched off"]
    #[inline]
    pub fn s7retention(&mut self) -> _S7RETENTIONW {
        _S7RETENTIONW { w: self }
    }
    #[doc = "Bit 24 - Keep retention on RAM section S8 when RAM section is switched off"]
    #[inline]
    pub fn s8retention(&mut self) -> _S8RETENTIONW {
        _S8RETENTIONW { w: self }
    }
    #[doc = "Bit 25 - Keep retention on RAM section S9 when RAM section is switched off"]
    #[inline]
    pub fn s9retention(&mut self) -> _S9RETENTIONW {
        _S9RETENTIONW { w: self }
    }
    #[doc = "Bit 26 - Keep retention on RAM section S10 when RAM section is switched off"]
    #[inline]
    pub fn s10retention(&mut self) -> _S10RETENTIONW {
        _S10RETENTIONW { w: self }
    }
    #[doc = "Bit 27 - Keep retention on RAM section S11 when RAM section is switched off"]
    #[inline]
    pub fn s11retention(&mut self) -> _S11RETENTIONW {
        _S11RETENTIONW { w: self }
    }
    #[doc = "Bit 28 - Keep retention on RAM section S12 when RAM section is switched off"]
    #[inline]
    pub fn s12retention(&mut self) -> _S12RETENTIONW {
        _S12RETENTIONW { w: self }
    }
    #[doc = "Bit 29 - Keep retention on RAM section S13 when RAM section is switched off"]
    #[inline]
    pub fn s13retention(&mut self) -> _S13RETENTIONW {
        _S13RETENTIONW { w: self }
    }
    #[doc = "Bit 30 - Keep retention on RAM section S14 when RAM section is switched off"]
    #[inline]
    pub fn s14retention(&mut self) -> _S14RETENTIONW {
        _S14RETENTIONW { w: self }
    }
    #[doc = "Bit 31 - Keep retention on RAM section S15 when RAM section is switched off"]
    #[inline]
    pub fn s15retention(&mut self) -> _S15RETENTIONW {
        _S15RETENTIONW { w: self }
    }
}
