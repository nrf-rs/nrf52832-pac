#[doc = "Reader of register T3"]
pub type R = crate::R<u32, super::T3>;
#[doc = "Reader of field `T`"]
pub type T_R = crate::R<u8, u8>;
impl R {
    #[doc = "Bits 0:7 - T (segment end)register."]
    #[inline(always)]
    pub fn t(&self) -> T_R {
        T_R::new((self.bits & 0xff) as u8)
    }
}
